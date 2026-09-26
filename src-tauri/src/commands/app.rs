use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, Window};

use crate::app::state::{AppSettings, PersistedState, RuntimeState};
use crate::app::workers::trim_process_memory;
use crate::settings::set_autostart;

static PENDING_NAVIGATION: Mutex<Option<String>> = Mutex::new(None);

pub fn set_pending_navigation(page: String) {
    if let Ok(mut lock) = PENDING_NAVIGATION.lock() {
        *lock = Some(page);
    }
}

#[tauri::command]
pub fn get_pending_navigation() -> Option<String> {
    if let Ok(mut lock) = PENDING_NAVIGATION.lock() {
        lock.take()
    } else {
        None
    }
}

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
    let hwnd = app
        .get_webview_window("main")
        .and_then(|w| w.hwnd().ok())
        .map(|h| h.0 as isize)
        .unwrap_or(0);

    // Launch elevated process FIRST - if user cancels UAC prompt, current app stays open
    crate::admin::restart_as_admin_with_hwnd(hwnd, Some("tweaks"))?;

    // Elevated process is launched and running. Release mutex so it can acquire ownership.
    crate::ffi::release_single_instance();
    state.signal_shutdown();

    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }

    let app_handle = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(300));
        app_handle.exit(0);
        std::thread::sleep(std::time::Duration::from_millis(400));
        std::process::exit(0);
    });

    Ok(())
}

#[tauri::command]
pub fn trim_memory() {
    trim_process_memory();
}
