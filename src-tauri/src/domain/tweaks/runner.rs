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

#[cfg(not(target_os = "windows"))]
pub fn hkcu_string(_path: &str, _name: &str) -> Option<String> {
    None
}
