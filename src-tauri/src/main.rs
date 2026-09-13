#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod webview_check {
    use std::{
        ffi::c_void,
        path::{Path, PathBuf},
    };
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    const WEBVIEW_BOOTSTRAPPER_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

    #[link(name = "User32")]
    unsafe extern "system" {
        fn MessageBoxW(
            hwnd: *mut c_void,
            text: *const u16,
            caption: *const u16,
            utype: u32,
        ) -> i32;
    }

    #[link(name = "Shell32")]
    unsafe extern "system" {
        fn ShellExecuteW(
            hwnd: *mut c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show_cmd: i32,
        ) -> isize;
    }

    #[link(name = "urlmon")]
    unsafe extern "system" {
        fn URLDownloadToFileW(
            p_caller: *mut c_void,
            sz_url: *const u16,
            sz_file_name: *const u16,
            dw_reserved: u32,
            lpfn_cb: *mut c_void,
        ) -> i32;
    }

    const MB_OKCANCEL: u32 = 0x0000_0001;
    const MB_ICONWARNING: u32 = 0x0000_0030;
    const MB_ICONINFORMATION: u32 = 0x0000_0040;
    const MB_SETFOREGROUND: u32 = 0x0001_0000;
    const MB_TOPMOST: u32 = 0x0004_0000;
    const IDOK: i32 = 1;
    const SW_SHOWNORMAL: i32 = 1;

    fn wide_null(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn ensure_webview2_available() {
        if is_webview2_installed() {
            return;
        }

        let title = wide_null("Synchro Nova — WebView2 Runtime");
        let prompt = wide_null(
            "Microsoft Edge WebView2 Runtime не найден на вашем компьютере.\n\n\
            Для работы Synchro Nova требуется этот компонент.\n\n\
            Нажмите «ОК», чтобы скачать и установить его автоматически, или «Отмена» для выхода."
        );

        let choice = unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                prompt.as_ptr(),
                title.as_ptr(),
                MB_OKCANCEL | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST,
            )
        };

        if choice == IDOK {
            let temp_installer = std::env::temp_dir().join("MicrosoftEdgeWebview2Setup.exe");
            let downloaded = download_file(WEBVIEW_BOOTSTRAPPER_URL, &temp_installer);

            if downloaded && temp_installer.exists() {
                let op = wide_null("open");
                let file = wide_null(&temp_installer.to_string_lossy());

                unsafe {
                    ShellExecuteW(
                        std::ptr::null_mut(),
                        op.as_ptr(),
                        file.as_ptr(),
                        std::ptr::null(),
                        std::ptr::null(),
                        SW_SHOWNORMAL,
                    );
                }

                let notice = wide_null(
                    "Установщик WebView2 запущен.\n\n\
                    После завершения установки перезапустите Synchro Nova."
                );
                unsafe {
                    MessageBoxW(
                        std::ptr::null_mut(),
                        notice.as_ptr(),
                        title.as_ptr(),
                        MB_ICONINFORMATION | MB_SETFOREGROUND,
                    );
                }
            } else {
                let err_msg = wide_null(
                    "Не удалось автоматически загрузить установщик.\n\n\
                    Пожалуйста, скачайте Microsoft Edge WebView2 Runtime вручную с официального сайта Microsoft."
                );
                unsafe {
                    MessageBoxW(
                        std::ptr::null_mut(),
                        err_msg.as_ptr(),
                        title.as_ptr(),
                        MB_ICONWARNING | MB_SETFOREGROUND,
                    );
                }
            }
        }

        std::process::exit(0);
    }

    fn is_webview2_installed() -> bool {
        const GUIDS: &[&str] = &[
            "{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
            "{F3017226-9E47-4762-B580-BD29424F88FB}",
        ];

        let hives = [
            (HKEY_LOCAL_MACHINE, "SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients"),
            (HKEY_LOCAL_MACHINE, "SOFTWARE\\Microsoft\\EdgeUpdate\\Clients"),
            (HKEY_CURRENT_USER, "SOFTWARE\\Microsoft\\EdgeUpdate\\Clients"),
        ];

        for (hive, base_path) in hives {
            let root = RegKey::predef(hive);
            for guid in GUIDS {
                let subkey = format!("{base_path}\\{guid}");
                if let Ok(key) = root.open_subkey(&subkey) {
                    if let Ok(version) = key.get_value::<String, _>("pv") {
                        let trimmed = version.trim();
                        if !trimmed.is_empty() && trimmed != "0.0.0.0" {
                            return true;
                        }
                    }
                }
            }
        }

        let mut candidate_dirs = Vec::new();
        if let Ok(prog_x86) = std::env::var("ProgramFiles(x86)") {
            candidate_dirs.push(PathBuf::from(prog_x86).join("Microsoft\\EdgeWebView\\Application"));
        }
        if let Ok(prog) = std::env::var("ProgramFiles") {
            candidate_dirs.push(PathBuf::from(prog).join("Microsoft\\EdgeWebView\\Application"));
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            candidate_dirs.push(PathBuf::from(local).join("Microsoft\\EdgeWebView\\Application"));
        }
        if let Ok(drive) = std::env::var("SystemDrive") {
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files (x86)\\Microsoft\\EdgeWebView\\Application")));
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files\\Microsoft\\EdgeWebView\\Application")));
        }

        for dir in candidate_dirs {
            if has_webview2_in_dir(&dir) {
                return true;
            }
        }

        false
    }

    fn has_webview2_in_dir(path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let exe = entry.path().join("msedgewebview2.exe");
                    if exe.exists() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn download_file(url: &str, dest: &Path) -> bool {
        let wide_url = wide_null(url);
        let wide_dest = wide_null(&dest.to_string_lossy());

        let res = unsafe {
            URLDownloadToFileW(
                std::ptr::null_mut(),
                wide_url.as_ptr(),
                wide_dest.as_ptr(),
                0,
                std::ptr::null_mut(),
            )
        };

        if res == 0 && dest.exists() {
            return true;
        }

        let ps_exe = std::env::var("SystemRoot")
            .map(|root| PathBuf::from(root).join("System32\\WindowsPowerShell\\v1.0\\powershell.exe"))
            .unwrap_or_else(|_| PathBuf::from("powershell.exe"));
        let mut cmd = std::process::Command::new(ps_exe);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x0800_0000);
        }
        let safe_url = url.replace('\'', "''");
        let safe_dest = dest.to_string_lossy().replace('\'', "''");
        let status = cmd
            .args([
                "-NoProfile",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &format!(
                    "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; (New-Object Net.WebClient).DownloadFile('{safe_url}', '{safe_dest}')"
                ),
            ])
            .status();

        status.map(|s| s.success()).unwrap_or(false)
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    webview_check::ensure_webview2_available();

    synchro_lib::run();
}
