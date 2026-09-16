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
