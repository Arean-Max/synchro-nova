use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use crate::app::state::ColorSettings;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JuicyScreenshotResult {
    pub success: bool,
    pub message: String,
    pub file_path: Option<String>,
}

#[cfg(target_os = "windows")]
use std::ffi::c_void;

#[cfg(target_os = "windows")]
#[link(name = "User32")]
unsafe extern "system" {
    fn GetDC(hwnd: *mut c_void) -> *mut c_void;
    fn ReleaseDC(hwnd: *mut c_void, hdc: *mut c_void) -> i32;
    fn GetSystemMetrics(n_index: i32) -> i32;
    fn OpenClipboard(hwnd: *mut c_void) -> i32;
    fn CloseClipboard() -> i32;
    fn EmptyClipboard() -> i32;
    fn SetClipboardData(format: u32, handle: *mut c_void) -> *mut c_void;
    fn GetForegroundWindow() -> *mut c_void;
    fn GetCursorPos(point: *mut POINT) -> i32;
    fn MonitorFromWindow(hwnd: *mut c_void, flags: u32) -> *mut c_void;
    fn MonitorFromPoint(point: POINT, flags: u32) -> *mut c_void;
    fn GetMonitorInfoW(hmonitor: *mut c_void, info: *mut MONITORINFO) -> i32;
    fn RegisterHotKey(hwnd: *mut c_void, id: i32, fs_modifiers: u32, vk: u32) -> i32;
    fn UnregisterHotKey(hwnd: *mut c_void, id: i32) -> i32;
    fn GetMessageW(msg: *mut MSG, hwnd: *mut c_void, msg_filter_min: u32, msg_filter_max: u32) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "Gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(hdc: *mut c_void) -> *mut c_void;
    fn CreateCompatibleBitmap(hdc: *mut c_void, cx: i32, cy: i32) -> *mut c_void;
    fn SelectObject(hdc: *mut c_void, hgdiobj: *mut c_void) -> *mut c_void;
    fn BitBlt(
        hdc_dest: *mut c_void,
        x_dest: i32,
        y_dest: i32,
        w: i32,
        h: i32,
        hdc_src: *mut c_void,
        x_src: i32,
        y_src: i32,
        rop: u32,
    ) -> i32;
    fn DeleteDC(hdc: *mut c_void) -> i32;
    fn DeleteObject(ho: *mut c_void) -> i32;
    fn GetDIBits(
        hdc: *mut c_void,
        hbm: *mut c_void,
        start: u32,
        lines: u32,
        bits: *mut c_void,
        bmi: *mut BITMAPINFO,
        usage: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "Kernel32")]
unsafe extern "system" {
    fn GlobalAlloc(flags: u32, bytes: usize) -> *mut c_void;
    fn GlobalLock(handle: *mut c_void) -> *mut c_void;
    fn GlobalUnlock(handle: *mut c_void) -> i32;
    fn GlobalFree(handle: *mut c_void) -> *mut c_void;
}

const SM_CXSCREEN: i32 = 0;
const SM_CYSCREEN: i32 = 1;
const MONITOR_DEFAULTTONEAREST: u32 = 2;
const SRCCOPY: u32 = 0x00CC_0020;
const DIB_RGB_COLORS: u32 = 0;
const CF_DIB: u32 = 8;
const GMEM_MOVEABLE: u32 = 0x0002;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
struct MONITORINFO {
    pub cb_size: u32,
    pub rc_monitor: RECT,
    pub rc_work: RECT,
    pub dw_flags: u32,
}

#[repr(C)]
struct MSG {
    pub hwnd: *mut c_void,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: POINT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct BITMAPINFOHEADER {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

#[repr(C)]
struct BITMAPINFO {
    bmi_header: BITMAPINFOHEADER,
    bmi_colors: [u32; 1],
}

struct ReleaseDcGuard {
    hwnd: *mut c_void,
    hdc: *mut c_void,
}

impl Drop for ReleaseDcGuard {
    fn drop(&mut self) {
        if !self.hdc.is_null() {
            unsafe {
                ReleaseDC(self.hwnd, self.hdc);
            }
        }
    }
}

struct DeleteDcGuard(*mut c_void);
impl Drop for DeleteDcGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                DeleteDC(self.0);
            }
        }
    }
}

struct DeleteObjectGuard(*mut c_void);
impl Drop for DeleteObjectGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub fn capture_juicy_screenshot(color: &ColorSettings) -> Result<JuicyScreenshotResult, String> {
    unsafe {
        let mut x_src = 0;
        let mut y_src = 0;
        let mut width = 0;
        let mut height = 0;

        let fg_hwnd = GetForegroundWindow();
        let hmon = if !fg_hwnd.is_null() {
            MonitorFromWindow(fg_hwnd, MONITOR_DEFAULTTONEAREST)
        } else {
            std::ptr::null_mut()
        };

        let hmon = if !hmon.is_null() {
            hmon
        } else {
            let mut pt = POINT::default();
            GetCursorPos(&mut pt);
            MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST)
        };

