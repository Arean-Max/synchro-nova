use serde::{Deserialize, Serialize};
use std::process::{Command, Output};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakApplyResult {
    pub id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakStatus {
    pub id: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakRegistrySnapshot {
    hive: String,
    path: String,
    name: String,
    value: Option<TweakRegistryValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum TweakRegistryValue {
    Dword(u32),
    Text(String),
}

pub(crate) fn apply_selected_tweaks(ids: Vec<String>) -> Vec<TweakApplyResult> {
    ids.into_iter().take(80).map(|id| apply_one(&id)).collect()
}

pub(crate) fn collect_tweak_statuses() -> Vec<TweakStatus> {
    known_tweak_ids()
        .iter()
        .map(|id| TweakStatus {
            id: (*id).to_string(),
            installed: is_tweak_applied(id),
        })
        .collect()
}

pub(crate) fn collect_tweak_registry_snapshot() -> Vec<TweakRegistrySnapshot> {
    registry_backup_targets()
        .iter()
        .map(|target| TweakRegistrySnapshot {
            hive: target.hive.to_string(),
            path: target.path.to_string(),
            name: target.name.to_string(),
            value: read_registry_value(target),
        })
        .collect()
}

pub(crate) fn restore_tweak_registry_snapshot(snapshot: &[TweakRegistrySnapshot]) -> Vec<String> {
    snapshot
        .iter()
        .filter_map(|entry| restore_registry_value(entry).err())
        .collect()
}

#[derive(Clone, Copy)]
struct RegistryTarget {
    hive: &'static str,
    path: &'static str,
    name: &'static str,
    value_kind: RegistryValueKind,
}

#[derive(Clone, Copy)]
enum RegistryValueKind {
    Dword,
    Text,
}

static REGISTRY_BACKUP_TARGETS: &[RegistryTarget] = &[
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\GameBar",
        "AutoGameModeEnabled",
    ),
    reg_dword("HKCU", "Software\\Microsoft\\GameBar", "AllowAutoGameMode"),
    reg_dword("HKCU", "Software\\Microsoft\\GameBar", "ShowStartupPanel"),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\GameBar",
        "UseNexusForGameBarEnabled",
    ),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_Enabled"),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
        "AppCaptureEnabled",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
        "HistoricalCaptureEnabled",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
        "AllowGameDVR",
    ),
    reg_text("HKCU", "Control Panel\\Mouse", "MouseSpeed"),
    reg_text("HKCU", "Control Panel\\Mouse", "MouseThreshold1"),
    reg_text("HKCU", "Control Panel\\Mouse", "MouseThreshold2"),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo",
        "Enabled",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Privacy",
        "TailoredExperiencesWithDiagnosticDataEnabled",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
        "EnableActivityFeed",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
        "PublishUserActivities",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Policies\\Microsoft\\Windows\\System",
        "UploadUserActivities",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Clipboard",
        "EnableClipboardHistory",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Clipboard",
        "CloudClipboardAutomaticUpload",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization",
        "DODownloadMode",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "HwSchMode",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        "OverlayTestMode",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "DisableOverlays",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "NetworkThrottlingIndex",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        "GPU Priority",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        "Priority",
    ),
    reg_text(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        "Scheduling Category",
    ),
    reg_text(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        "SFIO Priority",
    ),
    reg_dword(
        "HKLM",
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "SystemResponsiveness",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects",
        "VisualFXSetting",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        "EnableTransparency",
    ),
    reg_dword(
        "HKCU",
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Serialize",
        "StartupDelayInMSec",
    ),
    reg_text("HKCU", "Control Panel\\Desktop", "MenuShowDelay"),
];

fn registry_backup_targets() -> &'static [RegistryTarget] {
    REGISTRY_BACKUP_TARGETS
}

const fn reg_dword(hive: &'static str, path: &'static str, name: &'static str) -> RegistryTarget {
    RegistryTarget {
        hive,
        path,
        name,
        value_kind: RegistryValueKind::Dword,
    }
}

const fn reg_text(hive: &'static str, path: &'static str, name: &'static str) -> RegistryTarget {
    RegistryTarget {
        hive,
        path,
        name,
        value_kind: RegistryValueKind::Text,
    }
}

