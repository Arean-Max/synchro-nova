use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::AppHandle;

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
static USER_SUMMONED_MENU: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "windows")]
static WATCHER_APP_HANDLE: std::sync::OnceLock<AppHandle> = std::sync::OnceLock::new();

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

#[cfg(target_os = "windows")]
pub fn evaluate_and_update(_app_handle: &AppHandle) {
    let is_fs = is_foreground_fullscreen();
    let current = get_fullscreen_state();

    if is_fs {
        if current != FullscreenState::FullscreenForeground {
            if let Ok(mut lock) = CURRENT_STATE.lock() {
                *lock = FullscreenState::FullscreenForeground;
            }
            crate::domain::color::reapply_active_color();
        }
    } else {
        if current != FullscreenState::NormalDesktop {
            if let Ok(mut lock) = CURRENT_STATE.lock() {
                *lock = FullscreenState::NormalDesktop;
            }
            crate::domain::color::reapply_active_color();
        }
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn winevent_proc(
    _hook: *mut std::ffi::c_void,
    event: u32,
    _hwnd: *mut std::ffi::c_void,
    _id_object: i32,
    _id_child: i32,
    _event_thread: u32,
    _event_time_ms: u32,
) {
    if event == crate::platform::ffi::EVENT_SYSTEM_FOREGROUND {
        if let Some(app) = WATCHER_APP_HANDLE.get() {
            evaluate_and_update(app);
        }
    }
}

pub fn spawn_fullscreen_watcher(app_handle: AppHandle, shutdown: Arc<AtomicBool>) {
    #[cfg(target_os = "windows")]
    {
        let _ = WATCHER_APP_HANDLE.set(app_handle.clone());

        std::thread::Builder::new()
            .name("synchro-fullscreen-watcher".to_string())
            .spawn(move || {
                use crate::platform::ffi::{
                    winapi, MSG, EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT,
                    WINEVENT_SKIPOWNPROCESS,
                };

                std::thread::sleep(Duration::from_millis(2000));
                evaluate_and_update(&app_handle);

                unsafe {
                    let hook = winapi::SetWinEventHook(
                        EVENT_SYSTEM_FOREGROUND,
                        EVENT_SYSTEM_FOREGROUND,
                        std::ptr::null_mut(),
                        Some(winevent_proc),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
                    );

                    if !hook.is_null() {
                        let mut msg = std::mem::zeroed::<MSG>();

                        while !shutdown.load(Ordering::Relaxed) {
                            let ret = winapi::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
                            if ret <= 0 {
                                break;
                            }

                            winapi::TranslateMessage(&msg);
                            winapi::DispatchMessageW(&msg);
                        }

                        winapi::UnhookWinEvent(hook);
                    } else {
                        while !shutdown.load(Ordering::Relaxed) {
                            std::thread::sleep(Duration::from_millis(3000));
                            evaluate_and_update(&app_handle);
                        }
                    }
                }
            })
            .ok();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = (app_handle, shutdown);
    }
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
