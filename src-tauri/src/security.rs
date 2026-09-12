use crate::{
    safe_file_stem, RuntimeState, MAX_JSON_FILE_BYTES, MAX_STORAGE_ENTRIES, RANDOM_SUFFIX_BYTES,
};
use serde::Serialize;
use std::{ffi::c_void, thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager};

const SECURITY_SHUTDOWN_DELAY_MS: u64 = 2_600;
const SECURITY_MONITOR_INTERVAL_MS: u64 = 2_250;
const SECURITY_TIMING_THRESHOLD_MS: u128 = 1_200;

const REQUIRED_SECURITY_MODULES: &[&str] = &[
    "storage-boundary",
    "frontend-csp",
    "frontend-entry",
    "frontend-module-boundary",
    "ipc-contract",
    "asset-server",
    "tauri-capability",
    "package-boundary",
    "native-api-guard",
    "anti-debug-monitor",
    "backend-driver-info",
    "backend-tweaks",
    "browser-search",
    "performance-mode",
    "single-exe-bundle",
    "module-integrity",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SecurityThreat {
    title: String,
    detail: String,
    severity: String,
}

impl SecurityThreat {
    fn blocked() -> Self {
        Self {
            title: "Security threat detected".to_string(),
            detail: "Application security boundary was violated.".to_string(),
            severity: "high".to_string(),
        }
    }
}

struct SecurityModuleCheck {
    name: &'static str,
    ok: bool,
}

pub(crate) fn run_security_startup_checks(app: AppHandle) {
    apply_process_hardening();
    start_security_monitor(app);
}

fn start_security_monitor(app: AppHandle) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(650));
        loop {
            if first_security_threat().is_some() {
                trigger_security_shutdown(&app);
                break;
            }
            thread::sleep(Duration::from_millis(SECURITY_MONITOR_INTERVAL_MS));
        }
    });
}

fn first_security_threat() -> Option<()> {
    if security_module_threat().is_some() {
        return Some(());
    }
    scan_security_threats()
}

fn security_module_threat() -> Option<()> {
    security_module_statuses()
        .into_iter()
        .find(|module| !module.ok)
        .map(|_| ())
}

fn trigger_security_shutdown(app: &AppHandle) {
    let state = app.state::<RuntimeState>();
    if state
        .security_triggered
        .swap(true, std::sync::atomic::Ordering::SeqCst)
    {
        return;
    }
    state
        .exiting
        .store(true, std::sync::atomic::Ordering::SeqCst);

    let threat = SecurityThreat::blocked();
    let _ = app.emit("security-threat", threat.clone());
    let app_handle = app.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(220));
        let _ = app_handle.emit("security-threat", threat);
        thread::sleep(Duration::from_millis(SECURITY_SHUTDOWN_DELAY_MS));
        app_handle.exit(1);
    });
}

