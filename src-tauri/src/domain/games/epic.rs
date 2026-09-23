#![allow(dead_code)]

use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};
use super::common::*;

pub fn epic_games() -> Vec<GameEntry> {
    let Some(program_data) = env_path("PROGRAMDATA") else {
        return Vec::new();
    };
    let manifests = program_data
        .join("Epic")
        .join("EpicGamesLauncher")
        .join("Data")
        .join("Manifests");
    let Ok(entries) = fs::read_dir(manifests) else {
        return Vec::new();
    };

    let mut games = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("item") {
            continue;
        }

        let Some(json) = read_json_value(&path) else {
            continue;
        };
        let Some(name) = json_string(&json, "DisplayName").map(clean_game_name) else {
            continue;
        };
        if name.is_empty() || is_launcher_client(&name) {
            continue;
        }

        let app_name = json_string(&json, "AppName").unwrap_or(&name);
        let install_location = json_string(&json, "InstallLocation").map(PathBuf::from);
        let launch_executable = json_string(&json, "LaunchExecutable");
        let launch_args = json_string(&json, "LaunchCommand")
            .map(split_command_line)
            .unwrap_or_default();
        let namespace = json_string(&json, "CatalogNamespace");
        let catalog_item = json_string(&json, "CatalogItemId");
        let preview = game_preview_image(&name, install_location.as_deref(), None, Vec::new());
        let epic_icon = install_location.as_ref().and_then(|root| {
            let direct_candidates = [
                root.join("icon.ico"),
                root.join("icon.png"),
                root.join("logo.png"),
                root.join(format!("{}.ico", clean_game_name(&name))),
            ];
            for cand in direct_candidates {
                if cand.is_file() {
                    return Some(cand.to_string_lossy().into_owned());
                }
            }
            None
        });
        let logo = epic_icon.or_else(|| game_logo_image(&name, install_location.as_deref(), Vec::new()));
        let launch = if let (Some(namespace), Some(catalog_item)) = (namespace, catalog_item) {
            LaunchTarget::Url(format!(
                "com.epicgames.launcher://apps/{namespace}%3A{catalog_item}%3A{app_name}?action=launch&silent=true"
            ))
        } else if let (Some(root), Some(exe)) = (install_location.as_ref(), launch_executable) {
            LaunchTarget::Exe {
                path: root.join(exe),
                args: launch_args,
                cwd: Some(root.clone()),
            }
        } else {
            LaunchTarget::None
        };

        let launchable = !matches!(launch, LaunchTarget::None);
        games.push(GameEntry {
            public: InstalledGame {
                id: stable_id("epic", &format!("{app_name}:{}", install_location_key(&install_location))),
                name,
                source: "Epic Games".to_string(),
                image_path: preview,
                logo_path: logo,
                launchable,
            },
            launch,
        });
    }

    games
}

pub fn read_json_value(path: &Path) -> Option<Value> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
}

pub fn json_string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty())
}

pub fn split_command_line(value: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for ch in value.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if ch.is_whitespace() && !in_quotes {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
            continue;
        }
        current.push(ch);
    }

    if !current.is_empty() {
        args.push(current);
    }
    args
}

pub fn install_location_key(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}
