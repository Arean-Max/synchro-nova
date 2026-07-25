
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
