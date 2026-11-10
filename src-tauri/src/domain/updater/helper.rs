use super::logging::log;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use sha2::{Digest, Sha256};

#[cfg(target_os = "windows")]
use std::os::windows::fs::OpenOptionsExt;

#[derive(Debug, Clone)]
pub struct HelperArgs {
    pub target: PathBuf,
    pub payload: PathBuf,
    pub wait_pid: u32,
    pub backup: PathBuf,
    pub expected_sha256: Option<String>,
    pub no_relaunch: bool,
}

impl HelperArgs {
    pub fn parse(args: &[String]) -> Option<Self> {
        let mut target = None;
        let mut payload = None;
        let mut wait_pid = None;
        let mut backup = None;
        let mut expected_sha256 = None;
        let mut no_relaunch = false;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--target" => {
                    if i + 1 < args.len() {
                        target = Some(PathBuf::from(&args[i + 1]));
                        i += 1;
                    }
                }
                "--payload" => {
                    if i + 1 < args.len() {
                        payload = Some(PathBuf::from(&args[i + 1]));
                        i += 1;
                    }
                }
                "--wait-pid" => {
                    if i + 1 < args.len() {
                        wait_pid = args[i + 1].parse::<u32>().ok();
                        i += 1;
                    }
                }
                "--backup" => {
                    if i + 1 < args.len() {
                        backup = Some(PathBuf::from(&args[i + 1]));
                        i += 1;
                    }
                }
                "--expected-sha256" => {
                    if i + 1 < args.len() {
                        expected_sha256 = Some(args[i + 1].trim().to_string());
                        i += 1;
                    }
                }
                "--no-relaunch" => {
                    no_relaunch = true;
                }
                _ => {}
            }
            i += 1;
        }

        let target_path = target?;
        let payload_path = payload?;
        let pid = wait_pid.unwrap_or(0);
        let backup_path = backup.unwrap_or_else(|| {
            let mut b = target_path.clone();
            b.set_extension("exe.old");
            b
        });

        Some(Self {
            target: target_path,
            payload: payload_path,
            wait_pid: pid,
            backup: backup_path,
            expected_sha256,
            no_relaunch,
        })
    }
}

pub fn validate_target_path(target: &Path) -> Result<(), String> {
    let file_name = target
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Target path lacks a valid file name".to_string())?
        .to_lowercase();

    if !file_name.ends_with(".exe") || !file_name.contains("synchro") {
        return Err(format!(
            "Target executable name must be a Synchro binary ending with .exe, got '{}'",
            file_name
        ));
    }

    let target_str = target.to_string_lossy().to_lowercase();
    let sys_root = std::env::var("SystemRoot")
        .unwrap_or_else(|_| "c:\\windows".to_string())
        .to_lowercase();

    if target_str.starts_with(&sys_root)
        || target_str.contains("\\system32")
        || target_str.contains("\\syswow64")
        || target_str.contains("\\winre")
    {
        return Err("Target path cannot point into Windows system directories".to_string());
    }

    Ok(())
}

pub fn validate_payload_path(payload: &Path) -> Result<(), String> {
    let update_dir = super::client::get_update_dir();
    let canonical_dir = update_dir
        .canonicalize()
        .unwrap_or_else(|_| update_dir.clone());
    let canonical_payload = payload
        .canonicalize()
        .unwrap_or_else(|_| payload.to_path_buf());

    if !canonical_payload.starts_with(&canonical_dir) {
        return Err("Payload must reside within the authenticated Synchro update directory".to_string());
    }
    Ok(())
}

