

use std::path::Path;
use super::types::*;
use super::winapi;

#[cfg(target_os = "windows")]
pub fn open_path_or_url(target: &str) -> Result<(), String> {
    let operation = wide_null("open");
    let file = wide_null(target);
    let result = unsafe {
        winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    if result <= 32 {
        Err(format!("ShellExecuteW failed for target: {target}"))
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn open_path_or_url(_target: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn runas_executable(path: &Path) -> Result<(), String> {
    runas_executable_with_hwnd_and_args(path, 0, None)
}

#[cfg(target_os = "windows")]
pub fn runas_executable_with_args(path: &Path, args: Option<&str>) -> Result<(), String> {
    runas_executable_with_hwnd_and_args(path, 0, args)
}

#[cfg(target_os = "windows")]
pub fn runas_executable_with_hwnd_and_args(path: &Path, hwnd: isize, args: Option<&str>) -> Result<(), String> {
    let operation = wide_null("runas");
    let path_str = path.to_string_lossy();
    let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
    let file = wide_null(clean_path);
    let params = args.map(wide_null);
    let params_ptr = params.as_ref().map(|p| p.as_ptr()).unwrap_or(std::ptr::null());
    let dir_buf = path.parent().map(|p| {
        let s = p.to_string_lossy();
        s.strip_prefix(r"\\?\").unwrap_or(&s).to_string()
    });
    let dir = dir_buf.as_deref().map(wide_null);
    let dir_ptr = dir.as_ref().map(|d| d.as_ptr()).unwrap_or(std::ptr::null());

    let mut sei = ShellExecuteInfoW::default();
    sei.f_mask = SEE_MASK_NOCLOSEPROCESS;
    sei.hwnd = hwnd as *mut c_void;
    sei.lp_verb = operation.as_ptr();
    sei.lp_file = file.as_ptr();
    sei.lp_parameters = params_ptr;
    sei.lp_directory = dir_ptr;
    sei.n_show = SW_SHOWNORMAL;

    let success = unsafe { winapi::ShellExecuteExW(&mut sei) };
    if success == 0 {
        let err = unsafe { winapi::GetLastError() };
        if err == ERROR_CANCELLED {
            return Err("Administrator restart was cancelled by user".to_string());
        }
        return Err(format!("Failed to execute process with elevated privileges (error {err})"));
    }

    if !sei.h_process.is_null() {
        unsafe {
            winapi::CloseHandle(sei.h_process);
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable_with_args(_path: &Path, _args: Option<&str>) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable_with_hwnd_and_args(_path: &Path, _hwnd: isize, _args: Option<&str>) -> Result<(), String> {
    Ok(())
}
