pub mod common;
pub mod detector;
pub mod epic;
pub mod launcher;
pub mod registry;
pub mod riot;
pub mod rust;
pub mod steam;

pub use detector::{list_installed_games, launch_installed_game, InstalledGame};
pub use rust::{is_rust_process_running, set_rust_holosight_black, BlackHoloConfigResult};
pub use steam::{read_steam_libraries, steam_install_root};