#[cfg(target_os = "windows")]
fn read_registry_value(target: &RegistryTarget) -> Option<TweakRegistryValue> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    let root = match target.hive {
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        _ => return None,
    };
    let key = root.open_subkey(target.path).ok()?;
    match target.value_kind {
        RegistryValueKind::Dword => key
            .get_value(target.name)
            .ok()
            .map(TweakRegistryValue::Dword),
        RegistryValueKind::Text => key
            .get_value(target.name)
            .ok()
            .map(TweakRegistryValue::Text),
    }
}

#[cfg(not(target_os = "windows"))]
fn read_registry_value(_target: &RegistryTarget) -> Option<TweakRegistryValue> {
    None
}

#[cfg(target_os = "windows")]
fn restore_registry_value(entry: &TweakRegistrySnapshot) -> Result<(), String> {
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    let root = match entry.hive.as_str() {
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        _ => return Err(format!("Unsupported registry hive {}", entry.hive)),
    };

    if let Some(value) = &entry.value {
        let (key, _) = root
            .create_subkey(&entry.path)
            .map_err(|error| format!("{}\\{}: {error}", entry.hive, entry.path))?;
        match value {
            TweakRegistryValue::Dword(value) => key
                .set_value(&entry.name, value)
                .map_err(|error| format!("{}: {error}", entry.name)),
            TweakRegistryValue::Text(value) => key
                .set_value(&entry.name, value)
                .map_err(|error| format!("{}: {error}", entry.name)),
        }
    } else {
        if let Ok(key) = root.open_subkey_with_flags(&entry.path, winreg::enums::KEY_SET_VALUE) {
            let _ = key.delete_value(&entry.name);
        }
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn restore_registry_value(_entry: &TweakRegistrySnapshot) -> Result<(), String> {
    Ok(())
}

fn is_admin_tweak(id: &str) -> bool {
    matches!(
        id,
        "disable-gamedvr"
            | "mmcss-games-priority"
            | "system-responsiveness-10"
            | "hags-on"
            | "mpo-disable"
            | "activity-history-off"
            | "delivery-optimization-lan"
            | "trim-enable"
            | "ntfs-last-access-off"
            | "hibernate-off"
            | "rss-on"
            | "rsc-off"
            | "ecn-off"
            | "winsock-reset"
            | "network-throttle-off"
            | "restore-point-first"
    )
}

fn apply_one(id: &str) -> TweakApplyResult {
    if is_admin_tweak(id) && !crate::admin::is_running_elevated() {
        return failed(
            id,
            "Requires administrator privileges; restart Synchro as administrator to apply this tweak",
        );
    }
    match id {
        "game-mode-on" => apply_game_mode(id),
        "disable-gamedvr" => apply_disable_gamedvr(id),
        "disable-bg-recording" => apply_disable_bg_recording(id),
        "gamebar-startup-off" => apply_gamebar_startup_off(id),
        "pointer-precision-off" => apply_pointer_precision_off(id),
        "advertising-id-off" => apply_advertising_id_off(id),
        "tailored-experiences-off" => apply_tailored_experiences_off(id),
        "activity-history-off" => apply_activity_history_off(id),
        "clipboard-cloud-off" => apply_clipboard_cloud_off(id),
        "delivery-optimization-lan" => apply_delivery_optimization_lan(id),
        "power-plan-high" => run_powercfg(
            id,
            &["/setactive", "SCHEME_MIN"],
            "High performance power plan selected",
        ),
        "ultimate-performance-plan" => apply_ultimate_performance(id),
        "dns-cache-flush" => {
            run_command_result(id, "ipconfig", &["/flushdns"], "DNS cache refreshed")
        }
        "tcp-autotune-normal" => run_netsh(
            id,
            &[
                "interface",
                "tcp",
                "set",
                "global",
                "autotuninglevel=normal",
            ],
            "TCP autotuning restored to normal",
        ),
        "rss-on" => run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rss=enabled"],
            "Receive-side scaling enabled",
        ),
        "rsc-off" => run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rsc=disabled"],
            "Receive segment coalescing disabled",
        ),
        "ecn-off" => run_netsh(
            id,
            &[
                "interface",
                "tcp",
                "set",
                "global",
                "ecncapability=disabled",
            ],
            "ECN disabled",
        ),
        "hags-on" => apply_hags(id),
        "mpo-disable" => apply_mpo_disable(id),
        "network-throttle-off" => apply_network_throttle_off(id),
        "mmcss-games-priority" => apply_mmcss_games_priority(id),
        "system-responsiveness-10" => apply_system_responsiveness(id),
        "usb-selective-suspend-off" => apply_usb_selective_suspend(id),
        "visual-effects-performance" => apply_visual_effects_performance(id),
        "transparency-off" => apply_transparency_off(id),
        "startup-delay-off" => apply_startup_delay_off(id),
        "menu-show-delay-low" => apply_menu_show_delay_low(id),
        "flush-arp-cache" => run_command_result(
            id,
            "netsh",
            &["interface", "ip", "delete", "arpcache"],
            "ARP cache refreshed",
        ),
        "winsock-reset" => run_netsh(
            id,
            &["winsock", "reset"],
            "Winsock reset requested; reboot required",
        ),
        "ntfs-last-access-off" => run_command_result(
            id,
            "fsutil",
            &["behavior", "set", "disableLastAccess", "1"],
            "NTFS last access updates disabled",
        ),
        "trim-enable" => run_command_result(
            id,
            "fsutil",
            &["behavior", "set", "DisableDeleteNotify", "0"],
            "TRIM notifications enabled",
        ),
        "hibernate-off" => run_command_result(
            id,
            "powercfg",
            &["/hibernate", "off"],
            "Hibernate disabled; Fast Startup also disabled",
        ),
        "restore-point-first" => apply_restore_point(id),
        "memory-compression-keep" => skipped(id, "Memory compression left enabled"),
        "firewall-keep-on" => skipped(id, "Firewall left enabled"),
        "signed-driver-only" => skipped(id, "Driver signature policy left intact"),
        "dynamic-tick-off" | "platform-tick-force" => blocked(
            id,
            "BCDEdit timer forcing is debug-oriented and was not applied automatically",
        ),
        "pagefile-off" | "memory-integrity-off" | "driver-msi-bulk" | "gpu-msi-mode" => blocked(
            id,
            "Risky system-wide change blocked by the safe apply engine",
        ),
        _ => skipped(
            id,
            "This tweak is advisory or experimental and has no safe automatic action yet",
        ),
    }
}

