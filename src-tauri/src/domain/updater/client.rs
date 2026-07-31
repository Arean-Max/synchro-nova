use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

static LATEST_SHA_URL: Mutex<Option<String>> = Mutex::new(None);
static IS_DOWNLOADING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
    pub update_available: bool,
    pub latest_version: String,
    pub current_version: String,
    pub download_url: Option<String>,
    pub asset_size: u64,
    pub changelog: String,
}

fn get_curl_cmd() -> Command {
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let curl_exe = std::path::PathBuf::from(system_root).join("System32").join("curl.exe");
    if curl_exe.exists() {
        Command::new(curl_exe)
    } else {
        Command::new("curl.exe")
    }
}

pub fn is_newer_version(latest_tag: &str, current_version: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        let clean = v.trim().trim_start_matches('v').trim_start_matches('V');
        clean
            .split('.')
            .filter_map(|s| s.split('-').next().unwrap_or(s).parse::<u32>().ok())
            .collect()
    };
    let l_parts = parse(latest_tag);
    let c_parts = parse(current_version);
    for i in 0..l_parts.len().max(c_parts.len()) {
        let l = *l_parts.get(i).unwrap_or(&0);
        let c = *c_parts.get(i).unwrap_or(&0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

pub fn get_update_dir() -> PathBuf {
    std::env::temp_dir().join("synchro_update")
}

pub fn get_update_file_path() -> PathBuf {
    get_update_dir().join("synchro_latest.exe")
}

pub fn fetch_latest_release_info() -> Result<UpdateCheckResult, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();

    let mut cmd = get_curl_cmd();
    cmd.args([
        "-s",
        "--max-time",
        "30",
        "-H",
        "User-Agent: Synchro-Nova-Updater",
        "-H",
        "Accept: application/vnd.github.v3+json",
        "https://api.github.com/repos/Arean-Max/synchro-nova/releases/latest",
    ]);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to query GitHub API via curl: {}", e))?;

    if !output.status.success() {
        return Err("GitHub API query exited with non-zero status".to_string());
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse GitHub release JSON: {}", e))?;

    let tag_name = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing tag_name in GitHub release".to_string())?;

    let clean_version = tag_name.trim_start_matches('v').to_string();
    let body = json
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut download_url = None;
    let mut asset_size: u64 = 0;
    let mut sha256_url = None;

    if let Some(assets) = json.get("assets").and_then(|v| v.as_array()) {
        for asset in assets {
            let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
            if name.eq_ignore_ascii_case("SHA256SUMS.txt") {
                sha256_url = asset
                    .get("browser_download_url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            if name == "synchro.exe" {
                download_url = asset
                    .get("browser_download_url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                asset_size = asset.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
            }
        }

        if download_url.is_none() {
            for asset in assets {
                let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
                if name.ends_with(".exe") {
                    download_url = asset
                        .get("browser_download_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    asset_size = asset.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
                    break;
                }
            }
        }
    }

    if let Ok(mut lock) = LATEST_SHA_URL.lock() {
        *lock = sha256_url;
    }

    let update_available = is_newer_version(tag_name, &current_version);

    Ok(UpdateCheckResult {
        update_available,
        latest_version: clean_version,
        current_version,
        download_url,
        asset_size,
        changelog: body,
    })
}
