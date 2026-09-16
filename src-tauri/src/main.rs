#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod webview_check {
    use std::path::{Path, PathBuf};
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    const WEBVIEW_BOOTSTRAPPER_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

    use synchro_lib::ffi::{
        download_url_to_file, open_path_or_url, show_message_box, IDOK, MB_ICONINFORMATION,
        MB_ICONWARNING, MB_OKCANCEL, MB_SETFOREGROUND, MB_TOPMOST,
    };

    pub fn ensure_webview2_available() {
        if is_webview2_installed() {
            return;
        }

        let title = "Synchro Nova — WebView2 Runtime";
        let prompt = "Microsoft Edge WebView2 Runtime не найден на вашем компьютере.\n\n\
            Для работы Synchro Nova требуется этот компонент.\n\n\
            Нажмите «ОК», чтобы скачать и установить его автоматически, или «Отмена» для выхода.";

        let choice = show_message_box(
            title,
            prompt,
            MB_OKCANCEL | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST,
        );

        if choice == IDOK {
            let temp_installer = std::env::temp_dir().join("MicrosoftEdgeWebview2Setup.exe");
            let downloaded = download_file(WEBVIEW_BOOTSTRAPPER_URL, &temp_installer);

            if downloaded && temp_installer.exists() {
                let _ = open_path_or_url(&temp_installer.to_string_lossy());

                let notice = "Установщик WebView2 запущен.\n\n\
                    После завершения установки перезапустите Synchro Nova.";
                show_message_box(
                    title,
                    notice,
                    MB_ICONINFORMATION | MB_SETFOREGROUND,
                );
            } else {
                let err_msg = "Не удалось автоматически загрузить установщик.\n\n\
                    Пожалуйста, скачайте Microsoft Edge WebView2 Runtime вручную с официального сайта Microsoft.";
                show_message_box(
                    title,
                    err_msg,
                    MB_ICONWARNING | MB_SETFOREGROUND,
                );
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
        if download_url_to_file(url, dest).is_ok() && dest.exists() {
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
