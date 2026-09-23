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
