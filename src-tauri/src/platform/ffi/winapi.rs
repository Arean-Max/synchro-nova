#![allow(dead_code)]

#[cfg(target_os = "windows")]
use std::ffi::c_void;
use super::types::*;

#[cfg(target_os = "windows")]
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
    pub fn SetConsoleCtrlHandler(
        handler_routine: Option<unsafe extern "system" fn(u32) -> i32>,
        add: i32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
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

    pub fn ShellExecuteExW(p_exec_info: *mut ShellExecuteInfoW) -> i32;
}

#[cfg(target_os = "windows")]
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
    pub fn ChangeDisplaySettingsW(
        lp_dev_mode: *mut c_void,
        dw_flags: u32,
    ) -> i32;
    pub fn GetAsyncKeyState(v_key: i32) -> i16;
    pub fn RegisterHotKey(hwnd: *mut c_void, id: i32, fs_modifiers: u32, vk: u32) -> i32;
    pub fn UnregisterHotKey(hwnd: *mut c_void, id: i32) -> i32;
    pub fn GetMessageW(
        msg: *mut MSG,
        hwnd: *mut c_void,
        msg_filter_min: u32,
        msg_filter_max: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "Gdi32")]
unsafe extern "system" {
    pub fn SetDeviceGammaRamp(hdc: *mut c_void, ramp: *const GammaRamp) -> i32;
    pub fn GetDeviceGammaRamp(hdc: *mut c_void, ramp: *mut GammaRamp) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "Magnification")]
unsafe extern "system" {
    pub fn MagInitialize() -> i32;
    pub fn MagSetFullscreenColorEffect(effect: *const MagColorEffect) -> i32;
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
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
