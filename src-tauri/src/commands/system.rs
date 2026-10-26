use crate::app::state::DriverInfo;
use crate::browser::open_driver_search_url;
use crate::driver_info;

#[tauri::command]
pub fn open_driver_search(query: String) -> Result<(), String> {
    open_driver_search_url(&query)
}

#[tauri::command]
pub fn scan_drivers() -> Vec<DriverInfo> {
    driver_info::collect_drivers()
}

pub fn validate_external_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err("Only secure HTTPS URLs are permitted".to_string());
    }
    if url.len() > 2048 {
        return Err("URL exceeds maximum permitted length".to_string());
    }
    if url
        .chars()
        .any(|ch| ch.is_control() || ch.is_whitespace() || matches!(ch, '"' | '\'' | '`' | '<' | '>' | '^' | '|' | '&'))
    {
        return Err("URL contains illegal characters".to_string());
    }
    let after_scheme = &url["https://".len()..];
    let host = after_scheme.split(&['/', '?', '#'][..]).next().unwrap_or("");
    if host.is_empty() || !host.contains('.') || host.starts_with('.') || host.ends_with('.') {
        return Err("URL does not contain a valid host name".to_string());
    }
    if host.chars().any(|c| !c.is_ascii_alphanumeric() && c != '.' && c != '-') {
        return Err("Host contains invalid characters".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    validate_external_url(&url)?;
    crate::platform::ffi::open_path_or_url(&url).map_err(|e| format!("Failed to open browser: {e}"))
}

#[tauri::command]
pub fn open_windows_driver_updates() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        crate::platform::ffi::open_path_or_url("ms-settings:windowsupdate-optionalupdates")
            .or_else(|_| crate::platform::ffi::open_path_or_url("ms-settings:windowsupdate"))
            .map_err(|e| format!("Failed to open Windows Update: {e}"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Not supported on this OS".to_string())
    }
}

#[tauri::command]
pub fn restart_graphics_driver() -> Result<(), String> {
    crate::platform::ffi::restart_graphics_driver()
}

#[tauri::command]
pub fn get_security_guard_status() -> crate::platform::security_guard::SecurityGuardStatus {
    crate::platform::security_guard::get_security_guard_status()
}

#[tauri::command]
pub fn audit_anticheat_environment() -> crate::platform::security_guard::AntiCheatAuditReport {
    crate::platform::security_guard::invalidate_security_caches();
    crate::platform::security_guard::audit_anticheat_integrity()
}

#[tauri::command]
pub fn fix_anticheat_environment() -> crate::platform::security_guard::AntiCheatFixResult {
    crate::platform::security_guard::fix_anticheat_integrity()
}
