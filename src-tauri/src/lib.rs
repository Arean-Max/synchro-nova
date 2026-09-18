use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, State, WindowEvent,
};

mod admin;
mod browser;
mod color;
mod driver_info;
mod games;
mod live_metrics;
mod settings;
mod system_info;
mod tweaks;
pub mod ffi;

use admin::{is_running_elevated, restart_as_admin as restart_current_process_as_admin};
use browser::open_driver_search_url;
use color::apply_color_transform;
use games::InstalledGame;
use live_metrics::{collect_live_metrics, CpuSample, LiveMetrics};
use settings::set_autostart;
use system_info::collect_system_characteristics;
use tweaks::{
    apply_selected_tweaks, collect_tweak_registry_snapshot, collect_tweak_statuses,
    restore_tweak_registry_snapshot, TweakApplyResult, TweakRegistrySnapshot, TweakStatus,
};

const MAX_JSON_FILE_BYTES: u64 = 256 * 1024;
const MAX_JSON_STRING_BYTES: usize = MAX_JSON_FILE_BYTES as usize;
const MAX_DISPLAY_NAME_CHARS: usize = 64;
const MAX_STORAGE_ENTRIES: usize = 200;
const MAX_STORAGE_SCAN_ENTRIES: usize = 1_000;
const RANDOM_SUFFIX_BYTES: usize = 6;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ColorSettings {
    saturation: f32,
    hue: f32,
    contrast: f32,
    gamma: f32,
    enabled: bool,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            saturation: 100.0,
            hue: 0.0,
            contrast: 100.0,
            gamma: 100.0,
            enabled: true,
        }
    }
}

impl ColorSettings {
    fn sanitized(mut self) -> Self {
        self.saturation = clamp_finite(self.saturation, 0.0, 200.0, 100.0);
        self.hue = clamp_finite(self.hue, -180.0, 180.0, 0.0);
        self.contrast = clamp_finite(self.contrast, 50.0, 150.0, 100.0);
        self.gamma = clamp_finite(self.gamma, 50.0, 150.0, 100.0);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub apply_instantly: bool,
    pub save_color_correction: bool,
    pub autostart_windows: bool,
    pub close_to_tray: bool,
    pub start_minimized: bool,
    pub auto_backup_on_start: bool,
    pub accepted_agreement: bool,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            apply_instantly: true,
            save_color_correction: true,
            autostart_windows: false,
            close_to_tray: true,
            start_minimized: false,
            auto_backup_on_start: true,
            accepted_agreement: true,
            language: "en".to_string(),
        }
    }
}

