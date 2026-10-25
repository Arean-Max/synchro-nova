pub mod ffi;
pub mod fullscreen_watcher;
pub mod live_metrics;
pub mod security_guard;

pub use fullscreen_watcher::{get_fullscreen_state, set_user_summoned, FullscreenState};
pub use security_guard::{
    get_security_guard_status, verify_process_access_safety, verify_registry_write_safety,
    SecurityGuardStatus, SecurityIncident,
};
