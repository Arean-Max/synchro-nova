use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[cfg(target_os = "windows")]
use crate::platform::ffi::{winapi, GammaRamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

impl GpuVendor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Nvidia => "nvidia",
            Self::Amd => "amd",
            Self::Intel => "intel",
            Self::Unknown => "unknown",
        }
    }


    pub fn curve_profile_name(&self) -> &'static str {
        match self {
            Self::Nvidia => "NVIDIA Smoothstep (170-220, 1.5% floor)",
            Self::Amd => "AMD Radeon Dynamic (165-225, 1.0% floor)",
            _ => "Standard Safe Gamma (170-220, 1.5% floor)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlackHoloStatus {
    pub active: bool,
    pub gpu_vendor: String,
    pub gpu_name: String,
    pub curve_profile: String,
    pub hotkey: String,
    pub rust_synced: bool,
    pub error: Option<String>,
}

#[cfg(target_os = "windows")]
static ORIGINAL_RAMP: Mutex<Option<GammaRamp>> = Mutex::new(None);

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
static WATCHDOG_RUNNING: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "windows")]
pub fn detect_gpu_vendor() -> (GpuVendor, String) {
    for idx in 0..8 {
        let mut device = crate::platform::ffi::DisplayDeviceW::default();
        let ok = unsafe {
            crate::platform::ffi::winapi::EnumDisplayDevicesW(std::ptr::null(), idx, &mut device, 0)
        };
        if ok == 0 {
            break;
        }

        let name = crate::utf16z_to_string(&device.device_string);
        let lower = name.to_lowercase();
        if lower.contains("nvidia") || lower.contains("geforce") || lower.contains("rtx") || lower.contains("gtx") {
            return (GpuVendor::Nvidia, name);
        }
        if lower.contains("amd") || lower.contains("radeon") || lower.contains("rx ") {
            return (GpuVendor::Amd, name);
        }
        if lower.contains("intel") || lower.contains("arc") || lower.contains("iris") || lower.contains("uhd") {
            return (GpuVendor::Intel, name);
        }
    }
    (GpuVendor::Unknown, "Standard Display Adapter".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn detect_gpu_vendor() -> (GpuVendor, String) {
    (GpuVendor::Unknown, "Unknown GPU".to_string())
}


#[cfg(target_os = "windows")]
unsafe extern "system" fn console_ctrl_handler(ctrl_type: u32) -> i32 {
    const CTRL_C_EVENT: u32 = 0;
    const CTRL_BREAK_EVENT: u32 = 1;
    const CTRL_CLOSE_EVENT: u32 = 2;
    const CTRL_SHUTDOWN_EVENT: u32 = 6;

    if matches!(ctrl_type, CTRL_C_EVENT | CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_SHUTDOWN_EVENT) {
        let _ = restore_original_system_ramp();
    }
    0
}

#[cfg(target_os = "windows")]
pub fn restore_original_system_ramp() -> Result<(), String> {
    unsafe {
        let hdc = winapi::GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            return Err("Failed to obtain screen DC".to_string());
        }

        let mut linear = GammaRamp::default();
        for i in 0..256 {
            let val = (i as u32 * 65535 / 255) as u16;
            linear.red[i] = val;
            linear.green[i] = val;
            linear.blue[i] = val;
        }

        let res = if let Ok(guard) = ORIGINAL_RAMP.lock() {
            if let Some(ref orig) = *guard {
                winapi::SetDeviceGammaRamp(hdc, orig)
            } else {
                winapi::SetDeviceGammaRamp(hdc, &linear)
            }
        } else {
            winapi::SetDeviceGammaRamp(hdc, &linear)
        };

        let _ = winapi::ReleaseDC(std::ptr::null_mut(), hdc);
        if res == 0 {
            let err = winapi::GetLastError();
            return Err(format!("Failed to restore gamma ramp (Win32 Error {})", err));
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn restore_original_system_ramp() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn ensure_cleanup_handlers_registered() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe {
            winapi::SetConsoleCtrlHandler(Some(console_ctrl_handler), 1);
        }
    });
}

pub fn stop_ramp_watchdog() {
    WATCHDOG_RUNNING.store(false, Ordering::SeqCst);
}

#[cfg(target_os = "windows")]
pub fn set_hardware_black_holo(enabled: bool) -> BlackHoloStatus {
    ensure_cleanup_handlers_registered();
    let (vendor, gpu_name) = detect_gpu_vendor();

    if enabled {
        let _ = restore_original_system_ramp();
        IS_ACTIVE.store(true, Ordering::SeqCst);
        let rust_res = crate::domain::games::set_rust_holosight_black(true);

        BlackHoloStatus {
            active: true,
            gpu_vendor: vendor.as_str().to_string(),
            gpu_name,
            curve_profile: "Cross-Channel Direct Matrix".to_string(),
            hotkey: "".to_string(),
            rust_synced: rust_res.success,
            error: None,
        }
    } else {
        let res = restore_original_system_ramp();
        IS_ACTIVE.store(false, Ordering::SeqCst);
        let rust_res = crate::domain::games::set_rust_holosight_black(false);

        let err = match res {
            Ok(_) => None,
            Err(e) => Some(e),
        };

        BlackHoloStatus {
            active: false,
            gpu_vendor: vendor.as_str().to_string(),
            gpu_name,
            curve_profile: "Standard".to_string(),
            hotkey: "".to_string(),
            rust_synced: rust_res.success,
            error: err,
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_hardware_black_holo(_enabled: bool) -> BlackHoloStatus {
    let (vendor, gpu_name) = detect_gpu_vendor();
    BlackHoloStatus {
        active: false,
        gpu_vendor: vendor.as_str().to_string(),
        gpu_name,
        curve_profile: vendor.curve_profile_name().to_string(),
        hotkey: "".to_string(),
        rust_synced: false,
        error: Some("Hardware Black Holo is only supported on Windows".to_string()),
    }
}

pub fn is_black_holo_active() -> bool {
    IS_ACTIVE.load(Ordering::SeqCst)
}

pub fn get_hardware_black_holo_status() -> BlackHoloStatus {
    let (vendor, gpu_name) = detect_gpu_vendor();
    BlackHoloStatus {
        active: IS_ACTIVE.load(Ordering::SeqCst),
        gpu_vendor: vendor.as_str().to_string(),
        gpu_name,
        curve_profile: vendor.curve_profile_name().to_string(),
        hotkey: "".to_string(),
        rust_synced: true,
        error: None,
    }
}

#[cfg(target_os = "windows")]
pub fn start_black_holo_hotkey_listener(_app_handle: Option<tauri::AppHandle>) {
}

#[cfg(not(target_os = "windows"))]
pub fn start_black_holo_hotkey_listener(_app_handle: Option<tauri::AppHandle>) {}