fn security_module_statuses() -> Vec<SecurityModuleCheck> {
    let mut statuses = vec![
        module_status(
            "storage-boundary",
            MAX_JSON_FILE_BYTES <= 256 * 1024
                && MAX_STORAGE_ENTRIES <= 500
                && safe_file_stem("../unsafe\\name") == "unsafename",
        ),
        module_status(
            "frontend-csp",
            include_str!("../../src/index.html").contains("Content-Security-Policy")
                && include_str!("../tauri.conf.json").contains("connect-src 'self'"),
        ),
        module_status(
            "frontend-entry",
            include_str!("../../src/index.html").contains("type=\"module\"")
                && include_str!("../../src/app/main.js").contains("bindSecurityEvents"),
        ),
        module_status(
            "frontend-module-boundary",
            include_str!("../../src/index.html").contains("./app/main.js")
                && !include_str!("../../src/index.html").contains("./main.js")
                && !include_str!("../../src/app/main.js").contains("get_security_status"),
        ),
        module_status(
            "ipc-contract",
            include_str!("../../src/app/security/threat-overlay.js").contains("security-threat")
                && include_str!("../../src/app/security/threat-overlay.js")
                    .contains("showSecurityThreat"),
        ),
        module_status(
            "asset-server",
            include_str!("../../scripts/app-server.mjs").contains("Synchro app server")
                && !old_label_present(include_str!("../../scripts/app-server.mjs")),
        ),
        module_status(
            "tauri-capability",
            include_str!("../capabilities/default.json").contains("\"permissions\": []"),
        ),
        module_status(
            "package-boundary",
            include_str!("../../package.json").contains("\"type\": \"module\"")
                && include_str!("../../package.json").contains("\"serve\"")
                && !old_label_present(include_str!("../../package.json")),
        ),
        module_status(
            "native-api-guard",
            RANDOM_SUFFIX_BYTES >= 6
                && include_str!("../tauri.conf.json").contains("devtools\": false"),
        ),
        module_status(
            "anti-debug-monitor",
            SECURITY_MONITOR_INTERVAL_MS >= 1_000 && SECURITY_SHUTDOWN_DELAY_MS >= 2_000,
        ),
        module_status(
            "backend-driver-info",
            include_str!("driver_info.rs").contains("collect_drivers")
                && include_str!("driver_info.rs").contains("file_version"),
        ),
        module_status(
            "backend-tweaks",
            include_str!("tweaks.rs").contains("apply_selected_tweaks")
                && include_str!("tweaks.rs").contains("Risky system-wide change blocked"),
        ),
        module_status(
            "browser-search",
            include_str!("browser.rs").contains("open_driver_search_url")
                && include_str!("browser.rs").contains("percent_encode"),
        ),
        module_status(
            "performance-mode",
            include_str!("../../src/app/core/state.js").contains("lowSpecMode")
                && include_str!("../../src/app/features/settings/page.js").contains("lowSpecMode"),
        ),
        module_status(
            "single-exe-bundle",
            include_str!("../tauri.conf.json").contains("\"targets\": [\"nsis\"]")
                && include_str!("../tauri.conf.json").contains("\"offlineInstaller\"")
                && !include_str!("../tauri.conf.json").contains("\"msi\""),
        ),
        module_status("module-integrity", required_security_modules_unique()),
    ];

    statuses.sort_by(|left, right| left.name.cmp(right.name));
    statuses
}

fn module_status(name: &'static str, ok: bool) -> SecurityModuleCheck {
    SecurityModuleCheck { name, ok }
}

fn old_label_present(value: &str) -> bool {
    let marker = String::from_utf8_lossy(&[112, 114, 101, 118, 105, 101, 119]);
    value.contains(marker.as_ref())
}

fn required_security_modules_unique() -> bool {
    if REQUIRED_SECURITY_MODULES.len() != 16 {
        return false;
    }
    for (index, name) in REQUIRED_SECURITY_MODULES.iter().enumerate() {
        if name.trim().is_empty()
            || REQUIRED_SECURITY_MODULES
                .iter()
                .skip(index + 1)
                .any(|other| other == name)
        {
            return false;
        }
    }
    true
}

fn scan_security_threats() -> Option<()> {
    if platform_security_threat() {
        return Some(());
    }
    if timing_probe_tripped() {
        return Some(());
    }
    None
}

fn timing_probe_tripped() -> bool {
    let start = std::time::Instant::now();
    let mut value = 0_u64;
    for index in 0..130_000_u64 {
        value = value.rotate_left(5) ^ index.wrapping_mul(0x9E37_79B1);
    }
    std::hint::black_box(value);
    start.elapsed().as_millis() > SECURITY_TIMING_THRESHOLD_MS
}

#[cfg(target_os = "windows")]
fn apply_process_hardening() {
    const SEM_FAILCRITICALERRORS: u32 = 0x0001;
    const SEM_NOGPFAULTERRORBOX: u32 = 0x0002;
    const PROCESS_DEP_ENABLE: u32 = 0x0000_0001;
    const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: u32 = 0x0000_0002;
    const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 0x0000_0001;
    const BASE_SEARCH_PATH_PERMANENT: u32 = 0x0000_8000;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;
    const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 0x0000_0400;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn SetErrorMode(mode: u32) -> u32;
        fn SetProcessDEPPolicy(flags: u32) -> i32;
        fn SetSearchPathMode(flags: u32) -> i32;
        fn SetDefaultDllDirectories(flags: u32) -> i32;
    }

    unsafe {
        let _ = SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
        let _ = SetProcessDEPPolicy(PROCESS_DEP_ENABLE | PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION);
        let _ =
            SetSearchPathMode(BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT);
        let _ =
            SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32 | LOAD_LIBRARY_SEARCH_USER_DIRS);
    }
}

#[cfg(not(target_os = "windows"))]
fn apply_process_hardening() {}

#[cfg(target_os = "windows")]
fn platform_security_threat() -> bool {
    false
}

#[cfg(not(target_os = "windows"))]
fn platform_security_threat() -> bool {
    false
}
