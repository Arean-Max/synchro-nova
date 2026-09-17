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

#[cfg(target_os = "windows")]
pub mod winapi {
    use super::*;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        pub fn GetCurrentProcess() -> *mut c_void;
        pub fn GetCurrentProcessId() -> u32;
        pub fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> *mut c_void;
        pub fn CreateJobObjectW(job_attributes: *mut c_void, name: *const u16) -> *mut c_void;
        pub fn AssignProcessToJobObject(job: *mut c_void, process: *mut c_void) -> i32;
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

    #[link(name = "urlmon")]
    unsafe extern "system" {
        pub fn URLDownloadToFileW(
            p_caller: *mut c_void,
            sz_url: *const u16,
            sz_file_name: *const u16,
            dw_reserved: u32,
            lpfn_cb: *mut c_void,
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
pub fn init_process_tree_job() {
    PROCESS_JOB.get_or_init(|| {
        unsafe {
            let job = winapi::CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
            if job.is_null() || job as isize == -1 {
                return None;
            }
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
    unsafe {
        // 1. Trim the main application process
        let _ = winapi::SetProcessWorkingSetSize(winapi::GetCurrentProcess(), usize::MAX, usize::MAX);

        // 2. Discover direct child and descendant WebView2 processes strictly rooted at this process
        let my_pid = winapi::GetCurrentProcessId();
        let snapshot = winapi::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot.is_null() || snapshot as isize == -1 {
            return;
        }

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

        let mut webview_pids = Vec::with_capacity(8);
        let mut added = true;

        while added {
            added = false;
            for (pid, parent_pid, exe_name) in &all_entries {
                if *pid != my_pid && !tree_pids.contains(pid) && tree_pids.contains(parent_pid) {
                    let len = exe_name.iter().position(|&c| c == 0).unwrap_or(exe_name.len());
                    let name = String::from_utf16_lossy(&exe_name[..len]);
                    if name.eq_ignore_ascii_case("msedgewebview2.exe") {
                        tree_pids.push(*pid);
                        webview_pids.push(*pid);
                        added = true;
                    }
                }
            }
        }

        // 3. Trim working sets for the discovered WebView2 child processes
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

    unsafe {
        let _ = winapi::SetProcessDEPPolicy(PROCESS_DEP_ENABLE);
        let _ = winapi::SetSearchPathMode(
            BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE | BASE_SEARCH_PATH_PERMANENT,
        );
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
    let operation = wide_null("runas");
    let file = wide_null(&path.to_string_lossy());
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
        Err("Failed to execute process with elevated privileges".to_string())
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn runas_executable(_path: &Path) -> Result<(), String> {
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
pub fn download_url_to_file(url: &str, destination: &Path) -> Result<(), String> {
    let sz_url = wide_null(url);
    let sz_file = wide_null(&destination.to_string_lossy());
    let hr = unsafe {
        winapi::URLDownloadToFileW(
            std::ptr::null_mut(),
            sz_url.as_ptr(),
            sz_file.as_ptr(),
            0,
            std::ptr::null_mut(),
        )
    };
    if hr == 0 {
        Ok(())
    } else {
        Err(format!("URLDownloadToFileW returned error HRESULT: {hr:#010x}"))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn download_url_to_file(_url: &str, _destination: &Path) -> Result<(), String> {
    Err("Download not supported on non-Windows platform".to_string())
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
