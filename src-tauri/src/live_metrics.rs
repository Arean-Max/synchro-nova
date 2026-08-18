use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveMetrics {
    pub cpu_usage_percent: String,
    pub gpu_usage_percent: String,
    pub ram_available_gb: String,
    pub ram_used_gb: String,
    pub ram_used_percent: String,
    pub vram_total_gb: String,
    pub vram_used_gb: String,
    pub vram_used_percent: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CpuSample {
    idle: u64,
    kernel: u64,
    user: u64,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Clone, Copy)]
struct FileTime {
    low: u32,
    high: u32,
}

pub(crate) fn collect_live_metrics(previous: &mut Option<CpuSample>) -> LiveMetrics {
    let memory = read_memory_info();
    let vram = read_vram_info();
    let cpu_usage = read_cpu_usage(previous);
    let gpu_usage = read_gpu_usage();
    LiveMetrics {
        cpu_usage_percent: cpu_usage.to_string(),
        gpu_usage_percent: gpu_usage.to_string(),
        ram_available_gb: format!("{:.1}", bytes_to_gb(memory.available_bytes)),
        ram_used_gb: format!(
            "{:.1}",
            bytes_to_gb(memory.total_bytes.saturating_sub(memory.available_bytes))
        ),
        ram_used_percent: memory.used_percent().to_string(),
        vram_total_gb: format!("{:.1}", bytes_to_gb(vram.total_bytes)),
        vram_used_gb: format!("{:.1}", bytes_to_gb(vram.used_bytes)),
        vram_used_percent: vram.used_percent().to_string(),
    }
}

#[derive(Default)]
struct MemoryInfo {
    total_bytes: u64,
    available_bytes: u64,
}

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

#[derive(Default)]
struct VramInfo {
    total_bytes: u64,
    used_bytes: u64,
}

impl VramInfo {
    fn used_percent(&self) -> u64 {
        if self.total_bytes == 0 {
            0
        } else {
            ((self.used_bytes as f64 / self.total_bytes as f64) * 100.0).round() as u64
        }
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

#[cfg(target_os = "windows")]
fn read_cpu_usage(previous: &mut Option<CpuSample>) -> u64 {
    let current = match read_cpu_sample() {
        Some(value) => value,
        None => return 0,
    };

    let base = match previous.replace(current) {
        Some(value) => value,
        None => {
            std::thread::sleep(std::time::Duration::from_millis(110));
            let next = match read_cpu_sample() {
                Some(value) => value,
                None => return 0,
            };
            *previous = Some(next);
            current
        }
    };

    cpu_percent(base, previous.unwrap_or(current))
}

#[cfg(not(target_os = "windows"))]
fn read_cpu_usage(_previous: &mut Option<CpuSample>) -> u64 {
    0
}

fn cpu_percent(previous: CpuSample, current: CpuSample) -> u64 {
    let idle = current.idle.saturating_sub(previous.idle);
    let kernel = current.kernel.saturating_sub(previous.kernel);
    let user = current.user.saturating_sub(previous.user);
    let total = kernel.saturating_add(user);
    if total == 0 {
        return 0;
    }
    let busy = total.saturating_sub(idle);
    ((busy as f64 / total as f64) * 100.0)
        .round()
        .clamp(0.0, 100.0) as u64
}

#[cfg(target_os = "windows")]
fn read_cpu_sample() -> Option<CpuSample> {
    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn GetSystemTimes(
            idle_time: *mut FileTime,
            kernel_time: *mut FileTime,
            user_time: *mut FileTime,
        ) -> i32;
    }

    let mut idle = FileTime { low: 0, high: 0 };
    let mut kernel = FileTime { low: 0, high: 0 };
    let mut user = FileTime { low: 0, high: 0 };
    let ok = unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) != 0 };
    if !ok {
        return None;
    }
    Some(CpuSample {
        idle: filetime_to_u64(idle),
        kernel: filetime_to_u64(kernel),
        user: filetime_to_u64(user),
    })
}

