static ORIGINAL_GAMMA_RAMP: std::sync::OnceLock<crate::platform::ffi::GammaRamp> = std::sync::OnceLock::new();
static MAG_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
static MAG_INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(target_os = "windows")]
fn ensure_mag_initialized() {
    if !MAG_INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
        unsafe {
            if crate::platform::ffi::winapi::MagInitialize() != 0 {
                MAG_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn ensure_original_gamma_captured() {
    if ORIGINAL_GAMMA_RAMP.get().is_none() {
        unsafe {
            let hdc = crate::platform::ffi::winapi::GetDC(std::ptr::null_mut());
            if !hdc.is_null() {
                let mut orig = crate::platform::ffi::GammaRamp::default();
                if crate::platform::ffi::winapi::GetDeviceGammaRamp(hdc, &mut orig) != 0 {
                    let _ = ORIGINAL_GAMMA_RAMP.set(orig);
                }
                let _ = crate::platform::ffi::winapi::ReleaseDC(std::ptr::null_mut(), hdc);
            }
        }
    }
}
use crate::app::state::ColorSettings;
use std::sync::Mutex;

static ACTIVE_COLOR: Mutex<Option<(ColorSettings, bool)>> = Mutex::new(None);

pub fn start_color_guard() {}

pub fn stop_color_guard() {
    if let Ok(mut lock) = ACTIVE_COLOR.lock() {
        *lock = None;
    }
}

fn set_active_color(color: Option<(ColorSettings, bool)>) {
    if let Ok(mut lock) = ACTIVE_COLOR.lock() {
        *lock = color;
    }
}

pub fn get_active_color() -> Option<ColorSettings> {
    if let Ok(lock) = ACTIVE_COLOR.lock() {
        lock.as_ref().map(|(c, _)| c.clone())
    } else {
        None
    }
}

pub fn reapply_active_color() {
    std::thread::Builder::new()
        .name("synchro-color-worker".to_string())
        .stack_size(128 * 1024)
        .spawn(|| {
            if let Ok(lock) = ACTIVE_COLOR.lock() {
                if let Some((ref color, show_on_recordings)) = *lock {
                    let _ = apply_color_transform(color, show_on_recordings);
                }
            }
        })
        .ok();
}

#[cfg(target_os = "windows")]
pub fn apply_color_transform(color: &ColorSettings, show_on_recordings: bool) -> Result<(), String> {
    if !color.enabled {
        set_active_color(None);
        return reset_color_transform();
    }

    set_active_color(Some((color.clone(), show_on_recordings)));

    let _ = apply_gamma_ramp(color.gamma);

    let has_matrix_effect = !color.active_filter.is_empty()
        || (color.saturation - 100.0).abs() >= 0.1
        || color.hue.abs() >= 0.1
        || (color.contrast - 100.0).abs() >= 0.1
        || color.black_holo > 0.0
        || (color.gamma - 100.0).abs() >= 0.1;

    if !has_matrix_effect {
        let _guard = MAG_MUTEX.lock().ok();
        unsafe {
            ensure_mag_initialized();
            let effect = crate::platform::ffi::MagColorEffect {
                transform: identity_matrix(),
            };
            let _ = crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect);
        }
    } else {
        let _ = apply_magnification_color(color, show_on_recordings);
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn apply_color_transform(_color: &ColorSettings, _show_on_recordings: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn reset_color_transform() -> Result<(), String> {
    set_active_color(None);
    let effect = crate::platform::ffi::MagColorEffect {
        transform: identity_matrix(),
    };

    {
        let _guard = MAG_MUTEX.lock().ok();
        unsafe {
            ensure_mag_initialized();
            let _ = crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect);
        }
    }
    if !super::black_holo::is_black_holo_active() {
        if let Some(orig) = ORIGINAL_GAMMA_RAMP.get() {
            unsafe {
                let hdc = crate::platform::ffi::winapi::GetDC(std::ptr::null_mut());
                if !hdc.is_null() {
                    let _ = crate::platform::ffi::winapi::SetDeviceGammaRamp(hdc, orig);
                    let _ = crate::platform::ffi::winapi::ReleaseDC(std::ptr::null_mut(), hdc);
                }
            }
        } else {
            let _ = apply_gamma_ramp(100.0);
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn reset_color_transform() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_magnification_color(color: &ColorSettings, include_matrix_gamma: bool) -> Result<(), String> {
    let matrix = build_color_matrix(color, include_matrix_gamma);
    let effect = crate::platform::ffi::MagColorEffect { transform: matrix };

    let _guard = MAG_MUTEX.lock().ok();
    ensure_mag_initialized();
    unsafe {
        if crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect) == 0 {
            let delays = [40, 120, 250];
            for ms in delays {
                std::thread::sleep(std::time::Duration::from_millis(ms));
                let _ = crate::platform::ffi::winapi::MagInitialize();
                MAG_INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
                if crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect) != 0 {
                    break;
                }
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_gamma_ramp(gamma_percent: f32) -> Result<(), String> {
    ensure_original_gamma_captured();
    let gamma = (gamma_percent / 100.0).clamp(0.5, 1.5);
    let exponent = 1.0 / gamma;
    let mut ramp = crate::platform::ffi::GammaRamp::default();

    for index in 0..256 {
        let normalized = index as f32 / 255.0;
        let corrected = normalized.powf(exponent);

        let val = (corrected * 65535.0).round() as u16;
        ramp.red[index] = val;
        ramp.green[index] = val;
        ramp.blue[index] = val;
    }

    unsafe {
        let hdc = crate::platform::ffi::winapi::GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            return Err("Failed to acquire display device context".to_string());
        }

        let result = crate::platform::ffi::winapi::SetDeviceGammaRamp(hdc, &ramp);
        let _ = crate::platform::ffi::winapi::ReleaseDC(std::ptr::null_mut(), hdc);

        if result == 0 {
            return Err("Display driver rejected gamma ramp".to_string());
        }
    }

    Ok(())
}

pub(crate) fn gamma_fallback_matrix(gamma_percent: f32) -> [f32; 25] {
    let gamma = (gamma_percent / 100.0).clamp(0.5, 1.5);
    if (gamma - 1.0).abs() < 0.005 {
        return identity_matrix();
    }
    let lift = (gamma - 1.0) * 0.35;
    let gain = (1.0 + lift).clamp(0.7, 1.25);
    let mut m = identity_matrix();
    m[0] = gain;
    m[6] = gain;
    m[12] = gain;
    m
}

pub(crate) fn build_color_matrix(color: &ColorSettings, include_matrix_gamma: bool) -> [f32; 25] {
    let mut matrix = identity_matrix();

    let saturation = color.saturation / 100.0;
    let contrast = color.contrast / 100.0;

    match color.active_filter.as_str() {
        "rust_cold_tactical" => {
            matrix = multiply_matrix(matrix, saturation_matrix(saturation));
            matrix = multiply_matrix(matrix, channel_gain_matrix(0.92, 1.00, 1.08));
            matrix = multiply_matrix(matrix, contrast_matrix(contrast));
        }
        "rust_midnight_neon" => {
            matrix = multiply_matrix(matrix, saturation_matrix(saturation));
            matrix = multiply_matrix(matrix, channel_gain_matrix(0.88, 0.94, 1.25));
            matrix = multiply_matrix(matrix, shadow_bias_matrix(-0.015, 0.00, 0.04));
            matrix = multiply_matrix(matrix, contrast_matrix(contrast));
        }
        "clear_sight" => {
            matrix = multiply_matrix(matrix, saturation_matrix(saturation));
            matrix = multiply_matrix(matrix, channel_gain_matrix(1.02, 1.04, 1.00));
            matrix = multiply_matrix(matrix, contrast_matrix(contrast));
        }
        _ => {
            matrix = multiply_matrix(matrix, saturation_matrix(saturation));
            matrix = multiply_matrix(matrix, contrast_matrix(contrast));
        }
    }

    if color.hue.abs() >= 0.1 {
        matrix = multiply_matrix(matrix, hue_matrix(color.hue.to_radians()));
    }
    if color.black_holo > 0.0 {
        matrix = multiply_matrix(matrix, black_holo_matrix(color.black_holo / 100.0));
    }
    if include_matrix_gamma && (color.gamma - 100.0).abs() >= 0.1 {
        matrix = multiply_matrix(matrix, gamma_fallback_matrix(color.gamma));
    }
    matrix
}

fn channel_gain_matrix(r: f32, g: f32, b: f32) -> [f32; 25] {
    let mut m = identity_matrix();
    m[0] = r;
    m[6] = g;
    m[12] = b;
    m
}

fn shadow_bias_matrix(r_off: f32, g_off: f32, b_off: f32) -> [f32; 25] {
    let mut m = identity_matrix();
    m[20] = r_off;
    m[21] = g_off;
    m[22] = b_off;
    m
}

fn black_holo_matrix(strength: f32) -> [f32; 25] {
    let k = strength.clamp(0.0, 1.0);
    let r_to_g = 0.5 * k;
    let g_to_g = 1.0 - k;
    let b_to_g = 0.5 * k;

    [
        1.0,    r_to_g, 0.0, 0.0, 0.0,
        0.0,    g_to_g, 0.0, 0.0, 0.0,
        0.0,    b_to_g, 1.0, 0.0, 0.0,
        0.0,    0.0,    0.0, 1.0, 0.0,
        0.0,    0.0,    0.0, 0.0, 1.0,
    ]
}

fn identity_matrix() -> [f32; 25] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn saturation_matrix(saturation: f32) -> [f32; 25] {
    let rw = 0.2126;
    let gw = 0.7152;
    let bw = 0.0722;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let id = identity_matrix();
        assert_eq!(id[0], 1.0);
        assert_eq!(id[6], 1.0);
        assert_eq!(id[12], 1.0);
        assert_eq!(id[18], 1.0);
        assert_eq!(id[24], 1.0);
        assert_eq!(id[1], 0.0);
    }

    #[test]
    fn test_multiply_identity() {
        let id = identity_matrix();
        let mult = multiply_matrix(id, id);
        assert_eq!(id, mult);
    }

    #[test]
    fn test_black_holo_matrix_bounds() {
        let m = black_holo_matrix(0.5);
        assert_eq!(m[0], 1.0);
        assert_eq!(m[24], 1.0);
    }
}
