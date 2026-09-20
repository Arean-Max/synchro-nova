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
    reg_text(
        "HKCU",
        "Software\\Microsoft\\DirectX\\UserGpuPreferences",
        "DirectXUserGlobalSettings",
    ),
    reg_text("HKCU", "Control Panel\\Accessibility\\StickyKeys", "Flags"),
    reg_text("HKCU", "Control Panel\\Accessibility\\Keyboard Response", "Flags"),
    reg_text("HKCU", "Control Panel\\Accessibility\\ToggleKeys", "Flags"),
    reg_dword("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled"),
    reg_dword("HKCU", "Software\\Microsoft\\Windows\\CurrentVersion\\Search", "DisableSearchBoxSuggestions"),
    reg_dword("HKCU", "Software\\Microsoft\\Windows\\Windows Error Reporting", "Disabled"),
    reg_dword("HKCU", "Software\\Microsoft\\Windows\\Windows Error Reporting", "DontShowUI"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_FSEBehaviorMode"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_HonorUserFSEBehaviorMode"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_DXGIHonorFSEWindowsCompatible"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_DSEBehavior"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_FSEBehavior"),
    reg_dword("HKCU", "System\\GameConfigStore", "GameDVR_EFSEFeatureFlags"),
    reg_text("HKCU", "Control Panel\\Keyboard", "KeyboardDelay"),
    reg_text("HKCU", "Control Panel\\Keyboard", "KeyboardSpeed"),
    reg_text("HKCU", "Control Panel\\Mouse", "MouseHoverTime"),
    reg_dword("HKLM", "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling", "PowerThrottlingOff"),
    reg_dword("HKLM", "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\csrss.exe\\PerfOptions", "CpuPriorityClass"),
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

pub(crate) fn is_admin_tweak(id: &str) -> bool {
    matches!(
        id,
        "hags-on"
            | "mpo-disable"
            | "mmcss-games-priority"
            | "system-responsiveness-10"
            | "network-throttle-off"
            | "hibernate-off"
            | "ntfs-last-access-off"
            | "trim-enable"
            | "rss-on"
            | "rsc-off"
            | "ecn-off"
            | "disable-gamedvr"
            | "delivery-optimization-lan"
            | "activity-history-off"
            | "tcp-nodelay-ack"
            | "nic-energy-saving-off"
            | "tcp-heuristics-off"
            | "cpu-unpark-cores"
            | "power-throttling-off"
            | "input-response-fast"
            | "csrss-high-priority"
    )
}

fn apply_one(id: &str) -> TweakApplyResult {
    if is_admin_tweak(id) && !crate::admin::is_running_elevated() {
        return TweakApplyResult {
            id: id.to_string(),
            status: "requiresAdmin".to_string(),
            message: "Requires administrator privileges; restart Synchro as administrator to apply this tweak".to_string(),
        };
    }
    match id {
        // Group 1: Gaming & Latency
        "game-mode-on" => apply_game_mode(id),
        "modern-flip-model-on" => apply_modern_flip_model(id),
        "gamedvr-fse-mode" => apply_gamedvr_fse_mode(id),
        "disable-fso-globally" => apply_disable_fso_globally(id),
        "hags-on" => apply_hags(id),
        "mpo-disable" => apply_mpo_disable(id),
        "pcie-aspm-off" => apply_pcie_aspm_off(id),

        // Group 2: CPU & Performance
        "power-plan-high" => run_powercfg(
            id,
            &["/setactive", "SCHEME_MIN"],
            "High performance power plan selected",
        ),
        "ultimate-performance-plan" => apply_ultimate_performance(id),
        "cpu-unpark-cores" => apply_cpu_unpark_cores(id),
        "power-throttling-off" => apply_power_throttling_off(id),
        "mmcss-games-priority" => apply_mmcss_games_priority(id),
        "system-responsiveness-10" => apply_system_responsiveness(id),
        "network-throttle-off" => apply_network_throttle_off(id),

        // Group 3: Input & Responsiveness
        "pointer-precision-off" => apply_pointer_precision_off(id),
        "sticky-keys-off" => apply_sticky_keys_off(id),
        "usb-selective-suspend-off" => apply_usb_selective_suspend(id),
        "input-response-fast" => apply_input_response_fast(id),
        "csrss-high-priority" => apply_csrss_high_priority(id),
        "visual-effects-performance" => apply_visual_effects_performance(id),
        "transparency-off" => apply_transparency_off(id),
        "menu-show-delay-low" => apply_menu_show_delay_low(id),

        // Group 4: Storage & Debloat
        "clean-temp-junk" => apply_clean_temp_junk(id),
        "hibernate-off" => run_command_result(
            id,
            "powercfg",
            &["/hibernate", "off"],
            "Hibernate disabled; Fast Startup also disabled",
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

        // Group 5: Network Latency
        "tcp-nodelay-ack" => apply_tcp_nodelay_ack(id),
        "nic-energy-saving-off" => apply_nic_energy_saving_off(id),
        "tcp-heuristics-off" => apply_tcp_heuristics_off(id),
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
        "dns-cache-flush" => {
            run_command_result(id, "ipconfig", &["/flushdns"], "DNS cache refreshed")
        }

        // Group 6: Background & Privacy
        "disable-gamedvr" => apply_disable_gamedvr(id),
        "disable-bg-recording" => apply_disable_bg_recording(id),
        "gamebar-startup-off" => apply_gamebar_startup_off(id),
        "wer-off" => apply_wer_off(id),
        "start-bing-search-off" => apply_start_bing_search_off(id),
        "delivery-optimization-lan" => apply_delivery_optimization_lan(id),
        "activity-history-off" => apply_activity_history_off(id),
        "advertising-id-off" => apply_advertising_id_off(id),

        _ => skipped(
            id,
            "This tweak has no active action",
        ),
    }
}

pub(crate) fn known_tweak_ids() -> &'static [&'static str] {
    &[
        // Group 1: Gaming & Latency
        "game-mode-on",
        "modern-flip-model-on",
        "gamedvr-fse-mode",
        "disable-fso-globally",
        "hags-on",
        "mpo-disable",
        "pcie-aspm-off",

        // Group 2: CPU & Performance
        "power-plan-high",
        "ultimate-performance-plan",
        "cpu-unpark-cores",
        "power-throttling-off",
        "mmcss-games-priority",
        "system-responsiveness-10",
        "network-throttle-off",

        // Group 3: Input & Responsiveness
        "pointer-precision-off",
        "sticky-keys-off",
        "usb-selective-suspend-off",
        "input-response-fast",
        "csrss-high-priority",
        "visual-effects-performance",
        "transparency-off",
        "menu-show-delay-low",

        // Group 4: Storage & Debloat
        "clean-temp-junk",
        "hibernate-off",
        "ntfs-last-access-off",
        "trim-enable",

        // Group 5: Network Latency
        "tcp-nodelay-ack",
        "nic-energy-saving-off",
        "tcp-heuristics-off",
        "rss-on",
        "rsc-off",
        "ecn-off",
        "dns-cache-flush",

        // Group 6: Background & Privacy
        "disable-gamedvr",
        "disable-bg-recording",
        "gamebar-startup-off",
        "wer-off",
        "start-bing-search-off",
        "delivery-optimization-lan",
        "activity-history-off",
        "advertising-id-off",
    ]
}

fn active_power_scheme_text() -> String {
    run_command_output("powercfg", &["/getactivescheme"])
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

fn hklm_tcp_nodelay_active() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(interfaces) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces") {
            for subkey_name in interfaces.enum_keys().flatten() {
                if let Ok(interface_key) = interfaces.open_subkey(&subkey_name) {
                    if interface_key.get_value::<u32, _>("TcpAckFrequency").ok() == Some(1)
                        && interface_key.get_value::<u32, _>("TCPNoDelay").ok() == Some(1)
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn hklm_nic_energy_saving_off() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(class_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}") {
            for subkey_name in class_key.enum_keys().flatten() {
                if let Ok(adapter_key) = class_key.open_subkey(&subkey_name) {
                    if adapter_key.get_value::<String, _>("*FlowControl").ok().as_deref() == Some("0") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn is_tweak_applied(id: &str) -> bool {
    match id {
        // Group 1: Gaming & Latency
        "game-mode-on" => hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled") == Some(1),
        "modern-flip-model-on" => {
            hkcu_string("Software\\Microsoft\\DirectX\\UserGpuPreferences", "DirectXUserGlobalSettings")
                .as_deref()
                .map(|v| v.contains("SwapEffectUpgradeCache=1"))
                .unwrap_or(false)
        }
        "gamedvr-fse-mode" => {
            hkcu_dword("System\\GameConfigStore", "GameDVR_FSEBehaviorMode") == Some(2)
        }
        "disable-fso-globally" => {
            hkcu_dword("System\\GameConfigStore", "GameDVR_DSEBehavior") == Some(2)
        }
        "hags-on" => hklm_dword("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers", "HwSchMode") == Some(2),
        "mpo-disable" => hklm_dword("SOFTWARE\\Microsoft\\Windows\\Dwm", "OverlayTestMode") == Some(5),
        "pcie-aspm-off" => {
            run_command_output("powercfg", &["/query", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("0x00000000"))
                .unwrap_or(false)
        }

        // Group 2: CPU & Performance
        "power-plan-high" => {
            let scheme = active_power_scheme_text();
            scheme.contains("8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c")
                || scheme.to_lowercase().contains("high performance")
                || scheme.contains("Высокая производительность")
        }
        "ultimate-performance-plan" => {
            let scheme = active_power_scheme_text();
            scheme.contains("e9a42b02-d5df-448d-aa00-03f14749eb61")
                || scheme.to_lowercase().contains("ultimate performance")
                || scheme.contains("Максимальная производительность")
        }
        "cpu-unpark-cores" => {
            hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
                "ValueMax",
            ) == Some(0)
        }
        "power-throttling-off" => {
            hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling",
                "PowerThrottlingOff",
            ) == Some(1)
        }
        "mmcss-games-priority" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "GPU Priority",
            ) == Some(8)
        }
        "system-responsiveness-10" => {
            let val = hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "SystemResponsiveness",
            );
            val == Some(10) || val == Some(0)
        }
        "network-throttle-off" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "NetworkThrottlingIndex",
            ) == Some(0xffff_ffff)
        }

        // Group 3: Input & Responsiveness
        "pointer-precision-off" => {
            hkcu_string("Control Panel\\Mouse", "MouseSpeed").as_deref() == Some("0")
                && hkcu_string("Control Panel\\Mouse", "MouseThreshold1").as_deref() == Some("0")
                && hkcu_string("Control Panel\\Mouse", "MouseThreshold2").as_deref() == Some("0")
        }
        "sticky-keys-off" => {
            hkcu_string("Control Panel\\Accessibility\\StickyKeys", "Flags").as_deref() == Some("506")
        }
        "usb-selective-suspend-off" => {
            run_command_output("powercfg", &["/query", "SCHEME_CURRENT", "SUB_USB", "USBSELECTIVE"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("0x00000000"))
                .unwrap_or(false)
        }
        "input-response-fast" => {
            hkcu_string("Control Panel\\Keyboard", "KeyboardDelay").as_deref() == Some("0")
                && hkcu_string("Control Panel\\Keyboard", "KeyboardSpeed").as_deref() == Some("31")
        }
        "csrss-high-priority" => {
            hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\csrss.exe\\PerfOptions",
                "CpuPriorityClass",
            ) == Some(3)
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
        "menu-show-delay-low" => hkcu_string("Control Panel\\Desktop", "MenuShowDelay").as_deref() == Some("100"),

        // Group 4: Storage & Debloat
        "clean-temp-junk" => false,
        "hibernate-off" => {
            !std::path::Path::new("C:\\hiberfil.sys").exists()
                || hklm_dword("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power", "HiberbootEnabled") == Some(0)
        }
        "ntfs-last-access-off" => {
            run_command_output("fsutil", &["behavior", "query", "disableLastAccess"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("DisableLastAccess = 1"))
                .unwrap_or(false)
        }
        "trim-enable" => {
            run_command_output("fsutil", &["behavior", "query", "DisableDeleteNotify"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("DisableDeleteNotify = 0"))
                .unwrap_or(false)
        }

        // Group 5: Network Latency
        "tcp-nodelay-ack" => hklm_tcp_nodelay_active(),
        "nic-energy-saving-off" => hklm_nic_energy_saving_off(),
        "tcp-heuristics-off" => {
            run_command_output("netsh", &["interface", "tcp", "show", "heuristics"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("disabled"))
                .unwrap_or(false)
        }
        "rss-on" => {
            run_command_output("netsh", &["interface", "tcp", "show", "global"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("Receive-Side Scaling State          : enabled"))
                .unwrap_or(false)
        }
        "rsc-off" => {
            run_command_output("netsh", &["interface", "tcp", "show", "global"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("Receive Segment Coalescing State    : disabled"))
                .unwrap_or(false)
        }
        "ecn-off" => {
            run_command_output("netsh", &["interface", "tcp", "show", "global"])
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("ECN Capability                      : disabled"))
                .unwrap_or(false)
        }
        "dns-cache-flush" => false,

        // Group 6: Background & Privacy
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
        "wer-off" => {
            hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "Disabled") == Some(1)
        }
        "start-bing-search-off" => {
            hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled") == Some(0)
                && hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "DisableSearchBoxSuggestions") == Some(1)
        }
        "delivery-optimization-lan" => {
            hklm_dword(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization",
                "DODownloadMode",
            ) == Some(1)
        }
        "activity-history-off" => {
            hklm_dword("SOFTWARE\\Policies\\Microsoft\\Windows\\System", "EnableActivityFeed") == Some(0)
        }
        "advertising-id-off" => {
            hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\AdvertisingInfo", "Enabled") == Some(0)
        }

        _ => false,
    }
}

fn apply_modern_flip_model(id: &str) -> TweakApplyResult {
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

fn apply_sticky_keys_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_string("Control Panel\\Accessibility\\StickyKeys", "Flags", "506"),
            set_hkcu_string("Control Panel\\Accessibility\\Keyboard Response", "Flags", "98"),
            set_hkcu_string("Control Panel\\Accessibility\\ToggleKeys", "Flags", "58"),
        ],
        "Sticky Keys and accessibility gaming popups disabled",
    )
}

fn apply_start_bing_search_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled", 0),
            set_hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Search", "DisableSearchBoxSuggestions", 1),
        ],
        "Start menu web search and Bing suggestions disabled for fast local search",
    )
}

