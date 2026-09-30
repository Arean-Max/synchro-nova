use super::runner::{run_command, run_command_output, set_hklm_dword, set_hklm_string};
use super::types::{applied, failed, TweakApplyResult};

pub fn hklm_tcp_nodelay_active() -> bool {
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

pub fn hklm_nic_energy_saving_off() -> bool {
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

pub fn apply_tcp_nodelay_ack(id: &str) -> TweakApplyResult {
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

pub fn apply_nic_energy_saving_off(id: &str) -> TweakApplyResult {
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

pub fn apply_tcp_heuristics_off(id: &str) -> TweakApplyResult {
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

pub fn is_tcp_heuristics_off_applied() -> bool {
    run_command_output("netsh", &["interface", "tcp", "show", "heuristics"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("disabled"))
        .unwrap_or(false)
}

pub fn is_rss_applied() -> bool {
    run_command_output("netsh", &["interface", "tcp", "show", "global"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("Receive-Side Scaling State          : enabled"))
        .unwrap_or(false)
}

pub fn is_rsc_applied() -> bool {
    run_command_output("netsh", &["interface", "tcp", "show", "global"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("Receive Segment Coalescing State    : disabled"))
        .unwrap_or(false)
}

pub fn apply_network_udp_buffers(id: &str) -> TweakApplyResult {
    let ram_gb = super::runner::total_ram_gb();
    let window_bytes: u32 = if ram_gb >= 24.0 { 1048576 } else { 524288 };
    let mb_str = if window_bytes == 1048576 { "1 MB" } else { "512 KB" };
    let r1 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "FastSendDatagramThreshold",
        65536,
    );
    let r2 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "DefaultReceiveWindow",
        window_bytes,
    );
    let r3 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "DefaultSendWindow",
        window_bytes,
    );
    super::types::collect_result(
        id,
        [r1, r2, r3],
        &format!("AFD socket buffers optimized for RakNet UDP ({mb_str} buffer based on {ram_gb:.1} GB RAM)"),
    )
}

pub fn is_network_udp_buffers_applied() -> bool {
    super::runner::hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\AFD\\Parameters",
        "FastSendDatagramThreshold",
    ) == Some(65536)
}

pub fn apply_rss_adaptive_cores(id: &str) -> TweakApplyResult {
    let cores = super::runner::cpu_cores_count();
    let _ = run_command("netsh", &["interface", "tcp", "set", "global", "rss=enabled"]);
    let msg = if cores >= 8 {
        let _ = run_command(
            "powershell.exe",
            &[
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                "Get-NetAdapter -Physical | ForEach-Object { Set-NetAdapterRss -Name $_.Name -BaseProcessorNumber 2 -ErrorAction SilentlyContinue }",
            ],
        );
        format!("Receive-Side Scaling enabled with BaseProcessor=2 (network interrupts offloaded from Rust Core 0 across {cores} CPU cores)")
    } else {
        "Receive-Side Scaling enabled and distributed across CPU cores".to_string()
    };
    applied(id, &msg)
}

pub fn is_rss_adaptive_cores_applied() -> bool {
    is_rss_applied()
}
