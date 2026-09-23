use std::fs;
use std::path::Path;
use tauri::{AppHandle, State};

use crate::app::state::{
    checked_child_json_path, clean_display_name, display_name_from_stem, file_timestamp,
    read_json_file, safe_file_stem, unique_available_stem, unique_id, unix_now,
    write_new_json_file, BackupEntry, BackupFile, ConfigEntry, ConfigFile, PersistedState,
    RuntimeState, MAX_STORAGE_ENTRIES, MAX_STORAGE_SCAN_ENTRIES,
};
use crate::domain::color::apply_color_transform;
use crate::domain::tweaks::{
    collect_tweak_registry_snapshot, create_system_restore_point, restore_tweak_registry_snapshot,
};
use crate::settings::set_autostart;

pub fn create_backup_file(
    dir: &Path,
    name: String,
    snapshot: PersistedState,
) -> Result<BackupEntry, String> {
    let created_at = unix_now();
    let display_name = clean_display_name(name);
    let id = unique_id(&display_name, created_at)?;
    let path = checked_child_json_path(dir, &id)?;
    let backup = BackupFile {
        name: display_name.clone(),
        created_at,
        state: snapshot,
        tweak_registry: collect_tweak_registry_snapshot(),
    };
    write_new_json_file(&path, &backup)?;
    let restore_desc = format!("Synchro - {}", display_name);
    std::thread::spawn(move || {
        create_system_restore_point(&restore_desc);
    });
    Ok(BackupEntry {
        id,
        name: display_name,
        created_at,
    })
}

pub fn read_backup_entries(dir: &Path) -> Result<Vec<BackupEntry>, String> {
    let mut entries = Vec::new();
    for (index, entry) in fs::read_dir(dir)
        .map_err(|error| format!("Failed to list backups: {error}"))?
        .enumerate()
    {
        if index >= MAX_STORAGE_SCAN_ENTRIES {
            break;
        }
        let entry = entry.map_err(|error| format!("Failed to read backup entry: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        if let Ok(file) = read_json_file::<BackupFile>(&path) {
            let id = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_string();
            entries.push(BackupEntry {
                id,
                name: file.name,
                created_at: file.created_at,
            });
        }
    }
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.created_at));
    entries.truncate(MAX_STORAGE_ENTRIES);
    Ok(entries)
}

pub fn read_config_entries(dir: &Path) -> Result<Vec<ConfigEntry>, String> {
    let mut entries = Vec::new();
    for (index, entry) in fs::read_dir(dir)
        .map_err(|error| format!("Failed to list configs: {error}"))?
        .enumerate()
    {
        if index >= MAX_STORAGE_SCAN_ENTRIES {
            break;
        }
        let entry = entry.map_err(|error| format!("Failed to read config entry: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        if let Ok(file) = read_json_file::<ConfigFile>(&path) {
            let id = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_string();
            entries.push(ConfigEntry {
                name: display_name_from_stem(&id),
                created_at: file_timestamp(&path),
                color: file.color.sanitized(),
                id,
            });
        }
    }
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.created_at));
    entries.truncate(MAX_STORAGE_ENTRIES);
    Ok(entries)
}

#[cfg(target_os = "windows")]
fn open_folder(path: &Path) -> Result<(), String> {
    let path = path
        .canonicalize()
        .map_err(|error| format!("Failed to resolve folder: {error}"))?;
    crate::platform::ffi::open_path_or_url(&path.to_string_lossy())
        .map_err(|_| "Failed to open folder".to_string())
}

#[cfg(not(target_os = "windows"))]
fn open_folder(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn list_backups(state: State<'_, RuntimeState>) -> Result<Vec<BackupEntry>, String> {
    read_backup_entries(&state.backups_dir)
}

#[tauri::command]
pub fn create_backup(
    name: Option<String>,
    state: State<'_, RuntimeState>,
) -> Result<Vec<BackupEntry>, String> {
    create_backup_file(
        &state.backups_dir,
        name.unwrap_or_else(|| "Manual backup".to_string()),
        state.snapshot()?,
    )?;
    read_backup_entries(&state.backups_dir)
}

#[tauri::command]
pub fn restore_backup(
    app: AppHandle,
    id: String,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let path = checked_child_json_path(&state.backups_dir, &id)?;
    let mut backup = read_json_file::<BackupFile>(&path)?;
    let current = state.snapshot()?;
    backup.state = backup.state.sanitized();
    backup.state.settings.accepted_agreement = current.settings.accepted_agreement;
    apply_color_transform(&backup.state.color, backup.state.settings.show_on_recordings)?;
    set_autostart(&app, backup.state.settings.autostart_windows)?;
    let _ = restore_tweak_registry_snapshot(&backup.tweak_registry);

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    *data = backup.state;
    let snapshot = data.clone();
    state.save(&snapshot)?;
    Ok(snapshot)
}

#[tauri::command]
pub fn delete_backup(id: String, state: State<'_, RuntimeState>) -> Result<Vec<BackupEntry>, String> {
    let path = checked_child_json_path(&state.backups_dir, &id)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|error| format!("Failed to delete backup: {error}"))?;
    }
    read_backup_entries(&state.backups_dir)
}

#[tauri::command]
pub fn list_configs(state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
    read_config_entries(&state.configs_dir)
}

#[tauri::command]
pub fn save_config(name: String, state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
    let snapshot = state.snapshot()?;
    let display_name = clean_display_name(name);
    let id = unique_available_stem(&state.configs_dir, &safe_file_stem(&display_name))?;
    let path = checked_child_json_path(&state.configs_dir, &id)?;
    let config = ConfigFile {
        color: snapshot.color,
        settings: snapshot.settings,
    };
    write_new_json_file(&path, &config)?;
    read_config_entries(&state.configs_dir)
}

#[tauri::command]
pub fn load_config(id: String, state: State<'_, RuntimeState>) -> Result<PersistedState, String> {
    let path = checked_child_json_path(&state.configs_dir, &id)?;
    let config = read_json_file::<ConfigFile>(&path)?;
    let current = state.snapshot()?;
    let mut settings = config.settings;
    settings.accepted_agreement = current.settings.accepted_agreement;
    Ok(PersistedState {
        color: config.color,
        settings,
        is_admin: current.is_admin,
    }
    .sanitized())
}

#[tauri::command]
pub fn apply_config(
    app: AppHandle,
    id: String,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let path = checked_child_json_path(&state.configs_dir, &id)?;
    let config = read_json_file::<ConfigFile>(&path)?;
    let current = state.snapshot()?;
    let mut settings = config.settings;
    settings.accepted_agreement = current.settings.accepted_agreement;
    let snapshot = PersistedState {
        color: config.color,
        settings,
        is_admin: current.is_admin,
    }
    .sanitized();
    apply_color_transform(&snapshot.color, snapshot.settings.show_on_recordings)?;
    set_autostart(&app, snapshot.settings.autostart_windows)?;

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    *data = snapshot;
    let stored = data.clone();
    state.save(&stored)?;
    Ok(stored)
}

#[tauri::command]
pub fn delete_config(id: String, state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
    let path = checked_child_json_path(&state.configs_dir, &id)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|error| format!("Failed to delete config: {error}"))?;
    }
    read_config_entries(&state.configs_dir)
}

#[tauri::command]
pub fn open_storage_folder(kind: String, state: State<'_, RuntimeState>) -> Result<(), String> {
    let path = match kind.as_str() {
        "backups" => state.backups_dir.clone(),
        "configs" => state.configs_dir.clone(),
        _ => return Err("Rejected unknown storage folder".to_string()),
    };
    open_folder(&path)
}
