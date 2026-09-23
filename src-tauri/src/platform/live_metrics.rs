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
pub struct CpuSample {
    idle: u64,
    kernel: u64,
    user: u64,
}

pub(crate) fn collect_live_metrics(previous: &mut Option<CpuSample>) -> LiveMetrics {
    let memory = read_memory_info();
    let (gpu_usage, dedicated_vram) = read_gpu_live_metrics();
    let total_vram = read_display_vram_total();
    let cpu_usage = read_cpu_usage(previous);
    LiveMetrics {
        cpu_usage_percent: cpu_usage.to_string(),
        gpu_usage_percent: gpu_usage.to_string(),
        ram_available_gb: format!("{:.1}", bytes_to_gb(memory.available_bytes)),
        ram_used_gb: format!(
            "{:.1}",
            bytes_to_gb(memory.total_bytes.saturating_sub(memory.available_bytes))
        ),
        ram_used_percent: memory.used_percent().to_string(),
        vram_total_gb: format!("{:.1}", bytes_to_gb(total_vram)),
        vram_used_gb: format!("{:.1}", bytes_to_gb(dedicated_vram)),
        vram_used_percent: if total_vram > 0 {
            ((dedicated_vram as f64 / total_vram as f64) * 100.0).round() as u64
        } else {
            0
        }.to_string(),
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
        None => return 0,
    };

    cpu_percent(base, current)
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
    let (idle, kernel, user) = super::ffi::read_system_times()?;
    Some(CpuSample {
        idle: filetime_to_u64(idle),
        kernel: filetime_to_u64(kernel),
        user: filetime_to_u64(user),
    })
}

#[cfg(target_os = "windows")]
fn filetime_to_u64(value: super::ffi::FileTime) -> u64 {
    ((value.high as u64) << 32) | value.low as u64
}

#[cfg(target_os = "windows")]
fn read_memory_info() -> MemoryInfo {
    if let Some(status) = super::ffi::read_memory_status_ex() {
        MemoryInfo {
            total_bytes: status.ull_total_phys,
            available_bytes: status.ull_avail_phys,
        }
    } else {
        MemoryInfo::default()
    }
}

#[cfg(not(target_os = "windows"))]
fn read_memory_info() -> MemoryInfo {
    MemoryInfo::default()
}

#[cfg(not(target_os = "windows"))]
fn read_gpu_live_metrics() -> (u64, u64) {
    (0, 0)
}

#[cfg(not(target_os = "windows"))]
fn read_display_vram_total() -> u64 {
    0
}

#[cfg(target_os = "windows")]
struct PdhSession {
    query: isize,
    engine_counter: isize,
    vram_counter: isize,
}

