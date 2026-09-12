#[cfg(target_os = "windows")]
use std::{ffi::c_void, path::Path};

#[cfg(target_os = "windows")]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("Failed to resolve current executable: {error}"))?;
    runas(&exe)
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn runas(path: &Path) -> Result<(), String> {
    #[link(name = "Shell32")]
    unsafe extern "system" {
        fn ShellExecuteW(
            hwnd: *mut c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show_cmd: i32,
        ) -> isize;
    }

    let operation = wide_null("runas");
    let file = wide_null(&path.to_string_lossy());
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        )
    };

    if result <= 32 {
        Err("Administrator restart was cancelled or blocked".to_string())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