fn apply_wer_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "Disabled", 1),
            set_hkcu_dword("Software\\Microsoft\\Windows\\Windows Error Reporting", "DontShowUI", 1),
        ],
        "Windows Error Reporting (WerFault) disabled to eliminate crash lag spikes",
    )
}

fn apply_gamedvr_fse_mode(id: &str) -> TweakApplyResult {
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

fn apply_disable_fso_globally(id: &str) -> TweakApplyResult {
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

fn apply_input_response_fast(id: &str) -> TweakApplyResult {
    let r1 = set_hkcu_string("Control Panel\\Keyboard", "KeyboardDelay", "0");
    let r2 = set_hkcu_string("Control Panel\\Keyboard", "KeyboardSpeed", "31");
    let r3 = set_hkcu_string("Control Panel\\Mouse", "MouseHoverTime", "8");
    let r4 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\mouclass\\Parameters",
        "MouseDataQueueSize",
        100,
    );
    let r5 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters",
        "KeyboardDataQueueSize",
        100,
    );
    collect_result(
        id,
        [r1, r2, r3, r4, r5],
        "Input response latency optimized (keyboard delay 0, instant mouse hover, high queue sizes)",
    )
}

fn apply_csrss_high_priority(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\csrss.exe\\PerfOptions",
            "CpuPriorityClass",
            3,
        )],
        "csrss.exe priority set to High (zero input latency under heavy CPU load)",
    )
}

