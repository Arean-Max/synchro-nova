pub mod common;
pub mod detector;
pub mod epic;
pub mod launcher;
pub mod registry;
pub mod riot;
pub mod steam;

pub use detector::{list_installed_games, launch_installed_game, InstalledGame};
pub use steam::{read_steam_libraries, steam_install_root};
