#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};
use super::common::*;

#[cfg(target_os = "windows")]
pub fn registry_launcher_games() -> Vec<GameEntry> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    let roots = [
        (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        ),
        (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            "Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        ),
        (
            RegKey::predef(HKEY_CURRENT_USER),
            "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        ),
    ];

    let mut games = Vec::new();
    for (root, path) in roots {
        let Ok(uninstall) = root.open_subkey(path) else {
            continue;
        };
        for key_name in uninstall.enum_keys().flatten() {
            let Ok(key) = uninstall.open_subkey(&key_name) else {
                continue;
            };
            let name = reg_string(&key, "DisplayName").map(|name| clean_game_name(&name));
            let Some(name) = name.filter(|name| !name.is_empty()) else {
                continue;
            };
            let publisher = reg_string(&key, "Publisher").unwrap_or_default();
            let Some(source) = launcher_source(&name, &publisher) else {
                continue;
            };
            if is_launcher_client(&name) {
                continue;
            }

            let install_location = reg_string(&key, "InstallLocation").map(PathBuf::from);
            let display_icon = reg_string(&key, "DisplayIcon").and_then(|value| display_icon_path(&value));
            let launch_path = display_icon
                .as_ref()
                .filter(|path| path.is_file() && !ignored_executable_name(path))
                .cloned()
                .or_else(|| install_location.as_deref().and_then(|path| find_launch_executable(path, &name)));
            let preview = game_preview_image(&name, install_location.as_deref(), None, Vec::new());
            let logo = display_icon
                .as_ref()
                .filter(|p| is_logo_image_file(p) || is_image_file(p))
                .map(|p| p.to_string_lossy().into_owned())
                .or_else(|| game_logo_image(&name, install_location.as_deref(), Vec::new()));
            let launch = launch_path
                .map(|path| LaunchTarget::Exe {
                    cwd: path.parent().map(Path::to_path_buf),
                    path,
                    args: Vec::new(),
                })
                .unwrap_or(LaunchTarget::None);

            let launchable = !matches!(launch, LaunchTarget::None);
            games.push(GameEntry {
                public: InstalledGame {
                    id: stable_id(
                        "win",
                        &format!("{source}:{name}:{}", super::epic::install_location_key(&install_location)),
                    ),
                    name,
                    source,
                    image_path: preview,
                    logo_path: logo,
                    launchable,
                },
                launch,
            });
        }
    }

    games
}

#[cfg(not(target_os = "windows"))]
pub fn registry_launcher_games() -> Vec<GameEntry> {
    Vec::new()
}

#[cfg(target_os = "windows")]
fn launcher_source(name: &str, publisher: &str) -> Option<String> {
    let value = format!("{name} {publisher}").to_lowercase();
    if value.contains("gog.com") || value.contains("gog sp.") {
        Some("GOG Galaxy".to_string())
    } else if value.contains("ubisoft") {
        Some("Ubisoft Connect".to_string())
    } else if value.contains("electronic arts") || value.contains(" ea ") || value.starts_with("ea sports") {
        Some("EA app".to_string())
    } else if value.contains("blizzard") || value.contains("battle.net") {
        Some("Battle.net".to_string())
    } else if value.contains("riot games") {
        Some("Riot Games".to_string())
    } else if value.contains("rockstar") {
        Some("Rockstar Games".to_string())
    } else if value.contains("bethesda") {
        Some("Bethesda".to_string())
    } else if value.contains("itch.io") || value.contains("itch corp") {
        Some("itch.io".to_string())
    } else if value.contains("xbox game studios") || value.contains("microsoft studios") {
        Some("Xbox".to_string())
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn reg_string(key: &winreg::RegKey, name: &str) -> Option<String> {
    key.get_value::<String, _>(name).ok().filter(|value| !value.trim().is_empty())
}

pub fn display_icon_path(value: &str) -> Option<PathBuf> {
    let trimmed = value.trim().trim_matches('"');
    let without_index = trimmed
        .split_once(".exe,")
        .map(|(path, _)| format!("{path}.exe"))
        .unwrap_or_else(|| trimmed.split(',').next().unwrap_or(trimmed).to_string());
    let path = PathBuf::from(without_index.trim().trim_matches('"'));
    if path.extension().and_then(|ext| ext.to_str()).is_some() {
        Some(path)
    } else {
        None
    }
}

pub fn find_launch_executable(root: &Path, name: &str) -> Option<PathBuf> {
    if !root.is_dir() {
        return None;
    }

    let normalized_name = fileish_name(name).to_lowercase();
    let mut best: Option<(u8, u64, PathBuf)> = None;
    let mut visited = 0_usize;
    scan_executables(root, 0, &normalized_name, &mut visited, &mut best);
    best.map(|(_, _, path)| path)
}

fn scan_executables(
    path: &Path,
    depth: usize,
    normalized_name: &str,
    visited: &mut usize,
    best: &mut Option<(u8, u64, PathBuf)>,
) {
    if depth > 4 || *visited >= 1_200 || !path.is_dir() {
        return;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        if *visited >= 1_200 {
            return;
        }
        *visited += 1;
        let path = entry.path();
        if path.is_dir() {
            scan_executables(&path, depth + 1, normalized_name, visited, best);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.eq_ignore_ascii_case("exe")) != Some(true)
            || ignored_executable_name(&path)
        {
            continue;
        }

        let file_stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
            .to_lowercase();
        let score = if !normalized_name.is_empty() && normalized_name.contains(&file_stem) {
            4
        } else if depth <= 1 {
            3
        } else if path.to_string_lossy().to_lowercase().contains("win64")
            || path.to_string_lossy().to_lowercase().contains("x64")
        {
            2
        } else {
            1
        };
        let size = fs::metadata(&path).map(|metadata| metadata.len()).unwrap_or(0);
        if best
            .as_ref()
            .map(|(best_score, best_size, _)| score > *best_score || score == *best_score && size > *best_size)
            .unwrap_or(true)
        {
            *best = Some((score, size, path));
        }
    }
}
