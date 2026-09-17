use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

const MAX_GAMES: usize = 180;
const MAX_IMAGE_SCAN_DEPTH: usize = 2;
const MAX_IMAGE_SCAN_ENTRIES: usize = 150;
const MAX_LOGO_SCAN_DEPTH: usize = 2;
const MAX_LOGO_SCAN_ENTRIES: usize = 150;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledGame {
    id: String,
    name: String,
    source: String,
    image_path: Option<String>,
    logo_path: Option<String>,
    launchable: bool,
}

#[derive(Debug, Clone)]
struct GameEntry {
    public: InstalledGame,
    launch: LaunchTarget,
}

#[derive(Debug, Clone)]
enum LaunchTarget {
    Url(String),
    Exe {
        path: PathBuf,
        args: Vec<String>,
        cwd: Option<PathBuf>,
    },
    None,
}

pub(crate) fn list_installed_games() -> Vec<InstalledGame> {
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

pub(crate) fn launch_installed_game(id: &str) -> Result<(), String> {
    let entry = collect_game_entries()
        .into_iter()
        .find(|entry| entry.public.id == id)
        .ok_or_else(|| "Game is not available anymore".to_string())?;
    launch_target(&entry.launch)
}

fn collect_game_entries() -> Vec<GameEntry> {
    let mut entries = Vec::new();
    entries.extend(steam_games());
    entries.extend(epic_games());
    entries.extend(riot_games());
    entries.extend(registry_launcher_games());
    dedupe_entries(entries)
}

fn dedupe_entries(entries: Vec<GameEntry>) -> Vec<GameEntry> {
    let mut ids = HashSet::new();
    let mut installed_keys = HashSet::new();
    let mut output = Vec::new();

    for entry in entries {
        let id_key = entry.public.id.to_lowercase();
        if !ids.insert(id_key) {
            continue;
        }

        let install_key = launch_install_key(&entry.launch)
            .unwrap_or_else(|| format!("{}:{}", entry.public.source, entry.public.name))
            .to_lowercase();
        if !installed_keys.insert(install_key) {
            continue;
        }

        output.push(entry);
    }

    output
}

fn launch_install_key(launch: &LaunchTarget) -> Option<String> {
    match launch {
        LaunchTarget::Exe { path, .. } => path.parent().map(|path| path.to_string_lossy().into_owned()),
        LaunchTarget::Url(url) => Some(url.clone()),
        LaunchTarget::None => None,
    }
}

fn steam_games() -> Vec<GameEntry> {
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

fn epic_games() -> Vec<GameEntry> {
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

fn riot_games() -> Vec<GameEntry> {
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

fn is_app_manifest(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    name.starts_with("appmanifest_") && name.ends_with(".acf")
}

fn read_app_manifest(path: &Path) -> Option<(String, String, Option<String>)> {
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

fn clean_game_name(name: &str) -> String {
    name.chars()
        .filter(|ch| !ch.is_control())
        .take(96)
        .collect::<String>()
        .trim()
        .to_string()
}

fn read_steam_libraries(steam_root: &Path) -> Vec<PathBuf> {
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

fn game_preview_image(
    name: &str,
    install_location: Option<&Path>,
    fallback: Option<String>,
    extra_roots: Vec<PathBuf>,
) -> Option<String> {
    if fallback.is_some() {
        return fallback;
    }

    let mut roots = Vec::new();
    if let Some(path) = install_location {
        roots.push(path.to_path_buf());
    }
    roots.extend(extra_roots);
    roots.extend(user_game_image_roots(name));

    find_best_screenshot(&roots)
}

fn game_logo_image(
    name: &str,
    install_location: Option<&Path>,
    extra_roots: Vec<PathBuf>,
) -> Option<String> {
    let mut roots = Vec::new();
    if let Some(path) = install_location {
        roots.push(path.to_path_buf());
    }
    roots.extend(extra_roots);
    find_best_logo(name, &roots)
}

fn find_best_screenshot(roots: &[PathBuf]) -> Option<String> {
    let mut best: Option<(u64, PathBuf)> = None;
    let mut visited = 0_usize;

    for root in roots {
        scan_image_dir(root, 0, &mut visited, &mut best);
        if visited >= MAX_IMAGE_SCAN_ENTRIES {
            break;
        }
    }

    best.map(|(_, path)| path.to_string_lossy().into_owned())
}

fn find_best_logo(name: &str, roots: &[PathBuf]) -> Option<String> {
    let mut best: Option<(i32, u64, PathBuf)> = None;
    let mut visited = 0_usize;
    let clean_name = fileish_name(name).to_lowercase();

    for root in roots {
        scan_logo_dir(root, 0, &mut visited, &mut best, &clean_name);
        if visited >= MAX_LOGO_SCAN_ENTRIES {
            break;
        }
    }

    best.map(|(_, _, path)| path.to_string_lossy().into_owned())
}

fn scan_image_dir(
    path: &Path,
    depth: usize,
    visited: &mut usize,
    best: &mut Option<(u64, PathBuf)>,
) {
    if depth > MAX_IMAGE_SCAN_DEPTH || *visited >= MAX_IMAGE_SCAN_ENTRIES || !path.is_dir() {
        return;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        if *visited >= MAX_IMAGE_SCAN_ENTRIES {
            return;
        }
        *visited += 1;
        let path = entry.path();
        if path.is_dir() {
            scan_image_dir(&path, depth + 1, visited, best);
            continue;
        }
        if !is_image_file(&path) || !is_screenshotish_path(&path) {
            continue;
        }
        let size = fs::metadata(&path).map(|metadata| metadata.len()).unwrap_or(0);
        if size > best.as_ref().map(|(best_size, _)| *best_size).unwrap_or(0) {
            *best = Some((size, path));
        }
    }
}

fn scan_logo_dir(
    path: &Path,
    depth: usize,
    visited: &mut usize,
    best: &mut Option<(i32, u64, PathBuf)>,
    clean_name: &str,
) {
    if depth > MAX_LOGO_SCAN_DEPTH || *visited >= MAX_LOGO_SCAN_ENTRIES || !path.is_dir() {
        return;
    }
    if is_screenshotish_path(path) {
        return;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        if *visited >= MAX_LOGO_SCAN_ENTRIES {
            return;
        }
        *visited += 1;
        let path = entry.path();
        if path.is_dir() {
            scan_logo_dir(&path, depth + 1, visited, best, clean_name);
            continue;
        }
        if !is_logo_image_file(&path) {
            continue;
        }
        let score = logo_path_score(&path, clean_name);
        if score <= 0 {
            continue;
        }
        let size = fs::metadata(&path).map(|metadata| metadata.len()).unwrap_or(0);
        if best
            .as_ref()
            .map(|(best_score, best_size, _)| score > *best_score || (score == *best_score && size > *best_size))
            .unwrap_or(true)
        {
            *best = Some((score, size, path));
        }
    }
}

fn is_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp")
    )
}

fn is_logo_image_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("jpg" | "jpeg" | "png" | "webp" | "bmp" | "ico" | "svg")
    )
}

fn logo_path_score(path: &Path, clean_name: &str) -> i32 {
    let value = path.to_string_lossy().to_lowercase();
    if is_screenshotish_path(path)
        || value.contains("uninstall")
        || value.contains("setup")
        || value.contains("cursor")
        || value.contains("loading")
    {
        return 0;
    }

    let file_name = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_lowercase();
    let mut score = 0;
    for (needle, points) in [
        ("logo", 120),
        ("logotype", 120),
        ("icon", 88),
        ("banner", 68),
        ("header", 60),
        ("capsule", 54),
        ("library", 48),
        ("cover", 34),
        ("poster", 28),
        ("splash", 24),
    ] {
        if value.contains(needle) {
            score = score.max(points);
        }
    }
    if !clean_name.is_empty() && fileish_name(&file_name).to_lowercase().contains(clean_name) {
        score += 12;
    }
    score
}

fn is_screenshotish_path(path: &Path) -> bool {
    let value = path.to_string_lossy().to_lowercase();
    value.contains("screenshot")
        || value.contains("screen shot")
        || value.contains("screenshots")
        || value.contains("capture")
        || value.contains("captures")
        || value.contains("photo")
        || value.contains("gallery")
}

fn user_game_image_roots(name: &str) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let Some(home) = env_path("USERPROFILE") else {
        return roots;
    };
    let clean = fileish_name(name);
    roots.push(home.join("Documents").join("My Games").join(name));
    roots.push(home.join("Documents").join("My Games").join(&clean));
    roots.push(home.join("Documents").join(name));
    roots.push(home.join("Pictures").join(name));
    roots.push(home.join("Videos").join(name));
    roots
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

fn read_json_value(path: &Path) -> Option<Value> {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
}

fn json_string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty())
}

