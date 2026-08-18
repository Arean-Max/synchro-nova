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
    debugger_attached() || suspicious_process_present() || suspicious_loaded_module_present()
}

#[cfg(not(target_os = "windows"))]
fn platform_security_threat() -> bool {
    false
}

#[cfg(target_os = "windows")]
fn debugger_attached() -> bool {
    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn IsDebuggerPresent() -> i32;
        fn CheckRemoteDebuggerPresent(process: *mut c_void, present: *mut i32) -> i32;
        fn GetCurrentProcess() -> *mut c_void;
    }

    unsafe {
        let process = GetCurrentProcess();
        if IsDebuggerPresent() != 0 {
            return true;
        }

        let mut present = 0;
        (CheckRemoteDebuggerPresent(process, &mut present) != 0 && present != 0)
            || nt_debugger_attached(process)
    }
}

#[cfg(target_os = "windows")]
fn nt_debugger_attached(process: *mut c_void) -> bool {
    #[link(name = "Ntdll")]
    unsafe extern "system" {
        fn NtQueryInformationProcess(
            process_handle: *mut c_void,
            process_information_class: u32,
            process_information: *mut c_void,
            process_information_length: u32,
            return_length: *mut u32,
        ) -> i32;
    }

    const PROCESS_DEBUG_PORT: u32 = 7;
    const PROCESS_DEBUG_OBJECT_HANDLE: u32 = 0x1e;
    const PROCESS_DEBUG_FLAGS: u32 = 0x1f;

    unsafe {
        let mut debug_port: usize = 0;
        if NtQueryInformationProcess(
            process,
            PROCESS_DEBUG_PORT,
            &mut debug_port as *mut _ as *mut c_void,
            std::mem::size_of::<usize>() as u32,
            std::ptr::null_mut(),
        ) >= 0
            && debug_port != 0
        {
            return true;
        }

        let mut debug_object: usize = 0;
        if NtQueryInformationProcess(
            process,
            PROCESS_DEBUG_OBJECT_HANDLE,
            &mut debug_object as *mut _ as *mut c_void,
            std::mem::size_of::<usize>() as u32,
            std::ptr::null_mut(),
        ) >= 0
            && debug_object != 0
        {
            return true;
        }

        let mut debug_flags: u32 = 1;
        NtQueryInformationProcess(
            process,
            PROCESS_DEBUG_FLAGS,
            &mut debug_flags as *mut _ as *mut c_void,
            std::mem::size_of::<u32>() as u32,
            std::ptr::null_mut(),
        ) >= 0
            && debug_flags == 0
    }
}

#[cfg(target_os = "windows")]
fn suspicious_process_present() -> bool {
    const EXACT: &[&str] = &[
        "x64dbg.exe",
        "x32dbg.exe",
        "ollydbg.exe",
        "ida.exe",
        "ida64.exe",
        "windbg.exe",
        "cdb.exe",
        "ntsd.exe",
        "procdump.exe",
        "processhacker.exe",
        "procmon.exe",
        "procexp.exe",
        "scylla.exe",
        "scylla_x64.exe",
        "scylla_x86.exe",
        "cheatengine.exe",
        "cheatengine-x86_64.exe",
        "ghidra.exe",
        "dnspy.exe",
        "ilspy.exe",
        "frida.exe",
        "frida-server.exe",
        "frida-helper.exe",
    ];
    const CONTAINS: &[&str] = &[
        "x64dbg",
        "x32dbg",
        "ollydbg",
        "windbg",
        "procdump",
        "processhacker",
        "scylla",
        "cheatengine",
        "frida",
    ];

    enumerate_process_names().into_iter().any(|name| {
        let lower = name.to_ascii_lowercase();
        EXACT.iter().any(|bad| lower == *bad) || CONTAINS.iter().any(|bad| lower.contains(bad))
    })
}

#[cfg(target_os = "windows")]
fn suspicious_loaded_module_present() -> bool {
    const SUSPICIOUS: &[&str] = &[
        "frida",
        "scylla",
        "titanhide",
        "x64dbg",
        "x32dbg",
        "ollydbg",
        "cheatengine",
        "vehdebug",
        "dbghelp_hook",
    ];

    enumerate_current_process_modules()
        .into_iter()
        .any(|module| {
            let lower_name = module.name.to_ascii_lowercase();
            let lower_path = module.path.to_ascii_lowercase();
            SUSPICIOUS.iter().any(|bad| lower_name.contains(bad))
                || suspicious_module_path(&lower_path)
        })
}

