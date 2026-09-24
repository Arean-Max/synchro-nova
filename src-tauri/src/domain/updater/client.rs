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
        clean.split('.').filter_map(|s| s.parse::<u32>().ok()).collect()
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

        // Fallback to any .exe if synchro.exe not found directly
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

pub fn spawn_download_worker(
    download_url: String,
    expected_size: u64,
) -> Result<(), String> {
    if IS_DOWNLOADING.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    let target_dir = get_update_dir();
    let target_file = get_update_file_path();

    if let Err(e) = fs::create_dir_all(&target_dir) {
        IS_DOWNLOADING.store(false, Ordering::SeqCst);
        return Err(format!("Failed to create temp update dir: {}", e));
    }

    let _ = fs::remove_file(&target_file);

    super::set_progress(super::UpdateProgress {
        status: "downloading".to_string(),
        downloaded_bytes: 0,
        total_bytes: expected_size,
        percent: 0,
        error: None,
    });

    std::thread::Builder::new()
        .name("synchro-downloader".to_string())
        .spawn(move || {
            let mut cmd = get_curl_cmd();
            cmd.args([
                "-L",
                "--fail",
                "--silent",
                "--show-error",
                "-o",
            ]);
            cmd.arg(&target_file);
            cmd.arg(&download_url);
            #[cfg(target_os = "windows")]
            cmd.creation_flags(CREATE_NO_WINDOW);

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    super::set_progress(super::UpdateProgress {
                        status: "error".to_string(),
                        downloaded_bytes: 0,
                        total_bytes: expected_size,
                        percent: 0,
                        error: Some(format!("Failed to spawn curl: {}", e)),
                    });
                    IS_DOWNLOADING.store(false, Ordering::SeqCst);
                    return;
                }
            };

            loop {
                std::thread::sleep(Duration::from_millis(70));

                if let Ok(meta) = fs::metadata(&target_file) {
                    let downloaded = meta.len();
                    let pct = if expected_size > 0 {
                        ((downloaded as f64 / expected_size as f64) * 100.0).clamp(0.0, 99.0) as u8
                    } else {
                        0
                    };

                    super::set_progress(super::UpdateProgress {
                        status: "downloading".to_string(),
                        downloaded_bytes: downloaded,
                        total_bytes: expected_size,
                        percent: pct,
                        error: None,
                    });
                }

                match child.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            let actual_size = fs::metadata(&target_file)
                                .map(|m| m.len())
                                .unwrap_or(0);

                            if actual_size < 500_000 {
                                let _ = fs::remove_file(&target_file);
                                super::set_progress(super::UpdateProgress {
                                    status: "error".to_string(),
                                    downloaded_bytes: actual_size,
                                    total_bytes: expected_size,
                                    percent: 0,
                                    error: Some("Downloaded file size is too small or incomplete".to_string()),
                                });
                                break;
                            }

                            // Verify valid PE header
                            let mut header_buf = [0u8; 2];
                            let is_valid_pe = if let Ok(mut file) = fs::File::open(&target_file) {
                                use std::io::Read;
                                file.read_exact(&mut header_buf).is_ok() && &header_buf == b"MZ"
                            } else {
                                false
                            };

                            if !is_valid_pe {
                                let _ = fs::remove_file(&target_file);
                                super::set_progress(super::UpdateProgress {
                                    status: "error".to_string(),
                                    downloaded_bytes: actual_size,
                                    total_bytes: expected_size,
                                    percent: 0,
                                    error: Some("Downloaded file is not a valid Windows executable".to_string()),
                                });
                                break;
                            }

                            // Verify SHA-256 against release SHA256SUMS.txt if available
                            let sha_url = LATEST_SHA_URL.lock().ok().and_then(|g| g.clone());
                            if let Some(sha_url) = sha_url {
                                let sha_file = target_dir.join("SHA256SUMS.txt");
                                let mut sha_cmd = get_curl_cmd();
                                sha_cmd.args(["-L", "--fail", "--silent", "-o"]);
                                sha_cmd.arg(&sha_file);
                                sha_cmd.arg(&sha_url);
                                #[cfg(target_os = "windows")]
                                sha_cmd.creation_flags(CREATE_NO_WINDOW);

                                if let Ok(sha_status) = sha_cmd.status() {
                                    if sha_status.success() {
                                        if let Ok(contents) = fs::read_to_string(&sha_file) {
                                            let mut expected_hash = None;
                                            for line in contents.lines() {
                                                let parts: Vec<&str> = line.split_whitespace().collect();
                                                if parts.len() >= 2 {
                                                    let hash = parts[0].trim().to_lowercase();
                                                    let file = parts[1].trim().trim_start_matches('*');
                                                    if file == "synchro.exe" || download_url.ends_with(file) {
                                                        expected_hash = Some(hash);
                                                        break;
                                                    }
                                                }
                                            }

                                            if let Some(expected) = expected_hash {
                                                let actual_hash = crate::infra::hash::sha256_file(&target_file).unwrap_or_default();
                                                if actual_hash.to_lowercase() != expected {
                                                    let _ = fs::remove_file(&target_file);
                                                    super::set_progress(super::UpdateProgress {
                                                        status: "error".to_string(),
                                                        downloaded_bytes: actual_size,
                                                        total_bytes: expected_size,
                                                        percent: 0,
                                                        error: Some(format!(
                                                            "SHA-256 verification failed (expected {}, got {})",
                                                            expected, actual_hash
                                                        )),
                                                    });
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            super::set_progress(super::UpdateProgress {
                                status: "ready".to_string(),
                                downloaded_bytes: actual_size,
                                total_bytes: actual_size,
                                percent: 100,
                                error: None,
                            });
                        } else {
                            super::set_progress(super::UpdateProgress {
                                status: "error".to_string(),
                                downloaded_bytes: 0,
                                total_bytes: expected_size,
                                percent: 0,
                                error: Some(format!("curl download failed with exit code {:?}", status.code())),
                            });
                        }
                        break;
                    }
                    Ok(None) => {}
                    Err(e) => {
                        super::set_progress(super::UpdateProgress {
                            status: "error".to_string(),
                            downloaded_bytes: 0,
                            total_bytes: expected_size,
                            percent: 0,
                            error: Some(format!("Error monitoring download child: {}", e)),
                        });
                        break;
                    }
                }
            }

            IS_DOWNLOADING.store(false, Ordering::SeqCst);
        })
        .map_err(|e| format!("Failed to spawn downloader thread: {}", e))?;

    Ok(())
}
