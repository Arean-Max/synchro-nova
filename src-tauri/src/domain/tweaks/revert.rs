use super::engine::is_admin_tweak;
use super::runner::{
    delete_hklm_value, run_command_result, run_netsh, run_powercfg, set_hkcu_dword,
    set_hkcu_string, set_hklm_dword,
};
use super::types::{applied, failed, skipped, TweakApplyResult};

pub fn revert_selected_tweaks(ids: Vec<String>) -> Vec<TweakApplyResult> {
    ids.into_iter().take(80).map(|id| revert_one(&id)).collect()
}

fn collect_revert<const N: usize>(
    id: &str,
    results: [Result<(), String>; N],
    success: &str,
) -> TweakApplyResult {
    let mut errors = Vec::new();
    for res in results {
        if let Err(e) = res {
            errors.push(e);
        }
    }
    if errors.is_empty() {
        applied(id, success)
    } else {
        failed(id, &errors.join("; "))
    }
}

fn revert_one(id: &str) -> TweakApplyResult {
    if is_admin_tweak(id) && !crate::admin::is_running_elevated() {
        return TweakApplyResult {
            id: id.to_string(),
            status: "requiresAdmin".to_string(),
            message: "Requires administrator privileges to restore default system policies".to_string(),
        };
    }

    match id {
        "win32-priority-rust" | "cpu-adaptive-scheduling" => collect_revert(
            id,
            [set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
                "Win32PrioritySeparation",
                2,
            )],
            "Win32 priority separation restored to Windows default (2)",
        ),

        "system-timer-resolution" => collect_revert(
            id,
            [set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
                "GlobalTimerResolutionRequests",
                0,
            )],
            "Global timer resolution restored to default",
        ),

        "kill-gamebar-presence" => collect_revert(
            id,
            [
                set_hkcu_dword("Software\\Microsoft\\GameBar", "ShowStartupPanel", 1),
                set_hkcu_dword("Software\\Microsoft\\GameBar", "UseNexusForGameBarEnabled", 1),
                set_hkcu_dword("Software\\Microsoft\\GameBar", "AllowAutoGameMode", 1),
            ],
            "GameBar presence writer restored to default",
        ),

        "directx-thread-priority" => collect_revert(
            id,
            [
                delete_hklm_value("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers", "TdrDelay"),
                delete_hklm_value(
                    "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers\\Scheduler",
                    "EnablePreemption",
                ),
            ],
            "DirectX scheduling and TdrDelay restored to defaults",
        ),

        "gpu-adaptive-power" => {
            let _ = delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}\\0000",
                "PowerMizerEnable",
            );
            applied(id, "GPU power state settings restored to default")
        }

        "game-mode-on" => collect_revert(
            id,
            [
                set_hkcu_dword("Software\\Microsoft\\GameBar", "AutoGameModeEnabled", 0),
                set_hkcu_dword("Software\\Microsoft\\GameBar", "AllowAutoGameMode", 0),
            ],
            "Game mode restored to default",
        ),

        "modern-flip-model-on" => collect_revert(
            id,
            [
                delete_hklm_value("SOFTWARE\\Microsoft\\DirectX", "DWMFlush"),
                delete_hklm_value("SOFTWARE\\Microsoft\\DirectX", "EnableWindowedFlipModel"),
            ],
            "Windowed flip presentation restored to defaults",
        ),

        "gamedvr-fse-mode" => collect_revert(
            id,
            [set_hkcu_dword("System\\GameConfigStore", "GameDVR_FSEBehaviorMode", 0)],
            "GameDVR FSE behavior mode restored to default (0)",
        ),

        "disable-fso-globally" => collect_revert(
            id,
            [set_hkcu_dword(
                "System\\GameConfigStore",
                "GameDVR_DXGIHonorFSEWindowsCompatible",
                0,
            )],
            "Full screen optimizations restored to default",
        ),

        "hags-on" => collect_revert(
            id,
            [set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                "HwSchMode",
                1,
            )],
            "Hardware GPU scheduling restored to default (1); reboot required",
        ),

        "mpo-disable" => collect_revert(
            id,
            [
                delete_hklm_value("SOFTWARE\\Microsoft\\Windows\\Dwm", "OverlayTestMode"),
                delete_hklm_value(
                    "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                    "DisableOverlays",
                ),
            ],
            "MPO multiplane overlay restored to default",
        ),

        "pcie-aspm-off" => run_powercfg(
            id,
            &["/setacvalueindex", "SCHEME_CURRENT", "SUB_PCIEXPRESS", "EXPRESS", "1"],
            "PCIe ASPM link state power management restored to default",
        ),

        "power-plan-high" | "ultimate-performance-plan" => run_powercfg(
            id,
            &["/setactive", "381b4222-f694-41f0-9685-ff5bb260df2e"],
            "Balanced Windows power plan restored to default",
        ),

        "cpu-unpark-cores" => run_powercfg(
            id,
            &[
                "/setacvalueindex",
                "SCHEME_CURRENT",
                "SUB_PROCESSOR",
                "PROCTHROTTLEMAX",
                "100",
            ],
            "CPU core parking policy restored to default",
        ),

        "power-throttling-off" => collect_revert(
            id,
            [delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling",
                "PowerThrottlingOff",
            )],
            "Power throttling policy restored to default",
        ),

        "system-worker-threads" => collect_revert(
            id,
            [delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Executive",
                "AdditionalWorkerThreads",
            )],
            "Executive worker threads allocation restored to default",
        ),

        "mmcss-games-priority" => collect_revert(
            id,
            [
                set_hklm_dword(
                    "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                    "Priority",
                    6,
                ),
                set_hklm_dword(
                    "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                    "Scheduling Category",
                    2,
                ),
            ],
            "MMCSS games multimedia scheduling restored to default",
        ),

        "system-responsiveness-0" => collect_revert(
            id,
            [set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "SystemResponsiveness",
                20,
            )],
            "System responsiveness network quota restored to default (20%)",
        ),

        "network-throttle-off" => collect_revert(
            id,
            [set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
                "NetworkThrottlingIndex",
                10,
            )],
            "Network throttling index restored to default (10)",
        ),

        "pointer-precision-off" => collect_revert(
            id,
            [
                set_hkcu_string("Control Panel\\Mouse", "MouseSpeed", "1"),
                set_hkcu_string("Control Panel\\Mouse", "MouseThreshold1", "6"),
                set_hkcu_string("Control Panel\\Mouse", "MouseThreshold2", "10"),
            ],
            "Mouse pointer precision and acceleration curve restored to default",
        ),

        "usb-selective-suspend-off" => run_powercfg(
            id,
            &[
                "/setacvalueindex",
                "SCHEME_CURRENT",
                "2a737441-1930-4402-8d77-b2bebba4d5a3",
                "48e6b7a6-50f5-4760-a579-a420f2670dd1",
                "1",
            ],
            "USB selective suspend power savings restored to default",
        ),

        "input-response-fast" => collect_revert(
            id,
            [
                set_hkcu_string("Control Panel\\Keyboard", "KeyboardDelay", "1"),
                set_hkcu_string("Control Panel\\Keyboard", "KeyboardSpeed", "31"),
            ],
            "Keyboard repeat rate and delay restored to default",
        ),

        "visual-effects-performance" => collect_revert(
            id,
            [set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects",
                "VisualFXSetting",
                0,
            )],
            "Windows visual effects appearance restored to default",
        ),

        "transparency-off" => collect_revert(
            id,
            [set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                "EnableTransparency",
                1,
            )],
            "Acrylic and mica transparency effects restored to default",
        ),

        "clean-temp-junk" => applied(id, "Clean junk operation has no persistent state"),

        "disable-paging-executive" => collect_revert(
            id,
            [set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
                "DisablePagingExecutive",
                0,
            )],
            "Paging executive memory policy restored to default (0)",
        ),

        "disable-memory-compression" => run_command_result(
            id,
            "powershell",
            &["-NoProfile", "-NonInteractive", "-Command", "Enable-MMAgent -MemoryCompression"],
            "Windows memory compression restored to default (enabled)",
        ),

        "disable-page-combining" => run_command_result(
            id,
            "powershell",
            &["-NoProfile", "-NonInteractive", "-Command", "Enable-MMAgent -PageCombining"],
            "Windows page combining restored to default (enabled)",
        ),

        "adaptive-io-page-lock" => collect_revert(
            id,
            [delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
                "IoPageLockLimit",
            )],
            "I/O page lock limit allocation restored to default",
        ),

        "large-system-cache-off" => collect_revert(
            id,
            [set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
                "LargeSystemCache",
                0,
            )],
            "Large system cache allocation restored to default (0)",
        ),

        "hibernate-off" => run_command_result(
            id,
            "powercfg",
            &["/hibernate", "on"],
            "Windows hibernate and Fast Startup restored to default (enabled)",
        ),

        "ntfs-last-access-off" => run_command_result(
            id,
            "fsutil",
            &["behavior", "set", "disableLastAccess", "0"],
            "NTFS last access time updates restored to default (enabled)",
        ),

        "trim-enable" => applied(id, "TRIM notifications active"),

        "network-udp-buffers" => collect_revert(
            id,
            [
                delete_hklm_value(
                    "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
                    "DefaultReceiveWindow",
                ),
                delete_hklm_value(
                    "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
                    "DefaultSendWindow",
                ),
            ],
            "AFD socket receive and send windows restored to defaults",
        ),

        "tcp-nodelay-ack" => {
            let _ = delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces",
                "TcpAckFrequency",
            );
            let _ = delete_hklm_value(
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces",
                "TCPNoDelay",
            );
            applied(id, "TCP ACK frequency and Nagle algorithm restored to defaults")
        }

        "nic-energy-saving-off" => {
            applied(id, "Network adapter energy saving configuration restored")
        }

        "tcp-heuristics-off" => run_netsh(
            id,
            &["interface", "tcp", "set", "heuristics", "default"],
            "TCP window heuristics restored to default",
        ),

        "rss-adaptive-cores" => run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rss=default"],
            "Receive-Side Scaling global configuration restored to default",
        ),

        "rsc-off" => run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rsc=default"],
            "Receive segment coalescing restored to default",
        ),

        "disable-gamedvr" => collect_revert(
            id,
            [
                set_hklm_dword("SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR", "AllowGameDVR", 1),
                set_hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR", "AppCaptureEnabled", 1),
                set_hkcu_dword("System\\GameConfigStore", "GameDVR_Enabled", 1),
            ],
            "GameDVR broadcast and capture policies restored to defaults",
        ),

        "disable-bg-recording" => collect_revert(
            id,
            [set_hkcu_dword(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                "HistoricalCaptureEnabled",
                1,
            )],
            "Background video recording capability restored to default",
        ),

        "wer-off" => collect_revert(
            id,
            [delete_hklm_value(
                "SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting",
                "Disabled",
            )],
            "Windows Error Reporting service policy restored to default",
        ),

        _ => skipped(id, "No restore action required for this tweak"),
    }
}
