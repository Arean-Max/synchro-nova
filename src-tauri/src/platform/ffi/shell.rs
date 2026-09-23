#![allow(dead_code)]

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
    runas_executable_with_args(path, None)
}

#[cfg(target_os = "windows")]
pub fn runas_executable_with_args(path: &Path, args: Option<&str>) -> Result<(), String> {
    let operation = wide_null("runas");
    let file = wide_null(&path.to_string_lossy());
    let params = args.map(wide_null);
    let params_ptr = params.as_ref().map(|p| p.as_ptr()).unwrap_or(std::ptr::null());
    let dir = path.parent().map(|p| wide_null(&p.to_string_lossy()));
    let dir_ptr = dir.as_ref().map(|d| d.as_ptr()).unwrap_or(std::ptr::null());

    let result = unsafe {
        winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            params_ptr,
            dir_ptr,
            SW_SHOWNORMAL,
        )
    };
    if result <= 32 {
        Err("Failed to execute process with elevated privileges".to_string())
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable_with_args(_path: &Path, _args: Option<&str>) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn restart_explorer() -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;

    // 1. Terminate existing explorer instances cleanly
    let mut kill_cmd = Command::new("taskkill");
    kill_cmd.args(["/F", "/IM", "explorer.exe"]);
    kill_cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    let _ = kill_cmd.status();

    // 2. Wait a brief moment for handle cleanup
    std::thread::sleep(std::time::Duration::from_millis(350));

    // 3. Broadcast WM_SETTINGCHANGE before spawning new explorer
    unsafe {
        let env_w = wide_null("Environment");
        let mut result: usize = 0;
        let _ = winapi::SendMessageTimeoutW(
            0xFFFF as *mut std::ffi::c_void, // HWND_BROADCAST
            0x001A,                          // WM_SETTINGCHANGE
            0,
            env_w.as_ptr() as isize,
            2,                               // SMTO_ABORTIFHUNG
            1000,
            &mut result,
        );
    }

    // 4. Launch fresh explorer.exe from SystemRoot
    let explorer_exe = if let Ok(root) = std::env::var("SystemRoot") {
        std::path::PathBuf::from(root).join("explorer.exe")
    } else {
        std::path::PathBuf::from("explorer.exe")
    };

    let mut start_cmd = Command::new(&explorer_exe);
    start_cmd.creation_flags(0x0800_0000);
    start_cmd
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to restart explorer: {e}"))
}

#[cfg(not(target_os = "windows"))]
pub fn restart_explorer() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn show_message_box(caption: &str, text: &str, utype: u32) -> i32 {
    let text_w = wide_null(text);
    let cap_w = wide_null(caption);
    unsafe {
        winapi::MessageBoxW(
            std::ptr::null_mut(),
            text_w.as_ptr(),
            cap_w.as_ptr(),
            utype,
        )
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_message_box(_caption: &str, _text: &str, _utype: u32) -> i32 {
    1
}