#[cfg(target_os = "windows")]
fn filetime_to_u64(value: FileTime) -> u64 {
    ((value.high as u64) << 32) | value.low as u64
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

#[cfg(not(target_os = "windows"))]
fn read_memory_info() -> MemoryInfo {
    MemoryInfo::default()
}

#[cfg(target_os = "windows")]
fn read_vram_info() -> VramInfo {
    VramInfo {
        total_bytes: read_display_vram_total(),
        used_bytes: read_gpu_dedicated_usage_bytes(),
    }
}

#[cfg(not(target_os = "windows"))]
fn read_vram_info() -> VramInfo {
    VramInfo::default()
}

#[cfg(target_os = "windows")]
fn read_gpu_usage() -> u64 {
    pdh_sum_counter("\\GPU Engine(*)\\Utilization Percentage", true)
        .round()
        .clamp(0.0, 100.0) as u64
}

#[cfg(not(target_os = "windows"))]
fn read_gpu_usage() -> u64 {
    0
}

#[cfg(target_os = "windows")]
fn read_gpu_dedicated_usage_bytes() -> u64 {
    pdh_sum_counter("\\GPU Adapter Memory(*)\\Dedicated Usage", false)
        .round()
        .max(0.0) as u64
}

#[cfg(target_os = "windows")]
fn pdh_sum_counter(path: &str, second_sample: bool) -> f64 {
    type PdhQuery = isize;
    type PdhCounter = isize;

    #[repr(C)]
    struct PdhFmtCounterValue {
        c_status: u32,
        _padding: u32,
        double_value: f64,
    }

    #[repr(C)]
    struct PdhFmtCounterValueItemW {
        name: *const u16,
        value: PdhFmtCounterValue,
    }

    #[link(name = "Pdh")]
    unsafe extern "system" {
        fn PdhOpenQueryW(data_source: *const u16, user_data: usize, query: *mut PdhQuery) -> u32;
        fn PdhAddEnglishCounterW(
            query: PdhQuery,
            full_counter_path: *const u16,
            user_data: usize,
            counter: *mut PdhCounter,
        ) -> u32;
        fn PdhCollectQueryData(query: PdhQuery) -> u32;
        fn PdhGetFormattedCounterArrayW(
            counter: PdhCounter,
            format: u32,
            buffer_size: *mut u32,
            item_count: *mut u32,
            item_buffer: *mut PdhFmtCounterValueItemW,
        ) -> u32;
        fn PdhCloseQuery(query: PdhQuery) -> u32;
    }

    const ERROR_SUCCESS: u32 = 0;
    const PDH_MORE_DATA: u32 = 0x8000_07D2;
    const PDH_FMT_DOUBLE: u32 = 0x0000_0200;

    let mut query: PdhQuery = 0;
    let open_status = unsafe { PdhOpenQueryW(std::ptr::null(), 0, &mut query) };
    if open_status != ERROR_SUCCESS || query == 0 {
        return 0.0;
    }

    let wide_path = wide_null(path);
    let mut counter: PdhCounter = 0;
    let add_status = unsafe { PdhAddEnglishCounterW(query, wide_path.as_ptr(), 0, &mut counter) };
    if add_status != ERROR_SUCCESS || counter == 0 {
        unsafe {
            PdhCloseQuery(query);
        }
        return 0.0;
    }

    unsafe {
        PdhCollectQueryData(query);
    }
    if second_sample {
        std::thread::sleep(std::time::Duration::from_millis(120));
        unsafe {
            PdhCollectQueryData(query);
        }
    }

    let mut buffer_size = 0u32;
    let mut item_count = 0u32;
    let first_status = unsafe {
        PdhGetFormattedCounterArrayW(
            counter,
            PDH_FMT_DOUBLE,
            &mut buffer_size,
            &mut item_count,
            std::ptr::null_mut(),
        )
    };
    if first_status != PDH_MORE_DATA || buffer_size == 0 || item_count == 0 {
        unsafe {
            PdhCloseQuery(query);
        }
        return 0.0;
    }

    let mut buffer = vec![0u8; buffer_size as usize];
    let items = buffer.as_mut_ptr() as *mut PdhFmtCounterValueItemW;
    let second_status = unsafe {
        PdhGetFormattedCounterArrayW(
            counter,
            PDH_FMT_DOUBLE,
            &mut buffer_size,
            &mut item_count,
            items,
        )
    };

    let sum = if second_status == ERROR_SUCCESS {
        let values = unsafe { std::slice::from_raw_parts(items, item_count as usize) };
        values
            .iter()
            .filter(|item| item.value.c_status == ERROR_SUCCESS)
            .map(|item| item.value.double_value.max(0.0))
            .sum()
    } else {
        0.0
    };

    unsafe {
        PdhCloseQuery(query);
    }
    sum
}

#[cfg(target_os = "windows")]
fn read_display_vram_total() -> u64 {
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

    let mut best = 0;
    for index in 0..16 {
        let mut device = DisplayDeviceW {
            cb: std::mem::size_of::<DisplayDeviceW>() as u32,
            device_name: [0; 32],
            device_string: [0; 128],
            state_flags: 0,
            device_id: [0; 128],
            device_key: [0; 128],
        };

        let ok = unsafe { EnumDisplayDevicesW(std::ptr::null(), index, &mut device, 0) != 0 };
        if !ok {
            break;
        }

        let path = registry_machine_path(&crate::utf16z_to_string(&device.device_key));
        if let Some(bytes) = registry_vram_bytes(&path) {
            best = best.max(bytes);
        }
    }
    best
}

#[cfg(target_os = "windows")]
fn registry_machine_path(path: &str) -> String {
    path.trim()
        .trim_start_matches("\\Registry\\Machine\\")
        .trim_start_matches("Registry\\Machine\\")
        .to_string()
}

#[cfg(target_os = "windows")]
fn registry_vram_bytes(path: &str) -> Option<u64> {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(path).ok()?;
    read_reg_u64(&key, "HardwareInformation.qwMemorySize")
        .or_else(|| read_reg_u64(&key, "HardwareInformation.MemorySize"))
}

#[cfg(target_os = "windows")]
fn read_reg_u64(key: &winreg::RegKey, name: &str) -> Option<u64> {
    key.get_value::<u64, _>(name).ok().or_else(|| {
        let raw = key.get_raw_value(name).ok()?;
        if raw.bytes.len() >= 8 {
            let mut buffer = [0u8; 8];
            buffer.copy_from_slice(&raw.bytes[..8]);
            Some(u64::from_le_bytes(buffer))
        } else if raw.bytes.len() >= 4 {
            let mut buffer = [0u8; 4];
            buffer.copy_from_slice(&raw.bytes[..4]);
            Some(u32::from_le_bytes(buffer) as u64)
        } else {
            None
        }
    })
}

#[cfg(target_os = "windows")]
fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
