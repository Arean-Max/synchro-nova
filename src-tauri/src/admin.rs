#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
pub(crate) fn restart_as_admin(target_page: Option<&str>) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("Failed to resolve current executable: {error}"))?;
    let pid = std::process::id();
    let args = if let Some(page) = target_page.filter(|p| matches!(*p, "color" | "tweaks" | "binds" | "settings" | "characteristics")) {
        format!("--restarted-from-pid {pid} --navigate-to {page}")
    } else {
        format!("--restarted-from-pid {pid}")
    };
    runas_without_hwnd(&exe, Some(&args))
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn restart_as_admin(_target_page: Option<&str>) -> Result<(), String> {
    Ok(())
}

pub(crate) fn is_running_elevated() -> bool {
    crate::ffi::is_user_admin()
}

#[cfg(target_os = "windows")]
fn runas_without_hwnd(path: &Path, args: Option<&str>) -> Result<(), String> {
    crate::ffi::runas_executable_with_hwnd_and_args(path, 0, args)
}
