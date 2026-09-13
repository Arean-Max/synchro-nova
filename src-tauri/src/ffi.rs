#![allow(dead_code)]

#[cfg(target_os = "windows")]
use std::ffi::c_void;
use std::path::Path;

pub fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FileTime {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryStatusEx {
    pub dw_length: u32,
    pub dw_memory_load: u32,
    pub ull_total_phys: u64,
    pub ull_avail_phys: u64,
    pub ull_total_page_file: u64,
    pub ull_avail_page_file: u64,
    pub ull_total_virtual: u64,
    pub ull_avail_virtual: u64,
    pub ull_avail_extended_virtual: u64,
}

impl Default for MemoryStatusEx {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.dw_length = std::mem::size_of::<Self>() as u32;
        s
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DisplayDeviceW {
    pub cb: u32,
    pub device_name: [u16; 32],
    pub device_string: [u16; 128],
    pub state_flags: u32,
    pub device_id: [u16; 128],
    pub device_key: [u16; 128],
}

impl Default for DisplayDeviceW {
    fn default() -> Self {
        let mut d: Self = unsafe { std::mem::zeroed() };
        d.cb = std::mem::size_of::<Self>() as u32;
        d
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PointL {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DevModeW {
    pub dm_device_name: [u16; 32],
    pub dm_spec_version: u16,
    pub dm_driver_version: u16,
    pub dm_size: u16,
    pub dm_driver_extra: u16,
    pub dm_fields: u32,
    pub dm_position: PointL,
    pub dm_display_orientation: u32,
    pub dm_display_fixed_output: u32,
    pub dm_color: i16,
    pub dm_duplex: i16,
    pub dm_y_resolution: i16,
    pub dm_tt_option: i16,
    pub dm_collate: i16,
    pub dm_form_name: [u16; 32],
    pub dm_log_pixels: u16,
    pub dm_bits_per_pel: u32,
    pub dm_pels_width: u32,
    pub dm_pels_height: u32,
    pub dm_display_flags: u32,
    pub dm_display_frequency: u32,
    pub dm_icm_method: u32,
    pub dm_icm_intent: u32,
    pub dm_media_type: u32,
    pub dm_dither_type: u32,
    pub dm_reserved1: u32,
    pub dm_reserved2: u32,
    pub dm_panning_width: u32,
    pub dm_panning_height: u32,
}

impl Default for DevModeW {
    fn default() -> Self {
        let mut d: Self = unsafe { std::mem::zeroed() };
        d.dm_size = std::mem::size_of::<Self>() as u16;
        d
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MagColorEffect {
    pub transform: [f32; 25],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GammaRamp {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}

impl Default for GammaRamp {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VsFixedFileInfo {
    pub dw_signature: u32,
    pub dw_struc_version: u32,
    pub dw_file_version_ms: u32,
    pub dw_file_version_ls: u32,
    pub dw_product_version_ms: u32,
    pub dw_product_version_ls: u32,
    pub dw_file_flags_mask: u32,
    pub dw_file_flags: u32,
    pub dw_file_os: u32,
    pub dw_file_type: u32,
    pub dw_file_subtype: u32,
    pub dw_file_date_ms: u32,
    pub dw_file_date_ls: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcessEntry32W {
    pub dw_size: u32,
    pub cnt_usage: u32,
    pub th32_process_id: u32,
    pub th32_default_heap_id: usize,
    pub th32_module_id: u32,
    pub cnt_threads: u32,
    pub th32_parent_process_id: u32,
    pub pc_pri_class_base: i32,
    pub dw_flags: u32,
    pub sz_exe_file: [u16; 260],
}

impl Default for ProcessEntry32W {
    fn default() -> Self {
        let mut entry: Self = unsafe { std::mem::zeroed() };
        entry.dw_size = std::mem::size_of::<Self>() as u32;
        entry
    }
}

pub const MB_OKCANCEL: u32 = 0x0000_0001;
pub const MB_ICONWARNING: u32 = 0x0000_0030;
pub const MB_ICONINFORMATION: u32 = 0x0000_0040;
pub const MB_SETFOREGROUND: u32 = 0x0001_0000;
pub const MB_TOPMOST: u32 = 0x0004_0000;
pub const IDOK: i32 = 1;
pub const SW_SHOWNORMAL: i32 = 1;
pub const ENUM_CURRENT_SETTINGS: u32 = 0xFFFF_FFFF;
pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;
pub const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
pub const ERROR_ALREADY_EXISTS: u32 = 183;
pub const SW_RESTORE: i32 = 9;
pub const SYNCHRONIZE: u32 = 0x0010_0000;

#[cfg(target_os = "windows")]
pub mod winapi {
    use super::*;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        pub fn GetCurrentProcess() -> *mut c_void;
        pub fn GetCurrentProcessId() -> u32;
        pub fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> *mut c_void;
        pub fn WaitForSingleObject(handle: *mut c_void, milliseconds: u32) -> u32;
        pub fn CreateMutexW(
            mutex_attributes: *mut c_void,
            initial_owner: i32,
            name: *const u16,
        ) -> *mut c_void;
        pub fn GetLastError() -> u32;
        pub fn CreateJobObjectW(job_attributes: *mut c_void, name: *const u16) -> *mut c_void;
        pub fn AssignProcessToJobObject(job: *mut c_void, process: *mut c_void) -> i32;
        pub fn SetInformationJobObject(
            job: *mut c_void,
            info_class: i32,
            job_info: *mut c_void,
            job_info_length: u32,
        ) -> i32;
        pub fn QueryInformationJobObject(
            job: *mut c_void,
            info_class: i32,
            job_info: *mut c_void,
            job_info_length: u32,
            return_length: *mut u32,
        ) -> i32;
        pub fn CloseHandle(handle: *mut c_void) -> i32;
        pub fn SetProcessWorkingSetSize(process: *mut c_void, min: usize, max: usize) -> i32;
        pub fn SetProcessDEPPolicy(flags: u32) -> i32;
        pub fn SetSearchPathMode(flags: u32) -> i32;
        pub fn SetDefaultDllDirectories(flags: u32) -> i32;
        pub fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
        pub fn GetSystemTimes(
            idle_time: *mut FileTime,
            kernel_time: *mut FileTime,
            user_time: *mut FileTime,
        ) -> i32;
        pub fn CreateToolhelp32Snapshot(flags: u32, process_id: u32) -> *mut c_void;
        pub fn Process32FirstW(snapshot: *mut c_void, entry: *mut ProcessEntry32W) -> i32;
        pub fn Process32NextW(snapshot: *mut c_void, entry: *mut ProcessEntry32W) -> i32;
        pub fn GetProcessHeap() -> *mut c_void;
        pub fn HeapSetInformation(
            heap_handle: *mut c_void,
            heap_information_class: i32,
            heap_information: *mut c_void,
            heap_information_length: usize,
        ) -> i32;
        pub fn LoadLibraryW(lib_file_name: *const u16) -> *mut c_void;
        pub fn GetProcAddress(module: *mut c_void, proc_name: *const u8) -> *mut c_void;
        pub fn FreeLibrary(module: *mut c_void) -> i32;
    }

    #[link(name = "Shell32")]
    unsafe extern "system" {
        pub fn ShellExecuteW(
            hwnd: *mut c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show_cmd: i32,
        ) -> isize;
    }

    #[link(name = "User32")]
    unsafe extern "system" {
        pub fn MessageBoxW(
            hwnd: *mut c_void,
            text: *const u16,
            caption: *const u16,
            utype: u32,
        ) -> i32;
        pub fn FindWindowW(class_name: *const u16, window_name: *const u16) -> *mut c_void;
        pub fn ShowWindow(hwnd: *mut c_void, cmd_show: i32) -> i32;
        pub fn SetForegroundWindow(hwnd: *mut c_void) -> i32;
        pub fn EnumDisplayDevicesW(
            device_name: *const u16,
            dev_num: u32,
            display_device: *mut DisplayDeviceW,
            flags: u32,
        ) -> i32;
        pub fn EnumDisplaySettingsW(
            device_name: *const u16,
            mode_num: u32,
            dev_mode: *mut DevModeW,
        ) -> i32;
        pub fn GetSystemMetrics(index: i32) -> i32;
        pub fn GetDC(hwnd: *mut c_void) -> *mut c_void;
        pub fn ReleaseDC(hwnd: *mut c_void, hdc: *mut c_void) -> i32;
        pub fn SendMessageTimeoutW(
            hwnd: *mut c_void,
            msg: u32,
            w_param: usize,
            l_param: isize,
            flags: u32,
            timeout: u32,
            result: *mut usize,
        ) -> isize;
        pub fn keybd_event(
            b_vk: u8,
            b_scan: u8,
            dw_flags: u32,
            dw_extra_info: usize,
        );
    }

    #[link(name = "Gdi32")]
    unsafe extern "system" {
        pub fn SetDeviceGammaRamp(hdc: *mut c_void, ramp: *const GammaRamp) -> i32;
    }

    #[link(name = "Magnification")]
    unsafe extern "system" {
        pub fn MagInitialize() -> i32;
        pub fn MagSetFullscreenColorEffect(effect: *const MagColorEffect) -> i32;
    }

    #[link(name = "Advapi32")]
    unsafe extern "system" {
        pub fn OpenProcessToken(
            process_handle: *mut c_void,
            desired_access: u32,
            token_handle: *mut *mut c_void,
        ) -> i32;
        pub fn GetTokenInformation(
            token_handle: *mut c_void,
            token_information_class: u32,
            token_information: *mut c_void,
            token_information_length: u32,
            return_length: *mut u32,
        ) -> i32;
    }

    #[link(name = "Version")]
    unsafe extern "system" {
        pub fn GetFileVersionInfoSizeW(file_name: *const u16, handle: *mut u32) -> u32;
        pub fn GetFileVersionInfoW(
            file_name: *const u16,
            handle: u32,
            len: u32,
            data: *mut c_void,
        ) -> i32;
        pub fn VerQueryValueW(
            block: *const c_void,
            sub_block: *const u16,
            buffer: *mut *mut c_void,
            len: *mut u32,
        ) -> i32;
    }
}

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

            let mut info = JobObjectExtendedLimitInformation::default();
            info.basic_limit_information.limit_flags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

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

                // Track process IDs belonging strictly to synchro's process hierarchy
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
    const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x0000_1000;
    const HEAP_ENABLE_TERMINATION_ON_CORRUPTION: i32 = 1;

    unsafe {
        // 1. Enforce Data Execution Prevention (DEP)
        let _ = winapi::SetProcessDEPPolicy(PROCESS_DEP_ENABLE);

        // 2. Enforce Safe Search Mode permanently (blocks DLL search in current working directory)
        let _ = winapi::SetSearchPathMode(
            BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT,
        );

        // 3. Restrict default DLL search directories to application dir and System32,
        // permanently eliminating Current Working Directory (CWD) binary planting / DLL hijacking
        let _ = winapi::SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);

        // 4. Terminate process immediately if heap corruption occurs (blocks heap exploitation)
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
pub fn open_path_or_url(target: &str) -> Result<(), String> {
    let operation = wide_null("open");
    let file = wide_null(target);
    let result = unsafe {
        winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    if result <= 32 {
        Err(format!("ShellExecuteW failed for target: {target}"))
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn open_path_or_url(_target: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn runas_executable(path: &Path) -> Result<(), String> {
    runas_executable_with_args(path, None)
}

#[cfg(target_os = "windows")]
pub fn runas_executable_with_args(path: &Path, args: Option<&str>) -> Result<(), String> {
    let operation = wide_null("runas");
    let file = wide_null(&path.to_string_lossy());
    let params = args.map(wide_null);
    let params_ptr = params.as_ref().map(|p| p.as_ptr()).unwrap_or(std::ptr::null());
    let dir = path.parent().map(|p| wide_null(&p.to_string_lossy()));
    let dir_ptr = dir.as_ref().map(|d| d.as_ptr()).unwrap_or(std::ptr::null());

    let result = unsafe {
        winapi::ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            params_ptr,
            dir_ptr,
            SW_SHOWNORMAL,
        )
    };
    if result <= 32 {
        Err("Failed to execute process with elevated privileges".to_string())
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable_with_args(_path: &Path, _args: Option<&str>) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn restart_explorer() -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;

    // 1. Terminate existing explorer instances cleanly
    let mut kill_cmd = Command::new("taskkill");
    kill_cmd.args(["/F", "/IM", "explorer.exe"]);
    kill_cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    let _ = kill_cmd.status();

    // 2. Wait a brief moment for handle cleanup
    std::thread::sleep(std::time::Duration::from_millis(350));

    // 3. Broadcast WM_SETTINGCHANGE before spawning new explorer
    unsafe {
        let env_w = wide_null("Environment");
        let mut result: usize = 0;
        let _ = winapi::SendMessageTimeoutW(
            0xFFFF as *mut std::ffi::c_void, // HWND_BROADCAST
            0x001A,                          // WM_SETTINGCHANGE
            0,
            env_w.as_ptr() as isize,
            2,                               // SMTO_ABORTIFHUNG
            1000,
            &mut result,
        );
    }

    // 4. Launch fresh explorer.exe from SystemRoot
    let explorer_exe = if let Ok(root) = std::env::var("SystemRoot") {
        std::path::PathBuf::from(root).join("explorer.exe")
    } else {
        std::path::PathBuf::from("explorer.exe")
    };

    let mut start_cmd = Command::new(&explorer_exe);
    start_cmd.creation_flags(0x0800_0000);
    start_cmd
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to restart explorer: {e}"))
}

#[cfg(not(target_os = "windows"))]
pub fn restart_explorer() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn restart_graphics_driver() -> Result<(), String> {
    const VK_LWIN: u8 = 0x5B;
    const VK_CONTROL: u8 = 0x11;
    const VK_SHIFT: u8 = 0x10;
    const VK_B: u8 = 0x42;
    const KEYEVENTF_KEYUP: u32 = 0x0002;

    unsafe {
        winapi::keybd_event(VK_LWIN, 0, 0, 0);
        winapi::keybd_event(VK_CONTROL, 0, 0, 0);
        winapi::keybd_event(VK_SHIFT, 0, 0, 0);
        winapi::keybd_event(VK_B, 0, 0, 0);

        std::thread::sleep(std::time::Duration::from_millis(60));

        winapi::keybd_event(VK_B, 0, KEYEVENTF_KEYUP, 0);
        winapi::keybd_event(VK_SHIFT, 0, KEYEVENTF_KEYUP, 0);
        winapi::keybd_event(VK_CONTROL, 0, KEYEVENTF_KEYUP, 0);
        winapi::keybd_event(VK_LWIN, 0, KEYEVENTF_KEYUP, 0);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn restart_graphics_driver() -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn show_message_box(caption: &str, text: &str, utype: u32) -> i32 {
    let text_w = wide_null(text);
    let cap_w = wide_null(caption);
    unsafe {
        winapi::MessageBoxW(
            std::ptr::null_mut(),
            text_w.as_ptr(),
            cap_w.as_ptr(),
            utype,
        )
    }
}

#[cfg(not(target_os = "windows"))]
pub fn show_message_box(_caption: &str, _text: &str, _utype: u32) -> i32 {
    1
}


#[cfg(target_os = "windows")]
pub fn read_memory_status_ex() -> Option<MemoryStatusEx> {
    let mut status = MemoryStatusEx::default();
    let ok = unsafe { winapi::GlobalMemoryStatusEx(&mut status) != 0 };
    if ok {
        Some(status)
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
pub fn read_memory_status_ex() -> Option<MemoryStatusEx> {
    None
}

#[cfg(target_os = "windows")]
pub fn read_system_times() -> Option<(FileTime, FileTime, FileTime)> {
    let mut idle = FileTime::default();
    let mut kernel = FileTime::default();
    let mut user = FileTime::default();
    let ok = unsafe { winapi::GetSystemTimes(&mut idle, &mut kernel, &mut user) != 0 };
    if ok {
        Some((idle, kernel, user))
    } else {
        None
    }
}

#[cfg(not(target_os = "windows"))]
pub fn read_system_times() -> Option<(FileTime, FileTime, FileTime)> {
    None
}

#[cfg(target_os = "windows")]
static SINGLE_INSTANCE_MUTEX_HANDLE: std::sync::atomic::AtomicPtr<c_void> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

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
pub fn release_single_instance() {
    let handle = SINGLE_INSTANCE_MUTEX_HANDLE.swap(std::ptr::null_mut(), std::sync::atomic::Ordering::SeqCst);
    if !handle.is_null() {
        unsafe {
            winapi::CloseHandle(handle);
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn release_single_instance() {}

#[cfg(target_os = "windows")]
pub fn ensure_single_instance(mutex_name: &str, window_title: &str) -> bool {
    ensure_single_instance_retry(mutex_name, window_title, false)
}

#[cfg(target_os = "windows")]
pub fn ensure_single_instance_retry(mutex_name: &str, window_title: &str, is_restart: bool) -> bool {
    let max_attempts = if is_restart { 30 } else { 1 };
    let name_w = wide_null(mutex_name);

    for attempt in 0..max_attempts {
        let handle = unsafe {
            winapi::CreateMutexW(std::ptr::null_mut(), 1, name_w.as_ptr())
        };

        if handle.is_null() {
            return true;
        }

        let last_err = unsafe { winapi::GetLastError() };
        if last_err == ERROR_ALREADY_EXISTS {
            unsafe {
                winapi::CloseHandle(handle);
            }

            if attempt + 1 < max_attempts {
                std::thread::sleep(std::time::Duration::from_millis(150));
                continue;
            }

            let title_w = wide_null(window_title);
            let hwnd = unsafe {
                winapi::FindWindowW(std::ptr::null(), title_w.as_ptr())
            };
            if !hwnd.is_null() {
                unsafe {
                    winapi::ShowWindow(hwnd, SW_RESTORE);
                    winapi::SetForegroundWindow(hwnd);
                }
            }
            return false;
        }

        SINGLE_INSTANCE_MUTEX_HANDLE.store(handle, std::sync::atomic::Ordering::SeqCst);
        return true;
    }

    false
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance(_mutex_name: &str, _window_title: &str) -> bool {
    true
}

#[cfg(not(target_os = "windows"))]
pub fn ensure_single_instance_retry(_mutex_name: &str, _window_title: &str, _is_restart: bool) -> bool {
    true
}

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

        winapi::FreeLibrary(srclient);
        success != 0 && status.n_status == 0
    }
}

#[cfg(not(target_os = "windows"))]
pub fn create_native_system_restore_point(_description: &str) -> bool {
    false
}


