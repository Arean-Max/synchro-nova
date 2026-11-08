use std::sync::{atomic::Ordering, Arc};
use tauri::{Manager, WindowEvent};

pub mod admin;
pub mod app;
pub mod browser;
pub mod commands;
pub mod domain;
pub mod driver_check;
pub mod driver_info;
pub mod infra;
pub mod platform;
pub mod settings;

pub use app::state::{AppSettings, ColorSettings, DriverInfo, PersistedState, RuntimeState};
pub use app::workers::trim_process_memory;
pub use domain::color;
pub use domain::tweaks;
pub use platform::ffi::{self, utf16z_to_string};

fn calculate_adaptive_window_size(screen_w: f64, screen_h: f64) -> (f64, f64) {
    let ideal_w = 1024.0;
    let ideal_h = 656.0;

    let max_w = (screen_w * 0.90).floor();
    let max_h = (screen_h * 0.88).floor();

    let scale = (max_w / ideal_w).min(max_h / ideal_h).min(1.0);
    let final_w = (ideal_w * scale).round().max(860.0);
    let final_h = (ideal_h * scale).round().max(540.0);

    (final_w, final_h)
}

pub fn suspend_window_webview(_window: &tauri::Window) {}
pub fn resume_window_webview(_window: &tauri::Window) {}

pub fn configure_main_window(window: &tauri::WebviewWindow) {
    let _ = window.set_resizable(false);
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
        (1024.0, 656.0)
    };

    let target_size = tauri::LogicalSize::new(target_w, target_h);
    let _ = window.set_size(target_size);
    let _ = window.set_min_size(Some(target_size));
    let _ = window.set_max_size(Some(target_size));
    let _ = window.set_shadow(false);

    if let Some(icon) = window.app_handle().default_window_icon().cloned() {
        let _ = window.set_icon(icon);
    }

    #[cfg(target_os = "windows")]
    if let Ok(hwnd) = window.hwnd() {
        platform::ffi::eliminate_window_borders(hwnd.0 as isize);
        platform::ffi::set_window_taskbar_icon(hwnd.0 as isize);
    }
    #[cfg(target_os = "windows")]
    let _ = window.with_webview(|webview| {
        let controller = webview.controller();
        unsafe {
            let _ = controller.SetIsVisible(true);
            let _ = controller.NotifyParentWindowPositionChanged();
        }
        if let Ok(core) = unsafe { controller.CoreWebView2() } {
            let mut pid = 0u32;
            if unsafe { core.BrowserProcessId(&mut pid) }.is_ok() && pid != 0 {
                platform::ffi::register_webview_pid(pid);
            }
        }
    });

    let _ = window.center();
}

pub fn hide_main_window(app: &tauri::AppHandle) {
    platform::fullscreen_watcher::set_user_summoned(false);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
        trim_process_memory();
    }
}

pub fn destroy_main_window(app: &tauri::AppHandle) {
    hide_main_window(app);
}