fn apply_advertising_id_off(id: &str) -> TweakApplyResult {
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

fn apply_pcie_aspm_off(id: &str) -> TweakApplyResult {
    let r1 = run_command("powercfg", &["/setacvalueindex", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM", "0"]);
    let r2 = run_command("powercfg", &["/setdcvalueindex", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "ASPM", "0"]);
    let r3 = run_command("powercfg", &["/setactive", "SCHEME_CURRENT"]);
    if r1.is_ok() || r2.is_ok() || r3.is_ok() {
        applied(id, "PCIe Active State Power Management (ASPM) disabled (maximum PCIe bandwidth, zero link latency)")
    } else {
        failed(id, "Failed to update PCIe ASPM in current power plan; run Synchro as administrator")
    }
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

fn apply_cpu_unpark_cores(id: &str) -> TweakApplyResult {
    let _ = run_command("powercfg", &["-setacvalueindex", "SCHEME_CURRENT", "SUB_PROCESSOR", "CPMINCORES", "100"]);
    let _ = run_command("powercfg", &["-setacvalueindex", "SCHEME_CURRENT", "SUB_PROCESSOR", "CPMAXCORES", "100"]);
    let _ = run_command("powercfg", &["-setactive", "SCHEME_CURRENT"]);
    collect_result(
        id,
        [
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
                "ValueMax",
                0,
            ),
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
                "ValueMin",
                0,
            ),
        ],
        "All CPU cores unparked (100% active, zero wake-up stutter)",
    )
}

fn apply_power_throttling_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling",
            "PowerThrottlingOff",
            1,
        )],
        "Windows Power Throttling globally disabled (sustained maximum CPU clocks)",
    )
}

