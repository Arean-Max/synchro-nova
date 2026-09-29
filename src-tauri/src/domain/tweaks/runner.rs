use std::process::{Command, Output};
use super::types::{applied, failed, TweakApplyResult};

pub fn run_netsh(id: &str, args: &[&str], message: &str) -> TweakApplyResult {
    run_command_result(id, "netsh", args, message)
}

pub fn run_powercfg(id: &str, args: &[&str], message: &str) -> TweakApplyResult {
    run_command_result(id, "powercfg", args, message)
}

pub fn run_command_result(id: &str, program: &str, args: &[&str], success: &str) -> TweakApplyResult {
    match run_command(program, args) {
        Ok(_) => applied(id, success),
        Err(error) => failed(id, &error),
    }
}

pub fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    run_command_output(program, args).map(|_| ())
}

pub fn resolve_system_program(program: &str) -> std::path::PathBuf {
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

pub fn run_command_output(program: &str, args: &[&str]) -> Result<Output, String> {
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

pub fn output_guid(output: &Output) -> Option<String> {
    first_guid(&output.stdout).or_else(|| first_guid(&output.stderr))
}

pub fn first_guid(bytes: &[u8]) -> Option<String> {
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
pub fn hide_console(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    command.creation_flags(0x0800_0000);
}

#[cfg(not(target_os = "windows"))]
pub fn hide_console(_command: &mut Command) {}

#[cfg(target_os = "windows")]
pub fn set_hkcu_dword(path: &str, name: &str, value: u32) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::set_dword(
        crate::infra::registry::RootKey::Hkcu,
        path,
        name,
        value,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn set_hkcu_dword(_path: &str, _name: &str, _value: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn set_hklm_dword(path: &str, name: &str, value: u32) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::set_dword(
        crate::infra::registry::RootKey::Hklm,
        path,
        name,
        value,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn set_hklm_dword(_path: &str, _name: &str, _value: u32) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn set_hkcu_string(path: &str, name: &str, value: &str) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::set_string(
        crate::infra::registry::RootKey::Hkcu,
        path,
        name,
        value,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn set_hkcu_string(_path: &str, _name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn set_hklm_string(path: &str, name: &str, value: &str) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::set_string(
        crate::infra::registry::RootKey::Hklm,
        path,
        name,
        value,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn set_hklm_string(_path: &str, _name: &str, _value: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn hkcu_dword(path: &str, name: &str) -> Option<u32> {
    crate::infra::registry::SafeRegistry::get_dword(
        crate::infra::registry::RootKey::Hkcu,
        path,
        name,
    )
    .ok()
}

#[cfg(not(target_os = "windows"))]
pub fn hkcu_dword(_path: &str, _name: &str) -> Option<u32> {
    None
}

#[cfg(target_os = "windows")]
pub fn hklm_dword(path: &str, name: &str) -> Option<u32> {
    crate::infra::registry::SafeRegistry::get_dword(
        crate::infra::registry::RootKey::Hklm,
        path,
        name,
    )
    .ok()
}

#[cfg(not(target_os = "windows"))]
pub fn hklm_dword(_path: &str, _name: &str) -> Option<u32> {
    None
}

#[cfg(target_os = "windows")]
pub fn hkcu_string(path: &str, name: &str) -> Option<String> {
    crate::infra::registry::SafeRegistry::get_string(
        crate::infra::registry::RootKey::Hkcu,
        path,
        name,
    )
    .ok()
}

#[cfg(target_os = "windows")]
pub fn delete_hklm_value(path: &str, name: &str) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::delete_value(
        crate::infra::registry::RootKey::Hklm,
        path,
        name,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn delete_hklm_value(_path: &str, _name: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn delete_hkcu_value(path: &str, name: &str) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::delete_value(
        crate::infra::registry::RootKey::Hkcu,
        path,
        name,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn delete_hkcu_value(_path: &str, _name: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn delete_hklm_tree(path: &str) -> Result<(), String> {
    crate::infra::registry::SafeRegistry::delete_tree(
        crate::infra::registry::RootKey::Hklm,
        path,
    )
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn delete_hklm_tree(_path: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn total_ram_gb() -> f64 {
    crate::platform::ffi::read_memory_status_ex()
        .map(|s| s.ull_total_phys as f64 / 1024.0 / 1024.0 / 1024.0)
        .unwrap_or(16.0)
}

#[cfg(not(target_os = "windows"))]
pub fn total_ram_gb() -> f64 {
    16.0
}

pub fn cpu_cores_count() -> usize {
    std::thread::available_parallelism()
        .map(|c| c.get())
        .unwrap_or(4)
}

#[cfg(target_os = "windows")]
pub fn is_intel_hybrid() -> bool {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0") {
        if let Ok(name) = key.get_value::<String, _>("ProcessorNameString") {
            let lower = name.to_lowercase();
            let is_intel = lower.contains("intel");
            let has_hybrid = lower.contains("12th")
                || lower.contains("13th")
                || lower.contains("14th")
                || lower.contains("15th")
                || lower.contains("ultra")
                || (is_intel && cpu_cores_count() >= 12);
            return is_intel && has_hybrid;
        }
    }
    false
}

#[cfg(not(target_os = "windows"))]
pub fn is_intel_hybrid() -> bool {
    false
}

#[cfg(target_os = "windows")]
pub fn detected_gpu_vendor() -> &'static str {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e972-e325-11ce-bfc1-08002be10318}";
    if let Ok(class_key) = hklm.open_subkey(class_path) {
        for subkey_name in class_key.enum_keys().flatten() {
            if let Ok(key) = class_key.open_subkey(&subkey_name) {
                let desc = key.get_value::<String, _>("DriverDesc").unwrap_or_default().to_lowercase();
                let provider = key.get_value::<String, _>("ProviderName").unwrap_or_default().to_lowercase();
                if desc.contains("nvidia") || provider.contains("nvidia") {
                    return "nvidia";
                }
                if desc.contains("amd") || desc.contains("radeon") || provider.contains("advanced micro devices") {
                    return "amd";
                }
            }
        }
    }
    "unknown"
}

#[cfg(not(target_os = "windows"))]
pub fn detected_gpu_vendor() -> &'static str {
    "unknown"
}

