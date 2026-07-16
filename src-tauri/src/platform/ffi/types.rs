
pub use std::ffi::c_void;

pub fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn utf16z_to_string(raw: &[u16]) -> String {
    let len = raw
        .iter()
        .position(|value| *value == 0)
        .unwrap_or(raw.len());
    String::from_utf16_lossy(&raw[..len]).trim().to_string()
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FileTime {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryStatusEx {
    pub dw_length: u32,
    pub dw_memory_load: u32,
    pub ull_total_phys: u64,
    pub ull_avail_phys: u64,
    pub ull_total_page_file: u64,
    pub ull_avail_page_file: u64,
    pub ull_total_virtual: u64,
    pub ull_avail_virtual: u64,
    pub ull_avail_extended_virtual: u64,
}

impl Default for MemoryStatusEx {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.dw_length = std::mem::size_of::<Self>() as u32;
        s
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayDeviceW {
    pub cb: u32,
    pub device_name: [u16; 32],
    pub device_string: [u16; 128],
    pub state_flags: u32,
    pub device_id: [u16; 128],
    pub device_key: [u16; 128],
}

impl Default for DisplayDeviceW {
    fn default() -> Self {
        let mut d: Self = unsafe { std::mem::zeroed() };
        d.cb = std::mem::size_of::<Self>() as u32;
        d
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PointL {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Msg {
    pub hwnd: *mut std::ffi::c_void,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: PointL,
}

pub type MSG = Msg;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DevModeW {
    pub dm_device_name: [u16; 32],
    pub dm_spec_version: u16,
    pub dm_driver_version: u16,
    pub dm_size: u16,
    pub dm_driver_extra: u16,
    pub dm_fields: u32,
    pub dm_position: PointL,
    pub dm_display_orientation: u32,
    pub dm_display_fixed_output: u32,
    pub dm_color: i16,
    pub dm_duplex: i16,
    pub dm_y_resolution: i16,
    pub dm_tt_option: i16,
    pub dm_collate: i16,
    pub dm_form_name: [u16; 32],
    pub dm_log_pixels: u16,
    pub dm_bits_per_pel: u32,
    pub dm_pels_width: u32,
    pub dm_pels_height: u32,
    pub dm_display_flags: u32,
    pub dm_display_frequency: u32,
    pub dm_icm_method: u32,
    pub dm_icm_intent: u32,
    pub dm_media_type: u32,
    pub dm_dither_type: u32,
    pub dm_reserved1: u32,
    pub dm_reserved2: u32,
    pub dm_panning_width: u32,
    pub dm_panning_height: u32,
}

impl Default for DevModeW {
    fn default() -> Self {
        let mut d: Self = unsafe { std::mem::zeroed() };
        d.dm_size = std::mem::size_of::<Self>() as u16;
        d
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MagColorEffect {
    pub transform: [f32; 25],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GammaRamp {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
