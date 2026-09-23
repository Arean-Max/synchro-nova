#![allow(dead_code)]

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use super::common::*;

pub fn steam_games() -> Vec<GameEntry> {
    let Some(steam_root) = steam_install_root() else {
        return Vec::new();
    };

    let mut libraries = vec![steam_root.clone()];
    libraries.extend(read_steam_libraries(&steam_root));

    let mut seen_libraries = HashSet::new();
    let mut seen_games = HashSet::new();
    let mut games = Vec::new();

    for library in libraries {
        let key = library.to_string_lossy().to_lowercase();
        if !seen_libraries.insert(key) {
            continue;
        }

        let steamapps = library.join("steamapps");
        let Ok(entries) = fs::read_dir(&steamapps) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !is_app_manifest(&path) {
                continue;
            }
            let Some((app_id, name, install_dir)) = read_app_manifest(&path) else {
                continue;
            };
            if !seen_games.insert(app_id.clone()) {
                continue;
            }

            let install_location = install_dir
                .filter(|value| !value.is_empty())
                .map(|value| steamapps.join("common").join(value));
            let fallback = steam_library_image_path(&steam_root, &app_id);
            let logo = steam_library_logo_path(&steam_root, &app_id)
                .or_else(|| game_logo_image(&name, install_location.as_deref(), Vec::new()));
            let preview = game_preview_image(
                &name,
                install_location.as_deref(),
                fallback,
                steam_screenshot_roots(&steam_root, &app_id),
            );

            games.push(GameEntry {
                public: InstalledGame {
                    id: format!("steam:{app_id}"),
                    name,
                    source: "Steam".to_string(),
                    image_path: preview,
                    logo_path: logo,
                    launchable: true,
                },
                launch: LaunchTarget::Url(format!("steam://rungameid/{app_id}")),
            });
        }
    }

    games
}

pub fn is_app_manifest(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    name.starts_with("appmanifest_") && name.ends_with(".acf")
}

pub fn read_app_manifest(path: &Path) -> Option<(String, String, Option<String>)> {
    let raw = fs::read_to_string(path).ok()?;
    let mut app_id = String::new();
    let mut name = String::new();
    let mut install_dir = None;

    for line in raw.lines() {
        let values = quoted_values(line);
        if values.len() < 2 {
            continue;
        }
        match values[0].as_str() {
            "appid" => app_id = values[1].trim().to_string(),
            "name" => name = clean_game_name(&values[1]),
            "installdir" => install_dir = Some(values[1].trim().to_string()),
            _ => {}
        }
    }

    if app_id.is_empty()
        || !app_id.chars().all(|ch| ch.is_ascii_digit())
        || name.is_empty()
    {
        None
    } else {
        Some((app_id, name, install_dir))
    }
}

pub fn read_steam_libraries(steam_root: &Path) -> Vec<PathBuf> {
    let path = steam_root.join("steamapps").join("libraryfolders.vdf");
    let Ok(raw) = fs::read_to_string(path) else {
        return Vec::new();
    };

    raw.lines()
        .filter_map(|line| {
            let values = quoted_values(line);
            if values.len() >= 2
                && (values[0] == "path"
                    || (values[0].chars().all(|ch| ch.is_ascii_digit())
                        && looks_like_path(&values[1])))
            {
                Some(PathBuf::from(&values[1]))
            } else {
                None
            }
        })
        .collect()
}

fn looks_like_path(value: &str) -> bool {
    value.contains(":\\")
        || value.contains(":/")
        || value.starts_with("\\\\")
        || value.starts_with('/')
}

fn quoted_values(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;

    for ch in line.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        if in_quote {
            if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                values.push(current.clone());
                current.clear();
                in_quote = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' {
            in_quote = true;
        }
    }

    values
}

fn steam_screenshot_roots(steam_root: &Path, app_id: &str) -> Vec<PathBuf> {
    let userdata = steam_root.join("userdata");
    let Ok(users) = fs::read_dir(userdata) else {
        return Vec::new();
    };

    users
        .flatten()
        .map(|user| {
            user.path()
                .join(app_id)
                .join("remote")
                .join(app_id)
                .join("screenshots")
        })
        .collect()
}