pub fn verify_payload_integrity(payload: &Path, expected_sha: &str) -> Result<(), String> {
    if !payload.is_file() {
        return Err(format!("Payload file does not exist at '{}'", payload.display()));
    }

    let meta = fs::metadata(payload).map_err(|e| format!("Failed to read payload metadata: {}", e))?;
    if meta.len() < 500_000 || meta.len() > 150_000_000 {
        return Err(format!(
            "Payload size {} bytes is outside valid boundaries [500KB - 150MB]",
            meta.len()
        ));
    }

    use std::io::Read;
    let mut f = fs::File::open(payload).map_err(|e| format!("Cannot open payload: {}", e))?;
    let mut header = [0u8; 2];
    f.read_exact(&mut header).map_err(|e| format!("Failed to read payload header: {}", e))?;
    if &header != b"MZ" {
        return Err("Payload is not a valid Windows PE binary (missing MZ header)".to_string());
    }

    let expected = expected_sha.trim().to_lowercase();
    if expected.len() != 64 || !expected.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Malformed expected SHA-256 parameter".to_string());
    }

    let actual_hash = crate::infra::hash::sha256_file(payload)
        .map_err(|e| format!("Failed to compute payload SHA-256: {}", e))?
        .to_lowercase();
    if actual_hash != expected {
        return Err(format!(
            "Payload SHA-256 mismatch inside helper: expected {}, got {}",
            expected, actual_hash
        ));
    }
    log("HELPER", &format!("Payload SHA-256 verified inside helper: {}", actual_hash));

    #[cfg(target_os = "windows")]
    {
        let payload_signed = crate::platform::ffi::verify_embedded_signature(payload);
        if !payload_signed {
            log("HELPER_WARN", "Payload is not digitally signed; proceeding with verified SHA-256 hash");
        }
    }

    Ok(())
}

