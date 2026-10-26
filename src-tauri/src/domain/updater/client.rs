use super::logging::log;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const MIN_UPDATE_SIZE_BYTES: u64 = 500_000;
const MAX_UPDATE_SIZE_BYTES: u64 = 150_000_000;

static IS_DOWNLOADING: AtomicBool = AtomicBool::new(false);
static LATEST_RELEASE_CACHE: Mutex<Option<UpdateCheckResult>> = Mutex::new(None);
static VERIFIED_SHA256: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckResult {
    pub update_available: bool,
    pub latest_version: String,
    pub current_version: String,
    pub download_url: Option<String>,
    pub sha_manifest_url: Option<String>,
    pub asset_size: u64,
    pub changelog: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}

impl ParsedVersion {
    pub fn parse(s: &str) -> Option<Self> {
        let clean = s.trim().trim_start_matches('v').trim_start_matches('V');
        let mut parts = clean.splitn(2, '-');
        let core = parts.next()?;
        let pre = parts.next().map(|p| p.trim().to_lowercase());

        let mut nums = core.split('.');
        let major = nums.next()?.parse::<u32>().ok()?;
        let minor = nums.next().unwrap_or("0").parse::<u32>().ok()?;
        let patch = nums.next().unwrap_or("0").parse::<u32>().ok()?;

        Some(Self {
            major,
            minor,
            patch,
            pre,
        })
    }

    pub fn is_newer_than(&self, current: &Self) -> bool {
        if self.major != current.major {
            return self.major > current.major;
        }
        if self.minor != current.minor {
            return self.minor > current.minor;
        }
        if self.patch != current.patch {
            return self.patch > current.patch;
        }

        match (&self.pre, &current.pre) {
            (None, Some(_)) => true, // release > prerelease of same core
            (Some(_), None) => false, // prerelease < release of same core
            (Some(a), Some(b)) => a > b,
            (None, None) => false, // same version
        }
    }
}

pub fn is_newer_version(latest_tag: &str, current_version: &str) -> bool {
    match (ParsedVersion::parse(latest_tag), ParsedVersion::parse(current_version)) {
        (Some(latest), Some(current)) => latest.is_newer_than(&current),
        _ => false,
    }
}

pub fn parse_sha256_manifest(contents: &str, target_filename: &str) -> Result<String, String> {
    let mut found_hash = None;
    let target_lower = target_filename.to_lowercase();

    for (line_no, line) in contents.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let hash = parts[0].trim().to_lowercase();
        let fname = parts[1].trim().trim_start_matches('*').to_lowercase();

        if fname == target_lower
            || fname.ends_with(&format!("/{}", target_lower))
            || fname.ends_with(&format!("\\{}", target_lower))
        {
            if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("Invalid SHA-256 hash format at line {}: {}", line_no + 1, hash));
            }
            if let Some(existing) = &found_hash {
                if existing != &hash {
                    return Err(format!("Ambiguous conflicting SHA-256 entries found for '{}'", target_filename));
                }
            }
            found_hash = Some(hash);
        }
    }

    found_hash.ok_or_else(|| format!("Target file '{}' not found in SHA256SUMS.txt manifest", target_filename))
}

pub fn validate_update_url(url: &str) -> Result<(), String> {
    let lower = url.trim().to_lowercase();
    let is_valid = lower.starts_with("https://github.com/arean-max/synchro-nova/releases/")
        || lower.starts_with("https://objects.githubusercontent.com/")
        || lower.starts_with("https://api.github.com/repos/arean-max/synchro-nova/releases");

    if !is_valid {
        return Err("Update URL must strictly originate from official Synchro Nova GitHub releases".to_string());
    }
    Ok(())
}

pub fn get_curl_cmd() -> Command {
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let curl_exe = std::path::PathBuf::from(system_root).join("System32").join("curl.exe");
    if curl_exe.is_file() {
        Command::new(curl_exe)
    } else {
        Command::new("curl.exe")
    }
}

pub fn get_update_dir() -> PathBuf {
    std::env::temp_dir().join("synchro_update")
}

pub fn get_staging_dir(version: &str) -> PathBuf {
    let clean = version.trim().trim_start_matches('v').trim_start_matches('V');
    get_update_dir().join(format!("staging_{clean}"))
}

pub fn get_update_file_path() -> PathBuf {
    if let Ok(guard) = LATEST_RELEASE_CACHE.lock() {
        if let Some(cached) = &*guard {
            return get_staging_dir(&cached.latest_version).join("synchro.exe");
        }
    }
    get_update_dir().join("staging_latest").join("synchro.exe")
}