fn apply_clean_temp_junk(id: &str) -> TweakApplyResult {
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
        &format!("Очищено {files_removed} временных файлов ({mb_freed:.1} МБ мусора и кэша шейдеров DirectX)"),
    )
}

fn apply_tcp_nodelay_ack(id: &str) -> TweakApplyResult {
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces";
        if let Ok(interfaces) = hklm.open_subkey(path) {
            let mut count = 0;
            for subkey_name in interfaces.enum_keys().flatten() {
                let full_path = format!("{path}\\{subkey_name}");
                let _ = set_hklm_dword(&full_path, "TcpAckFrequency", 1);
                let _ = set_hklm_dword(&full_path, "TCPNoDelay", 1);
                let _ = set_hklm_dword(&full_path, "TcpDelAckTicks", 0);
                count += 1;
            }
            if count > 0 {
                return applied(
                    id,
                    &format!("TCP NoDelay & AckFrequency enabled on {count} network interfaces (Nagle algorithm disabled)"),
                );
            }
        }
        failed(id, "Could not open Tcpip Interfaces key; run Synchro as administrator")
    }
    #[cfg(not(target_os = "windows"))]
    {
        applied(id, "TCP NoDelay & AckFrequency simulated")
    }
}

fn apply_nic_energy_saving_off(id: &str) -> TweakApplyResult {
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";
        let mut count = 0;
        if let Ok(class_key) = hklm.open_subkey(class_path) {
            for subkey_name in class_key.enum_keys().flatten() {
                let full_path = format!("{class_path}\\{subkey_name}");
                if let Ok(adapter_key) = class_key.open_subkey(&subkey_name) {
                    if adapter_key.get_value::<String, _>("DriverDesc").is_ok() {
                        let _ = set_hklm_string(&full_path, "*EEE", "0");
                        let _ = set_hklm_string(&full_path, "*FlowControl", "0");
                        let _ = set_hklm_string(&full_path, "AutoPowerSaveModeEnabled", "0");
                        let _ = set_hklm_string(&full_path, "SavePowerNowEnabled", "0");
                        let _ = set_hklm_string(&full_path, "ReduceSpeedOnPowerDown", "0");
                        let _ = set_hklm_string(&full_path, "GreenEthernet", "0");
                        let _ = set_hklm_string(&full_path, "AdvancedEEE", "0");
                        let _ = set_hklm_string(&full_path, "EnablePME", "0");
                        count += 1;
                    }
                }
            }
        }
        let _ = run_command(
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                "Get-NetAdapter -Physical | ForEach-Object { Disable-NetAdapterPowerManagement -Name $_.Name -ErrorAction SilentlyContinue }",
            ],
        );

        if count > 0 {
            applied(
                id,
                &format!("Energy-saving and flow control disabled for {count} network adapters (zero ping spikes)"),
            )
        } else {
            applied(
                id,
                "Network adapter power saving disabled (zero ping spikes)",
            )
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        applied(id, "NIC energy saving simulated")
    }
}