pub fn run_updater_helper(args: &[String]) {
    log("HELPER", "Updater helper spawned");

    let parsed = match HelperArgs::parse(args) {
        Some(p) => p,
        None => {
            log("HELPER_ERR", "Missing required CLI arguments for helper execution");
            std::process::exit(1);
        }
    };

    log(
        "HELPER",
        &format!(
            "Target: '{}', Payload: '{}', WaitPID: {}, HasExpectedSha: {}",
            parsed.target.display(),
            parsed.payload.display(),
            parsed.wait_pid,
            parsed.expected_sha256.is_some()
        ),
    );

    if let Err(e) = validate_target_path(&parsed.target) {
        log("HELPER_ERR", &format!("Target validation rejected: {}", e));
        std::process::exit(1);
    }

    if let Err(e) = validate_payload_path(&parsed.payload) {
        log("HELPER_ERR", &format!("Payload location rejected: {}", e));
        std::process::exit(1);
    }

    let expected_sha = match parsed.expected_sha256.as_deref() {
        Some(sha) if !sha.is_empty() => sha,
        _ => {
            log("HELPER_ERR", "Missing required --expected-sha256 hash parameter");
            std::process::exit(1);
        }
    };

    #[cfg(target_os = "windows")]
    let mut payload_file = {
        let mut opts = std::fs::OpenOptions::new();
        opts.read(true);
        // Exclusively share for read only: prevents external modifications or deletion (anti-TOCTOU)
        opts.share_mode(1);
        match opts.open(&parsed.payload) {
            Ok(f) => f,
            Err(e) => {
                log("HELPER_ERR", &format!("Cannot acquire exclusive read lock on payload: {}", e));
                std::process::exit(1);
            }
        }
    };

    #[cfg(not(target_os = "windows"))]
    let mut payload_file = match std::fs::File::open(&parsed.payload) {
        Ok(f) => f,
        Err(e) => {
            log("HELPER_ERR", &format!("Cannot open payload: {}", e));
            std::process::exit(1);
        }
    };

    let meta = match payload_file.metadata() {
        Ok(m) => m,
        Err(e) => {
            log("HELPER_ERR", &format!("Failed to read payload metadata: {}", e));
            std::process::exit(1);
        }
    };

    if meta.len() < 500_000 || meta.len() > 150_000_000 {
        log("HELPER_ERR", &format!("Payload size {} outside valid boundaries [500KB - 150MB]", meta.len()));
        std::process::exit(1);
    }

    let mut header = [0u8; 2];
    if payload_file.read_exact(&mut header).is_err() || &header != b"MZ" {
        log("HELPER_ERR", "Payload is not a valid Windows PE binary (missing MZ header)");
        std::process::exit(1);
    }

    if let Err(e) = payload_file.seek(SeekFrom::Start(0)) {
        log("HELPER_ERR", &format!("Failed to rewind payload: {}", e));
        std::process::exit(1);
    }

    let mut hasher = Sha256::new();
    let mut hash_buf = [0u8; 64 * 1024];
    loop {
        match payload_file.read(&mut hash_buf) {
            Ok(0) => break,
            Ok(n) => hasher.update(&hash_buf[..n]),
            Err(e) => {
                log("HELPER_ERR", &format!("Failed to stream payload bytes for hashing: {}", e));
                std::process::exit(1);
            }
        }
    }
    let actual_hash = format!("{:x}", hasher.finalize()).to_lowercase();
    if actual_hash != expected_sha {
        log("HELPER_ERR", &format!("Payload SHA-256 mismatch inside helper: expected {}, got {}", expected_sha, actual_hash));
        std::process::exit(1);
    }
    log("HELPER", &format!("Payload SHA-256 verified on locked handle: {}", actual_hash));

    #[cfg(target_os = "windows")]
    {
        let payload_signed = crate::platform::ffi::verify_embedded_signature(&parsed.payload);
        if !payload_signed {
            log("HELPER_WARN", "Payload is not digitally signed; proceeding with verified SHA-256 hash");
        }
    }

    #[cfg(target_os = "windows")]
    {
        if parsed.wait_pid > 0 {
            log("HELPER", &format!("Waiting for parent process PID {} to exit...", parsed.wait_pid));
            crate::platform::ffi::wait_for_process_exit(parsed.wait_pid, 30_000);

            use crate::platform::ffi::{winapi, SYNCHRONIZE};
            let handle = unsafe { winapi::OpenProcess(SYNCHRONIZE, 0, parsed.wait_pid) };
            if !handle.is_null() {
                let status = unsafe { winapi::WaitForSingleObject(handle, 2000) };
                unsafe { winapi::CloseHandle(handle) };
                if status == 0x0000_0102 {
                    log("HELPER_ERR", "Parent process still running after 32 seconds; aborting to prevent lock collision");
                    std::process::exit(1);
                }
            }
        }

        std::thread::sleep(Duration::from_millis(250));

        if parsed.backup.is_file() {
            let _ = fs::remove_file(&parsed.backup);
        }

        let mut backup_ok = false;
        for attempt in 1..=10 {
            if !parsed.target.is_file() {
                backup_ok = true;
                break;
            }
            if fs::rename(&parsed.target, &parsed.backup).is_ok() {
                backup_ok = true;
                log("HELPER", &format!("Existing binary backed up to '{}' (attempt {attempt})", parsed.backup.display()));
                break;
            }
            std::thread::sleep(Duration::from_millis(150 + attempt * 50));
        }

        if !backup_ok {
            log("HELPER_ERR", &format!("Failed to rename target '{}' to backup; file may still be locked", parsed.target.display()));
            std::process::exit(1);
        }

        let mut install_ok = false;
        for attempt in 1..=5 {
            if let Err(e) = payload_file.seek(SeekFrom::Start(0)) {
                log("HELPER_WARN", &format!("Seek error on locked payload: {}", e));
                continue;
            }
            match fs::File::create(&parsed.target) {
                Ok(mut target_file) => {
                    if std::io::copy(&mut payload_file, &mut target_file).is_ok() {
                        if target_file.sync_all().is_ok() {
                            install_ok = true;
                            break;
                        }
                    }
                }
                Err(_) => {}
            }
            std::thread::sleep(Duration::from_millis(150 + attempt * 50));
        }

        drop(payload_file);
        let _ = fs::remove_file(&parsed.payload);

        if !install_ok {
            log("HELPER_ERR", "Failed to place new payload at target; initiating immediate rollback");
            if parsed.backup.is_file() {
                let _ = fs::rename(&parsed.backup, &parsed.target);
            }
            std::process::exit(1);
        }

        log("HELPER", "Binary replacement succeeded");

        if parsed.no_relaunch {
            log("HELPER", "Exiting cleanly without relaunch as requested");
            std::process::exit(0);
        }

        log("HELPER", "Launching updated application");

        let mut cmd = Command::new(&parsed.target);
        cmd.arg("--post-update-check");
        cmd.arg("--restarted-from-pid");
        cmd.arg(parsed.wait_pid.to_string());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                log("HELPER_ERR", &format!("Failed to spawn updated binary: {}; rolling back...", e));
                rollback_and_restore(&parsed.target, &parsed.backup);
                std::process::exit(1);
            }
        };

        let watchdog_start = Instant::now();
        let watchdog_timeout = Duration::from_secs(3);
        let mut crashed = false;

        while watchdog_start.elapsed() < watchdog_timeout {
            std::thread::sleep(Duration::from_millis(150));
            match child.try_wait() {
                Ok(Some(status)) => {
                    log(
                        "HELPER_ERR",
                        &format!("New version crashed during startup with exit status: {:?}; initiating rollback", status.code()),
                    );
                    crashed = true;
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    log("HELPER_WARN", &format!("Could not inspect child process: {}", e));
                }
            }
        }

        if crashed {
            rollback_and_restore(&parsed.target, &parsed.backup);
            std::process::exit(2);
        }

        log("SUCCESS", "Updated application is running stably; cleaning up backup and staging files");
        if parsed.backup.is_file() {
            let _ = fs::remove_file(&parsed.backup);
        }
        if let Some(parent) = parsed.payload.parent() {
            if parent.to_string_lossy().contains("synchro_update") {
                let _ = fs::remove_dir_all(parent);
            }
        }

        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        log("HELPER", "Helper execution simulated on non-windows platform");
        std::process::exit(0);
    }
}

