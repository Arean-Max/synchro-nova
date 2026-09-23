use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn create_system_restore_point(description: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        // 1. Try native WinAPI SRSetRestorePointW first
        if crate::platform::ffi::create_native_system_restore_point(description) {
            eprintln!("[RestorePoint] Native WinAPI restore point created: {}", description);
            return true;
        }

        // 2. Fallback to PowerShell Checkpoint-Computer
        let safe_desc: String = description
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(64)
            .collect();

        if safe_desc.is_empty() {
            return false;
        }

        let cmd = format!(
            "Set-ItemProperty -Path 'HKLM:\\Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore' -Name 'SystemRestorePointCreationFrequency' -Value 0 -Force -ErrorAction SilentlyContinue; Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction SilentlyContinue",
            safe_desc
        );

        let mut child = Command::new("powershell.exe");
        child.args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &cmd,
        ]);
        child.creation_flags(CREATE_NO_WINDOW);

        match child.status() {
            Ok(status) => {
                let success = status.success();
                if success {
                    eprintln!("[RestorePoint] PowerShell restore point created: {}", safe_desc);
                } else {
                    eprintln!("[RestorePoint] PowerShell Checkpoint-Computer exited with code: {:?}", status.code());
                }
                success
            }
            Err(e) => {
                eprintln!("[RestorePoint] Failed to launch PowerShell: {}", e);
                false
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = description;
        false
    }
}
