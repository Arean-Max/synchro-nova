use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

#[cfg(target_os = "windows")]
use crate::platform::ffi::{winapi, MSG};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Keybind {
    pub id: String,
    pub key: String,
    pub modifiers: u32,
    pub vk: u32,
    pub action: String,
    pub enabled: bool,
}

static CURRENT_KEYBINDS: Mutex<Vec<Keybind>> = Mutex::new(Vec::new());
static GLOBAL_APP_HANDLE: std::sync::OnceLock<AppHandle> = std::sync::OnceLock::new();
static LISTENER_THREAD_ID: AtomicU32 = AtomicU32::new(0);
static LAST_TRIGGER_MS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

const WM_USER_SYNC: u32 = 0x0400 + 101;
const WM_HOTKEY: u32 = 0x0312;

pub fn sync_hotkeys(keybinds: &[Keybind]) {
    if let Ok(mut guard) = CURRENT_KEYBINDS.lock() {
        *guard = keybinds.to_vec();
    }
    let tid = LISTENER_THREAD_ID.load(Ordering::SeqCst);
    if tid != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            winapi::PostThreadMessageW(tid, WM_USER_SYNC, 0, 0);
        }
    }
}

pub fn get_active_keybinds() -> Vec<Keybind> {
    CURRENT_KEYBINDS
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

#[cfg(target_os = "windows")]
pub fn start_global_hotkey_listener(app: AppHandle) {
    let _ = GLOBAL_APP_HANDLE.set(app);
    static STARTED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    STARTED.get_or_init(|| {
        std::thread::Builder::new()
            .name("synchro-global-hotkeys".to_string())
            .spawn(move || {
                let tid = unsafe { winapi::GetCurrentThreadId() };
                LISTENER_THREAD_ID.store(tid, Ordering::SeqCst);

                let mut dummy_msg = unsafe { std::mem::zeroed::<MSG>() };
                unsafe {
                    winapi::PeekMessageW(&mut dummy_msg, std::ptr::null_mut(), 0x0400, 0x0400, 0);
                }

                let mut registered_ids: Vec<i32> = Vec::new();
                let register_all = |registered: &mut Vec<i32>| {
                    for id in registered.drain(..) {
                        unsafe { winapi::UnregisterHotKey(std::ptr::null_mut(), id); }
                    }
                    if let Ok(binds) = CURRENT_KEYBINDS.lock() {
                        for (idx, kb) in binds.iter().enumerate() {
                            if kb.enabled && kb.vk > 0 {
                                let id = (idx as i32) + 1;
                                let mut fs_mods = kb.modifiers & 0x000F;
                                fs_mods |= 0x4000;
                                let ok = unsafe {
                                    winapi::RegisterHotKey(std::ptr::null_mut(), id, fs_mods, kb.vk)
                                };
                                if ok != 0 {
                                    registered.push(id);
                                }
                            }
                        }
                    }
                };

                register_all(&mut registered_ids);

                let mut msg = unsafe { std::mem::zeroed::<MSG>() };
                while unsafe { winapi::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) } > 0 {
                    if msg.message == WM_USER_SYNC {
                        register_all(&mut registered_ids);
                    } else if msg.message == WM_HOTKEY {
                        let hit_id = msg.w_param as i32;
                        let action_opt = {
                            CURRENT_KEYBINDS.lock().ok().and_then(|binds| {
                                let idx = (hit_id - 1) as usize;
                                binds.get(idx).map(|k| k.action.clone())
                            })
                        };
                        if let Some(action) = action_opt {
                            let now = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_millis() as u64)
                                .unwrap_or(0);
                            let last = LAST_TRIGGER_MS.load(Ordering::Relaxed);
                            if now.saturating_sub(last) > 250 {
                                LAST_TRIGGER_MS.store(now, Ordering::Relaxed);
                                if let Some(app) = GLOBAL_APP_HANDLE.get() {
                                    let app_clone = app.clone();
                                    std::thread::spawn(move || {
                                        execute_action(&action, &app_clone);
                                    });
                                }
                            }
                        }
                    } else {
                        unsafe {
                            winapi::TranslateMessage(&msg);
                            winapi::DispatchMessageW(&msg);
                        }
                    }
                }

                for id in registered_ids.drain(..) {
                    unsafe { winapi::UnregisterHotKey(std::ptr::null_mut(), id); }
                }
            })
            .ok();
    });
}

#[cfg(not(target_os = "windows"))]
pub fn start_global_hotkey_listener(_app: AppHandle) {}

