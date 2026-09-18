use crate::ColorSettings;
use std::sync::{Mutex, OnceLock};

static ACTIVE_COLOR: Mutex<Option<ColorSettings>> = Mutex::new(None);
static COLOR_GUARD_STARTED: OnceLock<()> = OnceLock::new();

#[cfg(target_os = "windows")]
pub(crate) fn start_color_guard() {
    COLOR_GUARD_STARTED.get_or_init(|| {
        std::thread::spawn(|| {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(2500));
                let color_opt = if let Ok(guard) = ACTIVE_COLOR.lock() {
                    guard.clone()
                } else {
                    None
                };
                if let Some(color) = color_opt {
                    if color.enabled {
                        // Reassert gamma ramp in case an exclusive fullscreen game reset it
                        let _ = apply_gamma_ramp(color.gamma);
                    }
                }
            }
        });
    });
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn start_color_guard() {}

fn set_active_color(color: Option<ColorSettings>) {
    if let Ok(mut guard) = ACTIVE_COLOR.lock() {
        *guard = color;
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn apply_color_transform(color: &ColorSettings) -> Result<(), String> {
    if !color.enabled {
        set_active_color(None);
        return reset_color_transform();
    }

    set_active_color(Some(color.clone()));

    // Try hardware LUT gamma ramp first (optimal for standard SDR monitors)
    let ramp_success = apply_gamma_ramp(color.gamma).is_ok();

    // If hardware gamma ramp was rejected (Windows Auto HDR, Advanced Color, or driver limitation),
    // incorporate software gamma gain into the DWM Magnification matrix so calibration never fails.
    apply_magnification_color(color, !ramp_success)?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn apply_color_transform(_color: &ColorSettings) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub(crate) fn reset_color_transform() -> Result<(), String> {
    set_active_color(None);
    let effect = crate::ffi::MagColorEffect {
        transform: identity_matrix(),
    };

    unsafe {
        let _ = crate::ffi::winapi::MagSetFullscreenColorEffect(&effect);
    }
    let _ = apply_gamma_ramp(100.0);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn reset_color_transform() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_magnification_color(color: &ColorSettings, include_matrix_gamma: bool) -> Result<(), String> {
    let matrix = build_color_matrix(color, include_matrix_gamma);
    let effect = crate::ffi::MagColorEffect { transform: matrix };

    static MAGNIFICATION_READY: OnceLock<Result<(), String>> = OnceLock::new();

    MAGNIFICATION_READY
        .get_or_init(|| unsafe {
            if crate::ffi::winapi::MagInitialize() == 0 {
                Err("Windows Magnification API initialization failed".to_string())
            } else {
                Ok(())
            }
        })
        .clone()?;

    unsafe {
        if crate::ffi::winapi::MagSetFullscreenColorEffect(&effect) == 0 {
            return Err("Windows Magnification API color effect failed".to_string());
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_gamma_ramp(gamma_percent: f32) -> Result<(), String> {
    let gamma = (gamma_percent / 100.0).clamp(0.5, 1.5);
    let exponent = 1.0 / gamma;
    let mut ramp = crate::ffi::GammaRamp::default();

    for index in 0..256 {
        let normalized = index as f32 / 255.0;
        let corrected = (normalized.powf(exponent) * 65535.0).round() as u16;
        ramp.red[index] = corrected;
        ramp.green[index] = corrected;
        ramp.blue[index] = corrected;
    }

    unsafe {
        let hdc = crate::ffi::winapi::GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            return Err("Failed to acquire display device context".to_string());
        }

        let result = crate::ffi::winapi::SetDeviceGammaRamp(hdc, &ramp);
        let _ = crate::ffi::winapi::ReleaseDC(std::ptr::null_mut(), hdc);

        if result == 0 {
            return Err("Display driver rejected gamma ramp".to_string());
        }
    }

    Ok(())
}

pub(crate) fn gamma_fallback_matrix(gamma_percent: f32) -> [f32; 25] {
    let gamma = (gamma_percent / 100.0).clamp(0.5, 1.5);
    let gain = gamma.powf(0.85);
    let mut m = identity_matrix();
    m[0] = gain;
    m[6] = gain;
    m[12] = gain;
    m
}

fn build_color_matrix(color: &ColorSettings, include_matrix_gamma: bool) -> [f32; 25] {
    let saturation = color.saturation / 100.0;
    let contrast = color.contrast / 100.0;
    let hue = color.hue.to_radians();

    let mut matrix = identity_matrix();
    matrix = multiply_matrix(matrix, saturation_matrix(saturation));
    matrix = multiply_matrix(matrix, hue_matrix(hue));
    matrix = multiply_matrix(matrix, contrast_matrix(contrast));
    if include_matrix_gamma {
        matrix = multiply_matrix(matrix, gamma_fallback_matrix(color.gamma));
    }
    matrix
}

fn identity_matrix() -> [f32; 25] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn saturation_matrix(saturation: f32) -> [f32; 25] {
    let rw = 0.3086;
    let gw = 0.6094;
    let bw = 0.0820;
    let inv = 1.0 - saturation;

    [
        inv * rw + saturation,
        inv * rw,
        inv * rw,
        0.0,
        0.0,
        inv * gw,
        inv * gw + saturation,
        inv * gw,
        0.0,
        0.0,
        inv * bw,
        inv * bw,
        inv * bw + saturation,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn hue_matrix(angle: f32) -> [f32; 25] {
    let cos = angle.cos();
    let sin = angle.sin();

    [
        0.213 + cos * 0.787 - sin * 0.213,
        0.213 - cos * 0.213 + sin * 0.143,
        0.213 - cos * 0.213 - sin * 0.787,
        0.0,
        0.0,
        0.715 - cos * 0.715 - sin * 0.715,
        0.715 + cos * 0.285 + sin * 0.140,
        0.715 - cos * 0.715 + sin * 0.715,
        0.0,
        0.0,
        0.072 - cos * 0.072 + sin * 0.928,
        0.072 - cos * 0.072 - sin * 0.283,
        0.072 + cos * 0.928 + sin * 0.072,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn contrast_matrix(contrast: f32) -> [f32; 25] {
    let offset = 0.5 * (1.0 - contrast);
    [
        contrast, 0.0, 0.0, 0.0, 0.0, 0.0, contrast, 0.0, 0.0, 0.0, 0.0, 0.0, contrast, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0, 0.0, offset, offset, offset, 0.0, 1.0,
    ]
}

fn multiply_matrix(left: [f32; 25], right: [f32; 25]) -> [f32; 25] {
    let mut result = [0.0; 25];

    for row in 0..5 {
        for column in 0..5 {
            let mut value = 0.0;
            for index in 0..5 {
                value += left[row * 5 + index] * right[index * 5 + column];
            }
            result[row * 5 + column] = value;
        }
    }

    result
}