pub fn fetch_latest_release_info() -> Result<UpdateCheckResult, String> {
    log("CHECK", "Checking for updates at official GitHub releases endpoint");
    super::set_status("checking", None);

    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let mut cmd = get_curl_cmd();
    cmd.args([
        "-s",
        "-L",
        "--connect-timeout",
        "15",
        "--max-time",
        "30",
        "-H",
        "User-Agent: Synchro-Nova-AutoUpdater",
        "-H",
        "Accept: application/vnd.github.v3+json",
        "https://api.github.com/repos/Arean-Max/synchro-nova/releases/latest",
    ]);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd
        .output()
        .map_err(|e| format!("Network request failed: could not execute curl: {}", e))?;

    if !output.status.success() {
        let msg = format!("GitHub releases API request failed with code {:?}", output.status.code());
        log("CHECK_ERR", &msg);
        super::set_status("error", Some(msg.clone()));
        return Err(msg);
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse GitHub releases response JSON: {}", e))?;

    let tag_name = json
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing tag_name in GitHub release".to_string())?;

    let clean_version = tag_name.trim_start_matches('v').trim_start_matches('V').to_string();
    let body = json
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut download_url = None;
    let mut asset_size: u64 = 0;
    let mut sha_manifest_url = None;

    if let Some(assets) = json.get("assets").and_then(|v| v.as_array()) {
        for asset in assets {
            let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
            if name.eq_ignore_ascii_case("SHA256SUMS.txt") {
                sha_manifest_url = asset
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
                if name.ends_with(".exe") && !name.contains("setup") {
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

    let update_available = is_newer_version(tag_name, &current_version);

    let result = UpdateCheckResult {
        update_available,
        latest_version: clean_version.clone(),
        current_version: current_version.clone(),
        download_url,
        sha_manifest_url,
        asset_size,
        changelog: body,
    };

    if let Ok(mut lock) = LATEST_RELEASE_CACHE.lock() {
        *lock = Some(result.clone());
    }

    if update_available {
        log("CHECK", &format!("New version v{clean_version} available (current v{current_version})"));
        super::set_progress(super::UpdateProgress {
            status: "available".to_string(),
            downloaded_bytes: 0,
            total_bytes: asset_size,
            percent: 0,
            latest_version: Some(clean_version),
            current_version,
            error: None,
        });
    } else {
        log("CHECK", &format!("Application is up to date (v{current_version})"));
        super::set_status("upToDate", None);
    }

    Ok(result)
}

pub fn spawn_download_worker(
    download_url: String,
    expected_size: u64,
    latest_version: String,
    sha_manifest_url: Option<String>,
) -> Result<(), String> {
    if IS_DOWNLOADING.swap(true, Ordering::SeqCst) {
        log("DOWNLOAD_WARN", "Download already in progress; ignoring duplicate trigger");
        return Ok(());
    }

    validate_update_url(&download_url)?;
    if let Some(ref sha_url) = sha_manifest_url {
        validate_update_url(sha_url)?;
    }

    let staging_dir = get_staging_dir(&latest_version);
    if let Err(e) = fs::create_dir_all(&staging_dir) {
        IS_DOWNLOADING.store(false, Ordering::SeqCst);
        let msg = format!("Failed to create staging directory: {}", e);
        log("DOWNLOAD_ERR", &msg);
        super::set_status("error", Some(msg.clone()));
        return Err(msg);
    }

    let target_file = staging_dir.join("synchro.exe");
    let part_file = staging_dir.join("synchro.exe.part");
    let manifest_file = staging_dir.join("SHA256SUMS.txt");

    let _ = fs::remove_file(&target_file);
    let _ = fs::remove_file(&part_file);

    super::set_progress(super::UpdateProgress {
        status: "downloading".to_string(),
        downloaded_bytes: 0,
        total_bytes: expected_size,
        percent: 0,
        latest_version: Some(latest_version.clone()),
        current_version: env!("CARGO_PKG_VERSION").to_string(),
        error: None,
    });

    log("DOWNLOAD", &format!("Starting background download of v{latest_version} from '{download_url}'"));

    std::thread::Builder::new()
        .name("synchro-updater-dl".to_string())
        .stack_size(128 * 1024)
        .spawn(move || {
            let sha_url = match sha_manifest_url {
                Some(u) => u,
                None => {
                    log("DOWNLOAD_ERR", "Release does not provide SHA256SUMS.txt manifest; rejecting unverified update");
                    super::set_status("error", Some("Release manifest SHA256SUMS.txt missing".to_string()));
                    IS_DOWNLOADING.store(false, Ordering::SeqCst);
                    return;
                }
            };

            let mut sha_cmd = get_curl_cmd();
            sha_cmd.args([
                "-L",
                "--fail",
                "--silent",
                "--show-error",
                "--connect-timeout",
                "15",
                "--max-time",
                "30",
                "-o",
            ]);
            sha_cmd.arg(&manifest_file);
            sha_cmd.arg(&sha_url);
            #[cfg(target_os = "windows")]
            sha_cmd.creation_flags(CREATE_NO_WINDOW);

            let sha_status = sha_cmd.status();
            if !sha_status.map(|s| s.success()).unwrap_or(false) {
                log("DOWNLOAD_ERR", "Failed to download SHA256SUMS.txt manifest");
                super::set_status("error", Some("Failed to retrieve SHA-256 release manifest".to_string()));
                IS_DOWNLOADING.store(false, Ordering::SeqCst);
                return;
            }

            let manifest_content = match fs::read_to_string(&manifest_file) {
                Ok(c) => c,
                Err(e) => {
                    log("DOWNLOAD_ERR", &format!("Failed to read SHA256SUMS.txt: {}", e));
                    super::set_status("error", Some("Corrupt checksum manifest".to_string()));
                    IS_DOWNLOADING.store(false, Ordering::SeqCst);
                    return;
                }
            };

            let expected_hash = match parse_sha256_manifest(&manifest_content, "synchro.exe") {
                Ok(h) => h,
                Err(e) => {
                    log("DOWNLOAD_ERR", &format!("Checksum parsing failed: {}", e));
                    super::set_status("error", Some(e));
                    IS_DOWNLOADING.store(false, Ordering::SeqCst);
                    return;
                }
            };

            let mut bin_cmd = get_curl_cmd();
            bin_cmd.args([
                "-L",
                "--fail",
                "--silent",
                "--show-error",
                "--connect-timeout",
                "20",
                "--max-time",
                "360",
                "-o",
            ]);
            bin_cmd.arg(&part_file);
            bin_cmd.arg(&download_url);
            #[cfg(target_os = "windows")]
            bin_cmd.creation_flags(CREATE_NO_WINDOW);

            let mut child = match bin_cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    log("DOWNLOAD_ERR", &format!("Failed to spawn curl for binary: {}", e));
                    super::set_status("error", Some("Failed to start download process".to_string()));
                    IS_DOWNLOADING.store(false, Ordering::SeqCst);
                    return;
                }
            };

            let start = std::time::Instant::now();
            let timeout = Duration::from_secs(380);

            loop {
                std::thread::sleep(Duration::from_millis(100));

                if start.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = fs::remove_file(&part_file);
                    log("DOWNLOAD_ERR", "Download timed out after 380 seconds");
                    super::set_status("error", Some("Download timed out".to_string()));
                    break;
                }

                if let Ok(meta) = fs::metadata(&part_file) {
                    let current_size = meta.len();
                    if current_size > MAX_UPDATE_SIZE_BYTES {
                        let _ = child.kill();
                        let _ = fs::remove_file(&part_file);
                        log("DOWNLOAD_ERR", &format!("File size {current_size} exceeds maximum safe limit"));
                        super::set_status("error", Some("Payload exceeded maximum allowed size".to_string()));
                        break;
                    }

                    let pct = if expected_size > 0 {
                        ((current_size as f64 / expected_size as f64) * 100.0).clamp(0.0, 99.0) as u8
                    } else {
                        0
                    };

                    super::set_progress(super::UpdateProgress {
                        status: "downloading".to_string(),
                        downloaded_bytes: current_size,
                        total_bytes: expected_size,
                        percent: pct,
                        latest_version: Some(latest_version.clone()),
                        current_version: env!("CARGO_PKG_VERSION").to_string(),
                        error: None,
                    });
                }

                match child.try_wait() {
                    Ok(Some(status)) => {
                        if !status.success() {
                            let _ = fs::remove_file(&part_file);
                            log("DOWNLOAD_ERR", &format!("curl terminated with error code {:?}", status.code()));
                            super::set_status("error", Some(format!("Download failed with code {:?}", status.code())));
                            break;
                        }

                        if let Err(e) = fs::rename(&part_file, &target_file) {
                            log("DOWNLOAD_ERR", &format!("Failed to rename part file: {}", e));
                            super::set_status("error", Some("Failed to finalize download file".to_string()));
                            break;
                        }

                        super::set_progress(super::UpdateProgress {
                            status: "verifying".to_string(),
                            downloaded_bytes: expected_size,
                            total_bytes: expected_size,
                            percent: 99,
                            latest_version: Some(latest_version.clone()),
                            current_version: env!("CARGO_PKG_VERSION").to_string(),
                            error: None,
                        });

                        log("VERIFY", "Validating downloaded update package integrity and authenticity");

                        if let Err(e) = verify_update_package(&target_file, &expected_hash) {
                            let _ = fs::remove_file(&target_file);
                            log("VERIFY_ERR", &format!("Verification failed: {}", e));
                            super::set_status("error", Some(e));
                            break;
                        }

                        log("VERIFY", "Update package passed all cryptographic integrity and authenticity checks");

                        if let Ok(mut guard) = VERIFIED_SHA256.lock() {
                            *guard = Some(expected_hash.clone());
                        }

                        let final_size = fs::metadata(&target_file).map(|m| m.len()).unwrap_or(0);
                        super::set_progress(super::UpdateProgress {
                            status: "ready".to_string(),
                            downloaded_bytes: final_size,
                            total_bytes: final_size,
                            percent: 100,
                            latest_version: Some(latest_version.clone()),
                            current_version: env!("CARGO_PKG_VERSION").to_string(),
                            error: None,
                        });

                        break;
                    }
                    Ok(None) => {}
                    Err(e) => {
                        log("DOWNLOAD_ERR", &format!("Child wait error: {}", e));
                        super::set_status("error", Some(e.to_string()));
                        break;
                    }
                }
            }

            IS_DOWNLOADING.store(false, Ordering::SeqCst);
        })
        .map_err(|e| format!("Failed to spawn updater thread: {}", e))?;

    Ok(())
}

pub fn get_verified_sha256() -> Option<String> {
    if let Ok(guard) = VERIFIED_SHA256.lock() {
        if let Some(ref hash) = *guard {
            return Some(hash.clone());
        }
    }

    let manifest_file = get_update_dir().join("SHA256SUMS.txt");
    if manifest_file.is_file() {
        if let Ok(content) = fs::read_to_string(&manifest_file) {
            if let Ok(hash) = parse_sha256_manifest(&content, "synchro.exe") {
                if let Ok(mut guard) = VERIFIED_SHA256.lock() {
                    *guard = Some(hash.clone());
                }
                return Some(hash);
            }
        }
    }

    None
}

pub fn verify_update_package(file_path: &Path, expected_sha256: &str) -> Result<(), String> {
    if !file_path.is_file() {
        return Err("Update payload does not exist on disk".to_string());
    }

    let meta = fs::metadata(file_path).map_err(|e| format!("Cannot read file metadata: {}", e))?;
    if meta.len() < MIN_UPDATE_SIZE_BYTES {
        return Err(format!("File size too small ({} bytes)", meta.len()));
    }
    if meta.len() > MAX_UPDATE_SIZE_BYTES {
        return Err(format!("File size exceeds safety limit ({} bytes)", meta.len()));
    }

    let mut header = [0u8; 2];
    {
        use std::io::Read;
        let mut f = fs::File::open(file_path).map_err(|e| e.to_string())?;
        f.read_exact(&mut header).map_err(|e| e.to_string())?;
    }
    if &header != b"MZ" {
        return Err("Update file is not a valid Windows PE binary (missing MZ header)".to_string());
    }

    let actual_sha = crate::infra::hash::sha256_file(file_path)
        .map_err(|e| format!("Failed to compute SHA-256: {}", e))?
        .to_lowercase();

    if actual_sha != expected_sha256.to_lowercase() {
        return Err(format!(
            "SHA-256 hash mismatch: expected {}, got {}",
            expected_sha256, actual_sha
        ));
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(current_exe) = std::env::current_exe() {
            let current_signed = crate::platform::ffi::verify_embedded_signature(&current_exe);
            let update_signed = crate::platform::ffi::verify_embedded_signature(file_path);

            if current_signed && !update_signed {
                return Err("Update binary is not digitally signed, but running instance requires trusted signature".to_string());
            }
        }
    }

    Ok(())
}

pub fn start_background_update_flow(auto_download: bool) {
    std::thread::Builder::new()
        .name("synchro-auto-check".to_string())
        .stack_size(64 * 1024)
        .spawn(move || {
            match fetch_latest_release_info() {
                Ok(info) => {
                    if info.update_available && auto_download {
                        if let Some(dl_url) = info.download_url {
                            let _ = spawn_download_worker(
                                dl_url,
                                info.asset_size,
                                info.latest_version,
                                info.sha_manifest_url,
                            );
                        }
                    }
                }
                Err(e) => {
                    log("CHECK_WARN", &format!("Background check completed with notice: {}", e));
                }
            }
        })
        .ok();
}
