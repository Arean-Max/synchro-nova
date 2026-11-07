pub mod black_holo;
pub mod transform;

pub use black_holo::{
    get_hardware_black_holo_status, is_black_holo_active, set_hardware_black_holo,
    stop_ramp_watchdog, BlackHoloStatus, GpuVendor,
};
pub use transform::{
    apply_color_transform, get_active_color, reapply_active_color, reset_color_transform,
    start_color_guard, stop_color_guard,
};

