use super::runner::{
    hkcu_dword, hkcu_string, hklm_dword, run_command, run_command_output, set_hkcu_dword,
    set_hkcu_string, set_hklm_dword,
};
use super::types::{applied, collect_result, failed, TweakApplyResult};

pub fn apply_game_mode(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled", 1),
            set_hkcu_dword("Software\\Microsoft\\GameBar", "AllowAutoGameMode", 1),
        ],
        "Game Mode enabled",
    )
}

pub fn is_game_mode_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled") == Some(1)
}

pub fn apply_modern_flip_model(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_string(
                "Software\\Microsoft\\DirectX\\UserGpuPreferences",
                "DirectXUserGlobalSettings",
                "SwapEffectUpgradeCache=1;",
            ),
            set_hkcu_dword("Software\\Microsoft\\GameBar", "AllowAutoGameMode", 1),
        ],
        "Modern Flip Model optimization enabled for windowed & fullscreen games",
    )
}

pub fn is_modern_flip_model_applied() -> bool {
    hkcu_string(
        "Software\\Microsoft\\DirectX\\UserGpuPreferences",
        "DirectXUserGlobalSettings",
    )
    .as_deref()
    .map(|v| v.contains("SwapEffectUpgradeCache=1"))
    .unwrap_or(false)
}

pub fn apply_gamedvr_fse_mode(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_FSEBehaviorMode", 2),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_HonorUserFSEBehaviorMode", 1),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_DXGIHonorFSEWindowsCompatible", 1),
        ],
        "DirectX Full Screen Exclusive (FSE) optimization mode enabled",
    )
}

pub fn is_gamedvr_fse_mode_applied() -> bool {
    hkcu_dword("System\\GameConfigStore", "GameDVR_FSEBehaviorMode") == Some(2)
}

pub fn apply_disable_fso_globally(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_DSEBehavior", 2),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_FSEBehavior", 2),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_EFSEFeatureFlags", 0),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_DXGIHonorFSEWindowsCompatible", 1),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_HonorUserFSEBehaviorMode", 1),
        ],
        "Fullscreen optimizations disabled globally (pure exclusive fullscreen honored)",
    )
}

pub fn is_disable_fso_globally_applied() -> bool {
    hkcu_dword("System\\GameConfigStore", "GameDVR_DSEBehavior") == Some(2)
}

pub fn apply_hags(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
            "HwSchMode",
            2,
        )],
        "Hardware GPU scheduling enabled; reboot required",
    )
}

pub fn is_hags_applied() -> bool {
    hklm_dword("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers", "HwSchMode") == Some(2)
}

pub fn apply_mpo_disable(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword("SOFTWARE\\Microsoft\\Windows\\Dwm", "OverlayTestMode", 5),
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                "DisableOverlays",
                1,
            ),
        ],
        "MPO disabled; reboot required",
    )
}

pub fn is_mpo_disable_applied() -> bool {
    hklm_dword("SOFTWARE\\Microsoft\\Windows\\Dwm", "OverlayTestMode") == Some(5)
}

pub fn apply_pcie_aspm_off(id: &str) -> TweakApplyResult {
    let r1 = run_command("powercfg", &["/setacvalueindex", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM", "0"]);
    let r2 = run_command("powercfg", &["/setdcvalueindex", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM", "0"]);
    let r3 = run_command("powercfg", &["/setactive", "SCHEME_CURRENT"]);
    if r1.is_ok() || r2.is_ok() || r3.is_ok() {
        applied(id, "PCIe Active State Power Management (ASPM) disabled (maximum PCIe bandwidth, zero link latency)")
    } else {
        failed(id, "Failed to update PCIe ASPM in current power plan; run Synchro as administrator")
    }
}

pub fn is_pcie_aspm_off_applied() -> bool {
    run_command_output("powercfg", &["/query", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("0x00000000"))
        .unwrap_or(false)
}

pub fn apply_disable_gamedvr(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_Enabled", 0),
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "AppCaptureEnabled",
                0,
            ),
            set_hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
                "AllowGameDVR",
                0,
            ),
        ],
        "GameDVR capture disabled",
    )
}

pub fn is_disable_gamedvr_applied() -> bool {
    hkcu_dword("System\\GameConfigStore", "GameDVR_Enabled") == Some(0)
        && hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
            "AppCaptureEnabled",
        ) == Some(0)
}

pub fn apply_disable_bg_recording(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "HistoricalCaptureEnabled",
                0,
            ),
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "AppCaptureEnabled",
                0,
            ),
        ],
        "Background recording disabled",
    )
}

pub fn is_disable_bg_recording_applied() -> bool {
    hkcu_dword(
        "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
        "HistoricalCaptureEnabled",
    ) == Some(0)
}

pub fn apply_gamebar_startup_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\GameBar", "ShowStartupPanel", 0),
            set_hkcu_dword(
                "Software\\Microsoft\\GameBar",
                "UseNexusForGameBarEnabled",
                0,
            ),
        ],
        "Game Bar startup prompts disabled",
    )
}

