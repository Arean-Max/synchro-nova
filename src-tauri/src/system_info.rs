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
    #[repr(C)]
    struct MemoryStatusEx {
        dw_length: u32,
        dw_memory_load: u32,
        ull_total_phys: u64,
        ull_avail_phys: u64,
        ull_total_page_file: u64,
        ull_avail_page_file: u64,
        ull_total_virtual: u64,
        ull_avail_virtual: u64,
        ull_avail_extended_virtual: u64,
    }

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
    }

    let mut status = MemoryStatusEx {
        dw_length: std::mem::size_of::<MemoryStatusEx>() as u32,
        dw_memory_load: 0,
        ull_total_phys: 0,
        ull_avail_phys: 0,
        ull_total_page_file: 0,
        ull_avail_page_file: 0,
        ull_total_virtual: 0,
        ull_avail_virtual: 0,
        ull_avail_extended_virtual: 0,
    };

    unsafe {
        if GlobalMemoryStatusEx(&mut status) == 0 {
            MemoryInfo::default()
        } else {
            MemoryInfo {
                total_bytes: status.ull_total_phys,
                available_bytes: status.ull_avail_phys,
            }
        }
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
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct PointL {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct DevModeW {
        dm_device_name: [u16; 32],
        dm_spec_version: u16,
        dm_driver_version: u16,
        dm_size: u16,
        dm_driver_extra: u16,
        dm_fields: u32,
        dm_position: PointL,
        dm_display_orientation: u32,
        dm_display_fixed_output: u32,
        dm_color: i16,
        dm_duplex: i16,
        dm_y_resolution: i16,
        dm_tt_option: i16,
        dm_collate: i16,
        dm_form_name: [u16; 32],
        dm_log_pixels: u16,
        dm_bits_per_pel: u32,
        dm_pels_width: u32,
        dm_pels_height: u32,
        dm_display_flags: u32,
        dm_display_frequency: u32,
        dm_icm_method: u32,
        dm_icm_intent: u32,
        dm_media_type: u32,
        dm_dither_type: u32,
        dm_reserved1: u32,
        dm_reserved2: u32,
        dm_panning_width: u32,
        dm_panning_height: u32,
    }

    #[link(name = "User32")]
    unsafe extern "system" {
        fn EnumDisplaySettingsW(
            device_name: *const u16,
            mode_num: u32,
            dev_mode: *mut DevModeW,
        ) -> i32;
        fn GetSystemMetrics(index: i32) -> i32;
    }

    const ENUM_CURRENT_SETTINGS: u32 = 0xFFFF_FFFF;
    const SM_CXSCREEN: i32 = 0;
    const SM_CYSCREEN: i32 = 1;

    let mut mode = DevModeW {
        dm_device_name: [0; 32],
        dm_spec_version: 0,
        dm_driver_version: 0,
        dm_size: std::mem::size_of::<DevModeW>() as u16,
        dm_driver_extra: 0,
        dm_fields: 0,
        dm_position: PointL { x: 0, y: 0 },
        dm_display_orientation: 0,
        dm_display_fixed_output: 0,
        dm_color: 0,
        dm_duplex: 0,
        dm_y_resolution: 0,
        dm_tt_option: 0,
        dm_collate: 0,
        dm_form_name: [0; 32],
        dm_log_pixels: 0,
        dm_bits_per_pel: 0,
        dm_pels_width: 0,
        dm_pels_height: 0,
        dm_display_flags: 0,
        dm_display_frequency: 0,
        dm_icm_method: 0,
        dm_icm_intent: 0,
        dm_media_type: 0,
        dm_dither_type: 0,
        dm_reserved1: 0,
        dm_reserved2: 0,
        dm_panning_width: 0,
        dm_panning_height: 0,
    };

    unsafe {
        if EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut mode) != 0 {
            return DisplayInfo {
                resolution: format!("{} x {}", mode.dm_pels_width, mode.dm_pels_height),
                refresh_rate: format!("{} Hz", mode.dm_display_frequency),
                color_depth: format!("{} bpp", mode.dm_bits_per_pel),
                refresh_rate_hz: mode.dm_display_frequency.to_string(),
            };
        }

        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
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
    #[repr(C)]
    struct DisplayDeviceW {
        cb: u32,
        device_name: [u16; 32],
        device_string: [u16; 128],
        state_flags: u32,
        device_id: [u16; 128],
        device_key: [u16; 128],
    }

    #[link(name = "User32")]
    unsafe extern "system" {
        fn EnumDisplayDevicesW(
            device_name: *const u16,
            dev_num: u32,
            display_device: *mut DisplayDeviceW,
            flags: u32,
        ) -> i32;
    }

    let mut names = Vec::new();
    for index in 0..16 {
        let mut device = DisplayDeviceW {
            cb: std::mem::size_of::<DisplayDeviceW>() as u32,
            device_name: [0; 32],
            device_string: [0; 128],
            state_flags: 0,
            device_id: [0; 128],
            device_key: [0; 128],
        };

        unsafe {
            if EnumDisplayDevicesW(std::ptr::null(), index, &mut device, 0) == 0 {
                break;
            }
        }

        let name = crate::utf16z_to_string(&device.device_string);
        if !name.is_empty() && !names.iter().any(|existing| existing == &name) {
            names.push(name);
        }
    }
    names
}
