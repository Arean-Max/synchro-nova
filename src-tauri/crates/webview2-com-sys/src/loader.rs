use std::sync::OnceLock;

#[cfg(target_arch = "x86_64")]
const LOADER_BYTES: &[u8] = include_bytes!("../x64/WebView2Loader.dll");

#[cfg(target_arch = "x86")]
const LOADER_BYTES: &[u8] = include_bytes!("../x86/WebView2Loader.dll");

#[cfg(target_arch = "aarch64")]
const LOADER_BYTES: &[u8] = include_bytes!("../arm64/WebView2Loader.dll");

type HMODULE = *mut core::ffi::c_void;
type FARPROC = *mut core::ffi::c_void;

extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> HMODULE;
    fn GetModuleHandleW(lpModuleName: *const u16) -> HMODULE;
    fn GetProcAddress(hModule: HMODULE, lpProcName: *const u8) -> FARPROC;
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn get_or_load_module() -> HMODULE {
    static MODULE: OnceLock<usize> = OnceLock::new();
    let handle_val = *MODULE.get_or_init(|| {
        unsafe {
            let name = to_wide("WebView2Loader.dll");

            // 1. Check if already loaded in this process
            let mut h = GetModuleHandleW(name.as_ptr());
            if !h.is_null() {
                return h as usize;
            }

            // 2. Check if next to current executable
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(dir) = exe_path.parent() {
                    let candidate = dir.join("WebView2Loader.dll");
                    if candidate.exists() {
                        let wide = to_wide(candidate.to_str().unwrap_or_default());
                        h = LoadLibraryW(wide.as_ptr());
                        if !h.is_null() {
                            return h as usize;
                        }
                    }
                }
            }

            // 3. Ensure unpacked in %LOCALAPPDATA%\SynchroNova\WebView2Loader.dll
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                let dir = std::path::PathBuf::from(local_app_data).join("SynchroNova");
                let _ = std::fs::create_dir_all(&dir);
                let dll_path = dir.join("WebView2Loader.dll");

                let need_write = match std::fs::metadata(&dll_path) {
                    Ok(meta) => meta.len() != LOADER_BYTES.len() as u64,
                    Err(_) => true,
                };

                if need_write {
                    let _ = std::fs::write(&dll_path, LOADER_BYTES);
                }

                if dll_path.exists() {
                    let wide = to_wide(dll_path.to_str().unwrap_or_default());
                    h = LoadLibraryW(wide.as_ptr());
                    if !h.is_null() {
                        return h as usize;
                    }
                }
            }

            // 4. Fallback to %TEMP%\SynchroNova\WebView2Loader.dll
            let temp_dir = std::env::temp_dir().join("SynchroNova");
            let _ = std::fs::create_dir_all(&temp_dir);
            let temp_dll = temp_dir.join("WebView2Loader.dll");
            let _ = std::fs::write(&temp_dll, LOADER_BYTES);
            if temp_dll.exists() {
                let wide = to_wide(temp_dll.to_str().unwrap_or_default());
                h = LoadLibraryW(wide.as_ptr());
                if !h.is_null() {
                    return h as usize;
                }
            }

            // 5. Final fallback to search path
            h = LoadLibraryW(name.as_ptr());
            h as usize
        }
    });

    handle_val as HMODULE
}

const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(-2147467259);

pub unsafe fn CompareBrowserVersions(
    version1: windows_core::PCWSTR,
    version2: windows_core::PCWSTR,
    result: *mut i32,
) -> windows_core::HRESULT {
    type FnType = unsafe extern "system" fn(
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut i32,
    ) -> windows_core::HRESULT;

    let h = get_or_load_module();
    if h.is_null() {
        return E_FAIL;
    }
    let proc = GetProcAddress(h, b"CompareBrowserVersions\0".as_ptr());
    if proc.is_null() {
        return E_FAIL;
    }
    let f: FnType = core::mem::transmute(proc);
    f(version1, version2, result)
}

pub unsafe fn CreateCoreWebView2Environment(
    environmentcreatedhandler: *mut core::ffi::c_void,
) -> windows_core::HRESULT {
    type FnType = unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT;

    let h = get_or_load_module();
    if h.is_null() {
        return E_FAIL;
    }
    let proc = GetProcAddress(h, b"CreateCoreWebView2Environment\0".as_ptr());
    if proc.is_null() {
        return E_FAIL;
    }
    let f: FnType = core::mem::transmute(proc);
    f(environmentcreatedhandler)
}

pub unsafe fn CreateCoreWebView2EnvironmentWithOptions(
    browserexecutablefolder: windows_core::PCWSTR,
    userdatafolder: windows_core::PCWSTR,
    environmentoptions: *mut core::ffi::c_void,
    environmentcreatedhandler: *mut core::ffi::c_void,
) -> windows_core::HRESULT {
    type FnType = unsafe extern "system" fn(
        windows_core::PCWSTR,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT;

    let h = get_or_load_module();
    if h.is_null() {
        return E_FAIL;
    }
    let proc = GetProcAddress(h, b"CreateCoreWebView2EnvironmentWithOptions\0".as_ptr());
    if proc.is_null() {
        return E_FAIL;
    }
    let f: FnType = core::mem::transmute(proc);
    f(
        browserexecutablefolder,
        userdatafolder,
        environmentoptions,
        environmentcreatedhandler,
    )
}

pub unsafe fn GetAvailableCoreWebView2BrowserVersionString(
    browserexecutablefolder: windows_core::PCWSTR,
    versioninfo: *mut windows_core::PWSTR,
) -> windows_core::HRESULT {
    type FnType = unsafe extern "system" fn(
        windows_core::PCWSTR,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT;

    let h = get_or_load_module();
    if h.is_null() {
        return E_FAIL;
    }
    let proc = GetProcAddress(h, b"GetAvailableCoreWebView2BrowserVersionString\0".as_ptr());
    if proc.is_null() {
        return E_FAIL;
    }
    let f: FnType = core::mem::transmute(proc);
    f(browserexecutablefolder, versioninfo)
}

pub unsafe fn GetAvailableCoreWebView2BrowserVersionStringWithOptions(
    browserexecutablefolder: windows_core::PCWSTR,
    environmentoptions: *mut core::ffi::c_void,
    versioninfo: *mut windows_core::PWSTR,
) -> windows_core::HRESULT {
    type FnType = unsafe extern "system" fn(
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT;

    let h = get_or_load_module();
    if h.is_null() {
        return E_FAIL;
    }
    let proc = GetProcAddress(
        h,
        b"GetAvailableCoreWebView2BrowserVersionStringWithOptions\0".as_ptr(),
    );
    if proc.is_null() {
        return E_FAIL;
    }
    let f: FnType = core::mem::transmute(proc);
    f(browserexecutablefolder, environmentoptions, versioninfo)
}
