#![allow(dead_code)]

use std::path::Path;
use super::common::LaunchTarget;

pub fn launch_target(target: &LaunchTarget) -> Result<(), String> {
    match target {
        LaunchTarget::Url(url) => open_url(url),
        LaunchTarget::Exe { path, args, cwd } => open_executable(path, args, cwd.as_deref()),
        LaunchTarget::None => Err("Game launch is not available for this entry".to_string()),
    }
}

pub(crate) fn is_safe_game_url(url: &str) -> bool {
    if url.is_empty() || url.len() > 512 {
        return false;
    }
    // Disallow control characters, quotes, and dangerous shell metacharacters
    if url.chars().any(|ch| ch.is_control() || matches!(ch, '"' | '\'' | '`' | '<' | '>' | '^' | '|' | '&')) {
        return false;
    }
    let lower = url.to_lowercase();
    if lower.starts_with("steam://") {
        if let Some(rest) = lower.strip_prefix("steam://rungameid/") {
            return !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit());
        }
        return false;
    }
    if lower.starts_with("com.epicgames.launcher://apps/") {
        if let Some(rest) = lower.strip_prefix("com.epicgames.launcher://apps/") {
            return !rest.is_empty()
                && rest
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '%' | ':' | '-' | '_' | '=' | '&' | '?' | '.'));
        }
        return false;
    }
    if lower.starts_with("riotclient://") {
        if let Some(rest) = lower.strip_prefix("riotclient://") {
            return !rest.is_empty()
                && rest
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '=' | '&' | '/' | '?'));
        }
        return false;
    }
    false
}

#[cfg(target_os = "windows")]
fn open_url(url: &str) -> Result<(), String> {
    if !is_safe_game_url(url) {
        return Err("Blocked launch of invalid or untrusted game URL".to_string());
    }
    shell_execute("open", url, None, None, "Failed to launch game")
}

#[cfg(not(target_os = "windows"))]
fn open_url(_url: &str) -> Result<(), String> {
    Err("Game launch is only available on Windows".to_string())
}

#[cfg(target_os = "windows")]
fn open_executable(path: &Path, args: &[String], cwd: Option<&Path>) -> Result<(), String> {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\") || path_str.starts_with("//") {
        return Err("Refusing to execute binary from remote network location".to_string());
    }
    if !path.is_file() {
        return Err("Game executable is missing".to_string());
    }
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_lowercase();
    if ext != "exe" {
        return Err("Target file is not an executable".to_string());
    }
    let args = if args.is_empty() {
        None
    } else {
        Some(join_command_args(args))
    };
    shell_execute(
        "open",
        &path_str,
        args.as_deref(),
        cwd.map(|path| path.to_string_lossy().into_owned()).as_deref(),
        "Failed to launch game",
    )
}

#[cfg(not(target_os = "windows"))]
fn open_executable(_path: &Path, _args: &[String], _cwd: Option<&Path>) -> Result<(), String> {
    Err("Game launch is only available on Windows".to_string())
}

#[cfg(target_os = "windows")]
fn shell_execute(
    operation: &str,
    file: &str,
    parameters: Option<&str>,
    directory: Option<&str>,
    error_message: &str,
) -> Result<(), String> {
    let operation = crate::platform::ffi::wide_null(operation);
    let file = crate::platform::ffi::wide_null(file);
    let parameters = parameters.map(crate::platform::ffi::wide_null);
    let directory = directory.map(crate::platform::ffi::wide_null);
    let result = unsafe {
        crate::platform::ffi::winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            parameters
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null()),
            directory
                .as_ref()
                .map(|value| value.as_ptr())
                .unwrap_or(std::ptr::null()),
            1,
        )
    };

    if result <= 32 {
        Err(error_message.to_string())
    } else {
        Ok(())
    }
}

pub(crate) fn quote_windows_arg(arg: &str) -> String {
    let clean: String = arg.chars().filter(|ch| !ch.is_control()).collect();
    if clean.is_empty() {
        return "\"\"".to_string();
    }
    if clean.chars().all(|ch| !ch.is_whitespace() && ch != '"') {
        return clean;
    }

    let mut result = String::with_capacity(clean.len() + 8);
    result.push('"');

    let mut backslash_count = 0;
    for ch in clean.chars() {
        if ch == '\\' {
            backslash_count += 1;
        } else if ch == '"' {
            for _ in 0..(backslash_count * 2 + 1) {
                result.push('\\');
            }
            result.push('"');
            backslash_count = 0;
        } else {
            for _ in 0..backslash_count {
                result.push('\\');
            }
            backslash_count = 0;
            result.push(ch);
        }
    }

    for _ in 0..(backslash_count * 2) {
        result.push('\\');
    }
    result.push('"');

    result
}

fn join_command_args(args: &[String]) -> String {
    args.iter()
        .map(|arg| quote_windows_arg(arg))
        .collect::<Vec<_>>()
        .join(" ")
}
