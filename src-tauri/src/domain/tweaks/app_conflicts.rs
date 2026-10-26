use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedAppConflictInfo {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub details: String,
}

pub fn detect_installed_tweak_apps() -> Vec<DetectedAppConflictInfo> {
    let mut apps = Vec::new();

    let sys_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let sys32 = std::path::PathBuf::from(&sys_root).join("System32");

    let mut pf_dirs = Vec::new();
    if let Ok(pf) = std::env::var("ProgramFiles") {
        pf_dirs.push(std::path::PathBuf::from(pf));
    }
    if let Ok(pfx86) = std::env::var("ProgramFiles(x86)") {
        pf_dirs.push(std::path::PathBuf::from(pfx86));
    }

    let has_gamebar = sys32.join("bcastdvr.exe").exists()
        || sys32.join("GameBarPresenceWriter.exe").exists();
    apps.push(DetectedAppConflictInfo {
        id: "xbox_game_bar".to_string(),
        name: "Xbox Game Bar".to_string(),
        installed: has_gamebar,
        details: if has_gamebar {
            "Xbox GameDVR / Game Bar capture active".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_discord = check_folder_in_local_or_roaming("Discord");
    apps.push(DetectedAppConflictInfo {
        id: "discord".to_string(),
        name: "Discord".to_string(),
        installed: has_discord,
        details: if has_discord {
            "Discord client detected".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_geforce = pf_dirs.iter().any(|dir| {
        dir.join("NVIDIA Corporation\\NVIDIA GeForce Experience").exists()
            || dir.join("NVIDIA Corporation\\NVIDIA App").exists()
    });
    apps.push(DetectedAppConflictInfo {
        id: "geforce_experience".to_string(),
        name: "NVIDIA App / ShadowPlay".to_string(),
        installed: has_geforce,
        details: if has_geforce {
            "NVIDIA overlay / shadowplay software detected".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_obs = pf_dirs.iter().any(|dir| dir.join("obs-studio").exists())
        || check_folder_in_roaming("obs-studio");
    apps.push(DetectedAppConflictInfo {
        id: "obs_studio".to_string(),
        name: "OBS Studio".to_string(),
        installed: has_obs,
        details: if has_obs {
            "OBS Studio capture software detected".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_rtss = pf_dirs.iter().any(|dir| dir.join("RivaTuner Statistics Server").exists());
    apps.push(DetectedAppConflictInfo {
        id: "rtss".to_string(),
        name: "RivaTuner (RTSS)".to_string(),
        installed: has_rtss,
        details: if has_rtss {
            "RTSS overlay software detected".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_steam = pf_dirs.iter().any(|dir| dir.join("Steam").exists())
        || crate::domain::games::steam_install_root().is_some();
    apps.push(DetectedAppConflictInfo {
        id: "steam".to_string(),
        name: "Steam".to_string(),
        installed: has_steam,
        details: if has_steam {
            "Steam client detected".to_string()
        } else {
            "Not detected".to_string()
        },
    });

    let has_vpn = check_vpn_present();
    apps.push(DetectedAppConflictInfo {
        id: "vpn".to_string(),
        name: "VPN Adapters".to_string(),
        installed: has_vpn,
        details: if has_vpn {
            "Virtual network tunnel interfaces detected".to_string()
        } else {
            "No virtual adapters detected".to_string()
        },
    });

    let has_bluetooth = check_bluetooth_present();
    apps.push(DetectedAppConflictInfo {
        id: "bluetooth".to_string(),
        name: "Bluetooth".to_string(),
        installed: has_bluetooth,
        details: if has_bluetooth {
            "Bluetooth radio adapter detected".to_string()
        } else {
            "No adapter detected".to_string()
        },
    });

    let has_fast_startup = check_fast_startup_enabled();
    apps.push(DetectedAppConflictInfo {
        id: "fast_startup".to_string(),
        name: "Fast Startup".to_string(),
        installed: has_fast_startup,
        details: if has_fast_startup {
            "Windows kernel hibernation enabled".to_string()
        } else {
            "Disabled".to_string()
        },
    });

    apps
}

fn check_folder_in_local_or_roaming(folder: &str) -> bool {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        if std::path::Path::new(&local).join(folder).exists() {
            return true;
        }
    }
    if let Ok(roaming) = std::env::var("APPDATA") {
        if std::path::Path::new(&roaming).join(folder).exists() {
            return true;
        }
    }
    false
}

fn check_folder_in_roaming(folder: &str) -> bool {
    if let Ok(roaming) = std::env::var("APPDATA") {
        if std::path::Path::new(&roaming).join(folder).exists() {
            return true;
        }
    }
    false
}

fn check_fast_startup_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SYSTEM\CurrentControlSet\Control\Session Manager\Power") {
            if let Ok(val) = key.get_value::<u32, _>("HiberbootEnabled") {
                return val == 1;
            }
        }
    }
    let sys_drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
    std::path::PathBuf::from(format!("{sys_drive}\\hiberfil.sys")).exists()
}

fn check_bluetooth_present() -> bool {
    let drivers = crate::driver_info::collect_drivers();
    drivers.iter().any(|d| {
        let text = format!("{} {} {}", d.name, d.class_name, d.provider).to_lowercase();
        text.contains("bluetooth") || text.contains("bth")
    })
}

fn check_vpn_present() -> bool {
    let drivers = crate::driver_info::collect_drivers();
    drivers.iter().any(|d| {
        let text = format!("{} {} {}", d.name, d.class_name, d.provider).to_lowercase();
        text.contains("wireguard") || text.contains("tap-") || text.contains("wintun") || text.contains("openvpn") || text.contains("nordlynx")
    })
}
