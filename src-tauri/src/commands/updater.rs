pub use crate::domain::updater::client::validate_update_url;
use crate::domain::updater::client::{
    fetch_latest_release_info, spawn_download_worker, UpdateCheckResult,
};
use crate::domain::updater::installer::apply_install;
use crate::domain::updater::{get_progress, UpdateProgress};

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateCheckResult, String> {
    tauri::async_runtime::spawn_blocking(fetch_latest_release_info)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn download_update(
    url: Option<String>,
    size: Option<u64>,
    version: Option<String>,
    manifest_url: Option<String>,
) -> Result<(), String> {
    let (target_url, target_size, target_version, target_manifest) = match (url, size, version, manifest_url) {
        (Some(u), Some(s), Some(v), m) => {
            validate_update_url(&u)?;
            if let Some(ref m_url) = m {
                validate_update_url(m_url)?;
            }
            (u, s, v, m)
        }
        _ => {
            let info = fetch_latest_release_info()?;
            let u = info
                .download_url
                .ok_or_else(|| "No downloadable update asset found in latest release".to_string())?;
            validate_update_url(&u)?;
            (u, info.asset_size, info.latest_version, info.sha_manifest_url)
        }
    };

    spawn_download_worker(target_url, target_size, target_version, target_manifest)
}

#[tauri::command]
pub fn get_update_progress() -> Result<UpdateProgress, String> {
    Ok(get_progress())
}

#[tauri::command]
pub fn install_update(no_relaunch: Option<bool>) -> Result<(), String> {
    apply_install(no_relaunch.unwrap_or(true))
}
