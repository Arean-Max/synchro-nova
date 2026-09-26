#![allow(dead_code)]

#[cfg(target_os = "windows")]
use std::ffi::c_void;
use std::path::Path;
use super::types::*;
use super::winapi;

#[cfg(target_os = "windows")]
static PROCESS_JOB: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Default)]
struct IoCounters {
    read_operation_count: u64,
    write_operation_count: u64,
    other_operation_count: u64,
    read_transfer_count: u64,
    write_transfer_count: u64,
    other_transfer_count: u64,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Default)]
struct JobObjectBasicLimitInformation {
    per_process_user_time_limit: i64,
    per_job_user_time_limit: i64,
    limit_flags: u32,
    minimum_working_set_size: usize,
    maximum_working_set_size: usize,
    active_process_limit: u32,
    affinity: usize,
    priority_class: u32,
    scheduling_class: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Default)]
struct JobObjectExtendedLimitInformation {
    basic_limit_information: JobObjectBasicLimitInformation,
    io_info: IoCounters,
    process_memory_limit: usize,
    job_memory_limit: usize,
    peak_process_memory_used: usize,
    peak_job_memory_used: usize,
}

#[cfg(target_os = "windows")]
pub fn init_process_tree_job() {
    PROCESS_JOB.get_or_init(|| {
        unsafe {
            let job = winapi::CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
            if job.is_null() || job as isize == -1 {
                return None;
            }

            const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION: i32 = 9;
            const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x0000_2000;
            const JOB_OBJECT_LIMIT_BREAKAWAY_OK: u32 = 0x0000_0800;
            const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: u32 = 0x0000_1000;

            let mut info = JobObjectExtendedLimitInformation::default();
            info.basic_limit_information.limit_flags =
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_BREAKAWAY_OK | JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK;

            let _ = winapi::SetInformationJobObject(
                job,
                JOB_OBJECT_EXTENDED_LIMIT_INFORMATION,
                &mut info as *mut _ as *mut c_void,
                std::mem::size_of::<JobObjectExtendedLimitInformation>() as u32,
            );

            if winapi::AssignProcessToJobObject(job, winapi::GetCurrentProcess()) == 0 {
                winapi::CloseHandle(job);
                return None;
            }
            Some(job as usize)
        }
    });
}

#[cfg(not(target_os = "windows"))]
pub fn init_process_tree_job() {}