impl AppSettings {
    fn sanitized(mut self) -> Self {
        if self.language != "ru" {
            self.language = "en".to_string();
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct PersistedState {
    color: ColorSettings,
    settings: AppSettings,
    #[serde(default)]
    is_admin: bool,
}

impl PersistedState {
    fn sanitized(mut self) -> Self {
        self.color = self.color.sanitized();
        self.settings = self.settings.sanitized();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BackupFile {
    name: String,
    created_at: u64,
    state: PersistedState,
    #[serde(default)]
    tweak_registry: Vec<TweakRegistrySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntry {
    id: String,
    name: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct ConfigFile {
    color: ColorSettings,
    settings: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigEntry {
    id: String,
    name: String,
    created_at: u64,
    color: ColorSettings,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverInfo {
    name: String,
    provider: String,
    version: String,
    date: String,
    class_name: String,
    status: String,
    path: String,
    search_query: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemCharacteristics {
    cpu: String,
    cpu_usage_percent: String,
    gpu: String,
    gpu_usage_percent: String,
    gpus: Vec<String>,
    ram: String,
    ram_available: String,
    ram_available_gb: String,
    ram_used_gb: String,
    ram_used_percent: String,
    os: String,
    architecture: String,
    display: String,
    refresh_rate: String,
    color_depth: String,
    driver: String,
    cpu_cores: String,
    gpu_count: String,
    ram_total_gb: String,
    vram_total_gb: String,
    vram_used_gb: String,
    vram_used_percent: String,
    refresh_rate_hz: String,
    drivers: Vec<DriverInfo>,
}

#[derive(Default)]
struct SystemCache {
    value: Option<SystemCharacteristics>,
    updated_at: Option<Instant>,
}

struct RuntimeState {
    app_dir: PathBuf,
    settings_path: PathBuf,
    backups_dir: PathBuf,
    configs_dir: PathBuf,
    data: Mutex<PersistedState>,
    system_cache: Mutex<SystemCache>,
    cpu_sample: Mutex<Option<CpuSample>>,
    exiting: AtomicBool,
}

impl RuntimeState {
    fn load(app: &AppHandle) -> Result<Self, String> {
        let app_dir = app
            .path()
            .app_data_dir()
            .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;
        let backups_dir = app_dir.join("backups");
        let configs_dir = app_dir.join("configs");
        fs::create_dir_all(&backups_dir)
            .map_err(|error| format!("Failed to create backups directory: {error}"))?;
        fs::create_dir_all(&configs_dir)
            .map_err(|error| format!("Failed to create configs directory: {error}"))?;

        let settings_path = app_dir.join("settings.json");
        let data = if settings_path.exists() {
            read_json_file::<PersistedState>(&settings_path).unwrap_or_default()
        } else {
            PersistedState::default()
        }
        .sanitized();

        Ok(Self {
            app_dir,
            settings_path,
            backups_dir,
            configs_dir,
            data: Mutex::new(data),
            system_cache: Mutex::new(SystemCache::default()),
            cpu_sample: Mutex::new(None),
            exiting: AtomicBool::new(false),
        })
    }

    fn snapshot(&self) -> Result<PersistedState, String> {
        let mut state = self.data
            .lock()
            .map_err(|_| "Settings lock poisoned".to_string())
            .map(|guard| guard.clone())?;
        state.is_admin = is_running_elevated();
        Ok(state)
    }

    fn save(&self, data: &PersistedState) -> Result<(), String> {
        write_json_file(&self.settings_path, data)
    }
}

#[tauri::command]
fn get_app_state(state: State<'_, RuntimeState>) -> Result<PersistedState, String> {
    state.snapshot()
}

#[tauri::command]
fn apply_color_settings(
    color: ColorSettings,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let color = color.sanitized();
    let current = state.snapshot()?;
    if current.color != color {
        apply_color_transform(&color)?;
    }

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    data.color = color;
    let snapshot = data.clone();

    if snapshot.settings.save_color_correction {
        state.save(&snapshot)?;
    }

    Ok(snapshot)
}

#[tauri::command]
fn update_app_settings(
    app: AppHandle,
    settings: AppSettings,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let settings = settings.sanitized();
    let current = state.snapshot()?;
    if current.settings.autostart_windows != settings.autostart_windows {
        set_autostart(&app, settings.autostart_windows)?;
    }

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    data.settings = settings;
    let snapshot = data.clone();
    state.save(&snapshot)?;
    Ok(snapshot)
}

#[tauri::command]
fn get_system_characteristics(
    state: State<'_, RuntimeState>,
) -> Result<SystemCharacteristics, String> {
    const CACHE_TTL: Duration = Duration::from_secs(8);

    let live = collect_system_live_metrics(&state)?;

    if let Ok(cache) = state.system_cache.lock() {
        if let (Some(value), Some(updated_at)) = (&cache.value, cache.updated_at) {
            if updated_at.elapsed() < CACHE_TTL {
                let mut snapshot = value.clone();
                apply_live_metrics(&mut snapshot, &live);
                return Ok(snapshot);
            }
        }
    }

    let mut collected = collect_system_characteristics();
    apply_live_metrics(&mut collected, &live);
    let mut cache = state
        .system_cache
        .lock()
        .map_err(|_| "System cache lock poisoned".to_string())?;
    cache.value = Some(collected.clone());
    cache.updated_at = Some(Instant::now());
    Ok(collected)
}

#[tauri::command]
fn get_system_live_metrics(state: State<'_, RuntimeState>) -> Result<LiveMetrics, String> {
    collect_system_live_metrics(&state)
}

fn collect_system_live_metrics(state: &State<'_, RuntimeState>) -> Result<LiveMetrics, String> {
    let mut sample = state
        .cpu_sample
        .lock()
        .map_err(|_| "CPU sample lock poisoned".to_string())?;
    Ok(collect_live_metrics(&mut sample))
}

fn apply_live_metrics(snapshot: &mut SystemCharacteristics, live: &LiveMetrics) {
    snapshot.cpu_usage_percent = live.cpu_usage_percent.clone();
    snapshot.gpu_usage_percent = live.gpu_usage_percent.clone();
    snapshot.ram_available_gb = live.ram_available_gb.clone();
    snapshot.ram_available = format!("{} GB", live.ram_available_gb);
    snapshot.ram_used_gb = live.ram_used_gb.clone();
    snapshot.ram_used_percent = live.ram_used_percent.clone();
    snapshot.vram_total_gb = live.vram_total_gb.clone();
    snapshot.vram_used_gb = live.vram_used_gb.clone();
    snapshot.vram_used_percent = live.vram_used_percent.clone();
}

#[tauri::command]
fn open_driver_search(query: String) -> Result<(), String> {
    open_driver_search_url(&query)
}

#[tauri::command]
fn list_installed_games() -> Vec<InstalledGame> {
    games::list_installed_games()
}

#[tauri::command]
fn launch_installed_game(id: String) -> Result<(), String> {
    games::launch_installed_game(&id)
}

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
fn apply_tweaks(
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
fn rollback_last_tweaks(
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
fn detect_installed_apps() -> Vec<system_info::DetectedAppConflictInfo> {
    system_info::detect_installed_tweak_apps()
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

#[tauri::command]
fn get_tweak_statuses() -> Result<Vec<TweakStatus>, String> {
    Ok(collect_tweak_statuses())
}

#[tauri::command]
fn list_backups(state: State<'_, RuntimeState>) -> Result<Vec<BackupEntry>, String> {
    read_backup_entries(&state.backups_dir)
}

#[tauri::command]
fn create_backup(
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
fn restore_backup(
    app: AppHandle,
    id: String,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let path = checked_child_json_path(&state.backups_dir, &id)?;
    let mut backup = read_json_file::<BackupFile>(&path)?;
    let current = state.snapshot()?;
    backup.state = backup.state.sanitized();
    backup.state.settings.accepted_agreement = current.settings.accepted_agreement;
    apply_color_transform(&backup.state.color)?;
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
fn delete_backup(id: String, state: State<'_, RuntimeState>) -> Result<Vec<BackupEntry>, String> {
    let path = checked_child_json_path(&state.backups_dir, &id)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|error| format!("Failed to delete backup: {error}"))?;
    }
    read_backup_entries(&state.backups_dir)
}

#[tauri::command]
fn list_configs(state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
    read_config_entries(&state.configs_dir)
}

#[tauri::command]
fn save_config(name: String, state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
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
fn load_config(id: String, state: State<'_, RuntimeState>) -> Result<PersistedState, String> {
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
fn apply_config(
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
    apply_color_transform(&snapshot.color)?;
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
fn delete_config(id: String, state: State<'_, RuntimeState>) -> Result<Vec<ConfigEntry>, String> {
    let path = checked_child_json_path(&state.configs_dir, &id)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|error| format!("Failed to delete config: {error}"))?;
    }
    read_config_entries(&state.configs_dir)
}

#[tauri::command]
fn open_storage_folder(kind: String, state: State<'_, RuntimeState>) -> Result<(), String> {
    let path = match kind.as_str() {
        "backups" => state.backups_dir.clone(),
        "configs" => state.configs_dir.clone(),
        _ => return Err("Rejected unknown storage folder".to_string()),
    };
    open_folder(&path)
}

#[tauri::command]
fn minimize_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
        trim_process_memory();
    }
}

#[tauri::command]
fn toggle_window_maximize(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let maximized = window.is_maximized().unwrap_or(false);
        if maximized {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
fn start_window_drag(window: tauri::WebviewWindow) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn close_window(app: AppHandle, state: State<'_, RuntimeState>) {
    if let Some(window) = app.get_webview_window("main") {
        let close_to_tray = state
            .snapshot()
            .map(|snapshot| snapshot.settings.close_to_tray)
            .unwrap_or(true);

        if close_to_tray {
            let _ = window.hide();
            trim_process_memory();
        } else {
            state.exiting.store(true, Ordering::SeqCst);
            app.exit(0);
        }
    }
}

#[tauri::command]
fn exit_app(app: AppHandle, state: State<'_, RuntimeState>) {
    state.exiting.store(true, Ordering::SeqCst);
    app.exit(0);
}

#[tauri::command]
fn restart_as_admin(app: AppHandle, state: State<'_, RuntimeState>) -> Result<(), String> {
    restart_current_process_as_admin()?;
    state.exiting.store(true, Ordering::SeqCst);
    crate::ffi::release_single_instance();
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    std::process::exit(0);
}

fn create_backup_file(
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
    Ok(BackupEntry {
        id,
        name: display_name,
        created_at,
    })
}

fn read_backup_entries(dir: &Path) -> Result<Vec<BackupEntry>, String> {
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

fn read_config_entries(dir: &Path) -> Result<Vec<ConfigEntry>, String> {
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

fn read_json_file<T>(path: &Path) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    validate_json_file(path)?;
    let raw = fs::read_to_string(path).map_err(|error| format!("Failed to read file: {error}"))?;
    serde_json::from_str(&raw).map_err(|error| format!("Failed to parse JSON: {error}"))
}

fn write_json_file<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    write_json_file_with_mode(path, value, true)
}

fn write_new_json_file<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    write_json_file_with_mode(path, value, false)
}

fn write_json_file_with_mode<T>(
    path: &Path,
    value: &T,
    replace_existing: bool,
) -> Result<(), String>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create directory: {error}"))?;
    }
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| format!("Failed to serialize JSON: {error}"))?;
    if json.len() > MAX_JSON_STRING_BYTES {
        return Err("Refusing to write oversized JSON file".to_string());
    }

    let temp_path = write_temp_json_file(path, &json)?;

    if path.exists() {
        if !replace_existing {
            let _ = fs::remove_file(&temp_path);
            return Err("Refusing to overwrite existing JSON file".to_string());
        }
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("Failed to inspect file: {error}"))?;
        if metadata.file_type().is_dir() {
            let _ = fs::remove_file(&temp_path);
            return Err("Refusing to replace a directory".to_string());
        }
        fs::remove_file(path).map_err(|error| format!("Failed to replace file: {error}"))?;
    }

    fs::rename(&temp_path, path).map_err(|error| {
        let _ = fs::remove_file(&temp_path);
        format!("Failed to commit JSON file: {error}")
    })
}

fn write_temp_json_file(path: &Path, json: &str) -> Result<PathBuf, String> {
    for _ in 0..8 {
        let temp_path = path.with_extension(format!(
            "tmp-{}-{}",
            std::process::id(),
            random_hex(RANDOM_SUFFIX_BYTES)?
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(mut file) => {
                if let Err(error) = file.write_all(json.as_bytes()) {
                    drop(file);
                    let _ = fs::remove_file(&temp_path);
                    return Err(format!("Failed to write temp file: {error}"));
                }
                if let Err(error) = file.sync_all() {
                    drop(file);
                    let _ = fs::remove_file(&temp_path);
                    return Err(format!("Failed to sync temp file: {error}"));
                }
                return Ok(temp_path);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(format!("Failed to create temp file: {error}")),
        }
    }

    Err("Failed to reserve a unique temp file".to_string())
}

fn checked_child_json_path(root: &Path, id: &str) -> Result<PathBuf, String> {
    let stem = safe_file_stem(id);
    let root = root
        .canonicalize()
        .map_err(|error| format!("Failed to resolve storage directory: {error}"))?;
    let path = root.join(format!("{stem}.json"));
    if !path.starts_with(&root) {
        return Err("Rejected unsafe storage path".to_string());
    }
    Ok(path)
}

fn validate_json_file(path: &Path) -> Result<(), String> {
    let metadata =
        fs::symlink_metadata(path).map_err(|error| format!("Failed to inspect file: {error}"))?;
    if !metadata.file_type().is_file() {
        return Err("Refusing to read a non-regular JSON file".to_string());
    }
    if metadata.len() > MAX_JSON_FILE_BYTES {
        return Err("Refusing to read oversized JSON file".to_string());
    }
    Ok(())
}

fn clean_display_name(name: String) -> String {
    let cleaned = name
        .chars()
        .filter(|ch| !ch.is_control())
        .collect::<String>();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "Untitled".to_string()
    } else {
        trimmed.chars().take(MAX_DISPLAY_NAME_CHARS).collect()
    }
}

fn display_name_from_stem(stem: &str) -> String {
    let spaced = stem
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if spaced.is_empty() {
        "Untitled".to_string()
    } else {
        let mut chars = spaced.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => "Untitled".to_string(),
        }
    }
}

fn file_timestamp(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or_else(unix_now)
}

fn is_reserved_windows_name(name: &str) -> bool {
    const RESERVED: &[&str] = &[
        "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7",
        "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8",
        "lpt9",
    ];
    RESERVED.contains(&name)
}

fn safe_file_stem(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
        } else if ch == '-' || ch == '_' || ch.is_whitespace() {
            output.push('-');
        }
    }
    while output.contains("--") {
        output = output.replace("--", "-");
    }
    let output = output.trim_matches('-').to_string();
    if output.is_empty() || is_reserved_windows_name(&output) {
        "item".to_string()
    } else {
        output
    }
}

fn unique_available_stem(root: &Path, preferred: &str) -> Result<String, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("Failed to resolve storage directory: {error}"))?;
    let stem = safe_file_stem(preferred);

    for index in 0..MAX_STORAGE_ENTRIES {
        let candidate = if index == 0 {
            stem.clone()
        } else {
            format!("{stem}-{}", index + 1)
        };
        let path = root.join(format!("{candidate}.json"));
        if !path.starts_with(&root) {
            return Err("Rejected unsafe storage path".to_string());
        }
        if !path.exists() {
            return Ok(candidate);
        }
    }

    Err("No available storage name".to_string())
}

fn unique_id(name: &str, created_at: u64) -> Result<String, String> {
    Ok(format!(
        "{}-{created_at}-{}",
        safe_file_stem(name),
        random_hex(RANDOM_SUFFIX_BYTES)?
    ))
}

fn random_hex(byte_count: usize) -> Result<String, String> {
    let mut bytes = vec![0_u8; byte_count];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("Failed to generate random id: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn short_date(seconds: u64) -> String {
    time::OffsetDateTime::from_unix_timestamp(seconds as i64)
        .map(|date_time| {
            let date = date_time.date();
            format!(
                "{:02}.{:02}.{:04}",
                date.day(),
                u8::from(date.month()),
                date.year()
            )
        })
        .unwrap_or_else(|_| "01.01.1970".to_string())
}

fn clamp_finite(value: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

fn install_tray(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &exit])?;

    let mut tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_window(app),
            "exit" => {
                let state = app.state::<RuntimeState>();
                state.exiting.store(true, Ordering::SeqCst);
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            }
            | TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                show_main_window(tray.app_handle());
            }
            _ => {}
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        tray = tray.icon(icon);
    }

    tray.build(app)?;
    Ok(())
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg(target_os = "windows")]
fn utf16z_to_string(raw: &[u16]) -> String {
    let len = raw
        .iter()
        .position(|value| *value == 0)
        .unwrap_or(raw.len());
    String::from_utf16_lossy(&raw[..len]).trim().to_string()
}

#[cfg(target_os = "windows")]
fn open_folder(path: &Path) -> Result<(), String> {
    let path = path
        .canonicalize()
        .map_err(|error| format!("Failed to resolve folder: {error}"))?;
    ffi::open_path_or_url(&path.to_string_lossy())
        .map_err(|_| "Failed to open folder".to_string())
}

#[cfg(not(target_os = "windows"))]
fn open_folder(_path: &Path) -> Result<(), String> {
    Ok(())
}

pub fn trim_process_memory() {
    ffi::trim_working_set();
}

#[tauri::command]
fn trim_memory() {
    trim_process_memory();
}

fn apply_process_hardening() {
    ffi::apply_process_hardening();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            apply_process_hardening();
            ffi::init_process_tree_job();

            let runtime = RuntimeState::load(app.handle())?;
            let initial = runtime.snapshot()?;
            if initial.settings.auto_backup_on_start {
                let _ = create_backup_file(
                    &runtime.backups_dir,
                    format!("Startup backup {}", short_date(unix_now())),
                    initial.clone(),
                );
            }
            app.manage(runtime);

            let _ = apply_color_transform(&initial.color);
            let _ = set_autostart(app.handle(), initial.settings.autostart_windows);
            install_tray(app.handle())?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_resizable(true);
                let _ = window.set_maximizable(true);
                let _ = window.center();
                if initial.settings.start_minimized {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            trim_process_memory();

            // Post-boot delayed working-set trims to reclaim initial WebView2 bootstrapping memory
            std::thread::spawn(|| {
                std::thread::sleep(Duration::from_millis(1500));
                trim_process_memory();
                std::thread::sleep(Duration::from_millis(2500));
                trim_process_memory();
            });

            // Gentle open-state memory trimmer: keeps WebView2 RAM rock-bottom while menu is open
            std::thread::spawn(|| {
                loop {
                    std::thread::sleep(Duration::from_secs(12));
                    trim_process_memory();
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            match event {
                WindowEvent::Focused(true) => {
                    std::thread::spawn(|| {
                        std::thread::sleep(Duration::from_millis(500));
                        trim_process_memory();
                    });
                }
                WindowEvent::Focused(false) => {
                    trim_process_memory();
                }
                WindowEvent::CloseRequested { api, .. } => {
                    let app = window.app_handle();
                    let state = app.state::<RuntimeState>();
                    if state.exiting.load(Ordering::SeqCst) {
                        return;
                    }

                    if let Ok(snapshot) = state.snapshot() {
                        if snapshot.settings.close_to_tray {
                            api.prevent_close();
                            let _ = window.hide();
                            trim_process_memory();
                        }
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_app_state,
            apply_color_settings,
            update_app_settings,
            get_system_characteristics,
            get_system_live_metrics,
            open_driver_search,
            list_installed_games,
            launch_installed_game,
            apply_tweaks,
            get_tweak_statuses,
            list_backups,
            create_backup,
            restore_backup,
            delete_backup,
            list_configs,
            save_config,
            load_config,
            apply_config,
            delete_config,
            open_storage_folder,
            minimize_window,
            toggle_window_maximize,
            start_window_drag,
            close_window,
            exit_app,
            restart_as_admin,
            trim_memory,
            rollback_last_tweaks,
            detect_installed_apps
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Synchro");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_sanitization() {
        let state = PersistedState::default().sanitized();
        assert_eq!(state.color.saturation, 100.0);
        assert_eq!(state.color.contrast, 100.0);
        assert_eq!(state.color.gamma, 100.0);
        assert_eq!(state.color.hue, 0.0);
        assert!(state.settings.apply_instantly);
    }

    #[test]
    fn test_is_running_elevated_callable() {
        let _elevated = admin::is_running_elevated();
    }

    #[test]
    fn test_collect_system_characteristics() {
        let chars = collect_system_characteristics();
        assert!(!chars.cpu.is_empty());
        assert!(!chars.os.is_empty());
        assert!(!chars.architecture.is_empty());
    }

    #[test]
    fn test_collect_live_metrics() {
        let mut sample = None;
        let metrics = collect_live_metrics(&mut sample);
        assert!(!metrics.ram_available_gb.is_empty());
        assert!(!metrics.ram_used_gb.is_empty());
    }

    #[test]
    fn test_collect_tweak_statuses() {
        let statuses = collect_tweak_statuses();
        assert!(!statuses.is_empty());
        assert!(statuses.iter().any(|s| s.id == "game-mode-on"));
    }

    #[test]
    fn test_driver_search_query_cleaning() {
        let clean = browser::open_driver_search_url("   ");
        assert!(clean.is_err());
    }

    #[test]
    fn test_safe_file_stem() {
        assert_eq!(safe_file_stem("My Backup 2026!"), "my-backup-2026");
        assert_eq!(safe_file_stem("---test---"), "test");
        assert_eq!(safe_file_stem("   "), "item");
    }

    #[test]
    fn test_safe_file_stem_windows_reserved() {
        assert_eq!(safe_file_stem("CON"), "item");
        assert_eq!(safe_file_stem("aux"), "item");
        assert_eq!(safe_file_stem("NUL"), "item");
        assert_eq!(safe_file_stem("COM1"), "item");
        assert_eq!(safe_file_stem("LPT9"), "item");
    }

    #[test]
    fn test_color_clamping_and_sanitization() {
        let raw = ColorSettings {
            enabled: true,
            saturation: f32::NAN,
            contrast: 999.0,
            gamma: -50.0,
            hue: f32::INFINITY,
        };
        let sanitized = raw.sanitized();
        assert_eq!(sanitized.saturation, 100.0); // fallback for NaN
        assert_eq!(sanitized.contrast, 150.0);   // clamped max (50..150)
        assert_eq!(sanitized.gamma, 50.0);       // clamped min
        assert_eq!(sanitized.hue, 0.0);          // fallback for Inf
    }

    #[test]
    fn test_app_settings_serde_and_sanitization() {
        let json = r#"{"applyInstantly":false,"lowSpecMode":true,"sendCrashTelemetry":true,"language":"fr"}"#;
        let parsed: AppSettings = serde_json::from_str(json).expect("deserialize AppSettings");
        let sanitized = parsed.sanitized();
        assert!(!sanitized.apply_instantly);
        assert_eq!(sanitized.language, "en"); // "fr" sanitized to "en"
    }

    #[test]
    fn test_tweak_audit_logging() {
        let temp_dir = std::env::temp_dir().join(format!("synchro_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);
        let results = vec![
            TweakApplyResult {
                id: "test-tweak-1".to_string(),
                status: "applied".to_string(),
                message: "Tweak applied successfully".to_string(),
            },
            TweakApplyResult {
                id: "test-tweak-2".to_string(),
                status: "skipped".to_string(),
                message: "Already up to date".to_string(),
            },
        ];
        log_tweaks_audit(&temp_dir, &results);
        let log_file = temp_dir.join("tweaks_audit.log");
        assert!(log_file.exists());
        let content = fs::read_to_string(&log_file).expect("read audit log");
        assert!(content.contains("test-tweak-1"));
        assert!(content.contains("applied"));
        assert!(content.contains("test-tweak-2"));
        assert!(content.contains("skipped"));
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_trim_process_memory() {
        trim_process_memory();
    }

    #[test]
    fn test_prune_safety_backups() {
        let temp_dir = std::env::temp_dir().join(format!("synchro_prune_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);
        let snapshot = PersistedState::default();
        for i in 0..14 {
            let _ = create_backup_file(
                &temp_dir,
                format!("Safety backup {i}"),
                snapshot.clone(),
            );
        }
        let entries = read_backup_entries(&temp_dir).unwrap();
        assert_eq!(entries.len(), 14);

        prune_safety_backups(&temp_dir);
        let remaining = read_backup_entries(&temp_dir).unwrap();
        assert_eq!(remaining.len(), 10);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_detect_installed_tweak_apps() {
        let apps = system_info::detect_installed_tweak_apps();
        assert!(!apps.is_empty());
        assert!(apps.iter().any(|a| a.id == "xbox_game_bar"));
    }

    #[test]
    fn test_quote_windows_arg() {
        assert_eq!(games::quote_windows_arg(""), "\"\"");
        assert_eq!(games::quote_windows_arg("simple"), "simple");
        assert_eq!(games::quote_windows_arg("hello world"), "\"hello world\"");
        assert_eq!(games::quote_windows_arg(r#"C:\Games\Rust Game\"#), r#""C:\Games\Rust Game\\""#);
        assert_eq!(games::quote_windows_arg("bad\nstring\r"), "badstring");
        assert_eq!(games::quote_windows_arg(r#"quote "inside""#), r#""quote \"inside\"""#);
    }

    #[test]
    fn test_safe_game_url() {
        assert!(games::is_safe_game_url("steam://rungameid/730"));
        assert!(games::is_safe_game_url("steam://rungameid/123456789"));
        assert!(!games::is_safe_game_url("steam://rungameid/invalid_chars"));
        assert!(!games::is_safe_game_url("steam://open/url/calc.exe"));
        assert!(games::is_safe_game_url("com.epicgames.launcher://apps/Fortnite?action=launch"));
        assert!(games::is_safe_game_url("riotclient://launch/valorant"));
        assert!(!games::is_safe_game_url("https://malicious.site/"));
        assert!(!games::is_safe_game_url("file:///c:/windows/system32/calc.exe"));
        assert!(!games::is_safe_game_url("steam://rungameid/730 & whoami"));
        assert!(!games::is_safe_game_url("steam://rungameid/730\nmalicious"));
    }
}
