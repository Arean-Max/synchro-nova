#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("Failed to resolve current executable: {error}"))?;
    let pid = std::process::id();
    let args = format!("--restarted-from-pid {pid}");
    runas(&exe, Some(&args))
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn restart_as_admin() -> Result<(), String> {
    Ok(())
}

pub(crate) fn is_running_elevated() -> bool {
    crate::ffi::is_user_admin()
}

#[cfg(target_os = "windows")]
fn runas(path: &Path, args: Option<&str>) -> Result<(), String> {
    crate::ffi::runas_executable_with_args(path, args)
        .map_err(|_| "Administrator restart was cancelled or blocked".to_string())
}