#[cfg(target_os = "windows")]
pub fn trim_working_set() {
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    static CHILD_CACHE: Mutex<(Vec<u32>, Option<Instant>)> = Mutex::new((Vec::new(), None));
    const CACHE_TTL: Duration = Duration::from_secs(30);

    unsafe {
        // 1. Trim the main application process
        let _ = winapi::SetProcessWorkingSetSize(winapi::GetCurrentProcess(), usize::MAX, usize::MAX);

        // 2. Check cached child WebView2 process IDs
        let mut webview_pids = Vec::new();
        let mut needs_refresh = true;

        if let Ok(guard) = CHILD_CACHE.lock() {
            if let Some(updated_at) = guard.1 {
                if updated_at.elapsed() < CACHE_TTL && !guard.0.is_empty() {
                    webview_pids = guard.0.clone();
                    needs_refresh = false;
                }
            }
        }

        // 3. Discover direct child and descendant WebView2 processes if cache expired
        if needs_refresh {
            let my_pid = winapi::GetCurrentProcessId();
            let snapshot = winapi::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if !snapshot.is_null() && snapshot as isize != -1 {
                let mut entry = ProcessEntry32W::default();
                let mut all_entries = Vec::with_capacity(128);

                if winapi::Process32FirstW(snapshot, &mut entry) != 0 {
                    loop {
                        all_entries.push((
                            entry.th32_process_id,
                            entry.th32_parent_process_id,
                            entry.sz_exe_file,
                        ));
                        if winapi::Process32NextW(snapshot, &mut entry) == 0 {
                            break;
                        }
                    }
                }
                winapi::CloseHandle(snapshot);

                let mut tree_pids = Vec::with_capacity(8);
                tree_pids.push(my_pid);

                let mut discovered = Vec::with_capacity(8);
                let mut added = true;

                while added {
                    added = false;
                    for (pid, parent_pid, exe_name) in &all_entries {
                        if *pid != my_pid && !tree_pids.contains(pid) && tree_pids.contains(parent_pid) {
                            let len = exe_name.iter().position(|&c| c == 0).unwrap_or(exe_name.len());
                            let name = String::from_utf16_lossy(&exe_name[..len]);
                            if name.eq_ignore_ascii_case("msedgewebview2.exe") {
                                tree_pids.push(*pid);
                                discovered.push(*pid);
                                added = true;
                            }
                        }
                    }
                }

                if let Ok(mut guard) = CHILD_CACHE.lock() {
                    *guard = (discovered.clone(), Some(Instant::now()));
                }
                webview_pids = discovered;
            }
        }

        // 4. Trim working sets for the discovered WebView2 child processes
        const PROCESS_SET_QUOTA: u32 = 0x0100;
        for pid in webview_pids {
            let handle = winapi::OpenProcess(PROCESS_SET_QUOTA, 0, pid);
            if !handle.is_null() && handle as isize != -1 {
                let _ = winapi::SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
                winapi::CloseHandle(handle);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn trim_working_set() {}

#[cfg(target_os = "windows")]
pub fn apply_process_hardening() {
    const PROCESS_DEP_ENABLE: u32 = 0x0000_0001;
    const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 0x0000_0001;
    const BASE_SEARCH_PATH_PERMANENT: u32 = 0x0000_8000;
    const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 0x0000_0200;
    const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x0000_1000;
    const HEAP_ENABLE_TERMINATION_ON_CORRUPTION: i32 = 1;

    unsafe {
        let _ = winapi::SetProcessDEPPolicy(PROCESS_DEP_ENABLE);
        let _ = winapi::SetSearchPathMode(
            BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT,
        );
        let _ = winapi::SetDefaultDllDirectories(
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_APPLICATION_DIR,
        );
        let heap = winapi::GetProcessHeap();
        if !heap.is_null() {
            let _ = winapi::HeapSetInformation(
                heap,
                HEAP_ENABLE_TERMINATION_ON_CORRUPTION,
                std::ptr::null_mut(),
                0,
            );
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn apply_process_hardening() {}

#[cfg(target_os = "windows")]
pub fn is_user_admin() -> bool {
    const TOKEN_QUERY: u32 = 0x0008;
    const TOKEN_ELEVATION: u32 = 20;

    #[repr(C)]
    struct TokenElevationStruct {
        token_is_elevated: u32,
    }

    unsafe {
        let mut token_handle: *mut c_void = std::ptr::null_mut();
        let process_handle = winapi::GetCurrentProcess();

        if winapi::OpenProcessToken(process_handle, TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TokenElevationStruct { token_is_elevated: 0 };
        let mut return_length: u32 = 0;

        let success = winapi::GetTokenInformation(
            token_handle,
            TOKEN_ELEVATION,
            &mut elevation as *mut _ as *mut c_void,
            std::mem::size_of::<TokenElevationStruct>() as u32,
            &mut return_length,
        );

        let _ = winapi::CloseHandle(token_handle);
        success != 0 && elevation.token_is_elevated != 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_user_admin() -> bool {
    false
}

#[cfg(target_os = "windows")]
pub fn wait_for_process_exit(pid: u32, timeout_ms: u32) {
    if pid == 0 {
        return;
    }
    let handle = unsafe { winapi::OpenProcess(SYNCHRONIZE, 0, pid) };
    if !handle.is_null() {
        unsafe {
            winapi::WaitForSingleObject(handle, timeout_ms);
            winapi::CloseHandle(handle);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn wait_for_process_exit(_pid: u32, _timeout_ms: u32) {}

#[cfg(target_os = "windows")]
pub fn verify_embedded_signature(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[repr(C)]
    struct Guid {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }

    #[repr(C)]
    struct WinTrustFileInfo {
        cb_struct: u32,
        pcwsz_file_path: *const u16,
        h_file: *mut c_void,
        pg_known_subject: *mut c_void,
    }

    #[repr(C)]
    struct WinTrustData {
        cb_struct: u32,
        p_policy_callback_data: *mut c_void,
        p_sip_client_data: *mut c_void,
        dw_ui_choice: u32,
        fdw_revocation_checks: u32,
        dw_union_choice: u32,
        p_file: *mut WinTrustFileInfo,
        dw_state_action: u32,
        h_wvt_state_data: *mut c_void,
        pwsz_url_reference: *mut u16,
        dw_prov_flags: u32,
        dw_ui_context: u32,
        p_signature_settings: *mut c_void,
    }

    const WTD_UI_NONE: u32 = 2;
    const WTD_REVOKE_NONE: u32 = 0;
    const WTD_CHOICE_FILE: u32 = 1;
    const WTD_STATEACTION_VERIFY: u32 = 1;
    const WTD_STATEACTION_CLOSE: u32 = 2;
    const WTD_SAFER_FLAG: u32 = 0x0000_0100;
    const WTD_REVOCATION_CHECK_NONE: u32 = 0x0000_0010;
    const WTD_CACHE_ONLY_URL_RETRIEVAL: u32 = 0x0000_1000;

    let path_w = wide_null(&path.to_string_lossy());
    let mut file_info = WinTrustFileInfo {
        cb_struct: std::mem::size_of::<WinTrustFileInfo>() as u32,
        pcwsz_file_path: path_w.as_ptr(),
        h_file: std::ptr::null_mut(),
        pg_known_subject: std::ptr::null_mut(),
    };

    let mut trust_data = WinTrustData {
        cb_struct: std::mem::size_of::<WinTrustData>() as u32,
        p_policy_callback_data: std::ptr::null_mut(),
        p_sip_client_data: std::ptr::null_mut(),
        dw_ui_choice: WTD_UI_NONE,
        fdw_revocation_checks: WTD_REVOKE_NONE,
        dw_union_choice: WTD_CHOICE_FILE,
        p_file: &mut file_info,
        dw_state_action: WTD_STATEACTION_VERIFY,
        h_wvt_state_data: std::ptr::null_mut(),
        pwsz_url_reference: std::ptr::null_mut(),
        dw_prov_flags: WTD_SAFER_FLAG | WTD_REVOCATION_CHECK_NONE | WTD_CACHE_ONLY_URL_RETRIEVAL,
        dw_ui_context: 0,
        p_signature_settings: std::ptr::null_mut(),
    };

    let action_guid = Guid {
        data1: 0x00aa_c56b,
        data2: 0xcd44,
        data3: 0x11d0,
        data4: [0x8c, 0xc2, 0x00, 0xc0, 0x4f, 0xc2, 0x95, 0xee],
    };

    unsafe {
        let wintrust_dll = winapi::LoadLibraryW(wide_null("wintrust.dll").as_ptr());
        if wintrust_dll.is_null() {
            return false;
        }

        type WinVerifyTrustFn = unsafe extern "system" fn(
            *mut c_void,
            *const Guid,
            *mut WinTrustData,
        ) -> i32;

        let proc = winapi::GetProcAddress(wintrust_dll, b"WinVerifyTrust\0".as_ptr());
        if proc.is_null() {
            winapi::FreeLibrary(wintrust_dll);
            return false;
        }

        let verify_fn: WinVerifyTrustFn = std::mem::transmute(proc);
        let status = verify_fn(std::ptr::null_mut(), &action_guid, &mut trust_data);

        trust_data.dw_state_action = WTD_STATEACTION_CLOSE;
        let _ = verify_fn(std::ptr::null_mut(), &action_guid, &mut trust_data);

        winapi::FreeLibrary(wintrust_dll);
        status == 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn verify_embedded_signature(_path: &Path) -> bool {
    true
}

#[repr(C)]
struct RestorePointInfoW {
    dw_event_type: u32,
    dw_restore_pt_type: u32,
    ll_sequence_number: i64,
    sz_description: [u16; 256],
}

#[repr(C)]
#[derive(Default)]
struct StateMgrStatus {
    n_status: u32,
    ll_sequence_number: i64,
}

#[cfg(target_os = "windows")]
pub fn create_native_system_restore_point(description: &str) -> bool {
    unsafe {
        let srclient = winapi::LoadLibraryW(wide_null("srclient.dll").as_ptr());
        if srclient.is_null() {
            return false;
        }

        type SRSetRestorePointWFn = unsafe extern "system" fn(
            *mut RestorePointInfoW,
            *mut StateMgrStatus,
        ) -> i32;

        let proc = winapi::GetProcAddress(srclient, b"SRSetRestorePointW\0".as_ptr());
        if proc.is_null() {
            winapi::FreeLibrary(srclient);
            return false;
        }

        let sr_fn: SRSetRestorePointWFn = std::mem::transmute(proc);

        let mut info = RestorePointInfoW {
            dw_event_type: 100, // BEGIN_SYSTEM_CHANGE
            dw_restore_pt_type: 12, // MODIFY_SETTINGS
            ll_sequence_number: 0,
            sz_description: [0; 256],
        };

        let clean: Vec<u16> = description
            .chars()
            .filter(|c| !c.is_control() && *c != '\0')
            .take(255)
            .flat_map(|c| {
                let mut buf = [0u16; 2];
                c.encode_utf16(&mut buf);
                buf.into_iter().take_while(|&u| u != 0).collect::<Vec<_>>()
            })
            .collect();

        for (i, &ch) in clean.iter().enumerate().take(255) {
            info.sz_description[i] = ch;
        }

        let mut status = StateMgrStatus::default();
        let success = sr_fn(&mut info, &mut status);

        let committed = if success != 0 && status.n_status == 0 {
            let mut end_info = RestorePointInfoW {
                dw_event_type: 101, // END_SYSTEM_CHANGE
                dw_restore_pt_type: 12, // MODIFY_SETTINGS
                ll_sequence_number: status.ll_sequence_number,
                sz_description: [0; 256],
            };
            let mut end_status = StateMgrStatus::default();
            let end_success = sr_fn(&mut end_info, &mut end_status);
            end_success != 0 && end_status.n_status == 0
        } else {
            false
        };

        winapi::FreeLibrary(srclient);
        committed
    }
}

#[cfg(not(target_os = "windows"))]
pub fn create_native_system_restore_point(_description: &str) -> bool {
    false
}
