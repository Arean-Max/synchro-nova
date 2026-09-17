#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod prerequisites_check {
    use std::path::{Path, PathBuf};
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

    const WEBVIEW_BOOTSTRAPPER_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";
    const VC_REDIST_X64_URL: &str = "https://aka.ms/vs/17/release/vc_redist.x64.exe";

    use synchro_lib::ffi::{
        download_url_to_file, show_message_box, IDOK, MB_ICONINFORMATION,
        MB_ICONWARNING, MB_OKCANCEL, MB_SETFOREGROUND, MB_TOPMOST,
    };

    pub fn ensure_runtime_prerequisites() {
        let missing_webview2 = !is_webview2_installed();
        let missing_vc_redist = !is_vc_redist_installed();

        if !missing_webview2 && !missing_vc_redist {
            return;
        }

        let mut missing_list = Vec::new();
        if missing_webview2 {
            missing_list.push("• Microsoft Edge WebView2 Runtime (компонент интерфейса)");
        }
        if missing_vc_redist {
            missing_list.push("• Microsoft Visual C++ 2015–2022 Redistributable (x64)");
        }

        let title = "Synchro Nova — Первоначальная настройка";
        let prompt = format!(
            "При первом запуске обнаружено отсутствие необходимых компонентов Windows:\n\n{}\n\n\
            Synchro Nova автоматически загрузит и установит их с официальных серверов Microsoft.\n\n\
            Нажмите «ОК», чтобы начать (займёт около 1 минуты). Приложение запустится автоматически.",
            missing_list.join("\n")
        );

        let choice = show_message_box(
            title,
            &prompt,
            MB_OKCANCEL | MB_ICONINFORMATION | MB_SETFOREGROUND | MB_TOPMOST,
        );

        if choice != IDOK {
            std::process::exit(0);
        }

        let temp_dir = std::env::temp_dir();

        // 1. Install VC++ Redistributable if missing
        if missing_vc_redist {
            let vc_installer = temp_dir.join("vc_redist.x64.exe");
            if download_file(VC_REDIST_X64_URL, &vc_installer) && vc_installer.exists() {
                let _ = run_installer_silent(&vc_installer, &["/install", "/quiet", "/norestart"]);
                let _ = std::fs::remove_file(&vc_installer);
            }
        }

        // 2. Install WebView2 Runtime if missing
        if missing_webview2 {
            let webview_installer = temp_dir.join("MicrosoftEdgeWebview2Setup.exe");
            if download_file(WEBVIEW_BOOTSTRAPPER_URL, &webview_installer) && webview_installer.exists() {
                let _ = run_installer_silent(&webview_installer, &["/silent", "/install"]);
                let _ = std::fs::remove_file(&webview_installer);

                // Wait up to 60 seconds for silent installer to finish registering
                let start = std::time::Instant::now();
                while start.elapsed() < std::time::Duration::from_secs(60) {
                    if is_webview2_installed() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1500));
                }
            }
        }

        // 3. Verify that WebView2 is available and relaunch smoothly
        if is_webview2_installed() {
            if let Ok(current_exe) = std::env::current_exe() {
                let _ = std::process::Command::new(current_exe).spawn();
                std::process::exit(0);
            }
        } else {
            let err_msg = "Не удалось автоматически завершить установку компонентов.\n\n\
                Убедитесь, что компьютер подключен к интернету, или установите Microsoft Edge WebView2 вручную с официального сайта Microsoft.";
            show_message_box(
                title,
                err_msg,
                MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST,
            );
            std::process::exit(1);
        }
    }

    pub(crate) fn is_vc_redist_installed() -> bool {
        let hives = [
            (HKEY_LOCAL_MACHINE, "SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\X64"),
            (HKEY_LOCAL_MACHINE, "SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\X64"),
        ];

        for (hive, subkey) in hives {
            let root = RegKey::predef(hive);
            if let Ok(key) = root.open_subkey(subkey) {
                if let Ok(installed) = key.get_value::<u32, _>("Installed") {
                    if installed == 1 {
                        return true;
                    }
                }
            }
        }

        if let Ok(sys_root) = std::env::var("SystemRoot") {
            let dll = PathBuf::from(sys_root).join("System32\\vcruntime140.dll");
            if dll.exists() {
                return true;
            }
        }

        false
    }

    pub(crate) fn is_webview2_installed() -> bool {
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

    fn run_installer_silent(exe_path: &Path, args: &[&str]) -> bool {
        let mut cmd = std::process::Command::new(exe_path);
        cmd.args(args);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
        }
        cmd.status().map(|s| s.success()).unwrap_or(false)
    }

    fn download_file(url: &str, dest: &Path) -> bool {
        // Tier 1: WinAPI URLDownloadToFileW
        if download_url_to_file(url, dest).is_ok() && dest.exists() && file_has_content(dest) {
            return true;
        }

        // Tier 2: System curl.exe
        if let Ok(sys_root) = std::env::var("SystemRoot") {
            let curl_exe = PathBuf::from(sys_root).join("System32\\curl.exe");
            if curl_exe.exists() {
                let mut cmd = std::process::Command::new(curl_exe);
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x0800_0000);
                }
                let status = cmd.args(["-sSL", url, "-o", &dest.to_string_lossy()]).status();
                if status.map(|s| s.success()).unwrap_or(false) && dest.exists() && file_has_content(dest) {
                    return true;
                }
            }
        }

        // Tier 3: PowerShell Net.WebClient with TLS 1.2
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

        status.map(|s| s.success()).unwrap_or(false) && dest.exists() && file_has_content(dest)
    }

    fn file_has_content(path: &Path) -> bool {
        std::fs::metadata(path).map(|m| m.len() > 1024).unwrap_or(false)
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        const MUTEX_NAME: &str = "Local\\SynchroNovaSingleInstanceMutex";
        const WINDOW_TITLE: &str = "Synchro Nova";
        if !synchro_lib::ffi::ensure_single_instance(MUTEX_NAME, WINDOW_TITLE) {
            std::process::exit(0);
        }
        prerequisites_check::ensure_runtime_prerequisites();
    }

    synchro_lib::run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_instance_checker() {
        #[cfg(target_os = "windows")]
        {
            let res = synchro_lib::ffi::ensure_single_instance("Local\\SynchroNovaTestMutex", "Synchro Nova Test");
            assert!(res);
        }
    }

    #[test]
    fn test_prerequisites_checkers_run() {
        #[cfg(target_os = "windows")]
        {
            let wb = super::prerequisites_check::is_webview2_installed();
            let vc = super::prerequisites_check::is_vc_redist_installed();
            println!("CHECKERS: webview2={}, vc_redist={}", wb, vc);
        }
    }
}
