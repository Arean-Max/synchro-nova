use tauri::State;

use crate::app::state::{ColorSettings, PersistedState, RuntimeState};
use crate::domain::color::apply_color_transform;
use crate::infra::screenshot::{capture_juicy_screenshot, JuicyScreenshotResult};

#[tauri::command]
pub fn apply_color_settings(
    color: ColorSettings,
    state: State<'_, RuntimeState>,
) -> Result<PersistedState, String> {
    let color = color.sanitized();
    let current = state.snapshot()?;
    apply_color_transform(&color, current.settings.show_on_recordings)?;

    {
        let mut data = state
            .data
            .lock()
            .map_err(|_| "Settings lock poisoned".to_string())?;
        data.color = color;
        data.is_admin = current.is_admin;
        let snapshot = data.clone();

        if snapshot.settings.save_color_correction {
            let path = state.settings_path.clone();
            let snap = snapshot.clone();
            std::thread::Builder::new()
                .name("synchro-async-save".to_string())
                .stack_size(64 * 1024)
                .spawn(move || {
                    let _ = crate::app::state::write_json_file(&path, &snap);
                    crate::infra::fast_cache::update_disk_cache(&snap.color, snap.settings.show_on_recordings, None);
                })
                .ok();
        }
    }

    state.snapshot()
}

#[tauri::command]
pub fn take_juicy_screenshot(
    state: State<'_, RuntimeState>,
) -> Result<JuicyScreenshotResult, String> {
    let current = state.snapshot()?;
    capture_juicy_screenshot(&current.color)
}

#[tauri::command]
pub fn get_black_holo_status() -> crate::domain::color::BlackHoloStatus {
    crate::domain::color::get_hardware_black_holo_status()
}

#[tauri::command]
pub fn toggle_hardware_black_holo(
    enabled: bool,
    state: State<'_, RuntimeState>,
) -> Result<crate::domain::color::BlackHoloStatus, String> {
    let status = crate::domain::color::set_hardware_black_holo(enabled);
    let (color_snapshot, show_on_recordings) = {
        let mut data = state
            .data
            .lock()
            .map_err(|_| "Settings lock poisoned".to_string())?;
        data.color.black_holo = if status.active { 100.0 } else { 0.0 };
        (data.color.clone(), data.settings.show_on_recordings)
    };
    let _ = crate::domain::color::apply_color_transform(&color_snapshot, show_on_recordings);
    Ok(status)
}
