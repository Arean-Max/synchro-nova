
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
    pub fn HeapCompact(heap_handle: *mut c_void, flags: u32) -> usize;
    pub fn LoadLibraryW(lib_file_name: *const u16) -> *mut c_void;
    pub fn GetProcAddress(module: *mut c_void, proc_name: *const u8) -> *mut c_void;
    pub fn FreeLibrary(module: *mut c_void) -> i32;
    pub fn SetConsoleCtrlHandler(
        handler_routine: Option<unsafe extern "system" fn(u32) -> i32>,
        add: i32,
    ) -> i32;
    pub fn SetPriorityClass(process: *mut c_void, priority_class: u32) -> i32;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetModuleHandleW(module_name: *const u16) -> *mut c_void;
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
