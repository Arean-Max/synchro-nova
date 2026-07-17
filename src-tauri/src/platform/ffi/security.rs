

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
            const JOB_OBJECT_LIMIT_PRIORITY_CLASS: u32 = 0x0000_0020;
            const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x0000_4000;

            let mut info = JobObjectExtendedLimitInformation::default();
            info.basic_limit_information.limit_flags =
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                | JOB_OBJECT_LIMIT_BREAKAWAY_OK
                | JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK
                | JOB_OBJECT_LIMIT_PRIORITY_CLASS;
            info.basic_limit_information.priority_class = BELOW_NORMAL_PRIORITY_CLASS;

            let _ = winapi::SetInformationJobObject(
                job,
                JOB_OBJECT_EXTENDED_LIMIT_INFORMATION,
                &mut info as *mut _ as *mut c_void,
                std::mem::size_of::<JobObjectExtendedLimitInformation>() as u32,
            );

            let _ = winapi::SetPriorityClass(winapi::GetCurrentProcess(), BELOW_NORMAL_PRIORITY_CLASS);

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
static REGISTERED_WEBVIEW_PID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn register_webview_pid(pid: u32) {
    #[cfg(target_os = "windows")]
    REGISTERED_WEBVIEW_PID.store(pid, std::sync::atomic::Ordering::Relaxed);
    #[cfg(not(target_os = "windows"))]
    let _ = pid;
}

#[cfg(target_os = "windows")]
pub fn trim_working_set() {
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    static CHILD_CACHE: Mutex<(Vec<u32>, Option<Instant>)> = Mutex::new((Vec::new(), None));
    const CACHE_TTL: Duration = Duration::from_secs(30);

    unsafe {
        let heap = winapi::GetProcessHeap();
        if !heap.is_null() {
            let _ = winapi::HeapCompact(heap, 0);
        }
        let _ = winapi::SetProcessWorkingSetSize(winapi::GetCurrentProcess(), usize::MAX, usize::MAX);

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
                let root_wv = REGISTERED_WEBVIEW_PID.load(std::sync::atomic::Ordering::Relaxed);
                if root_wv != 0 && root_wv != my_pid {
                    tree_pids.push(root_wv);
                    discovered.push(root_wv);
                }

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

        const PROCESS_TRIM_ACCESS: u32 = 0x0100 | 0x0400;
        for pid in webview_pids {
            let handle = winapi::OpenProcess(PROCESS_TRIM_ACCESS, 0, pid);
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
        const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x0000_4000;
        let _ = winapi::SetPriorityClass(winapi::GetCurrentProcess(), BELOW_NORMAL_PRIORITY_CLASS);
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