        if !hmon.is_null() {
            let mut mi = MONITORINFO {
                cb_size: std::mem::size_of::<MONITORINFO>() as u32,
                rc_monitor: RECT::default(),
                rc_work: RECT::default(),
                dw_flags: 0,
            };
            if GetMonitorInfoW(hmon, &mut mi) != 0 {
                let mon_w = mi.rc_monitor.right - mi.rc_monitor.left;
                let mon_h = mi.rc_monitor.bottom - mi.rc_monitor.top;
                if mon_w > 0 && mon_h > 0 {
                    x_src = mi.rc_monitor.left;
                    y_src = mi.rc_monitor.top;
                    width = mon_w;
                    height = mon_h;
                }
            }
        }

        if width <= 0 || height <= 0 {
            width = GetSystemMetrics(SM_CXSCREEN);
            height = GetSystemMetrics(SM_CYSCREEN);
            x_src = 0;
            y_src = 0;
        }

        if width <= 0 || height <= 0 {
            return Err("Invalid screen metrics".to_string());
        }

        let hdc_screen = GetDC(std::ptr::null_mut());
        if hdc_screen.is_null() {
            return Err("Failed to get screen DC".to_string());
        }
        let _screen_guard = ReleaseDcGuard {
            hwnd: std::ptr::null_mut(),
            hdc: hdc_screen,
        };

        let hdc_mem = CreateCompatibleDC(hdc_screen);
        if hdc_mem.is_null() {
            return Err("Failed to create memory DC".to_string());
        }
        let _dc_guard = DeleteDcGuard(hdc_mem);

        let hbm = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbm.is_null() {
            return Err("Failed to create compatible bitmap".to_string());
        }
        let _hbm_guard = DeleteObjectGuard(hbm);

        let old_hbm = SelectObject(hdc_mem, hbm);
        let blt_res = BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, x_src, y_src, SRCCOPY);
        SelectObject(hdc_mem, old_hbm);

        if blt_res == 0 {
            return Err("BitBlt screen capture failed".to_string());
        }

        let header = BITMAPINFOHEADER {
            bi_size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            bi_width: width,
            bi_height: height,
            bi_planes: 1,
            bi_bit_count: 32,
            bi_compression: 0,
            bi_size_image: (width * height * 4) as u32,
            bi_x_pels_per_meter: 3780,
            bi_y_pels_per_meter: 3780,
            bi_clr_used: 0,
            bi_clr_important: 0,
        };

        let mut bmi = BITMAPINFO {
            bmi_header: header,
            bmi_colors: [0; 1],
        };

        let num_pixels = (width * height) as usize;
        let mut pixels: Vec<u8> = vec![0; num_pixels * 4];

        let get_dib_res = GetDIBits(
            hdc_mem,
            hbm,
            0,
            height as u32,
            pixels.as_mut_ptr() as *mut c_void,
            &mut bmi,
            DIB_RGB_COLORS,
        );

        if get_dib_res == 0 {
            return Err("Failed to retrieve screen bitmap bits".to_string());
        }

        // Apply color calibration transform in-place
        let saturation = (color.saturation / 100.0).max(0.0);
        let contrast = (color.contrast / 100.0).max(0.0);
        let gamma = (color.gamma / 100.0).clamp(0.5, 2.0);
        let gamma_exp = 1.0 / gamma;
        let black_holo = color.black_holo > 0.0;

        for chunk in pixels.chunks_exact_mut(4) {
            let mut b = chunk[0] as f32 / 255.0;
            let mut g = chunk[1] as f32 / 255.0;
            let mut r = chunk[2] as f32 / 255.0;

            if black_holo && g > 0.35 && g > r * 1.30 && g > b * 1.30 {
                let green_ratio = (g - r.max(b)) / g;
                let factor = (1.0 - green_ratio * 0.95).max(0.04);
                r *= factor;
                g *= factor;
                b *= factor;
            }

            if (saturation - 1.0).abs() > 0.005 {
                let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                r = lum + (r - lum) * saturation;
                g = lum + (g - lum) * saturation;
                b = lum + (b - lum) * saturation;
            }

            if (contrast - 1.0).abs() > 0.005 {
                r = (r - 0.5) * contrast + 0.5;
                g = (g - 0.5) * contrast + 0.5;
                b = (b - 0.5) * contrast + 0.5;
            }

            if (gamma - 1.0).abs() > 0.005 {
                r = r.max(0.0).powf(gamma_exp);
                g = g.max(0.0).powf(gamma_exp);
                b = b.max(0.0).powf(gamma_exp);
            }

            chunk[0] = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
            chunk[1] = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
            chunk[2] = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
        }

        let header_size = std::mem::size_of::<BITMAPINFOHEADER>();
        let total_size = header_size + pixels.len();

        // 1. Copy to Windows Clipboard (CF_DIB)
        let h_global = GlobalAlloc(GMEM_MOVEABLE, total_size);
        if !h_global.is_null() {
            let p_mem = GlobalLock(h_global) as *mut u8;
            if !p_mem.is_null() {
                std::ptr::copy_nonoverlapping(&header as *const _ as *const u8, p_mem, header_size);
                std::ptr::copy_nonoverlapping(pixels.as_ptr(), p_mem.add(header_size), pixels.len());
                GlobalUnlock(h_global);

                let mut clipboard_success = false;
                if OpenClipboard(std::ptr::null_mut()) != 0 {
                    EmptyClipboard();
                    let set_res = SetClipboardData(CF_DIB, h_global);
                    if !set_res.is_null() {
                        clipboard_success = true;
                    }
                    CloseClipboard();
                }

                if !clipboard_success {
                    GlobalFree(h_global);
                }
            } else {
                GlobalFree(h_global);
            }
        }

        // 2. Stream directly to file without allocating a secondary buffer (Zero-Duplicate Memory Optimization)
        let mut saved_path_str = None;
        if let Some(user_profile) = std::env::var_os("USERPROFILE") {
            let screenshots_dir = PathBuf::from(user_profile).join("Pictures").join("Screenshots");
            if let Err(e) = fs::create_dir_all(&screenshots_dir) {
                eprintln!("[Screenshot] Failed to create screenshot directory: {}", e);
            } else {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let target_file = screenshots_dir.join(format!("Synchro_{}.bmp", timestamp));

                if let Ok(file) = File::create(&target_file) {
                    let mut writer = BufWriter::with_capacity(64 * 1024, file);
                    let mut write_success = true;

                    // BMP 14-byte File Header
                    let file_size = (14 + total_size) as u32;
                    let offset = (14 + header_size) as u32;
                    let mut bmp_file_header = [0u8; 14];
                    bmp_file_header[0] = b'B';
                    bmp_file_header[1] = b'M';
                    bmp_file_header[2..6].copy_from_slice(&file_size.to_le_bytes());
                    bmp_file_header[10..14].copy_from_slice(&offset.to_le_bytes());

                    if writer.write_all(&bmp_file_header).is_err() {
                        write_success = false;
                    }

                    // DIB Header
                    let header_bytes = std::slice::from_raw_parts(&header as *const _ as *const u8, header_size);
                    if write_success && writer.write_all(header_bytes).is_err() {
                        write_success = false;
                    }

                    // Pixel bits directly from `pixels`
                    if write_success && writer.write_all(&pixels).is_err() {
                        write_success = false;
                    }

                    if write_success && writer.flush().is_ok() {
                        saved_path_str = Some(target_file.to_string_lossy().to_string());
                    }
                }
            }
        }

        // Explicitly drop large pixel buffer immediately
        drop(pixels);

        Ok(JuicyScreenshotResult {
            success: true,
            message: "Juicy screenshot copied to clipboard (Ctrl+V)!".to_string(),
            file_path: saved_path_str,
        })
    }
}

