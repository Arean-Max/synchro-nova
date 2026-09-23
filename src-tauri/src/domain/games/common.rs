#![allow(dead_code)]

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

pub fn scan_image_dir(
    path: &Path,
    depth: usize,
    visited: &mut usize,
    best: &mut Option<(u64, PathBuf)>,
) {
    if depth > MAX_IMAGE_SCAN_DEPTH || *visited >= MAX_IMAGE_SCAN_ENTRIES || !path.is_dir() {
        return;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        if *visited >= MAX_IMAGE_SCAN_ENTRIES {
            return;
        }
        *visited += 1;
        let path = entry.path();
        if path.is_dir() {
            scan_image_dir(&path, depth + 1, visited, best);
            continue;
        }
        if !is_image_file(&path) || !is_screenshotish_path(&path) {
            continue;
        }
        let size = fs::metadata(&path).map(|metadata| metadata.len()).unwrap_or(0);
        if size > best.as_ref().map(|(best_size, _)| *best_size).unwrap_or(0) {
            *best = Some((size, path));
        }
    }
}

pub fn scan_logo_dir(
    path: &Path,
    depth: usize,
    visited: &mut usize,
    best: &mut Option<(i32, u64, PathBuf)>,
    clean_name: &str,
) {
    if depth > MAX_LOGO_SCAN_DEPTH || *visited >= MAX_LOGO_SCAN_ENTRIES || !path.is_dir() {
        return;
    }
    if is_screenshotish_path(path) {
        return;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        if *visited >= MAX_LOGO_SCAN_ENTRIES {
            return;
        }
        *visited += 1;
        let path = entry.path();
        if path.is_dir() {
            scan_logo_dir(&path, depth + 1, visited, best, clean_name);
            continue;
        }
        if !is_logo_image_file(&path) {
            continue;
        }
        let score = logo_path_score(&path, clean_name);
        if score <= 0 {
            continue;
        }
        let size = fs::metadata(&path).map(|metadata| metadata.len()).unwrap_or(0);
        if best
            .as_ref()
            .map(|(best_score, best_size, _)| score > *best_score || (score == *best_score && size > *best_size))
            .unwrap_or(true)
        {
            *best = Some((score, size, path));
        }
    }
}

pub fn logo_path_score(path: &Path, clean_name: &str) -> i32 {
    let value = path.to_string_lossy().to_lowercase();
    if is_screenshotish_path(path)
        || value.contains("uninstall")
        || value.contains("setup")
        || value.contains("cursor")
        || value.contains("loading")
    {
        return 0;
    }

    let file_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_lowercase();
    let mut score = 0;
    for (needle, points) in [
        ("logo", 120),
        ("logotype", 120),
        ("icon", 88),
        ("banner", 68),
        ("header", 60),
        ("capsule", 54),
        ("library", 48),
        ("cover", 34),
        ("poster", 28),
        ("splash", 24),
    ] {
        if value.contains(needle) {
            score = score.max(points);
        }
    }
    if !clean_name.is_empty() && fileish_name(&file_name).to_lowercase().contains(clean_name) {
        score += 12;
    }
    score
}

pub fn user_game_image_roots(name: &str) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let Some(home) = env_path("USERPROFILE") else {
        return roots;
    };
    let clean = fileish_name(name);
    roots.push(home.join("Documents").join("My Games").join(name));
    roots.push(home.join("Documents").join("My Games").join(&clean));
    roots.push(home.join("Documents").join(name));
    roots.push(home.join("Pictures").join(name));
    roots.push(home.join("Videos").join(name));
    roots
}