pub fn create_main_window(app: &tauri::AppHandle) -> tauri::Result<tauri::WebviewWindow> {
    for attempt in 0..10 {
        if let Some(existing) = app.get_webview_window("main") {
            configure_main_window(&existing);
            let _ = existing.unminimize();
            let _ = existing.show();
            let _ = existing.set_focus();
            return Ok(existing);
        }

        match tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
            .title("Synchro Nova")
            .inner_size(1024.0, 656.0)
            .resizable(false)
            .maximizable(false)
            .decorations(false)
            .shadow(false)
            .visible(false)
            .center()
            .on_navigation(|url| {
                let scheme = url.scheme();
                let host = url.host_str().unwrap_or("");
                let url_str = url.as_str();
                let is_local_origin = scheme == "tauri"
                    || scheme == "asset"
                    || host == "localhost"
                    || host == "127.0.0.1"
                    || host.ends_with(".localhost")
                    || url_str == "about:blank"
                    || url_str.contains("index.html");

                if is_local_origin {
                    true
                } else {
                    if scheme == "http" || scheme == "https" {
                        let _ = crate::platform::ffi::open_path_or_url(url_str);
                    }
                    false
                }
            })
            .build()
        {
            Ok(window) => {
                configure_main_window(&window);
                let _ = window.show();
                let _ = window.set_focus();
                return Ok(window);
            }
            Err(e) => {
                if attempt == 9 {
                    return Err(e);
                }
                std::thread::sleep(std::time::Duration::from_millis(30));
            }
        }
    }
    Err(tauri::Error::WindowNotFound)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            platform::ffi::apply_process_hardening();
            platform::ffi::init_process_tree_job();

            let runtime = RuntimeState::load(app.handle())?;
            let initial = runtime.snapshot()?;
            if initial.settings.start_minimized || initial.settings.disable_splash {
                runtime.splash_consumed.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            if initial.settings.auto_backup_on_start {
                let _ = commands::storage::create_backup_file(
                    &runtime.backups_dir,
                    format!("Startup backup {}", app::state::short_date(app::state::unix_now())),
                    initial.clone(),
                );
            }

            let trimmer_sync = Arc::clone(&runtime.trimmer_sync);
            let watcher_shutdown = Arc::clone(&runtime.exiting);
            app.manage(runtime);

            let _ = domain::color::apply_color_transform(&initial.color, initial.settings.show_on_recordings);
            infra::fast_cache::update_disk_cache(&initial.color, initial.settings.show_on_recordings, None);
            domain::color::start_color_guard();
            infra::screenshot::start_global_screenshot_listener();
            let _ = settings::set_autostart(app.handle(), initial.settings.autostart_windows);
            app::tray::install_tray(app.handle())?;

            let window = create_main_window(app.handle())?;
            if initial.settings.start_minimized {
                let _ = window.hide();
                trim_process_memory();
            }

            let eac_protect = initial.settings.eac_auto_protect;
            std::thread::Builder::new()
                .name("synchro-bg-integrity".to_string())
                .stack_size(64 * 1024)
                .spawn(move || {
                    #[cfg(target_os = "windows")]
                    {
                        domain::tweaks::gaming::clean_legacy_rust_ifeo();
                        domain::tweaks::system::clean_legacy_csrss_override();
                        if eac_protect {
                            let _ = platform::security_guard::fix_anticheat_integrity();
                        }
                    }
                })
                .ok();

            app::workers::spawn_memory_trimmer(trimmer_sync);

            #[cfg(target_os = "windows")]
            domain::hotkeys::start_global_hotkey_listener(app.handle().clone());
            domain::hotkeys::sync_hotkeys(&initial.keybinds);

            platform::fullscreen_watcher::spawn_fullscreen_watcher(app.handle().clone(), watcher_shutdown);
            platform::ffi::start_single_instance_listener(app.handle().clone());

            let auto_download = initial.settings.auto_update;
            std::thread::Builder::new()
                .name("synchro-auto-updater".to_string())
                .stack_size(64 * 1024)
                .spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    domain::updater::client::start_background_update_flow(auto_download);
                })
                .ok();

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            match event {
                WindowEvent::Focused(true) => {
                    platform::fullscreen_watcher::set_user_summoned(true);
                    domain::color::reapply_active_color();
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    if let Some(wv) = window.app_handle().get_webview_window("main") {
                        configure_main_window(&wv);
                    }
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
                            hide_main_window(window.app_handle());
                        } else {
                            state.signal_shutdown();
                            std::process::exit(0);
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
            commands::sync_keybinds,
            commands::get_keybinds,
            commands::open_driver_search,
            commands::apply_tweaks,
            commands::revert_tweaks,
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
            commands::consume_intro_splash,
            commands::exit_app,
            commands::restart_as_admin,
            commands::trim_memory,
            commands::rollback_last_tweaks,
            commands::detect_installed_apps,
            commands::restart_explorer,
            commands::restart_graphics_driver,
            commands::get_black_holo_status,
            commands::toggle_hardware_black_holo,
            commands::get_security_guard_status,
            commands::audit_anticheat_environment,
            commands::fix_anticheat_environment,
            commands::take_juicy_screenshot,
            commands::scan_drivers,
            commands::open_windows_driver_updates,
            commands::open_external_url,
            commands::get_pending_navigation,
            commands::check_for_updates,
            commands::download_update,
            commands::get_update_progress,
            commands::install_update,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build Synchro")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                let state = app.state::<RuntimeState>();
                if !state.exiting.load(Ordering::SeqCst) {
                    api.prevent_exit();
                }
            }
        });
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
                active_filter: String::new(),
            },
            settings: AppSettings {
                language: "unknown".to_string(),
                accent_color: "   ".to_string(),
                ..Default::default()
            },
            is_admin: false,
            keybinds: Vec::new(),
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
        let (w_1080, h_1080) = calculate_adaptive_window_size(1920.0, 1080.0);
        let (w_1440x1080, h_1440x1080) = calculate_adaptive_window_size(1440.0, 1080.0);
        let (w_2560x1920, h_2560x1920) = calculate_adaptive_window_size(2560.0, 1920.0);

        assert_eq!((w_1440x1080, h_1440x1080), (1024.0, 656.0));
        assert_eq!((w_2560x1920, h_2560x1920), (1024.0, 656.0));
        assert_eq!((w_1080, h_1080), (1024.0, 656.0));

        let (w_small, h_small) = calculate_adaptive_window_size(1280.0, 720.0);
        assert!(w_small >= 860.0 && w_small <= 1280.0);
        assert!(h_small >= 540.0 && h_small <= 720.0);
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
        assert!(is_newer_version("v2.2.4-fix", "2.2.2"));
        assert!(is_newer_version("v3.0.0", "2.2.2"));
        assert!(is_newer_version("2.3.0", "2.2.2"));
        assert!(is_newer_version("2.3.4", "2.3.4-rc1"));
        assert!(is_newer_version("2.3.5-beta", "2.3.4"));
        assert!(!is_newer_version("v2.2.2", "2.2.2"));
        assert!(!is_newer_version("v2.2.1", "2.2.2"));
        assert!(!is_newer_version("v1.9.9", "2.2.2"));
        assert!(!is_newer_version("2.3.4-beta", "2.3.4"));
        assert!(!is_newer_version("invalid-string", "2.3.4"));
    }

    #[test]
    fn test_sha256_manifest_parser() {
        use domain::updater::client::parse_sha256_manifest;

        let valid_manifest = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  synchro.exe\r\n\
a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90 *installer.exe\n";

        let hash = parse_sha256_manifest(valid_manifest, "synchro.exe").unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

        let hash_inst = parse_sha256_manifest(valid_manifest, "installer.exe").unwrap();
        assert_eq!(hash_inst, "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90");

        assert!(parse_sha256_manifest(valid_manifest, "missing.exe").is_err());

        let invalid_hash = "1234abcd  synchro.exe";
        assert!(parse_sha256_manifest(invalid_hash, "synchro.exe").is_err());

        let conflict = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  synchro.exe\n\
a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90  synchro.exe\n";
        assert!(parse_sha256_manifest(conflict, "synchro.exe").is_err());
    }

    #[test]
    fn test_helper_args_parsing() {
        use domain::updater::helper::HelperArgs;

        let args = vec![
            "synchro.exe".to_string(),
            "--updater-helper".to_string(),
            "--target".to_string(),
            "C:\\Program Files\\Synchro\\synchro.exe".to_string(),
            "--payload".to_string(),
            "C:\\Temp\\payload.exe".to_string(),
            "--wait-pid".to_string(),
            "12345".to_string(),
        ];

        let parsed = HelperArgs::parse(&args).unwrap();
        assert_eq!(parsed.target, std::path::PathBuf::from("C:\\Program Files\\Synchro\\synchro.exe"));
        assert_eq!(parsed.payload, std::path::PathBuf::from("C:\\Temp\\payload.exe"));
        assert_eq!(parsed.wait_pid, 12345);
        assert_eq!(parsed.backup, std::path::PathBuf::from("C:\\Program Files\\Synchro\\synchro.exe.old"));
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
        for val in &matrix {
            assert!(val.is_finite());
        }
        assert_eq!(matrix[18], 1.0);
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
        assert!(validate_update_url("https://objects.githubusercontent.com/production/synchro.exe").is_err());
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

    #[test]
    fn test_security_guard_anticheat_blocks() {
        use platform::security_guard::{verify_process_access_safety, verify_registry_write_safety, verify_registry_delete_safety};
        assert!(verify_process_access_safety("RustClient.exe", 0x0020).is_err());
        assert!(verify_process_access_safety("EasyAntiCheat.exe", 0x0010).is_err());
        assert!(verify_process_access_safety("cs2.exe", 0x0020).is_err());
        assert!(verify_process_access_safety("notepad.exe", 0x0001).is_ok());

        assert!(verify_registry_write_safety("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\RustClient.exe").is_err());
        assert!(verify_registry_write_safety("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\FortniteClient-Win64-Shipping.exe").is_err());
        assert!(verify_registry_write_safety("Software\\Microsoft\\GameBar").is_ok());

        assert!(verify_registry_delete_safety("SYSTEM\\CurrentControlSet").is_err());
        assert!(verify_registry_delete_safety("").is_err());
        assert!(verify_registry_delete_safety("Software\\Synchro\\Custom").is_ok());
    }

    #[test]
    fn test_anticheat_audit_runs_cleanly() {
        let audit = platform::security_guard::audit_anticheat_integrity();
        assert!(audit.active_shield);
    }
}
