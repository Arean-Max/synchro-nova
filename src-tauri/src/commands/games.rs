use tauri::Manager;
use crate::domain::games::{
    launch_installed_game as domain_launch_game, list_installed_games as domain_list_games,
    set_rust_holosight_black, BlackHoloConfigResult, InstalledGame,
};

#[tauri::command]
pub fn list_installed_games(app: tauri::AppHandle) -> Vec<InstalledGame> {
    let games = domain_list_games();
    let scope = app.asset_protocol_scope();
    for game in &games {
        if let Some(ref img) = game.image_path {
            let _ = scope.allow_file(std::path::Path::new(img));
        }
        if let Some(ref logo) = game.logo_path {
            let _ = scope.allow_file(std::path::Path::new(logo));
        }
    }
    games
}

#[tauri::command]
pub fn launch_installed_game(id: String) -> Result<(), String> {
    domain_launch_game(&id)
}

#[tauri::command]
pub fn set_rust_black_holo(enabled: bool) -> BlackHoloConfigResult {
    set_rust_holosight_black(enabled)
}
