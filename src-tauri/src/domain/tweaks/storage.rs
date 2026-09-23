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

pub fn apply_start_bing_search_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled", 0),
            set_hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "DisableSearchBoxSuggestions", 1),
        ],
        "Start menu web search and Bing suggestions disabled for fast local search",
    )
}

pub fn is_start_bing_search_off_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled") == Some(0)
        && hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "DisableSearchBoxSuggestions") == Some(1)
}

pub fn apply_delivery_optimization_lan(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization",
            "DODownloadMode",
            1,
        )],
        "Delivery Optimization limited to LAN policy",
    )
}

pub fn is_delivery_optimization_lan_applied() -> bool {
    hklm_dword(
        "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization",
        "DODownloadMode",
    ) == Some(1)
}

pub fn apply_activity_history_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
                "EnableActivityFeed",
                0,
            ),
            set_hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
                "PublishUserActivities",
                0,
            ),
            set_hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
                "UploadUserActivities",
                0,
            ),
        ],
        "Activity history policy disabled",
    )
}

pub fn is_activity_history_off_applied() -> bool {
    hklm_dword("SOFTWARE\\Policies\\Microsoft\\Windows\\System", "EnableActivityFeed") == Some(0)
}

pub fn apply_advertising_id_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo",
                "Enabled",
                0,
            ),
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
                0,
            ),
        ],
        "Advertising ID and tailored diagnostic experiences disabled",
    )
}

pub fn is_advertising_id_off_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo", "Enabled") == Some(0)
}

pub fn clean_directory_contents(dir: &std::path::Path, total_bytes: &mut u64, files_count: &mut usize) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
                let file_type = sym_meta.file_type();
                if file_type.is_symlink() {
                    // Do not traverse symlinks or directory junctions to prevent arbitrary file deletion
                    let len = sym_meta.len();
                    if std::fs::remove_file(&path).is_ok() || std::fs::remove_dir(&path).is_ok() {
                        *total_bytes += len;
                        *files_count += 1;
                    }
                } else if file_type.is_file() {
                    let len = sym_meta.len();
                    if std::fs::remove_file(&path).is_ok() {
                        *total_bytes += len;
                        *files_count += 1;
                    }
                } else if file_type.is_dir() {
                    clean_directory_contents(&path, total_bytes, files_count);
                    let _ = std::fs::remove_dir(&path);
                }
            }
        }
    }
}
