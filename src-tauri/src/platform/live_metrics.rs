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