#[cfg(not(target_os = "windows"))]
pub fn capture_juicy_screenshot(_color: &ColorSettings) -> Result<JuicyScreenshotResult, String> {
    Err("Screenshots only supported on Windows".to_string())
}

#[cfg(target_os = "windows")]
pub fn start_global_screenshot_listener() {
    static LISTENER_STARTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    LISTENER_STARTED.get_or_init(|| {
        std::thread::Builder::new()
            .name("synchro-screenshot-hotkey".to_string())
            .spawn(move || {
                const HOTKEY_ID: i32 = 0x534E; // 'SN'
                const VK_SNAPSHOT: u32 = 0x2C;
                const WM_HOTKEY: u32 = 0x0312;

                let registered = unsafe {
                    RegisterHotKey(std::ptr::null_mut(), HOTKEY_ID, 0, VK_SNAPSHOT)
                };

                if registered != 0 {
                    let mut msg = unsafe { std::mem::zeroed::<MSG>() };
                    while unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) } > 0 {
                        if msg.message == WM_HOTKEY {
                            if let Some(color) = crate::domain::color::get_active_color() {
                                if color.enabled {
                                    let _ = capture_juicy_screenshot(&color);
                                }
                            }
                        }
                    }
                    unsafe { UnregisterHotKey(std::ptr::null_mut(), HOTKEY_ID) };
                } else {
                    let mut was_pressed = false;
                    loop {
                        std::thread::sleep(std::time::Duration::from_millis(60));
                        let state = unsafe { crate::platform::ffi::winapi::GetAsyncKeyState(VK_SNAPSHOT as i32) };
                        let is_pressed = (state as u16 & 0x8000) != 0;
                        if is_pressed && !was_pressed {
                            was_pressed = true;
                            if let Some(color) = crate::domain::color::get_active_color() {
                                if color.enabled {
                                    let _ = capture_juicy_screenshot(&color);
                                }
                            }
                        } else if !is_pressed {
                            was_pressed = false;
                        }
                    }
                }
            })
            .ok();
    });
}

#[cfg(not(target_os = "windows"))]
pub fn start_global_screenshot_listener() {}


