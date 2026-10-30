use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullscreenState {
    NormalDesktop,
    FullscreenForeground,
    MenuHidden,
    MenuVisible,
    TrayOnly,
    Transition,
    DisplayStateUnknown,
}

static CURRENT_STATE: Mutex<FullscreenState> = Mutex::new(FullscreenState::DisplayStateUnknown);
static AUTO_HIDDEN_BY_GAME: AtomicBool = AtomicBool::new(false);
static USER_SUMMONED_MENU: AtomicBool = AtomicBool::new(false);

pub fn get_fullscreen_state() -> FullscreenState {
    CURRENT_STATE
        .lock()
        .map(|s| *s)
        .unwrap_or(FullscreenState::DisplayStateUnknown)
}

pub fn set_user_summoned(summoned: bool) {
    USER_SUMMONED_MENU.store(summoned, Ordering::SeqCst);
    if summoned {
        if let Ok(mut lock) = CURRENT_STATE.lock() {
            *lock = FullscreenState::MenuVisible;
        }
    }
}

#[cfg(target_os = "windows")]
pub fn is_foreground_fullscreen() -> bool {
    use crate::platform::ffi::{winapi, MONITORINFO, RECT};

    unsafe {
        let hwnd = winapi::GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }

        let mut fg_pid: u32 = 0;
        winapi::GetWindowThreadProcessId(hwnd, &mut fg_pid);
        let my_pid = winapi::GetCurrentProcessId();
        if fg_pid == 0 || fg_pid == my_pid {
            return false;
        }

        if winapi::IsWindowVisible(hwnd) == 0 || winapi::IsIconic(hwnd) != 0 {
            return false;
        }

        // Exclude Windows shell surfaces (Taskbar, Desktop, Start Menu) from fullscreen game detection.
        let mut class_name_buf = [0u16; 256];
        let len = winapi::GetClassNameW(hwnd, class_name_buf.as_mut_ptr(), 256);
        if len > 0 {
            let class_name = String::from_utf16_lossy(&class_name_buf[..len as usize]).to_lowercase();
            if class_name == "shell_traywnd"
                || class_name == "progman"
                || class_name == "workerw"
                || class_name == "windows.ui.core.corewindow"
            {
                return false;
            }
        }

        let mut win_rect = RECT::default();
        if winapi::GetWindowRect(hwnd, &mut win_rect) == 0 {
            return false;
        }

        const MONITOR_DEFAULTTONEAREST: u32 = 2;
        let h_monitor = winapi::MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        if h_monitor.is_null() {
            return false;
        }

        let mut mi = MONITORINFO::default();
        if winapi::GetMonitorInfoW(h_monitor, &mut mi) == 0 {
            return false;
        }

        win_rect.left <= mi.rc_monitor.left
            && win_rect.top <= mi.rc_monitor.top
            && win_rect.right >= mi.rc_monitor.right
            && win_rect.bottom >= mi.rc_monitor.bottom
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_foreground_fullscreen() -> bool {
    false
}

pub fn spawn_fullscreen_watcher(app_handle: AppHandle, shutdown: Arc<AtomicBool>) {
    std::thread::Builder::new()
        .name("synchro-fullscreen-watcher".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let mut consecutive_fullscreen_ticks = 0u32;
            let mut consecutive_desktop_ticks = 0u32;

            while !shutdown.load(Ordering::Relaxed) {
                let sleep_ms = if consecutive_fullscreen_ticks >= 4 { 4000 } else { 250 };
                std::thread::sleep(Duration::from_millis(sleep_ms));

                let is_fs = is_foreground_fullscreen();

                if is_fs {
                    consecutive_fullscreen_ticks = consecutive_fullscreen_ticks.saturating_add(1);
                    consecutive_desktop_ticks = 0;
                } else {
                    consecutive_desktop_ticks = consecutive_desktop_ticks.saturating_add(1);
                    consecutive_fullscreen_ticks = 0;
                }

                let current = get_fullscreen_state();
                if is_fs && consecutive_fullscreen_ticks >= 2 {
                    if USER_SUMMONED_MENU.load(Ordering::SeqCst) {
                        continue;
                    }

                    if current != FullscreenState::MenuHidden && current != FullscreenState::FullscreenForeground {
                        if let Ok(mut lock) = CURRENT_STATE.lock() {
                            *lock = FullscreenState::Transition;
                        }

                        if app_handle.get_webview_window("main").is_some() {
                            crate::hide_main_window(&app_handle);
                            AUTO_HIDDEN_BY_GAME.store(true, Ordering::SeqCst);
                        }

                        if let Ok(mut lock) = CURRENT_STATE.lock() {
                            *lock = FullscreenState::MenuHidden;
                        }
                    }
                } else if !is_fs && consecutive_desktop_ticks >= 2 {
                    USER_SUMMONED_MENU.store(false, Ordering::SeqCst);

                    if current == FullscreenState::MenuHidden || current == FullscreenState::FullscreenForeground {
                        if let Ok(mut lock) = CURRENT_STATE.lock() {
                            *lock = FullscreenState::NormalDesktop;
                        }

                        AUTO_HIDDEN_BY_GAME.store(false, Ordering::SeqCst);
                    } else if current == FullscreenState::DisplayStateUnknown {
                        if let Ok(mut lock) = CURRENT_STATE.lock() {
                            *lock = FullscreenState::NormalDesktop;
                        }
                    }
                }
            }
        })
        .ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let mut state = FullscreenState::NormalDesktop;
        assert_eq!(state, FullscreenState::NormalDesktop);

        state = FullscreenState::Transition;
        assert_eq!(state, FullscreenState::Transition);

        state = FullscreenState::MenuHidden;
        assert_eq!(state, FullscreenState::MenuHidden);
    }
}
