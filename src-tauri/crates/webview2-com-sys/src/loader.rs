use std::sync::OnceLock;

#[cfg(target_arch = "x86_64")]
const EMBEDDED_LOADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/loader_x64.bin"));

#[cfg(target_arch = "x86")]
const EMBEDDED_LOADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/loader_x86.bin"));

#[cfg(target_arch = "aarch64")]
const EMBEDDED_LOADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/loader_arm64.bin"));

type HMODULE = *mut core::ffi::c_void;
type FARPROC = *mut core::ffi::c_void;

extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> HMODULE;
    fn GetModuleHandleW(lpModuleName: *const u16) -> HMODULE;
    fn GetProcAddress(hModule: HMODULE, lpProcName: *const u8) -> FARPROC;
    fn FreeLibrary(hLibModule: HMODULE) -> i32;
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
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
    h_file: *mut core::ffi::c_void,
    pg_known_subject: *mut core::ffi::c_void,
}

#[repr(C)]
struct WinTrustData {
    cb_struct: u32,
    p_policy_callback_data: *mut core::ffi::c_void,
    p_sip_client_data: *mut core::ffi::c_void,
    dw_ui_choice: u32,
    fdw_revocation_checks: u32,
    dw_union_choice: u32,
    p_file: *mut WinTrustFileInfo,
    dw_state_action: u32,
    h_wvt_state_data: *mut core::ffi::c_void,
    pwsz_url_reference: *mut u16,
    dw_prov_flags: u32,
    dw_ui_context: u32,
    p_signature_settings: *mut core::ffi::c_void,
}

/// Verifies that a given PE binary has a valid, untampered Authenticode signature
/// using Windows native WinVerifyTrust provider.
fn verify_authenticode(path: &std::path::Path) -> bool {
    if !path.is_file() {
        return false;
    }

    const WTD_UI_NONE: u32 = 2;
    const WTD_REVOKE_NONE: u32 = 0;
    const WTD_CHOICE_FILE: u32 = 1;
    const WTD_STATEACTION_VERIFY: u32 = 1;
    const WTD_STATEACTION_CLOSE: u32 = 2;
    const WTD_SAFER_FLAG: u32 = 0x0000_0100;
    const WTD_REVOCATION_CHECK_NONE: u32 = 0x0000_0010;
    const WTD_CACHE_ONLY_URL_RETRIEVAL: u32 = 0x0000_1000;

    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };
    let path_w = to_wide(path_str);

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
        let wintrust_dll = LoadLibraryW(to_wide("wintrust.dll").as_ptr());
        if wintrust_dll.is_null() {
            return false;
        }

        type WinVerifyTrustFn = unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *const Guid,
            *mut WinTrustData,
        ) -> i32;

        let proc = GetProcAddress(wintrust_dll, b"WinVerifyTrust\0".as_ptr());
        if proc.is_null() {
            let _ = FreeLibrary(wintrust_dll);
            return false;
        }

        let verify_fn: WinVerifyTrustFn = core::mem::transmute(proc);
        let status = verify_fn(std::ptr::null_mut(), &action_guid, &mut trust_data);

        trust_data.dw_state_action = WTD_STATEACTION_CLOSE;
        let _ = verify_fn(std::ptr::null_mut(), &action_guid, &mut trust_data);

        let _ = FreeLibrary(wintrust_dll);
        status == 0
    }
}

/// Ensures the candidate is an authentic Microsoft WebView2Loader binary matching
/// both byte length and valid Authenticode signature.
fn is_trusted_loader(path: &std::path::Path) -> bool {
    if !path.is_file() {
        return false;
    }
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.len() != EMBEDDED_LOADER.len() as u64 {
            return false;
        }
    } else {
        return false;
    }
    verify_authenticode(path)
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

            // 2. Check next to current executable (standard setup installer and portable zip)
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(dir) = exe_path.parent() {
                    let candidate = dir.join("WebView2Loader.dll");
                    if candidate.is_file() && is_trusted_loader(&candidate) {
                        let wide = to_wide(candidate.to_str().unwrap_or_default());
                        h = LoadLibraryW(wide.as_ptr());
                        if !h.is_null() {
                            return h as usize;
                        }
                    }
                }
            }

            // 3. Check installed location from Program Files (protected by admin privileges)
            if let Ok(prog_files) = std::env::var("ProgramFiles") {
                let candidate = std::path::PathBuf::from(prog_files)
                    .join("Synchro Nova")
                    .join("WebView2Loader.dll");
                if candidate.is_file() && is_trusted_loader(&candidate) {
                    let wide = to_wide(candidate.to_str().unwrap_or_default());
                    h = LoadLibraryW(wide.as_ptr());
                    if !h.is_null() {
                        return h as usize;
                    }
                }
            }

            // 4. Check installed location from NSIS per-user setup
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                let candidate = std::path::PathBuf::from(&local_app_data)
                    .join("Programs")
                    .join("synchro")
                    .join("WebView2Loader.dll");
                if candidate.is_file() && is_trusted_loader(&candidate) {
                    let wide = to_wide(candidate.to_str().unwrap_or_default());
                    h = LoadLibraryW(wide.as_ptr());
                    if !h.is_null() {
                        return h as usize;
                    }
                }
            }

            // 5. Standalone portable fallback: write genuine Microsoft-signed runtime loader
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                let dir = std::path::PathBuf::from(local_app_data).join("SynchroNova").join("bin");
                let _ = std::fs::create_dir_all(&dir);
                let dll_path = dir.join("WebView2Loader.dll");

                let trusted = is_trusted_loader(&dll_path);
                if !trusted {
                    // Overwrite any missing, truncated, or tampered file with genuine Microsoft bytes
                    let _ = std::fs::write(&dll_path, EMBEDDED_LOADER);
                }

                // Verify Authenticode signature before calling LoadLibraryW
                if dll_path.is_file() && is_trusted_loader(&dll_path) {
                    let wide = to_wide(dll_path.to_str().unwrap_or_default());
                    h = LoadLibraryW(wide.as_ptr());
                    if !h.is_null() {
                        return h as usize;
                    }
                }
            }

            // 6. Fallback to Windows default search path (System32)
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
