use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

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

    pub fn display_title(&self) -> &'static str {
        match self {
            Self::Nvidia => "NVIDIA GeForce",
            Self::Amd => "AMD Radeon",
            Self::Intel => "Intel Graphics",
            Self::Unknown => "Generic GPU",
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
#[cfg(target_os = "windows")]
static TARGET_BLACK_HOLO_RAMP: Mutex<Option<GammaRamp>> = Mutex::new(None);

static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
static WATCHDOG_RUNNING: AtomicBool = AtomicBool::new(false);
static HOTKEY_LISTENER_STARTED: AtomicBool = AtomicBool::new(false);

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

/// Generates a driver-validated hardware gamma LUT for Black Holosight.
///
/// Anti-Cheat: Strictly operates through user-mode WinAPI (SetDeviceGammaRamp) on the desktop DC.
/// No process handles, no DLL injection, no DirectX/Vulkan hooks, no memory reading.
///
/// NVIDIA Driver Check:
/// - Range 0..170: Linear gamma (y = x, i * 257)
/// - Range 170..220: Half-cosine wave smooth falloff from maximum (43690) to 1.5% floor (983)
/// - Range 220..255: Minimum floor held at 1.5% (prevents NVIDIA driver sanity-check rejection)
/// - Red and Blue channels maintain strict linear ramp (i * 257).
#[cfg(target_os = "windows")]
pub fn generate_black_holo_ramp(vendor: GpuVendor) -> GammaRamp {
    let mut ramp = GammaRamp::default();

    // R & B channels remain strict linear
    for i in 0..256 {
        let linear = (i as u32 * 65535 / 255) as u16;
        ramp.red[i] = linear;
        ramp.blue[i] = linear;
    }

    let (start_idx, end_idx, min_pct) = match vendor {
        GpuVendor::Amd => (165, 225, 0.010_f32),     // AMD allows slightly wider/crisper dynamic range
        _ => (170, 220, 0.015_f32),                  // NVIDIA & default: strict gradient smoothstep
    };

    let min_val = (65535.0 * min_pct).round() as f64;
    let start_val = (start_idx as f64) * 257.0;

    for i in 0..256 {
        if i <= start_idx {
            ramp.green[i] = (i as u32 * 257) as u16;
        } else if i <= end_idx {
            let t = (i - start_idx) as f64 / (end_idx - start_idx) as f64;
            // Half-wave cosine curve for C^1 continuous smooth falloff
            let w = 0.5 * (1.0 + (std::f64::consts::PI * t).cos());
            let val = (min_val + (start_val - min_val) * w).round();
            ramp.green[i] = val.clamp(min_val, 65535.0) as u16;
        } else {
            ramp.green[i] = min_val as u16;
        }
    }

    ramp
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

        let res = if let Ok(guard) = ORIGINAL_RAMP.lock() {
            if let Some(ref orig) = *guard {
                winapi::SetDeviceGammaRamp(hdc, orig)
            } else {
                let default_ramp = crate::platform::ffi::GammaRamp::default();
                winapi::SetDeviceGammaRamp(hdc, &default_ramp)
            }
        } else {
            0
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

#[cfg(target_os = "windows")]
fn start_ramp_watchdog() {
    if WATCHDOG_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    std::thread::Builder::new()
        .name("synchro-black-holo-watchdog".to_string())
        .spawn(|| {
            while WATCHDOG_RUNNING.load(Ordering::SeqCst) {
                std::thread::sleep(Duration::from_millis(1500));

                if !IS_ACTIVE.load(Ordering::SeqCst) {
                    continue;
                }

                let target = match TARGET_BLACK_HOLO_RAMP.lock() {
                    Ok(g) => g.clone(),
                    Err(_) => None,
                };

                let Some(target) = target else {
                    continue;
                };

                unsafe {
                    let hdc = winapi::GetDC(std::ptr::null_mut());
                    if hdc.is_null() {
                        continue;
                    }

                    let mut current = GammaRamp::default();
                    if winapi::GetDeviceGammaRamp(hdc, &mut current) != 0 {
                        // Check green channel entry at index 200 (target is ~1000, linear is >45000)
                        let target_g = target.green[200];
                        let current_g = current.green[200];
                        let diff = (current_g as i32 - target_g as i32).abs();

                        // Driver reset detected (e.g. after Alt-Tab or mode change)
                        if diff > 5000 {
                            let _ = winapi::SetDeviceGammaRamp(hdc, &target);
                        }
                    }
                    let _ = winapi::ReleaseDC(std::ptr::null_mut(), hdc);
                }
            }
        })
        .ok();
}

pub fn stop_ramp_watchdog() {
    WATCHDOG_RUNNING.store(false, Ordering::SeqCst);
}

#[cfg(target_os = "windows")]
pub fn set_hardware_black_holo(enabled: bool) -> BlackHoloStatus {
    ensure_cleanup_handlers_registered();
    let (vendor, gpu_name) = detect_gpu_vendor();

    unsafe {
        let hdc = winapi::GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            return BlackHoloStatus {
                active: false,
                gpu_vendor: vendor.as_str().to_string(),
                gpu_name,
                curve_profile: vendor.curve_profile_name().to_string(),
                hotkey: "F11".to_string(),
                rust_synced: false,
                error: Some("WinAPI Error: Failed to acquire display device context (GetDC null)".to_string()),
            };
        }

        if enabled {
            // 1. Save original system ramp once before modifying
            if let Ok(mut guard) = ORIGINAL_RAMP.lock() {
                if guard.is_none() {
                    let mut orig = GammaRamp::default();
                    if winapi::GetDeviceGammaRamp(hdc, &mut orig) != 0 {
                        *guard = Some(orig);
                    }
                }
            }

            // 2. Generate driver-validated ramp
            let target_ramp = generate_black_holo_ramp(vendor);
            if let Ok(mut guard) = TARGET_BLACK_HOLO_RAMP.lock() {
                *guard = Some(target_ramp);
            }

            // 3. Apply to hardware DAC
            let res = winapi::SetDeviceGammaRamp(hdc, &target_ramp);
            let _ = winapi::ReleaseDC(std::ptr::null_mut(), hdc);

            if res == 0 {
                let err = winapi::GetLastError();
                IS_ACTIVE.store(false, Ordering::SeqCst);
                let hint = match err {
                    87 => "Invalid parameter. Ensure Windows HDR is disabled or display color depth is 8-bit/10-bit SDR.",
                    _ => "Display driver rejected gamma ramp. Ensure custom color/calibration is allowed in GPU Control Panel.",
                };
                return BlackHoloStatus {
                    active: false,
                    gpu_vendor: vendor.as_str().to_string(),
                    gpu_name,
                    curve_profile: vendor.curve_profile_name().to_string(),
                    hotkey: "F11".to_string(),
                    rust_synced: false,
                    error: Some(format!("SetDeviceGammaRamp failed (Win32 Error {}): {}", err, hint)),
                };
            }

            IS_ACTIVE.store(true, Ordering::SeqCst);
            start_ramp_watchdog();

            // 4. Also synchronize Rust client.cfg
            let rust_res = crate::domain::games::set_rust_holosight_black(true);

            BlackHoloStatus {
                active: true,
                gpu_vendor: vendor.as_str().to_string(),
                gpu_name,
                curve_profile: vendor.curve_profile_name().to_string(),
                hotkey: "F11".to_string(),
                rust_synced: rust_res.success,
                error: None,
            }
        } else {
            let res = restore_original_system_ramp();
            let _ = winapi::ReleaseDC(std::ptr::null_mut(), hdc);
            IS_ACTIVE.store(false, Ordering::SeqCst);

            // Synchronize Rust client.cfg back
            let rust_res = crate::domain::games::set_rust_holosight_black(false);

            let err = match res {
                Ok(_) => None,
                Err(e) => Some(e),
            };

            BlackHoloStatus {
                active: false,
                gpu_vendor: vendor.as_str().to_string(),
                gpu_name,
                curve_profile: vendor.curve_profile_name().to_string(),
                hotkey: "F11".to_string(),
                rust_synced: rust_res.success,
                error: err,
            }
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
        hotkey: "F11".to_string(),
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
        hotkey: "F11".to_string(),
        rust_synced: true,
        error: None,
    }
}

#[cfg(target_os = "windows")]
pub fn start_black_holo_hotkey_listener(app_handle: Option<tauri::AppHandle>) {
    if HOTKEY_LISTENER_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    std::thread::Builder::new()
        .name("synchro-black-holo-hotkey".to_string())
        .spawn(move || {
            const HOTKEY_ID: i32 = 0x484F; // 'HO'
            const VK_F11: u32 = 0x7A;
            const WM_HOTKEY: u32 = 0x0312;

            unsafe {
                let registered = winapi::RegisterHotKey(std::ptr::null_mut(), HOTKEY_ID, 0, VK_F11);
                if registered != 0 {
                    let mut msg = std::mem::zeroed::<crate::platform::ffi::MSG>();
                    while winapi::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                        if msg.message == WM_HOTKEY {
                            let next_state = !IS_ACTIVE.load(Ordering::SeqCst);
                            let status = set_hardware_black_holo(next_state);
                            if let Some(ref app) = app_handle {
                                use tauri::Emitter;
                                let _ = app.emit("black-holo-toggled", &status);
                            }
                        }
                    }
                    winapi::UnregisterHotKey(std::ptr::null_mut(), HOTKEY_ID);
                }
            }
        })
        .ok();
}

#[cfg(not(target_os = "windows"))]
pub fn start_black_holo_hotkey_listener(_app_handle: Option<tauri::AppHandle>) {}
