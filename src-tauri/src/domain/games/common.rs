
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const MAX_IMAGE_SCAN_DEPTH: usize = 2;
pub const MAX_IMAGE_SCAN_ENTRIES: usize = 150;
pub const MAX_LOGO_SCAN_DEPTH: usize = 2;
pub const MAX_LOGO_SCAN_ENTRIES: usize = 150;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledGame {
    pub id: String,
    pub name: String,
    pub source: String,
    pub image_path: Option<String>,
    pub logo_path: Option<String>,
    pub launchable: bool,
}

#[derive(Debug, Clone)]
pub struct GameEntry {
    pub public: InstalledGame,
    pub launch: LaunchTarget,
}

#[derive(Debug, Clone)]
pub enum LaunchTarget {
    Url(String),
    Exe {
        path: PathBuf,
        args: Vec<String>,
        cwd: Option<PathBuf>,
    },
    None,
}

pub fn clean_game_name(name: &str) -> String {
    name.chars()
        .filter(|ch| !ch.is_control())
        .take(96)
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn fileish_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn stable_id(prefix: &str, key: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in key.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{prefix}:{hash:016x}")
}

pub fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from).filter(|path| path.exists())
}

pub fn is_launcher_client(name: &str) -> bool {
    let value = name.to_lowercase();
    [
        "launcher",
        "game launcher",
        "ubisoft connect",
        "ubisoft game launcher",
        "gog galaxy",
        "battle.net",
        "ea app",
        "origin",
        "riot client",
        "rockstar games launcher",
        "social club",
        "steam",
        "epic games launcher",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

pub fn ignored_executable_name(path: &Path) -> bool {
    let value = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_lowercase();
    [
        "unins",
        "uninstall",
        "setup",
        "install",
        "redist",
        "vcredist",
        "dxsetup",
        "crash",
        "report",
        "helper",
        "update",
        "launcher",
        "bootstrap",
        "service",
        "unitycrashhandler",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

pub fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp")
    )
}

pub fn is_logo_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp" | "ico" | "svg")
    )
}

pub fn is_screenshotish_path(path: &Path) -> bool {
    let value = path.to_string_lossy().to_lowercase();
    value.contains("screenshot")
        || value.contains("screen shot")
        || value.contains("screenshots")
        || value.contains("capture")
        || value.contains("captures")
        || value.contains("photo")
        || value.contains("gallery")
}

pub fn game_preview_image(
    name: &str,
    install_location: Option<&Path>,
    fallback: Option<String>,
    extra_roots: Vec<PathBuf>,
) -> Option<String> {
    if fallback.is_some() {
        return fallback;
    }

    let mut roots = Vec::new();
    if let Some(path) = install_location {
        roots.push(path.to_path_buf());
    }
    roots.extend(extra_roots);
    roots.extend(user_game_image_roots(name));

    find_best_screenshot(&roots)
}

pub fn game_logo_image(
    name: &str,
    install_location: Option<&Path>,
    extra_roots: Vec<PathBuf>,
) -> Option<String> {
    let mut roots = Vec::new();
    if let Some(path) = install_location {
        roots.push(path.to_path_buf());
    }
    roots.extend(extra_roots);
    find_best_logo(name, &roots)
}

pub fn find_best_screenshot(roots: &[PathBuf]) -> Option<String> {
    let mut best: Option<(u64, PathBuf)> = None;
    let mut visited = 0_usize;

    for root in roots {
        scan_image_dir(root, 0, &mut visited, &mut best);
        if visited >= MAX_IMAGE_SCAN_ENTRIES {
            break;
        }
    }

    best.map(|(_, path)| path.to_string_lossy().into_owned())
}

pub fn find_best_logo(name: &str, roots: &[PathBuf]) -> Option<String> {
    let mut best: Option<(i32, u64, PathBuf)> = None;
    let mut visited = 0_usize;
    let clean_name = fileish_name(name).to_lowercase();

    for root in roots {
        scan_logo_dir(root, 0, &mut visited, &mut best, &clean_name);
        if visited >= MAX_LOGO_SCAN_ENTRIES {
            break;
        }
    }

    best.map(|(_, _, path)| path.to_string_lossy().into_owned())
}
