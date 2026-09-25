use crate::domain::updater::client::{fetch_latest_release_info, spawn_download_worker, UpdateCheckResult};
use crate::domain::updater::installer::apply_install;
use crate::domain::updater::{get_progress, UpdateProgress};

#[tauri::command]
pub fn check_for_updates() -> Result<UpdateCheckResult, String> {
    fetch_latest_release_info()
}

pub fn validate_update_url(url: &str) -> Result<(), String> {
    crate::commands::system::validate_external_url(url)?;
    let lower = url.to_lowercase();
    let is_valid_source = lower.starts_with("https://github.com/arean-max/synchro-nova/releases/")
        || lower.starts_with("https://objects.githubusercontent.com/");
    if !is_valid_source {
        return Err("Update URL must originate from official Synchro Nova GitHub releases".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn download_update(url: Option<String>, size: Option<u64>) -> Result<(), String> {
    let (target_url, target_size) = match (url, size) {
        (Some(u), Some(s)) => {
            validate_update_url(&u)?;
            (u, s)
        }
        _ => {
            let info = fetch_latest_release_info()?;
            let u = info
                .download_url
                .ok_or_else(|| "No downloadable update asset found".to_string())?;
            validate_update_url(&u)?;
            (u, info.asset_size)
        }
    };

    spawn_download_worker(target_url, target_size)
}

#[tauri::command]
pub fn get_update_progress() -> Result<UpdateProgress, String> {
    Ok(get_progress())
}

#[tauri::command]
pub fn install_update() -> Result<(), String> {
    apply_install()
}
