use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

static ORIGINAL_HOLOSIGHT_COLOUR: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlackHoloConfigResult {
    pub success: bool,
    pub cfg_path: Option<String>,
    pub previous_value: Option<String>,
    pub current_value: Option<String>,
    pub modified: bool,
    pub rust_running: bool,
    pub message: String,
}

/// Passive process check via toolhelp snapshot.
/// Does not open handles to target processes or interact with process memory.
#[cfg(target_os = "windows")]
pub fn is_rust_process_running() -> bool {
    unsafe {
        use crate::platform::ffi::{winapi, ProcessEntry32W, TH32CS_SNAPPROCESS};
        let snapshot = winapi::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == -1 as _ {
            return false;
        }

        let mut entry = ProcessEntry32W::default();
        let mut found = false;
        if winapi::Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let len = entry
                    .sz_exe_file
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.sz_exe_file.len());
                let exe_name = String::from_utf16_lossy(&entry.sz_exe_file[..len]).to_lowercase();
                if exe_name == "rustclient.exe" || exe_name == "rust.exe" {
                    found = true;
                    break;
                }
                if winapi::Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        winapi::CloseHandle(snapshot);
        found
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_rust_process_running() -> bool {
    false
}

pub fn find_rust_client_cfg() -> Option<PathBuf> {
    // 1. Check Steam library folders
    if let Some(steam_root) = super::detector::steam_install_root() {
        let mut libraries = vec![steam_root.clone()];
        libraries.extend(super::detector::read_steam_libraries(&steam_root));
        for lib in libraries {
            let candidate = lib
                .join("steamapps")
                .join("common")
                .join("Rust")
                .join("cfg")
                .join("client.cfg");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    // 2. Common drive paths (C:, D:, E:, F:, G:)
    let drives = ["C", "D", "E", "F", "G"];
    for drive in &drives {
        let candidates = [
            format!(r"{}:\Program Files (x86)\Steam\steamapps\common\Rust\cfg\client.cfg", drive),
            format!(r"{}:\Program Files\Steam\steamapps\common\Rust\cfg\client.cfg", drive),
            format!(r"{}:\SteamLibrary\steamapps\common\Rust\cfg\client.cfg", drive),
            format!(r"{}:\Steam\steamapps\common\Rust\cfg\client.cfg", drive),
            format!(r"{}:\Games\Rust\cfg\client.cfg", drive),
            format!(r"{}:\Rust\cfg\client.cfg", drive),
        ];
        for path_str in &candidates {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}

pub(crate) fn parse_holosight_colour(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with("accessibility.holosightcolour") {
            if let Some(pos) = trimmed.find('"') {
                let rest = &trimmed[pos + 1..];
                if let Some(end_pos) = rest.find('"') {
                    return Some(rest[..end_pos].to_string());
                }
            } else {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].trim_matches('"').to_string());
                }
            }
        }
    }
    None
}

fn sync_autoexec_cfg(cfg_path: &PathBuf, enable_black: bool, orig_val: Option<&str>) {
    let autoexec_path = cfg_path.with_file_name("autoexec.cfg");
    let content = fs::read_to_string(&autoexec_path).unwrap_or_default();
    let mut lines: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().to_lowercase().starts_with("accessibility.holosightcolour"))
        .map(|s| s.to_string())
        .collect();

    if enable_black {
        lines.push(r#"accessibility.holosightcolour "2""#.to_string());
    } else if let Some(orig) = orig_val {
        if orig != "2" {
            lines.push(format!(r#"accessibility.holosightcolour "{}""#, orig));
        }
    }

    let mut new_autoexec = lines.join("\r\n");
    if !new_autoexec.is_empty() {
        new_autoexec.push_str("\r\n");
    }
    let _ = fs::write(&autoexec_path, new_autoexec);
}

pub fn set_rust_holosight_black(enabled: bool) -> BlackHoloConfigResult {
    let rust_running = is_rust_process_running();
    let Some(cfg_path) = find_rust_client_cfg() else {
        return BlackHoloConfigResult {
            success: false,
            cfg_path: None,
            previous_value: None,
            current_value: None,
            modified: false,
            rust_running,
            message: "Rust client.cfg not found".to_string(),
        };
    };

    let cfg_str = cfg_path.to_string_lossy().to_string();

    let Ok(content) = fs::read_to_string(&cfg_path) else {
        return BlackHoloConfigResult {
            success: false,
            cfg_path: Some(cfg_str),
            previous_value: None,
            current_value: None,
            modified: false,
            rust_running,
            message: "Failed to read client.cfg".to_string(),
        };
    };

    let current = parse_holosight_colour(&content);

    if enabled {
        // User is activating Black Holo:
        sync_autoexec_cfg(&cfg_path, true, None);

        // If it's already "2", do not change client.cfg!
        if current.as_deref() == Some("2") {
            if let Ok(mut guard) = ORIGINAL_HOLOSIGHT_COLOUR.lock() {
                if guard.is_none() {
                    *guard = Some("2".to_string());
                }
            }
            return BlackHoloConfigResult {
                success: true,
                cfg_path: Some(cfg_str),
                previous_value: Some("2".to_string()),
                current_value: Some("2".to_string()),
                modified: false,
                rust_running,
                message: if rust_running {
                    "accessibility.holosightcolour is \"2\" (In Rust F1 console: readcfg)".to_string()
                } else {
                    "accessibility.holosightcolour is already \"2\"".to_string()
                },
            };
        }

        // In other cases (e.g. "0", "1", "3"), forcibly change to "2" and save original
        let prev = current.unwrap_or_else(|| "0".to_string());
        if let Ok(mut guard) = ORIGINAL_HOLOSIGHT_COLOUR.lock() {
            if guard.is_none() {
                *guard = Some(prev.clone());
            }
        }

        // Replace or append accessibility.holosightcolour "2"
        let mut replaced = false;
        let mut new_lines = Vec::new();
        for line in content.lines() {
            if line.trim().to_lowercase().starts_with("accessibility.holosightcolour") {
                new_lines.push(r#"accessibility.holosightcolour "2""#.to_string());
                replaced = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
        if !replaced {
            new_lines.push(r#"accessibility.holosightcolour "2""#.to_string());
        }

        let new_content = new_lines.join("\r\n") + "\r\n";
        if fs::write(&cfg_path, new_content).is_ok() {
            BlackHoloConfigResult {
                success: true,
                cfg_path: Some(cfg_str),
                previous_value: Some(prev),
                current_value: Some("2".to_string()),
                modified: true,
                rust_running,
                message: if rust_running {
                    "accessibility.holosightcolour set to \"2\" (In Rust F1 console: readcfg)".to_string()
                } else {
                    "accessibility.holosightcolour set to \"2\"".to_string()
                },
            }
        } else {
            BlackHoloConfigResult {
                success: false,
                cfg_path: Some(cfg_str),
                previous_value: Some(prev),
                current_value: None,
                modified: false,
                rust_running,
                message: "Failed to write to client.cfg".to_string(),
            }
        }
    } else {
        // User is disabling Black Holo:
        let original_opt = ORIGINAL_HOLOSIGHT_COLOUR.lock().ok().and_then(|mut g| g.take());
        sync_autoexec_cfg(&cfg_path, false, original_opt.as_deref());

        if let Some(orig) = original_opt {
            if orig == "2" {
                // Was already 2 before, do nothing
                return BlackHoloConfigResult {
                    success: true,
                    cfg_path: Some(cfg_str),
                    previous_value: Some("2".to_string()),
                    current_value: Some("2".to_string()),
                    modified: false,
                    rust_running,
                    message: "Original value was already \"2\", not changed".to_string(),
                };
            }

            let mut replaced = false;
            let mut new_lines = Vec::new();
            for line in content.lines() {
                if line.trim().to_lowercase().starts_with("accessibility.holosightcolour") {
                    new_lines.push(format!(r#"accessibility.holosightcolour "{}""#, orig));
                    replaced = true;
                } else {
                    new_lines.push(line.to_string());
                }
            }
            if !replaced {
                new_lines.push(format!(r#"accessibility.holosightcolour "{}""#, orig));
            }

            let new_content = new_lines.join("\r\n") + "\r\n";
            if fs::write(&cfg_path, new_content).is_ok() {
                BlackHoloConfigResult {
                    success: true,
                    cfg_path: Some(cfg_str),
                    previous_value: Some("2".to_string()),
                    current_value: Some(orig.clone()),
                    modified: true,
                    rust_running,
                    message: if rust_running {
                        format!("Restored colour to \"{}\" (In Rust F1 console: readcfg)", orig)
                    } else {
                        format!("Restored accessibility.holosightcolour to \"{}\"", orig)
                    },
                }
            } else {
                BlackHoloConfigResult {
                    success: false,
                    cfg_path: Some(cfg_str),
                    previous_value: Some("2".to_string()),
                    current_value: None,
                    modified: false,
                    rust_running,
                    message: "Failed to restore client.cfg".to_string(),
                }
            }
        } else {
            BlackHoloConfigResult {
                success: true,
                cfg_path: Some(cfg_str),
                previous_value: current.clone(),
                current_value: current,
                modified: false,
                rust_running,
                message: "No previous value recorded".to_string(),
            }
        }
    }
}
