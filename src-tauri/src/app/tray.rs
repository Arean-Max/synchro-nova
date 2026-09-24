use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use super::state::RuntimeState;

pub fn install_tray(app: &AppHandle) -> tauri::Result<()> {
    let lang = app
        .try_state::<RuntimeState>()
        .and_then(|s| s.data.lock().ok().map(|d| d.settings.language.clone()))
        .unwrap_or_else(|| "en".to_string());
    let (show_text, exit_text) = if lang == "ru" {
        ("Открыть", "Выход")
    } else {
        ("Show", "Exit")
    };
    let show = MenuItem::with_id(app, "show", show_text, true, None::<&str>)?;
    let exit = MenuItem::with_id(app, "exit", exit_text, true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &exit])?;

    let mut tray = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_window(app),
            "exit" => {
                let state = app.state::<RuntimeState>();
                state.signal_shutdown();
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            }
            | TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } => {
                show_main_window(tray.app_handle());
            }
            _ => {}
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        tray = tray.icon(icon);
    }

    tray.build(app)?;
    Ok(())
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}
