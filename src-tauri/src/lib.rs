use std::sync::{atomic::Ordering, Arc};
use tauri::{Manager, WindowEvent};

pub mod admin;
pub mod app;
pub mod browser;
pub mod commands;
pub mod domain;
pub mod driver_info;
pub mod infra;
pub mod platform;
pub mod settings;
pub mod system_info;

// Backwards-compatible aliases for existing crate callers and tests
pub use app::state::{AppSettings, ColorSettings, DriverInfo, PersistedState, RuntimeState, SystemCharacteristics};
pub use app::workers::trim_process_memory;
pub use domain::color;
pub use domain::games;
pub use domain::tweaks;
pub use platform::ffi::{self, utf16z_to_string};

fn calculate_adaptive_window_size(screen_w: f64, screen_h: f64) -> (f64, f64) {
    let base_w: f64 = if screen_w <= 1300.0 || screen_h <= 740.0 {
        880.0
    } else if screen_w <= 1600.0 || screen_h <= 900.0 {
        940.0
    } else if screen_w >= 2400.0 && screen_h >= 1350.0 {
        1060.0
    } else {
        980.0
    };

    let base_h: f64 = (base_w / 1.58).round();
    let max_w: f64 = (screen_w * 0.86).floor();
    let max_h: f64 = (screen_h * 0.82).floor();

    let final_w = base_w.min(max_w).max(860.0);
    let final_h = base_h.min(max_h).max(540.0);

    (final_w, final_h)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            platform::ffi::apply_process_hardening();
            platform::ffi::init_process_tree_job();

            let runtime = RuntimeState::load(app.handle())?;
            let initial = runtime.snapshot()?;
            if initial.settings.auto_backup_on_start {
                let _ = commands::storage::create_backup_file(
                    &runtime.backups_dir,
                    format!("Startup backup {}", app::state::short_date(app::state::unix_now())),
                    initial.clone(),
                );
            }

            let trimmer_sync = Arc::clone(&runtime.trimmer_sync);
            app.manage(runtime);

            let _ = domain::color::apply_color_transform(&initial.color, initial.settings.show_on_recordings);
            domain::color::start_color_guard();
            infra::screenshot::start_global_screenshot_listener();
            let _ = settings::set_autostart(app.handle(), initial.settings.autostart_windows);
            app::tray::install_tray(app.handle())?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_resizable(true);
                let _ = window.set_maximizable(false);

                let (target_w, target_h) = if let Some(monitor) = window
                    .current_monitor()
                    .ok()
                    .flatten()
                    .or_else(|| window.primary_monitor().ok().flatten())
                {
                    let scale_factor = monitor.scale_factor();
                    let physical_size = monitor.size();
                    let screen_w = physical_size.width as f64 / scale_factor;
                    let screen_h = physical_size.height as f64 / scale_factor;

                    calculate_adaptive_window_size(screen_w, screen_h)
                } else {
                    (980.0, 620.0)
                };

                let target_size = tauri::LogicalSize::new(target_w, target_h);
                let _ = window.set_size(target_size);
                let _ = window.set_shadow(false);

                #[cfg(target_os = "windows")]
                if let Ok(hwnd) = window.hwnd() {
                    platform::ffi::eliminate_window_borders(hwnd.0 as isize);
                }

                let _ = window.center();
                if initial.settings.start_minimized {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            trim_process_memory();
            app::workers::spawn_memory_trimmer(trimmer_sync);

            #[cfg(target_os = "windows")]
            domain::color::black_holo::start_black_holo_hotkey_listener(Some(app.handle().clone()));

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            match event {
                WindowEvent::Focused(false) => {
                    trim_process_memory();
                }
                WindowEvent::CloseRequested { api, .. } => {
                    let app = window.app_handle();
                    let state = app.state::<RuntimeState>();
                    if state.exiting.load(Ordering::SeqCst) {
                        return;
                    }

                    if let Ok(snapshot) = state.snapshot() {
                        if snapshot.settings.close_to_tray {
                            api.prevent_close();
                            let _ = window.hide();
                            trim_process_memory();
                        } else {
                            state.signal_shutdown();
                        }
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_state,
            commands::apply_color_settings,
            commands::update_app_settings,
            commands::get_system_characteristics,
            commands::get_system_live_metrics,
            commands::open_driver_search,
            commands::list_installed_games,
            commands::launch_installed_game,
            commands::apply_tweaks,
            commands::get_tweak_statuses,
            commands::list_backups,
            commands::create_backup,
            commands::restore_backup,
            commands::delete_backup,
            commands::list_configs,
            commands::save_config,
            commands::load_config,
            commands::apply_config,
            commands::delete_config,
            commands::open_storage_folder,
            commands::minimize_window,
            commands::toggle_window_maximize,
            commands::start_window_drag,
            commands::close_window,
            commands::exit_app,
            commands::restart_as_admin,
            commands::trim_memory,
            commands::rollback_last_tweaks,
            commands::detect_installed_apps,
            commands::restart_explorer,
            commands::restart_graphics_driver,
            commands::set_rust_black_holo,
            commands::get_black_holo_status,
            commands::toggle_hardware_black_holo,
            commands::take_juicy_screenshot,
            commands::scan_drivers,
            commands::open_official_driver_url,
            commands::open_external_url,
            commands::get_pending_navigation,
            commands::check_for_updates,
            commands::download_update,
            commands::get_update_progress,
            commands::install_update,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Synchro");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_sanitization() {
        let state = PersistedState {
            color: ColorSettings {
                saturation: 999.0,
                hue: -999.0,
                contrast: -50.0,
                gamma: 300.0,
                black_holo: -20.0,
                enabled: true,
            },
            settings: AppSettings {
                language: "unknown".to_string(),
                accent_color: "   ".to_string(),
                ..Default::default()
            },
            is_admin: false,
        }
        .sanitized();

        assert_eq!(state.color.saturation, 200.0);
        assert_eq!(state.color.hue, -180.0);
        assert_eq!(state.color.contrast, 50.0);
        assert_eq!(state.color.gamma, 150.0);
        assert_eq!(state.color.black_holo, 0.0);
        assert_eq!(state.settings.language, "en");
        assert_eq!(state.settings.accent_color, "#ffffff");
    }

    #[test]
    fn test_safe_file_stem() {
        assert_eq!(app::state::safe_file_stem("My Awesome Config!"), "my-awesome-config");
        assert_eq!(app::state::safe_file_stem("..."), "item");
        assert_eq!(app::state::safe_file_stem("CON"), "item");
        assert_eq!(app::state::safe_file_stem("NUL"), "item");
    }

    #[test]
    fn test_adaptive_window_size() {
        let (w, h) = calculate_adaptive_window_size(1920.0, 1080.0);
        assert!(w >= 860.0 && w <= 1920.0);
        assert!(h >= 540.0 && h <= 1080.0);
    }

    #[test]
    fn test_url_validation() {
        assert!(commands::system::validate_external_url("https://nvidia.com").is_ok());
        assert!(commands::system::validate_external_url("http://insecure.com").is_err());
        assert!(commands::system::validate_external_url("https://bad site.com").is_err());
        assert!(commands::system::validate_external_url("https://test.com/path?arg=1&evil=true").is_err());
    }

    #[test]
    fn test_updater_version_comparison() {
        use domain::updater::client::is_newer_version;
        assert!(is_newer_version("v2.2.3", "2.2.2"));
        assert!(is_newer_version("v3.0.0", "2.2.2"));
        assert!(is_newer_version("2.3.0", "2.2.2"));
        assert!(!is_newer_version("v2.2.2", "2.2.2"));
        assert!(!is_newer_version("v2.2.1", "2.2.2"));
        assert!(!is_newer_version("v1.9.9", "2.2.2"));
    }

    #[test]
    fn test_tweak_catalog_integrity() {
        let ids = domain::tweaks::known_tweak_ids();
        assert!(!ids.is_empty());
        for id in ids {
            assert!(!id.trim().is_empty());
        }
    }

    #[test]
    fn test_color_matrix_generation() {
        let default_color = ColorSettings::default();
        let matrix = domain::color::transform::build_color_matrix(&default_color, true);
        assert_eq!(matrix.len(), 25);
        // All values should be finite numbers, not NaN or Infinity
        for val in &matrix {
            assert!(val.is_finite());
        }
        // Alpha channel (element 18, 4th diagonal element) should be 1.0
        assert_eq!(matrix[18], 1.0);
    }

    #[test]
    fn test_rust_holosight_colour_parser() {
        let cfg1 = "fps.limit 144\r\naccessibility.holosightcolour \"2\"\r\ngraphics.fov 90";
        assert_eq!(domain::games::rust::parse_holosight_colour(cfg1), Some("2".to_string()));

        let cfg2 = "accessibility.holosightcolour 1\r\ngraphics.fov 90";
        assert_eq!(domain::games::rust::parse_holosight_colour(cfg2), Some("1".to_string()));

        let cfg3 = "ACCESSIBILITY.HOLOSIGHTCOLOUR \"0\"\r\n";
        assert_eq!(domain::games::rust::parse_holosight_colour(cfg3), Some("0".to_string()));

        let cfg4 = "graphics.fov 90\r\ninput.sensitivity 0.5";
        assert_eq!(domain::games::rust::parse_holosight_colour(cfg4), None);
    }

    #[test]
    fn test_safe_registry_nonexistent_key_error() {
        let result = infra::registry::SafeRegistry::get_dword(
            infra::registry::RootKey::Hkcu,
            "Software\\SynchroDefinitelyNonExistentKey9872",
            "NonExistentValue",
        );
        assert!(result.is_err(), "Expected error when reading non-existent registry key");
    }

    #[test]
    fn test_update_url_validation() {
        use commands::updater::validate_update_url;
        assert!(validate_update_url("https://github.com/Arean-Max/synchro-nova/releases/download/v2.2.4/synchro.exe").is_ok());
        assert!(validate_update_url("https://objects.githubusercontent.com/production/synchro.exe").is_ok());
        assert!(validate_update_url("https://evil-site.com/synchro.exe").is_err());
        assert!(validate_update_url("http://github.com/Arean-Max/synchro-nova/releases/download/v2.2.4/synchro.exe").is_err());
        assert!(validate_update_url("https://github.com/attacker/repo/releases/download/bad.exe").is_err());
    }

    #[test]
    fn test_installer_nonexistent_file() {
        let temp_dir = domain::updater::client::get_update_dir();
        let target_file = temp_dir.join("synchro_latest.exe");
        let _ = std::fs::remove_file(&target_file);
        let res = domain::updater::installer::apply_install();
        assert!(res.is_err(), "Expected apply_install to fail when update file does not exist");
    }
}
