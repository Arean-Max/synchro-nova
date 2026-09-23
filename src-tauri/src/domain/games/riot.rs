#![allow(dead_code)]

use std::path::{Path, PathBuf};
use super::common::*;

pub fn riot_games() -> Vec<GameEntry> {
    let Some(riot_client) = riot_client_path() else {
        return Vec::new();
    };
    let root = riot_client
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .or_else(|| system_drive_root().map(|drive| drive.join("Riot Games")));
    let Some(root) = root else {
        return Vec::new();
    };

    let mut games = Vec::new();
    let league = root.join("League of Legends");
    if league.is_dir() {
        let name = "League of Legends".to_string();
        games.push(GameEntry {
            public: InstalledGame {
                id: "riot:league_of_legends".to_string(),
                name: name.clone(),
                source: "Riot Games".to_string(),
                image_path: game_preview_image(&name, Some(&league), None, Vec::new()),
                logo_path: game_logo_image(&name, Some(&league), Vec::new()),
                launchable: true,
            },
            launch: LaunchTarget::Exe {
                path: riot_client.clone(),
                args: vec![
                    "--launch-product=league_of_legends".to_string(),
                    "--launch-patchline=live".to_string(),
                ],
                cwd: riot_client.parent().map(Path::to_path_buf),
            },
        });
    }

    let valorant = root.join("VALORANT").join("live");
    if valorant.is_dir() {
        let name = "VALORANT".to_string();
        games.push(GameEntry {
            public: InstalledGame {
                id: "riot:valorant".to_string(),
                name: name.clone(),
                source: "Riot Games".to_string(),
                image_path: game_preview_image(&name, Some(&valorant), None, Vec::new()),
                logo_path: game_logo_image(&name, Some(&valorant), Vec::new()),
                launchable: true,
            },
            launch: LaunchTarget::Exe {
                path: riot_client.clone(),
                args: vec![
                    "--launch-product=valorant".to_string(),
                    "--launch-patchline=live".to_string(),
                ],
                cwd: riot_client.parent().map(Path::to_path_buf),
            },
        });
    }

    games
}

pub fn riot_client_path() -> Option<PathBuf> {
    let program_data = env_path("PROGRAMDATA")?;
    let installs = program_data
        .join("Riot Games")
        .join("RiotClientInstalls.json");
    let json = super::epic::read_json_value(&installs)?;
    super::epic::json_string(&json, "rc_default")
        .map(PathBuf::from)
        .filter(|path| path.is_file())
        .or_else(|| {
            system_drive_root()
                .map(|drive| drive.join("Riot Games").join("Riot Client").join("RiotClientServices.exe"))
                .filter(|path| path.is_file())
        })
}

pub fn system_drive_root() -> Option<PathBuf> {
    let drive = std::env::var("SystemDrive").ok()?;
    let root = PathBuf::from(format!("{drive}\\"));
    root.exists().then_some(root)
}
