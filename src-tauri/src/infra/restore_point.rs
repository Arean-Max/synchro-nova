use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn create_system_restore_point(description: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        if crate::platform::ffi::create_native_system_restore_point(description) {
            eprintln!("[RestorePoint] Native WinAPI restore point created: {}", description);
            return true;
        }

        let safe_desc: String = description
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(64)
            .collect();

        if safe_desc.is_empty() {
            return false;
        }

        let cmd = format!(
            "$p = 'HKLM:\\Software\\Microsoft\\Windows NT\\CurrentVersion\\SystemRestore'; \
             $k = Get-ItemProperty -Path $p -Name 'SystemRestorePointCreationFrequency' -ErrorAction SilentlyContinue; \
             Set-ItemProperty -Path $p -Name 'SystemRestorePointCreationFrequency' -Value 0 -Force -ErrorAction SilentlyContinue; \
             try {{ Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction SilentlyContinue }} \
             finally {{ \
                 if ($null -ne $k) {{ Set-ItemProperty -Path $p -Name 'SystemRestorePointCreationFrequency' -Value $k.SystemRestorePointCreationFrequency -Force -ErrorAction SilentlyContinue }} \
                 else {{ Remove-ItemProperty -Path $p -Name 'SystemRestorePointCreationFrequency' -ErrorAction SilentlyContinue }} \
             }}",
            safe_desc
        );

        let Ok(ps_path) = crate::domain::tweaks::runner::resolve_system_program("powershell") else {
            eprintln!("[RestorePoint] Failed to resolve PowerShell in System32");
            return false;
        };
        let mut child = Command::new(&ps_path);
        child.args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &cmd,
        ]);
        child.creation_flags(CREATE_NO_WINDOW);
        child.stdout(std::process::Stdio::null());
        child.stderr(std::process::Stdio::null());

        use std::time::{Duration, Instant};
        let mut spawned = match child.spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[RestorePoint] Failed to launch PowerShell: {}", e);
                return false;
            }
        };

        let start = Instant::now();
        let timeout = Duration::from_secs(25);
        let success = loop {
            match spawned.try_wait() {
                Ok(Some(status)) => {
                    let ok = status.success();
                    if ok {
                        eprintln!("[RestorePoint] PowerShell restore point created: {}", safe_desc);
                    } else {
                        eprintln!("[RestorePoint] PowerShell Checkpoint-Computer exited with code: {:?}", status.code());
                    }
                    break ok;
                }
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        let _ = spawned.kill();
                        let _ = spawned.wait();
                        eprintln!("[RestorePoint] PowerShell Checkpoint-Computer timed out after 25s");
                        break false;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    let _ = spawned.kill();
                    let _ = spawned.wait();
                    eprintln!("[RestorePoint] Failed to wait on PowerShell: {}", e);
                    break false;
                }
            }
        };
        success
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = description;
        false
    }
}
