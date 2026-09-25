use std::fs;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
pub fn apply_install() -> Result<(), String> {
    let temp_file = super::client::get_update_file_path();
    if !temp_file.exists() {
        return Err("Downloaded update file not found on disk".to_string());
    }

    let meta = fs::metadata(&temp_file)
        .map_err(|e| format!("Failed to read update file metadata: {}", e))?;
    if meta.len() < 500_000 {
        return Err(format!("Update file corrupted or incomplete: {} bytes", meta.len()));
    }

    // Verify valid PE header (fail-closed)
    let mut header_buf = [0u8; 2];
    {
        use std::io::Read;
        let mut file = fs::File::open(&temp_file)
            .map_err(|e| format!("Failed to open update file for verification: {}", e))?;
        file.read_exact(&mut header_buf)
            .map_err(|e| format!("Failed to read executable header from update file: {}", e))?;
    }
    if &header_buf != b"MZ" {
        let _ = fs::remove_file(&temp_file);
        return Err("Downloaded update is not a valid Windows executable (missing MZ signature)".to_string());
    }

    // Verify SHA-256 against release manifest if available in update cache
    let update_dir = super::client::get_update_dir();
    let sha_file = update_dir.join("SHA256SUMS.txt");
    if sha_file.exists() {
        let contents = fs::read_to_string(&sha_file)
            .map_err(|e| format!("Failed to read release SHA manifest: {}", e))?;
        let mut expected_hash = None;
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let hash = parts[0].trim().to_lowercase();
                let fname = parts[1].trim().trim_start_matches('*');
                if fname == "synchro.exe" || fname.ends_with(".exe") {
                    expected_hash = Some(hash);
                    break;
                }
            }
        }
        if let Some(expected) = expected_hash {
            let actual = crate::infra::hash::sha256_file(&temp_file)
                .map_err(|e| format!("Failed to compute update file SHA-256: {}", e))?;
            if actual.to_lowercase() != expected {
                let _ = fs::remove_file(&temp_file);
                return Err(format!(
                    "Integrity check failed: expected SHA-256 {}, got {}",
                    expected, actual
                ));
            }
        }
    }

    // Check Authenticode signature: if currently running executable is signed, update MUST also be signed
    if let Ok(current_path) = std::env::current_exe() {
        if crate::platform::ffi::verify_embedded_signature(&current_path)
            && !crate::platform::ffi::verify_embedded_signature(&temp_file)
        {
            let _ = fs::remove_file(&temp_file);
            return Err("Update binary does not possess a valid Authenticode signature matching running instance".to_string());
        }
    }

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to locate current executable path: {}", e))?;

    let current_str = current_exe.to_string_lossy().to_string();
    let temp_str = temp_file.to_string_lossy().to_string();

    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let cmd_exe = std::path::PathBuf::from(&system_root).join("System32").join("cmd.exe");
    let ping_exe = std::path::PathBuf::from(&system_root).join("System32").join("ping.exe");

    let cmd_script = format!(
        "\"{}\" 127.0.0.1 -n 2 >nul && copy /y \"{}\" \"{}\" >nul && start \"\" \"{}\"",
        ping_exe.to_string_lossy(), temp_str, current_str, current_str
    );

    // Test write permission to target binary (e.g. Program Files requires elevation)
    let needs_elevation = match fs::OpenOptions::new().write(true).open(&current_exe) {
        Ok(_) => false,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => true,
        Err(_) => false,
    };

    if needs_elevation {
        crate::platform::ffi::runas_executable_with_args(&cmd_exe, Some(&format!("/c {cmd_script}")))?;
    } else {
        let mut cmd = Command::new(&cmd_exe);
        cmd.args(["/c", &cmd_script]);
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.spawn()
            .map_err(|e| format!("Failed to spawn updater process: {}", e))?;
    }

    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn apply_install() -> Result<(), String> {
    Err("In-place self-updater only supported on Windows".to_string())
}