fn execute_action(action: &str, app: &AppHandle) {
    match action {
        "toggle_window" => {
            if let Some(window) = app.get_webview_window("main") {
                let is_visible = window.is_visible().unwrap_or(false);
                let is_minimized = window.is_minimized().unwrap_or(false);
                if !is_visible || is_minimized {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let _ = window.hide();
                    crate::app::workers::trim_process_memory();
                }
            }
        }
        "trim_memory" => {
            crate::app::workers::trim_process_memory();
            let _ = app.emit(
                "hotkey-triggered",
                serde_json::json!({
                    "action": "trim_memory",
                    "label": "RAM Trimmed"
                }),
            );
        }
        "reset_color" => {
            if let Some(state) = app.try_state::<crate::app::state::RuntimeState>() {
                let default_color = crate::app::state::ColorSettings::default();
                let recordings = state
                    .data
                    .lock()
                    .ok()
                    .map(|d| d.settings.show_on_recordings)
                    .unwrap_or(true);
                let _ = crate::domain::color::apply_color_transform(&default_color, recordings);
                if let Ok(mut guard) = state.data.lock() {
                    guard.color = default_color.clone();
                }
                let _ = app.emit(
                    "hotkey-triggered",
                    serde_json::json!({
                        "action": "reset_color",
                        "color": default_color
                    }),
                );
            }
        }
        "toggle_black_holo" => {
            let next = !crate::domain::color::black_holo::is_black_holo_active();
            let _ = crate::domain::color::black_holo::set_hardware_black_holo(next);
            let _ = app.emit(
                "hotkey-triggered",
                serde_json::json!({
                    "action": "toggle_black_holo",
                    "active": next
                }),
            );
        }
        "toggle_saturation" => {
            if let Some(state) = app.try_state::<crate::app::state::RuntimeState>() {
                let current = state
                    .data
                    .lock()
                    .ok()
                    .map(|d| d.color.clone())
                    .unwrap_or_default();
                let next_sat = if (current.saturation - 100.0).abs() < 5.0 {
                    160.0
                } else {
                    100.0
                };
                let mut updated = current;
                updated.saturation = next_sat;
                let recordings = state
                    .data
                    .lock()
                    .ok()
                    .map(|d| d.settings.show_on_recordings)
                    .unwrap_or(true);
                let _ = crate::domain::color::apply_color_transform(&updated, recordings);
                if let Ok(mut guard) = state.data.lock() {
                    guard.color = updated.clone();
                }
                let _ = app.emit(
                    "hotkey-triggered",
                    serde_json::json!({
                        "action": "toggle_saturation",
                        "color": updated
                    }),
                );
            }
        }
        _ => {
            if let Some(template_name) = action.strip_prefix("template:") {
                apply_template_by_name(template_name, app);
            }
        }
    }
}

fn apply_template_by_name(name: &str, app: &AppHandle) {
    let target = match name {
        "balanced" => Some(crate::app::state::ColorSettings {
            saturation: 160.0,
            hue: -5.0,
            contrast: 97.0,
            gamma: 118.0,
            black_holo: 0.0,
            enabled: true,
            active_filter: String::new(),
        }),
        "vibrant" => Some(crate::app::state::ColorSettings {
            saturation: 200.0,
            hue: -5.0,
            contrast: 95.0,
            gamma: 105.0,
            black_holo: 0.0,
            enabled: true,
            active_filter: String::new(),
        }),
            "soft" => Some(crate::app::state::ColorSettings {
                saturation: 150.0,
                hue: -5.0,
                contrast: 85.0,
                gamma: 115.0,
                black_holo: 0.0,
                enabled: true,
                active_filter: String::new(),
            }),
            "night" => Some(crate::app::state::ColorSettings {
                saturation: 120.0,
                hue: -5.0,
                contrast: 90.0,
                gamma: 150.0,
                black_holo: 0.0,
                enabled: true,
                active_filter: String::new(),
            }),
            "rust_cold_tactical" => Some(crate::app::state::ColorSettings {
                saturation: 78.0,
                hue: 0.0,
                contrast: 116.0,
                gamma: 108.0,
                black_holo: 0.0,
                enabled: true,
                active_filter: "rust_cold_tactical".to_string(),
            }),
            "rust_midnight_neon" => Some(crate::app::state::ColorSettings {
                saturation: 140.0,
                hue: 0.0,
                contrast: 122.0,
                gamma: 100.0,
                black_holo: 0.0,
                enabled: true,
                active_filter: "rust_midnight_neon".to_string(),
            }),
            _ => None,
        };

    if let Some(color) = target {
        if let Some(state) = app.try_state::<crate::app::state::RuntimeState>() {
            let recordings = state
                .data
                .lock()
                .ok()
                .map(|d| d.settings.show_on_recordings)
                .unwrap_or(true);
            let _ = crate::domain::color::apply_color_transform(&color, recordings);
            if let Ok(mut guard) = state.data.lock() {
                guard.color = color.clone();
            }
            let _ = app.emit(
                "hotkey-triggered",
                serde_json::json!({
                    "action": format!("template:{}", name),
                    "template": name,
                    "name": name,
                    "color": color
                }),
            );
        }
    }
}
