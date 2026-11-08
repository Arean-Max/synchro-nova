

#[cfg(target_os = "windows")]
use std::ffi::c_void;
use super::types::*;
use super::winapi;

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
        let dark_mode: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            20,
            &dark_mode as *const _ as _,
            std::mem::size_of::<i32>() as u32,
        );

        let no_border: u32 = 0xFFFF_FFFE;
        let _ = DwmSetWindowAttribute(
            hwnd,
            34,
            &no_border as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );

        let caption_color: u32 = 0x0014_1414;
        let _ = DwmSetWindowAttribute(
            hwnd,
            35,
            &caption_color as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );
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
    let max_attempts = if is_restart { 50 } else { 5 };
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

            notify_running_instance(window_title);
            return false;
        }

        SINGLE_INSTANCE_MUTEX_HANDLE.store(handle, std::sync::atomic::Ordering::SeqCst);
        return true;
    }

    true
}

#[cfg(target_os = "windows")]
pub fn notify_running_instance(window_title: &str) {
    use std::io::Write;
    let _ = std::fs::OpenOptions::new()
        .write(true)
        .open(r"\\.\pipe\SynchroNovaInstancePipe")
        .and_then(|mut f| f.write_all(b"SHOW"));

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
}

#[cfg(not(target_os = "windows"))]
pub fn notify_running_instance(_window_title: &str) {}

#[cfg(target_os = "windows")]
pub fn start_single_instance_listener(app: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("synchro-instance-pipe".to_string())
        .stack_size(64 * 1024)
        .spawn(move || {
            #[link(name = "Kernel32")]
            extern "system" {
                fn CreateNamedPipeW(
                    lpName: *const u16,
                    dwOpenMode: u32,
                    dwPipeMode: u32,
                    nMaxInstances: u32,
                    nOutBufferSize: u32,
                    nInBufferSize: u32,
                    nDefaultTimeOut: u32,
                    lpSecurityAttributes: *mut c_void,
                ) -> *mut c_void;
                fn ConnectNamedPipe(hNamedPipe: *mut c_void, lpOverlapped: *mut c_void) -> i32;
                fn DisconnectNamedPipe(hNamedPipe: *mut c_void) -> i32;
                fn ReadFile(
                    hFile: *mut c_void,
                    lpBuffer: *mut u8,
                    nNumberOfBytesToRead: u32,
                    lpNumberOfBytesRead: *mut u32,
                    lpOverlapped: *mut c_void,
                ) -> i32;
            }

            let pipe_name = wide_null(r"\\.\pipe\SynchroNovaInstancePipe");
            const PIPE_ACCESS_INBOUND: u32 = 0x00000001;
            const PIPE_TYPE_BYTE: u32 = 0x00000000;
            const PIPE_READMODE_BYTE: u32 = 0x00000000;
            const PIPE_WAIT: u32 = 0x00000000;

            loop {
                let handle = unsafe {
                    CreateNamedPipeW(
                        pipe_name.as_ptr(),
                        PIPE_ACCESS_INBOUND,
                        PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                        1,
                        64,
                        64,
                        1000,
                        std::ptr::null_mut(),
                    )
                };

                if handle.is_null() || handle as isize == -1 {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    continue;
                }

                let connected = unsafe { ConnectNamedPipe(handle, std::ptr::null_mut()) };
                if connected != 0 || unsafe { winapi::GetLastError() } == 535 {
                    let mut buf = [0u8; 16];
                    let mut read_bytes = 0u32;
                    let _ = unsafe { ReadFile(handle, buf.as_mut_ptr(), 16, &mut read_bytes, std::ptr::null_mut()) };
                    crate::app::tray::show_main_window(&app);
                    unsafe { DisconnectNamedPipe(handle); }
                }
                unsafe { winapi::CloseHandle(handle); }
            }
        })
        .ok();
}

#[cfg(not(target_os = "windows"))]
pub fn start_single_instance_listener(_app: tauri::AppHandle) {}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance(_mutex_name: &str, _window_title: &str) -> bool {
    true
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance_retry(_mutex_name: &str, _window_title: &str, _is_restart: bool) -> bool {
    true
}

#[cfg(target_os = "windows")]
pub fn set_window_taskbar_icon(hwnd: isize) {
    if hwnd == 0 {
        return;
    }
    unsafe {
        let mut hicon_large: *mut c_void = std::ptr::null_mut();
        let mut hicon_small: *mut c_void = std::ptr::null_mut();

        if let Ok(exe_path) = std::env::current_exe() {
            let wide_path = wide_null(&exe_path.to_string_lossy());
            let extracted = winapi::ExtractIconExW(
                wide_path.as_ptr(),
                0,
                &mut hicon_large,
                &mut hicon_small,
                1,
            );
            if extracted == 0 || hicon_large.is_null() {
                let hmod = winapi::GetModuleHandleW(std::ptr::null());
                hicon_large = winapi::LoadIconW(hmod, 32512 as *const u16);
                hicon_small = winapi::LoadIconW(hmod, 32512 as *const u16);
            }
        } else {
            let hmod = winapi::GetModuleHandleW(std::ptr::null());
            hicon_large = winapi::LoadIconW(hmod, 32512 as *const u16);
            hicon_small = winapi::LoadIconW(hmod, 32512 as *const u16);
        }

        let hwnd_ptr = hwnd as *mut c_void;
        const WM_SETICON: u32 = 0x0080;
        const ICON_SMALL: usize = 0;
        const ICON_BIG: usize = 1;
        const GCLP_HICON: i32 = -14;
        const GCLP_HICONSM: i32 = -34;

        if !hicon_large.is_null() {
            winapi::SendMessageW(hwnd_ptr, WM_SETICON, ICON_BIG, hicon_large as isize);
            winapi::SetClassLongPtrW(hwnd_ptr, GCLP_HICON, hicon_large as isize);
        }
        if !hicon_small.is_null() {
            winapi::SendMessageW(hwnd_ptr, WM_SETICON, ICON_SMALL, hicon_small as isize);
            winapi::SetClassLongPtrW(hwnd_ptr, GCLP_HICONSM, hicon_small as isize);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_window_taskbar_icon(_hwnd: isize) {}