fn known_tweak_ids() -> &'static [&'static str] {
    &[
        "game-mode-on",
        "disable-gamedvr",
        "disable-bg-recording",
        "gamebar-startup-off",
        "pointer-precision-off",
        "advertising-id-off",
        "tailored-experiences-off",
        "activity-history-off",
        "clipboard-cloud-off",
        "delivery-optimization-lan",
        "hags-on",
        "mpo-disable",
        "network-throttle-off",
        "mmcss-games-priority",
        "system-responsiveness-10",
        "visual-effects-performance",
        "transparency-off",
        "startup-delay-off",
        "menu-show-delay-low",
    ]
}

fn is_tweak_applied(id: &str) -> bool {
    match id {
        "game-mode-on" => hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled") == Some(1),
        "disable-gamedvr" => {
            hkcu_dword("System\\GameConfigStore", "GameDVR_Enabled") == Some(0)
                && hkcu_dword(
                    "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                    "AppCaptureEnabled",
                ) == Some(0)
        }
        "disable-bg-recording" => {
            hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "HistoricalCaptureEnabled",
            ) == Some(0)
        }
        "gamebar-startup-off" => hkcu_dword("Software\\Microsoft\\GameBar", "ShowStartupPanel") == Some(0),
        "pointer-precision-off" => {
            hkcu_string("Control Panel\\Mouse", "MouseSpeed").as_deref() == Some("0")
                && hkcu_string("Control Panel\\Mouse", "MouseThreshold1").as_deref() == Some("0")
                && hkcu_string("Control Panel\\Mouse", "MouseThreshold2").as_deref() == Some("0")
        }
        "advertising-id-off" => {
            hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo", "Enabled") == Some(0)
        }
        "tailored-experiences-off" => {
            hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
            ) == Some(0)
        }
        "activity-history-off" => {
            hklm_dword("SOFTWARE\\Policies\\Microsoft\\Windows\\System", "EnableActivityFeed") == Some(0)
        }
        "clipboard-cloud-off" => {
            hkcu_dword("Software\\Microsoft\\Clipboard", "CloudClipboardAutomaticUpload") == Some(0)
        }
        "delivery-optimization-lan" => {
            hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization",
                "DODownloadMode",
            ) == Some(1)
        }
        "hags-on" => hklm_dword("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers", "HwSchMode") == Some(2),
        "mpo-disable" => hklm_dword("SOFTWARE\\Microsoft\\Windows\\Dwm", "OverlayTestMode") == Some(5),
        "network-throttle-off" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "NetworkThrottlingIndex",
            ) == Some(0xffff_ffff)
        }
        "mmcss-games-priority" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "GPU Priority",
            ) == Some(8)
        }
        "system-responsiveness-10" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "SystemResponsiveness",
            ) == Some(10)
        }
        "visual-effects-performance" => {
            hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects", "VisualFXSetting")
                == Some(2)
        }
        "transparency-off" => {
            hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                "EnableTransparency",
            ) == Some(0)
        }
        "startup-delay-off" => {
            hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Serialize",
                "StartupDelayInMSec",
            ) == Some(0)
        }
        "menu-show-delay-low" => hkcu_string("Control Panel\\Desktop", "MenuShowDelay").as_deref() == Some("100"),
        _ => false,
    }
}

