use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, State};

use crate::app::state::{checked_child_json_path, short_date, unix_now, PersistedState, RuntimeState};
use crate::app::workers::trim_process_memory;
use crate::commands::storage::{create_backup_file, read_backup_entries, restore_backup};
use crate::domain::tweaks::{
    apply_selected_tweaks, collect_tweak_statuses, TweakApplyResult, TweakStatus,
};
use crate::platform::ffi::restart_explorer as ffi_restart_explorer;
use crate::system_info;

fn prune_safety_backups(dir: &Path) {
    if let Ok(entries) = read_backup_entries(dir) {
        let safety_entries: Vec<_> = entries
            .into_iter()
            .filter(|e| {
                e.name.starts_with("Safety backup")
                    || e.name.starts_with("Startup backup")
                    || e.name.starts_with("Before tweaks")
            })
            .collect();
        if safety_entries.len() > 10 {
            for old in &safety_entries[10..] {
                if let Ok(path) = checked_child_json_path(dir, &old.id) {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
}

#[tauri::command]
pub fn apply_tweaks(
    ids: Vec<String>,
    state: State<'_, RuntimeState>,
) -> Result<Vec<TweakApplyResult>, String> {
    if !ids.is_empty() {
        let timestamp = unix_now();
        let _ = create_backup_file(
            &state.backups_dir,
            format!("Safety backup {}", short_date(timestamp)),
            state.snapshot()?,
        );
        prune_safety_backups(&state.backups_dir);
    }
    let results = apply_selected_tweaks(ids);
    log_tweaks_audit(&state.app_dir, &results);
    trim_process_memory();
    Ok(results)
}

#[tauri::command]
pub fn get_tweak_statuses() -> Vec<TweakStatus> {
    collect_tweak_statuses()
}

#[tauri::command]
pub fn rollback_last_tweaks(
    app: AppHandle,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let entries = read_backup_entries(&state.backups_dir)?;
    let latest = entries
        .iter()
        .find(|e| {
            e.name.starts_with("Safety backup")
                || e.name.starts_with("Startup backup")
                || e.name.starts_with("Before tweaks")
        })
        .or_else(|| entries.first())
        .ok_or_else(|| "No safety backup available to restore".to_string())?;

    let restored = restore_backup(app, latest.id.clone(), state)?;
    Ok(restored)
}

#[tauri::command]
pub fn detect_installed_apps() -> Vec<system_info::DetectedAppConflictInfo> {
    system_info::detect_installed_tweak_apps()
}

#[tauri::command]
pub fn restart_explorer() -> Result<(), String> {
    ffi_restart_explorer()
}

fn log_tweaks_audit(app_dir: &Path, results: &[TweakApplyResult]) {
    let log_path = app_dir.join("tweaks_audit.log");
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    };
    let mut file = match OpenOptions::new().create(true).append(true).open(&log_path) {
        Ok(f) => f,
        Err(_) => return,
    };

    if let Ok(meta) = file.metadata() {
        if meta.len() > 512 * 1024 {
            drop(file);
            let _ = fs::rename(&log_path, app_dir.join("tweaks_audit.old.log"));
            file = match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&log_path)
            {
                Ok(f) => f,
                Err(_) => return,
            };
        }
    }

    for result in results {
        let line = format!(
            "[{now}] id=\"{}\" status=\"{}\" message=\"{}\"\n",
            result.id.replace('"', "\\\""),
            result.status.replace('"', "\\\""),
            result.message.replace('"', "\\\""),
        );
        let _ = file.write_all(line.as_bytes());
    }
}
