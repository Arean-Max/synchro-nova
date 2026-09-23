use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

use crate::admin::is_running_elevated;
use crate::color;
use crate::platform::live_metrics::CpuSample;

pub const MAX_JSON_FILE_BYTES: u64 = 256 * 1024;
pub const MAX_JSON_STRING_BYTES: usize = MAX_JSON_FILE_BYTES as usize;
pub const MAX_DISPLAY_NAME_CHARS: usize = 64;
pub const MAX_STORAGE_ENTRIES: usize = 200;
pub const MAX_STORAGE_SCAN_ENTRIES: usize = 1_000;
pub const RANDOM_SUFFIX_BYTES: usize = 6;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ColorSettings {
    pub saturation: f32,
    pub hue: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub black_holo: f32,
    pub enabled: bool,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            saturation: 100.0,
            hue: 0.0,
            contrast: 100.0,
            gamma: 100.0,
            black_holo: 0.0,
            enabled: true,
        }
    }
}

impl ColorSettings {
    pub fn sanitized(mut self) -> Self {
        self.saturation = clamp_finite(self.saturation, 0.0, 200.0, 100.0);
        self.hue = clamp_finite(self.hue, -180.0, 180.0, 0.0);
        self.contrast = clamp_finite(self.contrast, 50.0, 150.0, 100.0);
        self.gamma = clamp_finite(self.gamma, 50.0, 150.0, 100.0);
        self.black_holo = clamp_finite(self.black_holo, 0.0, 100.0, 0.0);
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
    pub show_on_recordings: bool,
    pub accent_color: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            apply_instantly: true,
            save_color_correction: true,
            autostart_windows: false,
            close_to_tray: true,
            start_minimized: false,
            auto_backup_on_start: false,
            accepted_agreement: true,
            language: "en".to_string(),
            show_on_recordings: true,
            accent_color: "#ffffff".to_string(),
        }
    }
}

impl AppSettings {
    pub fn sanitized(mut self) -> Self {
        if self.language != "ru" {
            self.language = "en".to_string();
        }
        if self.accent_color.trim().is_empty() {
            self.accent_color = "#ffffff".to_string();
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct PersistedState {
    pub color: ColorSettings,
    pub settings: AppSettings,
    #[serde(default)]
    pub is_admin: bool,
}

impl PersistedState {
    pub fn sanitized(mut self) -> Self {
        self.color = self.color.sanitized();
        self.settings = self.settings.sanitized();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupFile {
    pub name: String,
    pub created_at: u64,
    pub state: PersistedState,
    #[serde(default)]
    pub tweak_registry: Vec<crate::tweaks::TweakRegistrySnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupEntry {
    pub id: String,
    pub name: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct ConfigFile {
    pub color: ColorSettings,
    pub settings: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigEntry {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub color: ColorSettings,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverInfo {
    pub name: String,
    pub provider: String,
    pub version: String,
    pub date: String,
    pub class_name: String,
    pub status: String,
    pub path: String,
    pub search_query: String,
    pub hardware_id: String,
    pub vendor: String,
    pub is_outdated: bool,
    pub official_url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemCharacteristics {
    pub cpu: String,
    pub cpu_usage_percent: String,
    pub gpu: String,
    pub gpu_usage_percent: String,
    pub gpus: Vec<String>,
    pub ram: String,
    pub ram_available: String,
    pub ram_available_gb: String,
    pub ram_used_gb: String,
    pub ram_used_percent: String,
    pub os: String,
    pub architecture: String,
    pub display: String,
    pub refresh_rate: String,
    pub color_depth: String,
    pub driver: String,
    pub cpu_cores: String,
    pub gpu_count: String,
    pub ram_total_gb: String,
    pub vram_total_gb: String,
    pub vram_used_gb: String,
    pub vram_used_percent: String,
    pub refresh_rate_hz: String,
    pub drivers: Vec<DriverInfo>,
}

#[derive(Default)]
pub struct SystemCache {
    pub value: Option<SystemCharacteristics>,
    pub updated_at: Option<Instant>,
}

pub struct TrimmerSync {
    pub exited: Mutex<bool>,
    pub condvar: Condvar,
}

pub struct RuntimeState {
    pub app_dir: PathBuf,
    pub settings_path: PathBuf,
    pub backups_dir: PathBuf,
    pub configs_dir: PathBuf,
    pub data: Mutex<PersistedState>,
    pub system_cache: Mutex<SystemCache>,
    pub cpu_sample: Mutex<Option<CpuSample>>,
    pub exiting: Arc<AtomicBool>,
    pub trimmer_sync: Arc<TrimmerSync>,
}

impl RuntimeState {
    pub fn load(app: &AppHandle) -> Result<Self, String> {
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
            exiting: Arc::new(AtomicBool::new(false)),
            trimmer_sync: Arc::new(TrimmerSync {
                exited: Mutex::new(false),
                condvar: Condvar::new(),
            }),
        })
    }

    pub fn signal_shutdown(&self) {
        self.exiting.store(true, Ordering::SeqCst);
        if let Ok(mut lock) = self.trimmer_sync.exited.lock() {
            *lock = true;
            self.trimmer_sync.condvar.notify_all();
        }
        color::stop_color_guard();
    }

    pub fn snapshot(&self) -> Result<PersistedState, String> {
        let mut state = self.data
            .lock()
            .map_err(|_| "Settings lock poisoned".to_string())
            .map(|guard| guard.clone())?;
        state.is_admin = is_running_elevated();
        Ok(state)
    }

    pub fn save(&self, data: &PersistedState) -> Result<(), String> {
        write_json_file(&self.settings_path, data)
    }
}

pub fn clamp_finite(value: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

pub fn read_json_file<T>(path: &Path) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    validate_json_file(path)?;
    let raw = fs::read_to_string(path).map_err(|error| format!("Failed to read file: {error}"))?;
    serde_json::from_str(&raw).map_err(|error| format!("Failed to parse JSON: {error}"))
}

pub fn write_json_file<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    write_json_file_with_mode(path, value, true)
}

pub fn write_new_json_file<T>(path: &Path, value: &T) -> Result<(), String>
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

pub fn checked_child_json_path(root: &Path, id: &str) -> Result<PathBuf, String> {
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

pub fn validate_json_file(path: &Path) -> Result<(), String> {
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

pub fn clean_display_name(name: String) -> String {
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

pub fn display_name_from_stem(stem: &str) -> String {
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

pub fn file_timestamp(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or_else(unix_now)
}

pub fn is_reserved_windows_name(name: &str) -> bool {
    const RESERVED: &[&str] = &[
        "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7",
        "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8",
        "lpt9",
    ];
    RESERVED.contains(&name)
}

pub fn safe_file_stem(input: &str) -> String {
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

pub fn unique_available_stem(root: &Path, preferred: &str) -> Result<String, String> {
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

pub fn unique_id(name: &str, created_at: u64) -> Result<String, String> {
    Ok(format!(
        "{}-{created_at}-{}",
        safe_file_stem(name),
        random_hex(RANDOM_SUFFIX_BYTES)?
    ))
}

pub fn random_hex(byte_count: usize) -> Result<String, String> {
    let mut bytes = vec![0_u8; byte_count];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("Failed to generate random id: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

pub fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

pub fn short_date(seconds: u64) -> String {
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
