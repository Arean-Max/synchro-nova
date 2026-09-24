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

    // Verify valid PE header
    let mut header_buf = [0u8; 2];
    if let Ok(mut file) = fs::File::open(&temp_file) {
        use std::io::Read;
        if file.read_exact(&mut header_buf).is_ok() && &header_buf != b"MZ" {
            return Err("Downloaded update is not a valid Windows executable (missing MZ signature)".to_string());
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
