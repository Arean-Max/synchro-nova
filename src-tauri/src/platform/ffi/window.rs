#![allow(dead_code)]

#[cfg(target_os = "windows")]
use std::ffi::c_void;
use super::types::*;
use super::winapi;

#[cfg(target_os = "windows")]
type SubclassProc = unsafe extern "system" fn(isize, u32, usize, isize, usize, usize) -> isize;
#[cfg(target_os = "windows")]
type FnSetWindowSubclass = unsafe extern "system" fn(isize, SubclassProc, usize, usize) -> i32;
#[cfg(target_os = "windows")]
type FnDefSubclassProc = unsafe extern "system" fn(isize, u32, usize, isize) -> isize;

#[cfg(target_os = "windows")]
static SUBCLASS_FNS: std::sync::OnceLock<Option<(FnSetWindowSubclass, FnDefSubclassProc)>> = std::sync::OnceLock::new();

#[cfg(target_os = "windows")]
pub fn eliminate_window_borders(hwnd: isize) {
    if hwnd == 0 {
        return;
    }

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: isize,
            dw_attribute: u32,
            pv_attribute: *const std::ffi::c_void,
            cb_attribute: u32,
        ) -> i32;
    }

    unsafe {
        // 1. Force dark mode on window frame (attribute 20: DWMWA_USE_IMMERSIVE_DARK_MODE)
        let dark_mode: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            20,
            &dark_mode as *const _ as _,
            std::mem::size_of::<i32>() as u32,
        );

        // 2. Suppress DWM border rendering on Windows 11 (attribute 34: DWMWA_BORDER_COLOR = DWMWA_COLOR_NONE 0xFFFFFFFE)
        let no_border: u32 = 0xFFFF_FFFE;
        let _ = DwmSetWindowAttribute(
            hwnd,
            34,
            &no_border as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );

        // 3. Caption / title background color matching app background (#141414 -> 0x00141414)
        let caption_color: u32 = 0x0014_1414;
        let _ = DwmSetWindowAttribute(
            hwnd,
            35,
            &caption_color as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );

        // 4. Subclass window to prevent edge resize dragging without stripping WS_THICKFRAME
        let fns = SUBCLASS_FNS.get_or_init(|| {
            #[link(name = "kernel32")]
            extern "system" {
                fn LoadLibraryA(lpLibFileName: *const u8) -> *mut std::ffi::c_void;
                fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const u8) -> *mut std::ffi::c_void;
            }

            let mod_handle = LoadLibraryA(b"comctl32.dll\0".as_ptr());
            if mod_handle.is_null() {
                return None;
            }
            let p_set = GetProcAddress(mod_handle, b"SetWindowSubclass\0".as_ptr());
            let p_def = GetProcAddress(mod_handle, b"DefSubclassProc\0".as_ptr());
            if p_set.is_null() || p_def.is_null() {
                return None;
            }
            Some((
                std::mem::transmute::<_, FnSetWindowSubclass>(p_set),
                std::mem::transmute::<_, FnDefSubclassProc>(p_def),
            ))
        });

        if let Some((set_subclass, _)) = fns {
            unsafe extern "system" fn non_resizable_subclass(
                hwnd: isize,
                msg: u32,
                wparam: usize,
                lparam: isize,
                _id: usize,
                _data: usize,
            ) -> isize {
                const WM_NCHITTEST: u32 = 0x0084;
                const HTCLIENT: isize = 1;
                const HTLEFT: isize = 10;
                const HTRIGHT: isize = 11;
                const HTTOP: isize = 12;
                const HTTOPLEFT: isize = 13;
                const HTTOPRIGHT: isize = 14;
                const HTBOTTOM: isize = 15;
                const HTBOTTOMLEFT: isize = 16;
                const HTBOTTOMRIGHT: isize = 17;

                if let Some(Some((_, def_proc))) = SUBCLASS_FNS.get() {
                    if msg == WM_NCHITTEST {
                        let hit = def_proc(hwnd, msg, wparam, lparam);
                        if (HTLEFT..=HTBOTTOMRIGHT).contains(&hit) {
                            return HTCLIENT;
                        }
                        return hit;
                    }
                    return def_proc(hwnd, msg, wparam, lparam);
                }
                0
            }

            let _ = set_subclass(hwnd, non_resizable_subclass, 0x53594E43, 0);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn eliminate_window_borders(_hwnd: isize) {}

#[cfg(target_os = "windows")]
static SINGLE_INSTANCE_MUTEX_HANDLE: std::sync::atomic::AtomicPtr<c_void> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

#[cfg(target_os = "windows")]
pub fn release_single_instance() {
    let handle = SINGLE_INSTANCE_MUTEX_HANDLE.swap(std::ptr::null_mut(), std::sync::atomic::Ordering::SeqCst);
    if !handle.is_null() {
        unsafe {
            winapi::CloseHandle(handle);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn release_single_instance() {}

#[cfg(target_os = "windows")]
pub fn ensure_single_instance(mutex_name: &str, window_title: &str) -> bool {
    ensure_single_instance_retry(mutex_name, window_title, false)
}

#[cfg(target_os = "windows")]
pub fn ensure_single_instance_retry(mutex_name: &str, window_title: &str, is_restart: bool) -> bool {
    let max_attempts = if is_restart { 50 } else { 1 };
    let name_w = wide_null(mutex_name);

    for attempt in 0..max_attempts {
        let handle = unsafe {
            winapi::CreateMutexW(std::ptr::null_mut(), 1, name_w.as_ptr())
        };

        if handle.is_null() {
            return true;
        }

        let last_err = unsafe { winapi::GetLastError() };
        if last_err == ERROR_ALREADY_EXISTS {
            unsafe {
                winapi::CloseHandle(handle);
            }

            if attempt + 1 < max_attempts {
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            if !is_restart {
                let title_w = wide_null(window_title);
                let hwnd = unsafe {
                    winapi::FindWindowW(std::ptr::null(), title_w.as_ptr())
                };
                if !hwnd.is_null() {
                    unsafe {
                        winapi::ShowWindow(hwnd, SW_RESTORE);
                        winapi::SetForegroundWindow(hwnd);
                    }
                }
                return false;
            } else {
                return true;
            }
        }

        SINGLE_INSTANCE_MUTEX_HANDLE.store(handle, std::sync::atomic::Ordering::SeqCst);
        return true;
    }

    true
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance(_mutex_name: &str, _window_title: &str) -> bool {
    true
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance_retry(_mutex_name: &str, _window_title: &str, _is_restart: bool) -> bool {
    true
}
