use std::{
    collections::HashSet,
    sync::Mutex,
    time::{Duration, Instant},
};

pub use super::common::{GameEntry, InstalledGame, LaunchTarget};
pub use super::steam::{read_steam_libraries, steam_install_root};

const MAX_GAMES: usize = 180;
static GAME_CACHE: Mutex<(Vec<GameEntry>, Option<Instant>)> = Mutex::new((Vec::new(), None));
const GAME_CACHE_TTL: Duration = Duration::from_secs(60);

pub fn list_installed_games() -> Vec<InstalledGame> {
    let mut games = collect_game_entries()
        .into_iter()
        .map(|entry| entry.public)
        .collect::<Vec<_>>();
    games.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.source.cmp(&right.source))
    });
    games.truncate(MAX_GAMES);
    games
}

pub fn launch_installed_game(id: &str) -> Result<(), String> {
    let entry = collect_game_entries()
        .into_iter()
        .find(|entry| entry.public.id == id)
        .ok_or_else(|| "Game is not available anymore".to_string())?;
    super::launcher::launch_target(&entry.launch)
}

fn collect_game_entries() -> Vec<GameEntry> {
    if let Ok(guard) = GAME_CACHE.lock() {
        if let Some(updated_at) = guard.1 {
            if updated_at.elapsed() < GAME_CACHE_TTL && !guard.0.is_empty() {
                return guard.0.clone();
            }
        }
    }

    let mut entries = Vec::new();
    entries.extend(super::steam::steam_games());
    entries.extend(super::epic::epic_games());
    entries.extend(super::riot::riot_games());
    entries.extend(super::registry::registry_launcher_games());
    let deduped = dedupe_entries(entries);

    if let Ok(mut guard) = GAME_CACHE.lock() {
        *guard = (deduped.clone(), Some(Instant::now()));
    }

    deduped
}
