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
    if current.color != color {
        apply_color_transform(&color, current.settings.show_on_recordings)?;
    }

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    data.color = color;
    let snapshot = data.clone();

    if snapshot.settings.save_color_correction {
        state.save(&snapshot)?;
    }

    Ok(snapshot)
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
pub fn toggle_hardware_black_holo(enabled: bool) -> crate::domain::color::BlackHoloStatus {
    crate::domain::color::set_hardware_black_holo(enabled)
}