fn rollback_and_restore(target: &Path, backup: &Path) {
    if backup.is_file() {
        let _ = fs::remove_file(target);
        if fs::rename(backup, target).is_ok() {
            log("ROLLBACK", "Successfully restored previous working binary");
            let _ = Command::new(target).spawn();
        } else {
            log("ROLLBACK_CRIT", "Failed to restore backup binary during rollback");
        }
    } else {
        log("ROLLBACK_CRIT", "Backup binary missing; cannot perform rollback");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_args_parsing() {
        let args = vec![
            "--target".to_string(),
            r"C:\Apps\synchro.exe".to_string(),
            "--payload".to_string(),
            r"C:\Temp\update.bin".to_string(),
            "--wait-pid".to_string(),
            "1234".to_string(),
            "--expected-sha256".to_string(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        ];
        let parsed = HelperArgs::parse(&args).expect("Should parse valid args");
        assert_eq!(parsed.target, PathBuf::from(r"C:\Apps\synchro.exe"));
        assert_eq!(parsed.payload, PathBuf::from(r"C:\Temp\update.bin"));
        assert_eq!(parsed.wait_pid, 1234);
        assert_eq!(parsed.expected_sha256, Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()));
    }

    #[test]
    fn test_validate_target_path() {
        assert!(validate_target_path(Path::new(r"C:\Users\test\AppData\Local\Programs\synchro\synchro.exe")).is_ok());
        assert!(validate_target_path(Path::new(r"C:\Program Files\Synchro Nova\synchro.exe")).is_ok());
        assert!(validate_target_path(Path::new(r"C:\Users\tyler\Desktop\synchro-test-updater.exe")).is_ok());
        assert!(validate_target_path(Path::new(r"C:\Users\tyler\Desktop\synchro-portable.exe")).is_ok());

        assert!(validate_target_path(Path::new(r"C:\Windows\System32\cmd.exe")).is_err());
        assert!(validate_target_path(Path::new(r"C:\Windows\System32\synchro.exe")).is_err());
        assert!(validate_target_path(Path::new(r"C:\Apps\other_app.exe")).is_err());
    }
}
