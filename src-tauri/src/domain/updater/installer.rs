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

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to locate current executable path: {}", e))?;

    let current_str = current_exe.to_string_lossy().to_string();
    let temp_str = temp_file.to_string_lossy().to_string();

    let cmd_script = format!(
        "ping 127.0.0.1 -n 2 >nul & copy /y \"{}\" \"{}\" >nul & start \"\" \"{}\"",
        temp_str, current_str, current_str
    );

    let mut cmd = Command::new("cmd.exe");
    cmd.args(["/c", &cmd_script]);
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.spawn()
        .map_err(|e| format!("Failed to spawn updater process: {}", e))?;

    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn apply_install() -> Result<(), String> {
    Err("In-place self-updater only supported on Windows".to_string())
}
