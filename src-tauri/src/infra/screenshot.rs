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
