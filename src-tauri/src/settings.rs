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

    let approved_path = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";

    if enabled {
        run_key
            .set_value("Synchro", &exe_value)
            .map_err(|error| format!("Failed to enable autostart: {error}"))?;

        // Synchronize with Windows 10/11 Task Manager StartupApproved registry to unblock startup
        if let Ok((approved_key, _)) = hkcu.create_subkey(approved_path) {
            let enabled_bytes: [u8; 12] = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            let reg_val = winreg::RegValue {
                vtype: winreg::enums::REG_BINARY,
                bytes: enabled_bytes.to_vec(),
            };
            let _ = approved_key.set_raw_value("Synchro", &reg_val);
        }
    } else {
        match run_key.delete_value("Synchro") {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("Failed to disable autostart: {error}")),
        }

        if let Ok(approved_key) = hkcu.open_subkey_with_flags(approved_path, winreg::enums::KEY_WRITE) {
            let _ = approved_key.delete_value("Synchro");
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn set_autostart(_app: &AppHandle, _enabled: bool) -> Result<(), String> {
    Ok(())
}
