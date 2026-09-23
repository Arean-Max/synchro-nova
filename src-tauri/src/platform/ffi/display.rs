#![allow(dead_code)]

use super::types::*;
use super::winapi;

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
pub fn restart_graphics_driver() -> Result<(), String> {
    // Hardware display subsystem reset without synthetic keyboard injection (eliminating anti-cheat flags)
    unsafe {
        let _ = winapi::ChangeDisplaySettingsW(std::ptr::null_mut(), 0);
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn restart_graphics_driver() -> Result<(), String> {
    Ok(())
}
