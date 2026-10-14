use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::app::state::ColorSettings;
use crate::domain::games::InstalledGame;

pub const FAST_BOOT_MAGIC: &[u8; 4] = b"SNFB";
pub const FAST_BOOT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastBootSnapshot {
    pub magic: [u8; 4],
    pub version: u32,
    pub timestamp: u64,
    pub color: ColorSettings,
    pub precomputed_matrix: [f32; 25],
    pub show_on_recordings: bool,
    pub gpu_vendor: String,
    pub gpu_name: String,
    pub active_tweak_ids: Vec<String>,
    pub games: Vec<InstalledGame>,
}

static EARLY_WARMUP_SNAPSHOT: OnceLock<Option<FastBootSnapshot>> = OnceLock::new();
static PERSIST_LOCK: Mutex<()> = Mutex::new(());

pub fn get_fast_boot_path() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|p| {
            let dir = p.join("SynchroNova").join("cache");
            let _ = fs::create_dir_all(&dir);
            dir.join("fast_boot.bin")
        })
}

#[cfg(target_os = "windows")]
pub fn try_early_warmup() -> bool {
    let Some(path) = get_fast_boot_path() else {
        return false;
    };
    if !path.is_file() {
        return false;
    }

    let Ok(bytes) = fs::read(&path) else {
        return false;
    };

    if bytes.len() < 16 || &bytes[0..4] != FAST_BOOT_MAGIC {
        return false;
    }

    let Ok(snapshot) = bincode::deserialize::<FastBootSnapshot>(&bytes) else {
        return false;
    };

    if snapshot.version != FAST_BOOT_VERSION {
        return false;
    }

    // Instantly apply hardware display gamma & color transform directly in < 5ms
    if snapshot.color.enabled {
        let _ = crate::domain::color::apply_color_transform(
            &snapshot.color,
            snapshot.show_on_recordings,
        );
        if snapshot.color.black_holo > 0.0 {
            let _ = crate::domain::color::black_holo::set_hardware_black_holo(true);
        }
    }

    let _ = EARLY_WARMUP_SNAPSHOT.set(Some(snapshot));
    true
}

#[cfg(not(target_os = "windows"))]
pub fn try_early_warmup() -> bool {
    false
}

pub fn get_cached_snapshot() -> Option<FastBootSnapshot> {
    if let Some(Some(snapshot)) = EARLY_WARMUP_SNAPSHOT.get() {
        return Some(snapshot.clone());
    }

    let path = get_fast_boot_path()?;
    if !path.is_file() {
        return None;
    }
    let bytes = fs::read(&path).ok()?;
    if bytes.len() < 16 || &bytes[0..4] != FAST_BOOT_MAGIC {
        return None;
    }
    let snapshot = bincode::deserialize::<FastBootSnapshot>(&bytes).ok()?;
    if snapshot.version != FAST_BOOT_VERSION {
        return None;
    }
    Some(snapshot)
}

pub fn update_disk_cache(
    color: &ColorSettings,
    show_on_recordings: bool,
    games_override: Option<Vec<InstalledGame>>,
    tweaks_override: Option<Vec<String>>,
) {
    let _lock = match PERSIST_LOCK.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };

    let Some(target_path) = get_fast_boot_path() else {
        return;
    };

    let existing = get_cached_snapshot();
    let (vendor, gpu_name) = crate::domain::color::black_holo::detect_gpu_vendor();
    let matrix = crate::domain::color::transform::build_color_matrix(color, show_on_recordings);

    let games = games_override
        .or_else(|| existing.as_ref().map(|s| s.games.clone()))
        .unwrap_or_default();

    let active_tweak_ids = tweaks_override
        .or_else(|| existing.as_ref().map(|s| s.active_tweak_ids.clone()))
        .unwrap_or_default();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let snapshot = FastBootSnapshot {
        magic: *FAST_BOOT_MAGIC,
        version: FAST_BOOT_VERSION,
        timestamp,
        color: color.clone(),
        precomputed_matrix: matrix,
        show_on_recordings,
        gpu_vendor: vendor.as_str().to_string(),
        gpu_name,
        active_tweak_ids,
        games,
    };

    if let Ok(encoded) = bincode::serialize(&snapshot) {
        let temp_path = target_path.with_extension("bin.tmp");
        if fs::write(&temp_path, &encoded).is_ok() {
            let _ = fs::rename(&temp_path, &target_path);
        }
    }
}
