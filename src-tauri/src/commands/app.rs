use tauri::{AppHandle, State, Window};

use crate::admin::restart_as_admin as restart_current_process_as_admin;
use crate::app::state::{AppSettings, PersistedState, RuntimeState};
use crate::app::workers::trim_process_memory;
use crate::settings::set_autostart;

#[tauri::command]
pub fn get_app_state(state: State<'_, RuntimeState>) -> Result<PersistedState, String> {
    state.snapshot()
}

#[tauri::command]
pub fn update_app_settings(
    settings: AppSettings,
    state: State<'_, RuntimeState>,
    app: AppHandle,
) -> Result<PersistedState, String> {
    let settings = settings.sanitized();
    let current = state.snapshot()?;

    if current.settings.autostart_windows != settings.autostart_windows {
        let _ = set_autostart(&app, settings.autostart_windows);
    }

    if current.settings.show_on_recordings != settings.show_on_recordings {
        let _ = crate::domain::color::apply_color_transform(&current.color, settings.show_on_recordings);
    }

    let mut data = state
        .data
        .lock()
        .map_err(|_| "Settings lock poisoned".to_string())?;
    data.settings = settings;
    let snapshot = data.clone();
    state.save(&snapshot)?;

    Ok(snapshot)
}

#[tauri::command]
pub fn minimize_window(window: Window) {
    let _ = window.minimize();
}

#[tauri::command]
pub fn toggle_window_maximize(window: Window) {
    if let Ok(maximized) = window.is_maximized() {
        if maximized {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
pub fn start_window_drag(window: Window) {
    let _ = window.start_dragging();
}

#[tauri::command]
pub fn close_window(window: Window) {
    let _ = window.close();
}

#[tauri::command]
pub fn exit_app(app: AppHandle, state: State<'_, RuntimeState>) {
    state.signal_shutdown();
    app.exit(0);
}

#[tauri::command]
pub fn restart_as_admin(app: AppHandle, state: State<'_, RuntimeState>) -> Result<(), String> {
    state.signal_shutdown();
    let _ = app;
    restart_current_process_as_admin()
}

#[tauri::command]
pub fn trim_memory() {
    trim_process_memory();
}