fn apply_game_mode(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled", 1),
            set_hkcu_dword("Software\\Microsoft\\GameBar", "AllowAutoGameMode", 1),
        ],
        "Game Mode enabled",
    )
}

fn apply_disable_gamedvr(id: &str) -> TweakApplyResult {
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

fn apply_disable_bg_recording(id: &str) -> TweakApplyResult {
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

fn apply_gamebar_startup_off(id: &str) -> TweakApplyResult {
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

fn apply_pointer_precision_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_string("Control Panel\\Mouse", "MouseSpeed", "0"),
            set_hkcu_string("Control Panel\\Mouse", "MouseThreshold1", "0"),
            set_hkcu_string("Control Panel\\Mouse", "MouseThreshold2", "0"),
        ],
        "Pointer precision disabled",
    )
}

fn apply_advertising_id_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo",
            "Enabled",
            0,
        )],
        "Advertising ID disabled",
    )
}

fn apply_tailored_experiences_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Privacy",
            "TailoredExperiencesWithDiagnosticDataEnabled",
            0,
        )],
        "Tailored experiences disabled",
    )
}

fn apply_activity_history_off(id: &str) -> TweakApplyResult {
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

fn apply_clipboard_cloud_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword(
                "Software\\Microsoft\\Clipboard",
                "EnableClipboardHistory",
                0,
            ),
            set_hkcu_dword(
                "Software\\Microsoft\\Clipboard",
                "CloudClipboardAutomaticUpload",
                0,
            ),
        ],
        "Cloud clipboard disabled",
    )
}

fn apply_delivery_optimization_lan(id: &str) -> TweakApplyResult {
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

fn apply_hags(id: &str) -> TweakApplyResult {
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

fn apply_mpo_disable(id: &str) -> TweakApplyResult {
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

fn apply_network_throttle_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
            "NetworkThrottlingIndex",
            0xffff_ffff,
        )],
        "MMCSS network throttling disabled",
    )
}

fn apply_mmcss_games_priority(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "GPU Priority",
                8,
            ),
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "Priority",
                6,
            ),
            set_hklm_string(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "Scheduling Category",
                "High",
            ),
            set_hklm_string(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "SFIO Priority",
                "High",
            ),
        ],
        "MMCSS games priority updated; reboot recommended",
    )
}

fn apply_system_responsiveness(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
            "SystemResponsiveness",
            10,
        )],
        "System responsiveness reserve set to 10; reboot recommended",
    )
}

