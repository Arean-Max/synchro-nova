use super::types::{TweakRegistrySnapshot, TweakRegistryValue};

#[derive(Clone, Copy)]
pub struct RegistryTarget {
    pub hive: &'static str,
    pub path: &'static str,
    pub name: &'static str,
    pub value_kind: RegistryValueKind,
}

#[derive(Clone, Copy)]
pub enum RegistryValueKind {
    Dword,
    Text,
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

pub static REGISTRY_BACKUP_TARGETS: &[RegistryTarget] = &[
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

pub fn registry_backup_targets() -> &'static [RegistryTarget] {
    REGISTRY_BACKUP_TARGETS
}

pub fn collect_tweak_registry_snapshot() -> Vec<TweakRegistrySnapshot> {
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

pub fn restore_tweak_registry_snapshot(snapshot: &[TweakRegistrySnapshot]) -> Vec<String> {
    snapshot
        .iter()
        .filter_map(|entry| restore_registry_value(entry).err())
        .collect()
}

#[cfg(target_os = "windows")]
pub fn read_registry_value(target: &RegistryTarget) -> Option<TweakRegistryValue> {
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
pub fn read_registry_value(_target: &RegistryTarget) -> Option<TweakRegistryValue> {
    None
}

#[cfg(target_os = "windows")]
pub fn restore_registry_value(entry: &TweakRegistrySnapshot) -> Result<(), String> {
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
pub fn restore_registry_value(_entry: &TweakRegistrySnapshot) -> Result<(), String> {
    Ok(())
}