fn split_command_line(value: &str) -> Vec<String> {
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

fn install_location_key(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn fileish_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn stable_id(prefix: &str, key: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in key.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{prefix}:{hash:016x}")
}

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from).filter(|path| path.exists())
}

fn is_launcher_client(name: &str) -> bool {
    let value = name.to_lowercase();
    [
        "launcher",
        "game launcher",
        "ubisoft connect",
        "ubisoft game launcher",
        "gog galaxy",
        "battle.net",
        "ea app",
        "origin",
        "riot client",
        "rockstar games launcher",
        "social club",
        "steam",
        "epic games launcher",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

fn ignored_executable_name(path: &Path) -> bool {
    let value = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_lowercase();
    [
        "unins",
        "uninstall",
        "setup",
        "install",
        "redist",
        "vcredist",
        "dxsetup",
        "crash",
        "report",
        "helper",
        "update",
        "launcher",
        "bootstrap",
        "service",
        "unitycrashhandler",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

#[cfg(target_os = "windows")]
fn registry_launcher_games() -> Vec<GameEntry> {
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
                        &format!("{source}:{name}:{}", install_location_key(&install_location)),
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
fn registry_launcher_games() -> Vec<GameEntry> {
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

fn display_icon_path(value: &str) -> Option<PathBuf> {
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

fn find_launch_executable(root: &Path, name: &str) -> Option<PathBuf> {
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

#[cfg(target_os = "windows")]
fn steam_install_root() -> Option<PathBuf> {
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
            "SOFTWARE\\WOW6432Node\\Valve\\Steam",
        ),
    ];

    for (root, key_path) in roots {
        let Ok(key) = root.open_subkey(key_path) else {
            continue;
        };
        if let Some(root) = steam_root_from_key(&key) {
            return Some(root);
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
fn steam_install_root() -> Option<PathBuf> {
    None
}

fn riot_client_path() -> Option<PathBuf> {
    let program_data = env_path("PROGRAMDATA")?;
    let installs = program_data
        .join("Riot Games")
        .join("RiotClientInstalls.json");
    let json = read_json_value(&installs)?;
    json_string(&json, "rc_default")
        .map(PathBuf::from)
        .filter(|path| path.is_file())
        .or_else(|| {
            system_drive_root()
                .map(|drive| drive.join("Riot Games").join("Riot Client").join("RiotClientServices.exe"))
                .filter(|path| path.is_file())
        })
}

fn system_drive_root() -> Option<PathBuf> {
    let drive = std::env::var("SystemDrive").ok()?;
    let root = PathBuf::from(format!("{drive}\\"));
    root.exists().then_some(root)
}

fn launch_target(target: &LaunchTarget) -> Result<(), String> {
    match target {
        LaunchTarget::Url(url) => open_url(url),
        LaunchTarget::Exe { path, args, cwd } => open_executable(path, args, cwd.as_deref()),
        LaunchTarget::None => Err("Game launch is not available for this entry".to_string()),
    }
}

#[cfg(target_os = "windows")]
fn open_url(url: &str) -> Result<(), String> {
    let lower = url.to_lowercase();
    if !lower.starts_with("steam://")
        && !lower.starts_with("com.epicgames.launcher://")
        && !lower.starts_with("riotclient://")
    {
        return Err("Blocked launch of untrusted game URL scheme".to_string());
    }
    shell_execute("open", url, None, None, "Failed to launch game")
}

#[cfg(not(target_os = "windows"))]
fn open_url(_url: &str) -> Result<(), String> {
    Err("Game launch is only available on Windows".to_string())
}

#[cfg(target_os = "windows")]
fn open_executable(path: &Path, args: &[String], cwd: Option<&Path>) -> Result<(), String> {
    if !path.is_file() {
        return Err("Game executable is missing".to_string());
    }
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_lowercase();
    if ext != "exe" {
        return Err("Target file is not an executable".to_string());
    }
    let args = if args.is_empty() {
        None
    } else {
        Some(join_command_args(args))
    };
    shell_execute(
        "open",
        &path.to_string_lossy(),
        args.as_deref(),
        cwd.map(|path| path.to_string_lossy().into_owned()).as_deref(),
        "Failed to launch game",
    )
}

#[cfg(not(target_os = "windows"))]
fn open_executable(_path: &Path, _args: &[String], _cwd: Option<&Path>) -> Result<(), String> {
    Err("Game launch is only available on Windows".to_string())
}

#[cfg(target_os = "windows")]
fn shell_execute(
    operation: &str,
    file: &str,
    parameters: Option<&str>,
    directory: Option<&str>,
    error_message: &str,
) -> Result<(), String> {
    let operation = crate::ffi::wide_null(operation);
    let file = crate::ffi::wide_null(file);
    let parameters = parameters.map(crate::ffi::wide_null);
    let directory = directory.map(crate::ffi::wide_null);
    let result = unsafe {
        crate::ffi::winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null()),
            directory
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null()),
            1,
        )
    };

    if result <= 32 {
        Err(error_message.to_string())
    } else {
        Ok(())
    }
}

fn join_command_args(args: &[String]) -> String {
    args.iter()
        .map(|arg| {
            if arg.chars().all(|ch| !ch.is_whitespace() && ch != '"') {
                arg.clone()
            } else {
                format!("\"{}\"", arg.replace('"', "\\\""))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
