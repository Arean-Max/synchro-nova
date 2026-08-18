use crate::ColorSettings;
use std::{ffi::c_void, sync::OnceLock};

#[cfg(target_os = "windows")]
pub(crate) fn apply_color_transform(color: &ColorSettings) -> Result<(), String> {
    if !color.enabled {
        return Ok(());
    }

    apply_magnification_color(color)?;
    apply_gamma_ramp(color.gamma)?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn apply_color_transform(_color: &ColorSettings) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_magnification_color(color: &ColorSettings) -> Result<(), String> {
    #[repr(C)]
    struct MagColorEffect {
        transform: [f32; 25],
    }

    #[link(name = "Magnification")]
    unsafe extern "system" {
        fn MagInitialize() -> i32;
        fn MagSetFullscreenColorEffect(effect: *const MagColorEffect) -> i32;
    }

    let matrix = build_color_matrix(color);
    let effect = MagColorEffect { transform: matrix };

    static MAGNIFICATION_READY: OnceLock<Result<(), String>> = OnceLock::new();

    MAGNIFICATION_READY
        .get_or_init(|| unsafe {
            if MagInitialize() == 0 {
                Err("Windows Magnification API initialization failed".to_string())
            } else {
                Ok(())
            }
        })
        .clone()?;

    unsafe {
        if MagSetFullscreenColorEffect(&effect) == 0 {
            return Err("Windows Magnification API color effect failed".to_string());
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_gamma_ramp(gamma_percent: f32) -> Result<(), String> {
    #[repr(C)]
    struct GammaRamp {
        red: [u16; 256],
        green: [u16; 256],
        blue: [u16; 256],
    }

    #[link(name = "User32")]
    unsafe extern "system" {
        fn GetDC(hwnd: *mut c_void) -> *mut c_void;
        fn ReleaseDC(hwnd: *mut c_void, hdc: *mut c_void) -> i32;
    }

    #[link(name = "Gdi32")]
    unsafe extern "system" {
        fn SetDeviceGammaRamp(hdc: *mut c_void, ramp: *const GammaRamp) -> i32;
    }

    let gamma = (gamma_percent / 100.0).clamp(0.5, 1.5);
    let exponent = 1.0 / gamma;
    let mut ramp = GammaRamp {
        red: [0; 256],
        green: [0; 256],
        blue: [0; 256],
    };

    for index in 0..256 {
        let normalized = index as f32 / 255.0;
        let corrected = (normalized.powf(exponent) * 65535.0).round() as u16;
        ramp.red[index] = corrected;
        ramp.green[index] = corrected;
        ramp.blue[index] = corrected;
    }

    unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            return Err("Failed to acquire display device context".to_string());
        }

        let result = SetDeviceGammaRamp(hdc, &ramp);
        let _ = ReleaseDC(std::ptr::null_mut(), hdc);

        if result == 0 {
            return Err("Display driver rejected gamma ramp".to_string());
        }
    }

    Ok(())
}

fn build_color_matrix(color: &ColorSettings) -> [f32; 25] {
    let saturation = color.saturation / 100.0;
    let contrast = color.contrast / 100.0;
    let hue = color.hue.to_radians();

    let mut matrix = identity_matrix();
    matrix = multiply_matrix(matrix, saturation_matrix(saturation));
    matrix = multiply_matrix(matrix, hue_matrix(hue));
    matrix = multiply_matrix(matrix, contrast_matrix(contrast));
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
