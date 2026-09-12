use tauri::AppHandle;

#[cfg(target_os = "windows")]
pub(crate) fn set_autostart(app: &AppHandle, enabled: bool) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};

    let _ = app;
    let exe = std::env::current_exe()
        .map_err(|error| format!("Failed to resolve current executable path: {error}"))?;
    let exe_value = format!("\"{}\"", exe.display());
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run_key, _) = hkcu
        .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .map_err(|error| format!("Failed to open autostart registry key: {error}"))?;

    if enabled {
        run_key
            .set_value("Synchro", &exe_value)
            .map_err(|error| format!("Failed to enable autostart: {error}"))?;
    } else {
        match run_key.delete_value("Synchro") {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Failed to disable autostart: {error}")),
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn set_autostart(_app: &AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}