#[cfg(target_os = "windows")]
fn suspicious_module_path(path: &str) -> bool {
    path.ends_with(".dll")
        && (path.contains("\\appdata\\local\\temp\\")
            || path.contains("\\windows\\temp\\")
            || path.contains("\\temp\\"))
        && !path.contains("\\microsoft\\edgewebview\\")
}

#[cfg(target_os = "windows")]
fn enumerate_process_names() -> Vec<String> {
    #[repr(C)]
    struct ProcessEntry32W {
        dw_size: u32,
        cnt_usage: u32,
        th32_process_id: u32,
        th32_default_heap_id: usize,
        th32_module_id: u32,
        cnt_threads: u32,
        th32_parent_process_id: u32,
        pc_pri_class_base: i32,
        dw_flags: u32,
        sz_exe_file: [u16; 260],
    }

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, process_id: u32) -> *mut c_void;
        fn Process32FirstW(snapshot: *mut c_void, entry: *mut ProcessEntry32W) -> i32;
        fn Process32NextW(snapshot: *mut c_void, entry: *mut ProcessEntry32W) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
    }

    const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
    const INVALID_HANDLE_VALUE: isize = -1;

    let mut names = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot as isize == INVALID_HANDLE_VALUE {
            return names;
        }

        let mut entry = ProcessEntry32W {
            dw_size: std::mem::size_of::<ProcessEntry32W>() as u32,
            cnt_usage: 0,
            th32_process_id: 0,
            th32_default_heap_id: 0,
            th32_module_id: 0,
            cnt_threads: 0,
            th32_parent_process_id: 0,
            pc_pri_class_base: 0,
            dw_flags: 0,
            sz_exe_file: [0; 260],
        };

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name = crate::utf16z_to_string(&entry.sz_exe_file);
                if !name.is_empty() {
                    names.push(name);
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
    }

    names
}

#[cfg(target_os = "windows")]
struct ModuleObservation {
    name: String,
    path: String,
}

#[cfg(target_os = "windows")]
fn enumerate_current_process_modules() -> Vec<ModuleObservation> {
    #[repr(C)]
    struct ModuleEntry32W {
        dw_size: u32,
        th32_module_id: u32,
        th32_process_id: u32,
        glblcnt_usage: u32,
        proccnt_usage: u32,
        mod_base_addr: *mut u8,
        mod_base_size: u32,
        h_module: *mut c_void,
        sz_module: [u16; 256],
        sz_exe_path: [u16; 260],
    }

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, process_id: u32) -> *mut c_void;
        fn Module32FirstW(snapshot: *mut c_void, entry: *mut ModuleEntry32W) -> i32;
        fn Module32NextW(snapshot: *mut c_void, entry: *mut ModuleEntry32W) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
    }

    const TH32CS_SNAPMODULE: u32 = 0x0000_0008;
    const TH32CS_SNAPMODULE32: u32 = 0x0000_0010;
    const INVALID_HANDLE_VALUE: isize = -1;

    let mut modules = Vec::new();
    unsafe {
        let snapshot =
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, std::process::id());
        if snapshot as isize == INVALID_HANDLE_VALUE {
            return modules;
        }

        let mut entry = ModuleEntry32W {
            dw_size: std::mem::size_of::<ModuleEntry32W>() as u32,
            th32_module_id: 0,
            th32_process_id: 0,
            glblcnt_usage: 0,
            proccnt_usage: 0,
            mod_base_addr: std::ptr::null_mut(),
            mod_base_size: 0,
            h_module: std::ptr::null_mut(),
            sz_module: [0; 256],
            sz_exe_path: [0; 260],
        };

        if Module32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name = crate::utf16z_to_string(&entry.sz_module);
                if !name.is_empty() {
                    modules.push(ModuleObservation {
                        name,
                        path: crate::utf16z_to_string(&entry.sz_exe_path),
                    });
                }
                if Module32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
    }

    modules
}
