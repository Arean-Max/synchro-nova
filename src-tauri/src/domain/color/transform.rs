use crate::app::state::ColorSettings;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

struct ColorGuardState {
    active: Option<(ColorSettings, bool)>,
    exiting: bool,
}

static COLOR_GUARD_SYNC: OnceLock<Arc<(Mutex<ColorGuardState>, Condvar)>> = OnceLock::new();

fn get_guard_sync() -> &'static Arc<(Mutex<ColorGuardState>, Condvar)> {
    COLOR_GUARD_SYNC.get_or_init(|| {
        Arc::new((
            Mutex::new(ColorGuardState {
                active: None,
                exiting: false,
            }),
            Condvar::new(),
        ))
    })
}

#[cfg(target_os = "windows")]
pub fn start_color_guard() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        let sync = Arc::clone(get_guard_sync());
        std::thread::Builder::new()
            .name("synchro-color-guard".to_string())
            .spawn(move || {
                let (lock, cvar) = &*sync;
                loop {
                    let mut state = match lock.lock() {
                        Ok(g) => g,
                        Err(e) => e.into_inner(),
                    };

                    if state.exiting {
                        break;
                    }

                    state = match cvar.wait(state) {
                        Ok(g) => g,
                        Err(e) => e.into_inner(),
                    };

                    if state.exiting {
                        break;
                    }
                }
            })
            .ok();
    });
}

#[cfg(not(target_os = "windows"))]
pub fn start_color_guard() {}

pub fn stop_color_guard() {
    let sync = get_guard_sync();
    if let Ok(mut lock) = sync.0.lock() {
        lock.exiting = true;
        sync.1.notify_all();
    }
}

fn set_active_color(color: Option<(ColorSettings, bool)>) {
    let sync = get_guard_sync();
    if let Ok(mut lock) = sync.0.lock() {
        lock.active = color;
        sync.1.notify_all();
    }
}

pub fn get_active_color() -> Option<ColorSettings> {
    let sync = get_guard_sync();
    if let Ok(lock) = sync.0.lock() {
        lock.active.as_ref().map(|(c, _)| c.clone())
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub fn apply_color_transform(color: &ColorSettings, show_on_recordings: bool) -> Result<(), String> {
    if !color.enabled {
        set_active_color(None);
        return reset_color_transform();
    }

    set_active_color(Some((color.clone(), show_on_recordings)));

    let ramp_success = apply_gamma_ramp(color.gamma).is_ok();
    let include_matrix_gamma = show_on_recordings || !ramp_success;

    let has_matrix_effect = (color.saturation - 100.0).abs() >= 0.1
        || color.hue.abs() >= 0.1
        || (color.contrast - 100.0).abs() >= 0.1
        || color.black_holo > 0.0
        || (include_matrix_gamma && (color.gamma - 100.0).abs() >= 0.1);

    if !has_matrix_effect {
        unsafe {
            let effect = crate::platform::ffi::MagColorEffect {
                transform: identity_matrix(),
            };
            let _ = crate::platform::ffi::winapi::MagInitialize();
            let _ = crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect);
            let _ = crate::platform::ffi::winapi::mag_uninitialize();
        }
    } else {
        apply_magnification_color(color, include_matrix_gamma)?;
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

    unsafe {
        let _ = crate::platform::ffi::winapi::MagInitialize();
        let _ = crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect);
        let _ = crate::platform::ffi::winapi::mag_uninitialize();
    }
    if !super::black_holo::is_black_holo_active() {
        let _ = apply_gamma_ramp(100.0);
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

    unsafe {
        let _ = crate::platform::ffi::winapi::MagInitialize();
        if crate::platform::ffi::winapi::MagSetFullscreenColorEffect(&effect) == 0 {
            return Err("Windows Magnification API color effect failed".to_string());
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_gamma_ramp(gamma_percent: f32) -> Result<(), String> {
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
    let gain = gamma.powf(0.85);
    let mut m = identity_matrix();
    m[0] = gain;
    m[6] = gain;
    m[12] = gain;
    m
}

pub(crate) fn build_color_matrix(color: &ColorSettings, include_matrix_gamma: bool) -> [f32; 25] {
    let saturation = color.saturation / 100.0;
    let contrast = color.contrast / 100.0;
    let hue = color.hue.to_radians();

    let mut matrix = identity_matrix();
    matrix = multiply_matrix(matrix, saturation_matrix(saturation));
    matrix = multiply_matrix(matrix, hue_matrix(hue));
    matrix = multiply_matrix(matrix, contrast_matrix(contrast));
    if color.black_holo > 0.0 {
        matrix = multiply_matrix(matrix, black_holo_matrix(color.black_holo / 100.0));
    }
    if include_matrix_gamma {
        matrix = multiply_matrix(matrix, gamma_fallback_matrix(color.gamma));
    }
    matrix
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
