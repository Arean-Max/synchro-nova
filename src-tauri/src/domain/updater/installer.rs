use super::logging::log;
use std::fs;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[cfg(target_os = "windows")]
pub fn apply_install() -> Result<(), String> {
    let update_file = super::client::get_update_file_path();
    if !update_file.is_file() {
        let msg = "No verified update package ready on disk for installation".to_string();
        log("INSTALL_ERR", &msg);
        return Err(msg);
    }

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to locate current executable path: {}", e))?;

    let staging_dir = super::client::get_update_dir();
    let _ = fs::create_dir_all(&staging_dir);
    let helper_exe = staging_dir.join("updater_helper.exe");

    // Copy payload (or running exe) to helper path in temp dir
    // Using the update_file ensures the helper has the latest helper logic
    if let Err(e) = fs::copy(&update_file, &helper_exe) {
        // Fallback to copying current executable if payload copy fails
        if let Err(e2) = fs::copy(&current_exe, &helper_exe) {
            let msg = format!("Failed to prepare updater helper binary: {} / {}", e, e2);
            log("INSTALL_ERR", &msg);
            return Err(msg);
        }
    }

    let current_pid = std::process::id();
    let mut backup_file = current_exe.clone();
    backup_file.set_extension("exe.old");

    let needs_elevation = match fs::OpenOptions::new().write(true).open(&current_exe) {
        Ok(_) => false,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => true,
        Err(_) => false,
    };

    let target_str = current_exe.to_string_lossy().to_string();
    let payload_str = update_file.to_string_lossy().to_string();
    let backup_str = backup_file.to_string_lossy().to_string();
    let expected_hash = super::client::get_verified_sha256();

    let mut args_slice = vec![
        "--updater-helper".to_string(),
        "--target".to_string(),
        target_str,
        "--payload".to_string(),
        payload_str,
        "--wait-pid".to_string(),
        current_pid.to_string(),
        "--backup".to_string(),
        backup_str,
    ];
    if let Some(ref hash) = expected_hash {
        args_slice.push("--expected-sha256".to_string());
        args_slice.push(hash.clone());
    }

    log(
        "INSTALL",
        &format!(
            "Preparing installation: target='{}', needs_elevation={}, verified_sha256={:?}",
            current_exe.display(),
            needs_elevation,
            expected_hash
        ),
    );

    super::set_status("installing", None);

    if needs_elevation {
        let mut joined_args = format!(
            "--updater-helper --target \"{}\" --payload \"{}\" --wait-pid {} --backup \"{}\"",
            current_exe.display(),
            update_file.display(),
            current_pid,
            backup_file.display()
        );
        if let Some(ref hash) = expected_hash {
            joined_args.push_str(&format!(" --expected-sha256 {}", hash));
        }
        log("INSTALL", "Launching elevated update helper via UAC");
        crate::platform::ffi::runas_executable_with_args(&helper_exe, Some(&joined_args))?;
    } else {
        log("INSTALL", "Launching update helper process directly");
        let mut cmd = Command::new(&helper_exe);
        cmd.args(&args_slice);
        cmd.creation_flags(CREATE_NO_WINDOW);
        cmd.spawn()
            .map_err(|e| format!("Failed to spawn updater helper: {}", e))?;
    }

    // Gracefully exit current application to release locks
    log("INSTALL", "Exiting current process to allow update helper to perform swap");
    std::thread::sleep(std::time::Duration::from_millis(150));
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn apply_install() -> Result<(), String> {
    log("INSTALL", "apply_install simulated on non-windows platform");
    Ok(())
}