fn apply_usb_selective_suspend(id: &str) -> TweakApplyResult {
    let results = [
        run_command(
            "powercfg",
            &[
                "/setacvalueindex",
                "SCHEME_CURRENT",
                "SUB_USB",
                "USBSELECTIVE",
                "0",
            ],
        ),
        run_command(
            "powercfg",
            &[
                "/setdcvalueindex",
                "SCHEME_CURRENT",
                "SUB_USB",
                "USBSELECTIVE",
                "0",
            ],
        ),
        run_command("powercfg", &["/setactive", "SCHEME_CURRENT"]),
    ];
    collect_result(
        id,
        results,
        "USB selective suspend disabled for current power plan",
    )
}

fn apply_visual_effects_performance(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects",
            "VisualFXSetting",
            2,
        )],
        "Visual effects set to performance profile",
    )
}

fn apply_transparency_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
            "EnableTransparency",
            0,
        )],
        "Windows transparency disabled",
    )
}

fn apply_startup_delay_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Serialize",
            "StartupDelayInMSec",
            0,
        )],
        "Startup app launch delay disabled",
    )
}

fn apply_menu_show_delay_low(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_string(
            "Control Panel\\Desktop",
            "MenuShowDelay",
            "100",
        )],
        "Desktop menu delay reduced",
    )
}

fn apply_ultimate_performance(id: &str) -> TweakApplyResult {
    const ULTIMATE_GUID: &str = "e9a42b02-d5df-448d-aa00-03f14749eb61";

    let duplicate = run_command_output("powercfg", &["/duplicatescheme", ULTIMATE_GUID]);
    let mut candidates = Vec::new();
    let mut last_error = None;

    match duplicate {
        Ok(output) => {
            if let Some(guid) = output_guid(&output) {
                candidates.push(guid);
            }
        }
        Err(error) => {
            last_error = Some(error);
        }
    }

    if !candidates
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(ULTIMATE_GUID))
    {
        candidates.push(ULTIMATE_GUID.to_string());
    }

    for guid in candidates {
        let args = ["/setactive", guid.as_str()];
        match run_command("powercfg", &args) {
            Ok(_) => return applied(id, "Ultimate Performance power plan selected"),
            Err(error) => last_error = Some(error),
        }
    }

    failed(
        id,
        &last_error.unwrap_or_else(|| {
            "Unable to activate Ultimate Performance power plan; run Synchro as administrator"
                .to_string()
        }),
    )
}

fn apply_restore_point(id: &str) -> TweakApplyResult {
    run_command_result(
        id,
        "powershell.exe",
        &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Checkpoint-Computer -Description 'Synchro tweak backup' -RestorePointType 'MODIFY_SETTINGS'",
        ],
        "Restore point requested",
    )
}

fn run_netsh(id: &str, args: &[&str], message: &str) -> TweakApplyResult {
    run_command_result(id, "netsh", args, message)
}

fn run_powercfg(id: &str, args: &[&str], message: &str) -> TweakApplyResult {
    run_command_result(id, "powercfg", args, message)
}

fn collect_result<const N: usize>(
    id: &str,
    results: [Result<(), String>; N],
    success: &str,
) -> TweakApplyResult {
    let errors = results
        .into_iter()
        .filter_map(|result| result.err())
        .collect::<Vec<_>>();
    if errors.is_empty() {
        applied(id, success)
    } else {
        failed(id, &errors.join("; "))
    }
}

fn run_command_result(id: &str, program: &str, args: &[&str], success: &str) -> TweakApplyResult {
    match run_command(program, args) {
        Ok(_) => applied(id, success),
        Err(error) => failed(id, &error),
    }
}

fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    run_command_output(program, args).map(|_| ())
}

fn resolve_system_program(program: &str) -> std::path::PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(system_root) = std::env::var("SystemRoot") {
            let system32 = std::path::PathBuf::from(system_root).join("System32");
            let candidate = if program.ends_with(".exe") {
                system32.join(program)
            } else {
                system32.join(format!("{program}.exe"))
            };
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    std::path::PathBuf::from(program)
}

fn run_command_output(program: &str, args: &[&str]) -> Result<Output, String> {
    let resolved = resolve_system_program(program);
    let mut command = Command::new(&resolved);
    command.args(args);
    hide_console(&mut command);
    let output = command
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if output.status.success() {
        Ok(output)
    } else {
        let code = output
            .status
            .code()
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        Err(format!(
            "{program} exited with code {code}; run Synchro as administrator and try again"
        ))
    }
}

