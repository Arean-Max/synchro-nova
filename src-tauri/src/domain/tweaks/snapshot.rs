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
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
        "Win32PrioritySeparation",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
        "GlobalTimerResolutionRequests",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "DisablePagingExecutive",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "LargeSystemCache",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "DisablePageCombining",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "IoPageLockLimit",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Executive",
        "AdditionalWorkerThreads",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Executive",
        "AdditionalCriticalWorkerThreads",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "FastSendDatagramThreshold",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "DefaultReceiveWindow",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "DefaultSendWindow",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        "TdrDelay",
    ),
    reg_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler",
        "EnablePreemption",
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
    reg_text(
        "HKCU",
        "Software\\Microsoft\\DirectX\\UserGpuPreferences",
        "DirectXUserGlobalSettings",
    ),
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
    reg_dword("HKLM", "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c", "ValueMax"),
    reg_dword("HKLM", "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c", "ValueMin"),
    reg_dword("HKLM", "SYSTEM\\CurrentControlSet\\Services\\mouclass\\Parameters", "MouseDataQueueSize"),
    reg_dword("HKLM", "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters", "KeyboardDataQueueSize"),
];