#[cfg(target_os = "windows")]
impl Drop for PdhSession {
    fn drop(&mut self) {
        if self.query != 0 {
            unsafe {
                let _ = PdhCloseQuery(self.query);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn read_gpu_live_metrics() -> (u64, u64) {
    use std::sync::Mutex;
    static SESSION: Mutex<Option<PdhSession>> = Mutex::new(None);

    const ERROR_SUCCESS: u32 = 0;

    let mut guard = match SESSION.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };

    if guard.is_none() {
        let mut query: isize = 0;
        if unsafe { PdhOpenQueryW(std::ptr::null(), 0, &mut query) } == ERROR_SUCCESS && query != 0 {
            let engine_path = wide_null("\\GPU Engine(*)\\Utilization Percentage");
            let mut engine_counter: isize = 0;
            let _ = unsafe { PdhAddEnglishCounterW(query, engine_path.as_ptr(), 0, &mut engine_counter) };

            let vram_path = wide_null("\\GPU Adapter Memory(*)\\Dedicated Usage");
            let mut vram_counter: isize = 0;
            let _ = unsafe { PdhAddEnglishCounterW(query, vram_path.as_ptr(), 0, &mut vram_counter) };

            let _ = unsafe { PdhCollectQueryData(query) };
            *guard = Some(PdhSession {
                query,
                engine_counter,
                vram_counter,
            });
            return (0, 0);
        }
    }

    if let Some(session) = guard.as_ref() {
        let status = unsafe { PdhCollectQueryData(session.query) };
        if status == ERROR_SUCCESS {
            let gpu = read_pdh_counter_sum(session.engine_counter).round().clamp(0.0, 100.0) as u64;
            let vram = read_pdh_counter_sum(session.vram_counter).round().max(0.0) as u64;
            return (gpu, vram);
        }
    }

    (0, 0)
}

#[cfg(target_os = "windows")]
type PdhCounter = isize;

#[cfg(target_os = "windows")]
#[repr(C)]
struct PdhFmtCounterValue {
    c_status: u32,
    _padding: u32,
    double_value: f64,
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct PdhFmtCounterValueItemW {
    name: *const u16,
    value: PdhFmtCounterValue,
}

#[cfg(target_os = "windows")]
#[link(name = "Pdh")]
unsafe extern "system" {
    fn PdhOpenQueryW(data_source: *const u16, user_data: usize, query: *mut isize) -> u32;
    fn PdhAddEnglishCounterW(
        query: isize,
        full_counter_path: *const u16,
        user_data: usize,
        counter: *mut PdhCounter,
    ) -> u32;
    fn PdhCollectQueryData(query: isize) -> u32;
    fn PdhGetFormattedCounterArrayW(
        counter: PdhCounter,
        format: u32,
        buffer_size: *mut u32,
        item_count: *mut u32,
        item_buffer: *mut PdhFmtCounterValueItemW,
    ) -> u32;
    #[allow(dead_code)]
    fn PdhCloseQuery(query: isize) -> u32;
}

#[cfg(target_os = "windows")]
fn read_pdh_counter_sum(counter: isize) -> f64 {
    use std::cell::RefCell;
    thread_local! {
        static PDH_BUFFER: RefCell<Vec<u64>> = RefCell::new(Vec::with_capacity(512));
    }

    const ERROR_SUCCESS: u32 = 0;
    const PDH_MORE_DATA: u32 = 0x8000_07D2;
    const PDH_FMT_DOUBLE: u32 = 0x0000_0200;

    if counter == 0 {
        return 0.0;
    }

    let mut buffer_size = 0u32;
    let mut item_count = 0u32;
    let first = unsafe {
        PdhGetFormattedCounterArrayW(
            counter,
            PDH_FMT_DOUBLE,
            &mut buffer_size,
            &mut item_count,
            std::ptr::null_mut(),
        )
    };
    if first != PDH_MORE_DATA || buffer_size == 0 || item_count == 0 {
        return 0.0;
    }

    PDH_BUFFER.with(|buf_cell| {
        let mut buffer = buf_cell.borrow_mut();
        // buffer_size is in bytes. Ensure capacity in u64 words to guarantee 8-byte alignment
        let needed_words = (buffer_size as usize + 7) / 8;
        if buffer.len() < needed_words {
            buffer.resize(needed_words, 0);
        }
        let items = buffer.as_mut_ptr() as *mut PdhFmtCounterValueItemW;
        let mut current_size = (buffer.len() * 8) as u32;
        let second = unsafe {
            PdhGetFormattedCounterArrayW(
                counter,
                PDH_FMT_DOUBLE,
                &mut current_size,
                &mut item_count,
                items,
            )
        };

        let item_size = std::mem::size_of::<PdhFmtCounterValueItemW>();
        let max_items_fit = if item_size > 0 {
            (buffer.len() * 8) / item_size
        } else {
            0
        };

        if second == ERROR_SUCCESS && (item_count as usize) <= max_items_fit {
            let values = unsafe { std::slice::from_raw_parts(items, item_count as usize) };
            values
                .iter()
                .filter(|item| item.value.c_status == ERROR_SUCCESS)
                .map(|item| item.value.double_value.max(0.0))
                .sum()
        } else {
            0.0
        }
    })
}

#[cfg(target_os = "windows")]
fn read_display_vram_total() -> u64 {
    static TOTAL_VRAM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let cached = TOTAL_VRAM.load(std::sync::atomic::Ordering::Relaxed);
    if cached > 0 {
        return cached;
    }

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

    if best > 0 {
        TOTAL_VRAM.store(best, std::sync::atomic::Ordering::Relaxed);
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