fn output_guid(output: &Output) -> Option<String> {
    first_guid(&output.stdout).or_else(|| first_guid(&output.stderr))
}

fn first_guid(bytes: &[u8]) -> Option<String> {
    const GUID_LEN: usize = 36;
    if bytes.len() < GUID_LEN {
        return None;
    }

    bytes.windows(GUID_LEN).find_map(|window| {
        if window.iter().enumerate().all(|(index, byte)| {
            matches!(index, 8 | 13 | 18 | 23) && *byte == b'-'
                || !matches!(index, 8 | 13 | 18 | 23) && byte.is_ascii_hexdigit()
        }) {
            String::from_utf8(window.to_vec()).ok()
        } else {
            None
        }
    })
}

#[cfg(target_os = "windows")]
fn hide_console(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    command.creation_flags(0x0800_0000);
}

#[cfg(not(target_os = "windows"))]
fn hide_console(_command: &mut Command) {}

#[cfg(target_os = "windows")]
fn set_hkcu_dword(path: &str, name: &str, value: u32) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey(path)
        .map_err(|error| format!("{path}: {error}"))?;
    key.set_value(name, &value)
        .map_err(|error| format!("{name}: {error}"))
}

#[cfg(not(target_os = "windows"))]
fn set_hkcu_dword(_path: &str, _name: &str, _value: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_hklm_dword(path: &str, name: &str, value: u32) -> Result<(), String> {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm
        .create_subkey(path)
        .map_err(|error| format!("{path}: {error}"))?;
    key.set_value(name, &value)
        .map_err(|error| format!("{name}: {error}"))
}

#[cfg(not(target_os = "windows"))]
fn set_hklm_dword(_path: &str, _name: &str, _value: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_hkcu_string(path: &str, name: &str, value: &str) -> Result<(), String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey(path)
        .map_err(|error| format!("{path}: {error}"))?;
    key.set_value(name, &value)
        .map_err(|error| format!("{name}: {error}"))
}

#[cfg(not(target_os = "windows"))]
fn set_hkcu_string(_path: &str, _name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_hklm_string(path: &str, name: &str, value: &str) -> Result<(), String> {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm
        .create_subkey(path)
        .map_err(|error| format!("{path}: {error}"))?;
    key.set_value(name, &value)
        .map_err(|error| format!("{name}: {error}"))
}

#[cfg(not(target_os = "windows"))]
fn set_hklm_string(_path: &str, _name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn hkcu_dword(path: &str, name: &str) -> Option<u32> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(path)
        .ok()
        .and_then(|key| key.get_value::<u32, _>(name).ok())
}

#[cfg(not(target_os = "windows"))]
fn hkcu_dword(_path: &str, _name: &str) -> Option<u32> {
    None
}

#[cfg(target_os = "windows")]
fn hklm_dword(path: &str, name: &str) -> Option<u32> {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(path)
        .ok()
        .and_then(|key| key.get_value::<u32, _>(name).ok())
}

#[cfg(not(target_os = "windows"))]
fn hklm_dword(_path: &str, _name: &str) -> Option<u32> {
    None
}

#[cfg(target_os = "windows")]
fn hkcu_string(path: &str, name: &str) -> Option<String> {
    use winreg::{enums::HKEY_CURRENT_USER, RegKey};
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(path)
        .ok()
        .and_then(|key| key.get_value::<String, _>(name).ok())
}

#[cfg(not(target_os = "windows"))]
fn hkcu_string(_path: &str, _name: &str) -> Option<String> {
    None
}

fn applied(id: &str, message: &str) -> TweakApplyResult {
    result(id, "applied", message)
}

fn skipped(id: &str, message: &str) -> TweakApplyResult {
    result(id, "skipped", message)
}

fn blocked(id: &str, message: &str) -> TweakApplyResult {
    result(id, "blocked", message)
}

fn failed(id: &str, message: &str) -> TweakApplyResult {
    result(id, "failed", message)
}

fn result(id: &str, status: &str, message: &str) -> TweakApplyResult {
    TweakApplyResult {
        id: id.to_string(),
        status: status.to_string(),
        message: message.to_string(),
    }
}
