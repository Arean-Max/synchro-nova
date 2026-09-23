use crate::domain::games::{
    launch_installed_game as domain_launch_game, list_installed_games as domain_list_games,
    set_rust_holosight_black, BlackHoloConfigResult, InstalledGame,
};

#[tauri::command]
pub fn list_installed_games() -> Vec<InstalledGame> {
    domain_list_games()
}

#[tauri::command]
pub fn launch_installed_game(id: String) -> Result<(), String> {
    domain_launch_game(&id)
}

#[tauri::command]
pub fn set_rust_black_holo(enabled: bool) -> BlackHoloConfigResult {
    set_rust_holosight_black(enabled)
}