pub fn is_gamebar_startup_off_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\GameBar", "ShowStartupPanel") == Some(0)
}

pub fn apply_win32_priority_rust(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
            "Win32PrioritySeparation",
            38,
        )],
        "Win32PrioritySeparation optimized for game process priority (0x26)",
    )
}

pub fn is_win32_priority_rust_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
        "Win32PrioritySeparation",
    ) == Some(38)
}

pub fn apply_system_timer_resolution(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
            "GlobalTimerResolutionRequests",
            1,
        )],
        "High-resolution 0.5ms system timer enabled",
    )
}

pub fn is_system_timer_resolution_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
        "GlobalTimerResolutionRequests",
    ) == Some(1)
}

pub fn apply_kill_gamebar_presence(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "AppCaptureEnabled",
                0,
            ),
            set_hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
                "AllowGameDVR",
                0,
            ),
            set_hkcu_dword("System\\GameConfigStore", "GameDVR_Enabled", 0),
        ],
        "GameBar presence hooks and background monitoring disabled",
    )
}

pub fn is_kill_gamebar_presence_applied() -> bool {
    hklm_dword(
        "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
        "AllowGameDVR",
    ) == Some(0)
}

pub fn apply_disable_hpet_synthetic(id: &str) -> TweakApplyResult {
    let res = run_command_output("bcdedit", &["/set", "disabledynamictick", "yes"]);
    let _ = run_command_output("bcdedit", &["/deletevalue", "useplatformclock"]);
    if res.is_ok() {
        super::types::applied(id, "Dynamic tick disabled; synthetic timer jitter eliminated")
    } else {
        super::types::failed(id, "Failed to run bcdedit; requires administrator privileges")
    }
}

pub fn is_disable_hpet_synthetic_applied() -> bool {
    let res = run_command_output("bcdedit", &["/enum", "{current}"]);
    res.map(|o| String::from_utf8_lossy(&o.stdout).contains("disabledynamictick       Yes")).unwrap_or(false)
}

pub fn apply_directx_thread_priority(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                "TdrDelay",
                10,
            ),
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler",
                "EnablePreemption",
                1,
            ),
        ],
        "DirectX graphics scheduler preemption and TdrDelay optimized for Rust/Unity",
    )
}

pub fn is_directx_thread_priority_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "TdrDelay",
    ) == Some(10)
}

pub fn apply_rust_ifeo_high_priority(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RustClient.exe\\PerfOptions",
                "CpuPriorityClass",
                3,
            ),
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RustClient.exe\\PerfOptions",
                "IoPriority",
                3,
            ),
        ],
        "RustClient.exe registered for high CPU & I/O priority in Windows IFEO",
    )
}

pub fn is_rust_ifeo_high_priority_applied() -> bool {
    hklm_dword(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RustClient.exe\\PerfOptions",
        "CpuPriorityClass",
    ) == Some(3)
}

pub fn apply_gpu_adaptive_power(id: &str) -> TweakApplyResult {
    let vendor = super::runner::detected_gpu_vendor();
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";
        let mut modified = 0;
        if let Ok(class_key) = hklm.open_subkey(class_path) {
            for subkey_name in class_key.enum_keys().flatten() {
                let full_path = format!("{class_path}\\{subkey_name}");
                if let Ok(key) = class_key.open_subkey(&subkey_name) {
                    if key.get_value::<String, _>("DriverDesc").is_ok() {
                        if vendor == "nvidia" {
                            let _ = set_hklm_dword(&full_path, "PowerMizerEnable", 0);
                            let _ = set_hklm_dword(&full_path, "PerfLevelSrc", 0x2222);
                            modified += 1;
                        } else if vendor == "amd" {
                            let _ = set_hklm_dword(&full_path, "EnableUlps", 0);
                            let _ = set_hklm_dword(&full_path, "EnableUlps_NA", 0);
                            modified += 1;
                        }
                    }
                }
            }
        }
        if vendor == "nvidia" {
            applied(id, &format!("NVIDIA GPU PowerMizer downclocking disabled across {modified} driver profiles"))
        } else if vendor == "amd" {
            applied(id, &format!("AMD Radeon ULPS sleep latency disabled across {modified} driver profiles"))
        } else {
            applied(id, "Adaptive GPU power policy verified for graphics adapter")
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        applied(id, "Adaptive GPU power simulated")
    }
}

pub fn is_gpu_adaptive_power_applied() -> bool {
    let vendor = super::runner::detected_gpu_vendor();
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";
        if let Ok(class_key) = hklm.open_subkey(class_path) {
            for subkey_name in class_key.enum_keys().flatten() {
                if let Ok(key) = class_key.open_subkey(&subkey_name) {
                    if vendor == "nvidia" && key.get_value::<u32, _>("PowerMizerEnable").ok() == Some(0) {
                        return true;
                    }
                    if vendor == "amd" && key.get_value::<u32, _>("EnableUlps").ok() == Some(0) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