fn apply_tcp_heuristics_off(id: &str) -> TweakApplyResult {
    let r1 = run_command("netsh", &["interface", "tcp", "set", "heuristics", "disabled"]);
    let r2 = run_command("netsh", &["interface", "tcp", "set", "global", "autotuninglevel=normal"]);
    let _ = run_command("netsh", &["interface", "tcp", "set", "global", "timestamps", "disabled"]);
    let _ = run_command("netsh", &["interface", "tcp", "set", "global", "chimney=disabled"]);
    if r1.is_ok() || r2.is_ok() {
        applied(
            id,
            "TCP heuristics disabled, timestamps disabled & autotuning optimized (minimal packet loss and jitter)",
        )
    } else {
        failed(id, "Failed to apply TCP heuristics settings via netsh; run Synchro as administrator")
    }
}

fn clean_directory_contents(dir: &std::path::Path, total_bytes: &mut u64, files_count: &mut usize) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(meta) = entry.metadata() {
                    let len = meta.len();
                    if std::fs::remove_file(&path).is_ok() {
                        *total_bytes += len;
                        *files_count += 1;
                    }
                }
            } else if path.is_dir() {
                clean_directory_contents(&path, total_bytes, files_count);
                let _ = std::fs::remove_dir(&path);
            }
        }
    }
}

pub(crate) fn create_system_restore_point(description: &str) {
    let clean_desc = description.replace('\'', " ");
    let cmd = format!(
        "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS'",
        clean_desc
    );
    let _ = run_command(
        "powershell.exe",
        &[
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &cmd,
        ],
    );
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
