#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    restart_as_admin_with_hwnd(0, None)
}

#[cfg(target_os = "windows")]
pub(crate) fn restart_as_admin_with_hwnd(hwnd: isize, target_page: Option<&str>) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("Failed to resolve current executable: {error}"))?;
    let pid = std::process::id();
    let args = if let Some(page) = target_page {
        format!("--restarted-from-pid {pid} --navigate-to {page}")
    } else {
        format!("--restarted-from-pid {pid}")
    };
    runas_with_hwnd(&exe, hwnd, Some(&args))
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn restart_as_admin_with_hwnd(_hwnd: isize, _target_page: Option<&str>) -> Result<(), String> {
    Ok(())
}

pub(crate) fn is_running_elevated() -> bool {
    crate::ffi::is_user_admin()
}

#[cfg(target_os = "windows")]
fn runas_with_hwnd(path: &Path, hwnd: isize, args: Option<&str>) -> Result<(), String> {
    crate::ffi::runas_executable_with_hwnd_and_args(path, hwnd, args)
}
