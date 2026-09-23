use crate::{driver_info::collect_drivers, SystemCharacteristics};

#[cfg(target_os = "windows")]
pub(crate) fn collect_system_characteristics() -> SystemCharacteristics {
    let cpu = read_cpu_name();
    let os = read_os_name();
    let gpu_list = read_gpu_names();
    let gpus = if gpu_list.is_empty() {
        vec!["Unknown GPU".to_string()]
    } else {
        gpu_list
    };
    let display = read_display_info();
    let memory = read_memory_info();
    let drivers = collect_drivers();
    let cores = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(0);

    SystemCharacteristics {
        cpu,
        cpu_usage_percent: "0".to_string(),
        gpu: gpus
            .first()
            .cloned()
            .unwrap_or_else(|| "Unknown GPU".to_string()),
        gpu_usage_percent: "0".to_string(),
        gpus: gpus.clone(),
        ram: format!("{:.1} GB", bytes_to_gb(memory.total_bytes)),
        ram_available: format!("{:.1} GB", bytes_to_gb(memory.available_bytes)),
        ram_available_gb: format!("{:.1}", bytes_to_gb(memory.available_bytes)),
        ram_used_gb: format!(
            "{:.1}",
            bytes_to_gb(memory.total_bytes.saturating_sub(memory.available_bytes))
        ),
        ram_used_percent: memory.used_percent().to_string(),
        os,
        architecture: std::env::consts::ARCH.to_string(),
        display: display.resolution,
        refresh_rate: display.refresh_rate,
        color_depth: display.color_depth,
        driver: gpus
            .first()
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string()),
        cpu_cores: cores.to_string(),
        gpu_count: gpus.len().to_string(),
        ram_total_gb: format!("{:.1}", bytes_to_gb(memory.total_bytes)),
        vram_total_gb: "0.0".to_string(),
        vram_used_gb: "0.0".to_string(),
        vram_used_percent: "0".to_string(),
        refresh_rate_hz: display.refresh_rate_hz,
        drivers,
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn collect_system_characteristics() -> SystemCharacteristics {
    let cores = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(0);
    SystemCharacteristics {
        cpu: "Unsupported OS".to_string(),
        cpu_usage_percent: "0".to_string(),
        gpu: "Unsupported OS".to_string(),
        gpu_usage_percent: "0".to_string(),
        gpus: vec!["Unsupported OS".to_string()],
        ram: "Unknown".to_string(),
        ram_available: "Unknown".to_string(),
        ram_available_gb: "0".to_string(),
        ram_used_gb: "0".to_string(),
        ram_used_percent: "0".to_string(),
        os: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        display: "Unknown".to_string(),
        refresh_rate: "Unknown".to_string(),
        color_depth: "Unknown".to_string(),
        driver: "Unknown".to_string(),
        cpu_cores: cores.to_string(),
        gpu_count: "0".to_string(),
        ram_total_gb: "0".to_string(),
        vram_total_gb: "0".to_string(),
        vram_used_gb: "0".to_string(),
        vram_used_percent: "0".to_string(),
        refresh_rate_hz: "0".to_string(),
        drivers: Vec::new(),
    }
}

#[cfg(target_os = "windows")]
fn read_cpu_name() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
        .and_then(|key| key.get_value::<String, _>("ProcessorNameString"))
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|_| "Unknown CPU".to_string())
}

#[cfg(target_os = "windows")]
fn read_os_name() -> String {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = match hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        Ok(key) => key,
        Err(_) => return "Windows".to_string(),
    };
    let product = key
        .get_value::<String, _>("ProductName")
        .unwrap_or_else(|_| "Windows".to_string());
    let version = key
        .get_value::<String, _>("DisplayVersion")
        .or_else(|_| key.get_value::<String, _>("ReleaseId"))
        .unwrap_or_default();
    let build = key
        .get_value::<String, _>("CurrentBuild")
        .unwrap_or_default();

    [product, version, build]
        .into_iter()
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(target_os = "windows")]
#[derive(Default)]
struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
}

#[cfg(target_os = "windows")]
impl MemoryInfo {
    fn used_percent(&self) -> u64 {
        if self.total_bytes == 0 {
            0
        } else {
            let used = self.total_bytes.saturating_sub(self.available_bytes);
            ((used as f64 / self.total_bytes as f64) * 100.0).round() as u64
        }
    }
}

#[cfg(target_os = "windows")]
fn read_memory_info() -> MemoryInfo {
    if let Some(status) = crate::ffi::read_memory_status_ex() {
        MemoryInfo {
            total_bytes: status.ull_total_phys,
            available_bytes: status.ull_avail_phys,
        }
    } else {
        MemoryInfo::default()
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

#[cfg(target_os = "windows")]
#[derive(Default)]
struct DisplayInfo {
    resolution: String,
    refresh_rate: String,
    color_depth: String,
    refresh_rate_hz: String,
}

#[cfg(target_os = "windows")]
fn read_display_info() -> DisplayInfo {
    let mut mode = crate::ffi::DevModeW::default();

    unsafe {
        if crate::ffi::winapi::EnumDisplaySettingsW(std::ptr::null(), crate::ffi::ENUM_CURRENT_SETTINGS, &mut mode) != 0 {
            return DisplayInfo {
                resolution: format!("{} x {}", mode.dm_pels_width, mode.dm_pels_height),
                refresh_rate: format!("{} Hz", mode.dm_display_frequency),
                color_depth: format!("{} bpp", mode.dm_bits_per_pel),
                refresh_rate_hz: mode.dm_display_frequency.to_string(),
            };
        }

        let width = crate::ffi::winapi::GetSystemMetrics(crate::ffi::SM_CXSCREEN);
        let height = crate::ffi::winapi::GetSystemMetrics(crate::ffi::SM_CYSCREEN);
        DisplayInfo {
            resolution: format!("{width} x {height}"),
            refresh_rate: "Unknown".to_string(),
            color_depth: "Unknown".to_string(),
            refresh_rate_hz: "0".to_string(),
        }
    }
}

#[cfg(target_os = "windows")]
fn read_gpu_names() -> Vec<String> {
    let mut names = Vec::new();
    for index in 0..16 {
        let mut device = crate::ffi::DisplayDeviceW::default();

        unsafe {
            if crate::ffi::winapi::EnumDisplayDevicesW(std::ptr::null(), index, &mut device, 0) == 0 {
                break;
            }
        }

        const DISPLAY_DEVICE_MIRRORING_DRIVER: u32 = 0x0000_0008;
        if device.state_flags & DISPLAY_DEVICE_MIRRORING_DRIVER != 0 {
            continue;
        }

        let name = crate::utf16z_to_string(&device.device_string);
        if !name.is_empty() && !names.iter().any(|existing| existing == &name) {
            names.push(name);
        }
    }
    names
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedAppConflictInfo {
    pub id: String,
    pub name: String,
    pub installed: bool,
    pub details: String,
}

pub(crate) fn detect_installed_tweak_apps() -> Vec<DetectedAppConflictInfo> {
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

    // 1. Xbox Game Bar / GameDVR
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

    // 2. Discord
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

    // 3. GeForce Experience / NVIDIA App
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

    // 4. OBS Studio
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

    // 5. RivaTuner Statistics Server (RTSS)
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

    // 6. Steam
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

    // 7. VPN / Virtual Network Adapters
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

    // 8. Bluetooth
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

    // 9. Fast Startup (Hiberboot)
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
