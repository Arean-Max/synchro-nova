use super::runner::{
    hkcu_dword, hklm_dword, run_command_output, set_hkcu_dword, set_hklm_dword,
};
use super::types::{applied, collect_result, TweakApplyResult};

pub fn apply_clean_temp_junk(id: &str) -> TweakApplyResult {
    let mut total_bytes_freed: u64 = 0;
    let mut files_removed: usize = 0;

    let mut paths_to_clean = Vec::new();

    if let Ok(user_temp) = std::env::var("TEMP") {
        paths_to_clean.push(std::path::PathBuf::from(user_temp));
    }

    if let Ok(system_root) = std::env::var("SystemRoot") {
        paths_to_clean.push(std::path::PathBuf::from(system_root).join("Temp"));
    }

    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let local_path = std::path::PathBuf::from(local_app_data);
        paths_to_clean.push(local_path.join("D3DSCache"));
        paths_to_clean.push(local_path.join("CrashDumps"));
        paths_to_clean.push(local_path.join("Microsoft").join("Windows").join("WER").join("ReportArchive"));
        paths_to_clean.push(local_path.join("Microsoft").join("Windows").join("WER").join("ReportQueue"));
    }

    for dir in paths_to_clean {
        if dir.is_dir() {
            clean_directory_contents(&dir, &mut total_bytes_freed, &mut files_removed);
        }
    }

    let mb_freed = total_bytes_freed as f64 / (1024.0 * 1024.0);
    applied(
        id,
        &format!("Cleaned {files_removed} temporary files ({mb_freed:.1} MB junk and DirectX shader cache)"),
    )
}

pub fn is_hibernate_applied() -> bool {
    let sys_drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    let hiberfil = std::path::PathBuf::from(format!("{sys_drive}\\hiberfil.sys"));
    !hiberfil.exists()
        || hklm_dword("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power", "HiberbootEnabled") == Some(0)
}

pub fn is_ntfs_last_access_applied() -> bool {
    run_command_output("fsutil", &["behavior", "query", "disableLastAccess"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("DisableLastAccess = 1"))
        .unwrap_or(false)
}

pub fn is_trim_applied() -> bool {
    run_command_output("fsutil", &["behavior", "query", "DisableDeleteNotify"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("DisableDeleteNotify = 0"))
        .unwrap_or(false)
}

pub fn apply_wer_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "Disabled", 1),
            set_hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "DontShowUI", 1),
        ],
        "Windows Error Reporting (WerFault) disabled to eliminate crash lag spikes",
    )
}

pub fn is_wer_off_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "Disabled") == Some(1)
}

pub fn apply_disable_paging_executive(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
            "DisablePagingExecutive",
            1,
        )],
        "Kernel executive and drivers locked in RAM; disk paging disabled",
    )
}

pub fn is_disable_paging_executive_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "DisablePagingExecutive",
    ) == Some(1)
}

pub fn apply_disable_memory_compression(id: &str) -> TweakApplyResult {
    let res = run_command_output(
        "powershell",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Disable-MMAgent -MemoryCompression",
        ],
    );
    if res.is_ok() {
        applied(id, "Windows background memory compression disabled")
    } else {
        super::types::failed(id, "Failed to disable memory compression; requires administrator privileges")
    }
}

pub fn is_disable_memory_compression_applied() -> bool {
    run_command_output(
        "powershell",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "(Get-MMAgent).MemoryCompression",
        ],
    )
    .map(|o| String::from_utf8_lossy(&o.stdout).trim().eq_ignore_ascii_case("false"))
    .unwrap_or(false)
}

pub fn apply_large_system_cache_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
            "LargeSystemCache",
            0,
        )],
        "LargeSystemCache disabled to dedicate physical RAM to games",
    )
}