fn steam_library_image_path(steam_root: &Path, app_id: &str) -> Option<String> {
    let cache = steam_root.join("appcache").join("librarycache");
    let app_dir = cache.join(app_id);

    let candidates = [
        app_dir.join("library_600x900.jpg"),
        app_dir.join("library_600x900.png"),
        cache.join(format!("{app_id}_library_600x900.jpg")),
        cache.join(format!("{app_id}_library_600x900.png")),
        app_dir.join("library_capsule.jpg"),
        app_dir.join("library_hero.jpg"),
        app_dir.join("library_header.jpg"),
        app_dir.join("header.jpg"),
        cache.join(format!("{app_id}_header.jpg")),
        cache.join(format!("{app_id}_header.png")),
    ];

    for path in candidates {
        if path.is_file() {
            return Some(path.to_string_lossy().into_owned());
        }
    }

    if app_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&app_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let sub_candidates = [
                        p.join("library_600x900.jpg"),
                        p.join("library_capsule.jpg"),
                        p.join("library_header.jpg"),
                        p.join("library_hero.jpg"),
                        p.join("header.jpg"),
                    ];
                    for sub in sub_candidates {
                        if sub.is_file() {
                            return Some(sub.to_string_lossy().into_owned());
                        }
                    }
                } else if p.is_file() && is_image_file(&p) {
                    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_lowercase();
                    if stem.contains("library") || stem.contains("header") || stem.contains("capsule") {
                        return Some(p.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    None
}

fn steam_library_logo_path(steam_root: &Path, app_id: &str) -> Option<String> {
    let cache = steam_root.join("appcache").join("librarycache");
    let app_dir = cache.join(app_id);

    let candidates = [
        app_dir.join("logo.png"),
        app_dir.join("logo.webp"),
        app_dir.join("logo.jpg"),
        cache.join(format!("{app_id}_logo.png")),
        cache.join(format!("{app_id}_logo.webp")),
        cache.join(format!("{app_id}_logo.jpg")),
        app_dir.join("icon.png"),
        app_dir.join("icon.jpg"),
        app_dir.join("icon.ico"),
        cache.join(format!("{app_id}_icon.png")),
        cache.join(format!("{app_id}_icon.jpg")),
    ];

    for path in candidates {
        if path.is_file() {
            return Some(path.to_string_lossy().into_owned());
        }
    }

    if app_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&app_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let sub_candidates = [
                        p.join("logo.png"),
                        p.join("logo.webp"),
                        p.join("logo.jpg"),
                        p.join("icon.png"),
                        p.join("icon.ico"),
                    ];
                    for sub in sub_candidates {
                        if sub.is_file() {
                            return Some(sub.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
pub fn steam_install_root() -> Option<PathBuf> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    let roots = [
        (
            RegKey::predef(HKEY_CURRENT_USER),
            "Software\\Valve\\Steam",
        ),
        (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            "SOFTWARE\\Valve\\Steam",
        ),
        (
            RegKey::predef(HKEY_LOCAL_MACHINE),
            "SOFTWARE\\WOW6432Node\\Valve\\Steam",
        ),
    ];

    for (root, subkey) in roots {
        let Ok(key) = root.open_subkey(subkey) else {
            continue;
        };
        if let Some(root) = steam_root_from_key(&key) {
            return Some(root);
        }
    }

    if let Some(path) = env_path("ProgramFiles(x86)").map(|p| p.join("Steam")) {
        if path.exists() {
            return Some(path);
        }
    }
    if let Some(path) = env_path("ProgramFiles").map(|p| p.join("Steam")) {
        if path.exists() {
            return Some(path);
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn steam_root_from_key(key: &winreg::RegKey) -> Option<PathBuf> {
    if let Ok(path) = key.get_value::<String, _>("SteamPath") {
        let root = PathBuf::from(path);
        if root.exists() {
            return Some(root);
        }
    }
    if let Ok(exe) = key.get_value::<String, _>("SteamExe") {
        let root = PathBuf::from(exe).parent().map(Path::to_path_buf)?;
        if root.exists() {
            return Some(root);
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
pub fn steam_install_root() -> Option<PathBuf> {
    None
}
