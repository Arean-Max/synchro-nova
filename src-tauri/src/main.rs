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
        let vc_installer = temp_dir.join("vc_redist.x64.exe");
        let webview_installer = temp_dir.join("MicrosoftEdgeWebview2Setup.exe");

        if missing_vc_redist {
            if download_file(VC_REDIST_X64_URL, &vc_installer)
                && vc_installer.exists()
                && is_authenticode_valid(&vc_installer)
            {
                let _ = run_installer_silent(&vc_installer, &["/install", "/quiet", "/norestart"]);
            }
            if vc_installer.exists() {
                let _ = std::fs::remove_file(&vc_installer);
            }
        }

        if missing_webview2 {
            if download_file(WEBVIEW_BOOTSTRAPPER_URL, &webview_installer)
                && webview_installer.exists()
                && is_authenticode_valid(&webview_installer)
            {
                let _ = run_installer_silent(&webview_installer, &["/silent", "/install"]);

                let start = std::time::Instant::now();
                while start.elapsed() < std::time::Duration::from_secs(60) {
                    if is_webview2_installed() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1500));
                }
            }
            if webview_installer.exists() {
                let _ = std::fs::remove_file(&webview_installer);
            }
        }

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

        // Check local portable runtime directory next to executable
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                let local_wv = parent.join("webview2");
                if has_webview2_in_dir(&local_wv) || local_wv.join("msedgewebview2.exe").exists() {
                    std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", &local_wv);
                    return true;
                }
                let runtimes_wv = parent.join("runtimes").join("webview2");
                if has_webview2_in_dir(&runtimes_wv) || runtimes_wv.join("msedgewebview2.exe").exists() {
                    std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", &runtimes_wv);
                    return true;
                }
            }
        }

        let mut candidate_dirs = Vec::new();
        if let Ok(prog_x86) = std::env::var("ProgramFiles(x86)") {
            candidate_dirs.push(PathBuf::from(&prog_x86).join("Microsoft\\EdgeWebView\\Application"));
            candidate_dirs.push(PathBuf::from(&prog_x86).join("Microsoft\\Edge\\Application"));
        }
        if let Ok(prog) = std::env::var("ProgramFiles") {
            candidate_dirs.push(PathBuf::from(&prog).join("Microsoft\\EdgeWebView\\Application"));
            candidate_dirs.push(PathBuf::from(&prog).join("Microsoft\\Edge\\Application"));
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            candidate_dirs.push(PathBuf::from(local).join("Microsoft\\EdgeWebView\\Application"));
        }
        if let Ok(drive) = std::env::var("SystemDrive") {
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files (x86)\\Microsoft\\EdgeWebView\\Application")));
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files\\Microsoft\\EdgeWebView\\Application")));
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files (x86)\\Microsoft\\Edge\\Application")));
            candidate_dirs.push(PathBuf::from(format!("{drive}\\Program Files\\Microsoft\\Edge\\Application")));
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
                    let edge = entry.path().join("msedge.exe");
                    if exe.exists() || edge.exists() {
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

    pub(crate) fn is_authenticode_valid(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        // 1. Try in-process native WinVerifyTrust first (ultra fast, zero child processes)
        if synchro_lib::ffi::verify_embedded_signature(path) {
            return true;
        }

        // 2. Safe fallback to PowerShell if signature requires catalog verification
        let ps_exe = std::env::var("SystemRoot")
            .map(|root| PathBuf::from(root).join("System32\\WindowsPowerShell\\v1.0\\powershell.exe"))
            .unwrap_or_else(|_| PathBuf::from("powershell.exe"));
        let path_str = path.to_string_lossy().replace('\'', "''");
        let script = format!(
            "$sig = Get-AuthenticodeSignature -LiteralPath '{path_str}'; if ($sig.Status -eq 'Valid') {{ exit 0 }} else {{ exit 1 }}"
        );
        let mut cmd = std::process::Command::new(ps_exe);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x0800_0000);
        }
        cmd.args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn download_file(url: &str, dest: &Path) -> bool {
        if download_url_to_file(url, dest).is_ok() && dest.exists() && file_has_content(dest) {
            return true;
        }

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
        let script = format!(
            "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; (New-Object Net.WebClient).DownloadFile('{safe_url}', '{safe_dest}')"
        );
        let status = cmd
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
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
        let mut restarted_from_pid: Option<u32> = None;
        let args: Vec<String> = std::env::args().collect();
        for i in 0..args.len() {
            if args[i] == "--restarted-from-pid" && i + 1 < args.len() {
                if let Ok(pid) = args[i + 1].parse::<u32>() {
                    restarted_from_pid = Some(pid);
                }
            }
        }

        if let Some(old_pid) = restarted_from_pid {
            synchro_lib::ffi::wait_for_process_exit(old_pid, 5000);
        }

        const MUTEX_NAME: &str = "Local\\SynchroNovaSingleInstanceMutex";
        const WINDOW_TITLE: &str = "Synchro Nova";
        let is_restart = restarted_from_pid.is_some();
        if !synchro_lib::ffi::ensure_single_instance_retry(MUTEX_NAME, WINDOW_TITLE, is_restart) {
            std::process::exit(0);
        }

        // Isolate WebView2 browser user data and prevent white flash on window create
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let webview_cache = std::path::PathBuf::from(local_app_data)
                .join("SynchroNova")
                .join("EBWebView");
            let _ = std::fs::create_dir_all(&webview_cache);
            std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", webview_cache);
        }
        std::env::set_var("WEBVIEW2_DEFAULT_BACKGROUND_COLOR", "0xFF121214");

        // Optimize WebView2 runtime: eliminate telemetry, background network chatter, and limit memory cache
        if std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").is_err() {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                "--disable-background-networking \
                 --disable-component-update \
                 --disable-domain-reliability \
                 --disable-sync \
                 --no-pings \
                 --disable-client-side-phishing-detection \
                 --disable-breakpad \
                 --disable-speech-api \
                 --disable-features=Translate,OptimizationHints,MediaRouter,DialMediaRouteProvider,CalculateNativeWinOcclusion,InterestFeedContentSuggestions \
                 --disk-cache-size=33554432 \
                 --media-cache-size=16777216",
            );
        }

        prerequisites_check::ensure_runtime_prerequisites();
        ensure_windows_search_registered();
    }

    synchro_lib::run();
}

fn ensure_windows_search_registered() {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;

        if let Ok(current_exe) = std::env::current_exe() {
            let exe_str = current_exe.to_string_lossy().to_string();
            let parent_dir = current_exe
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);

            // 1. Register App Paths for "synchro.exe"
            if let Ok((key, _)) =
                hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\App Paths\synchro.exe")
            {
                let _ = key.set_value("", &exe_str);
                let _ = key.set_value("Path", &parent_dir);
            }

            // 2. Register App Paths for "synchro" (without extension)
            if let Ok((key, _)) =
                hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\App Paths\synchro")
            {
                let _ = key.set_value("", &exe_str);
                let _ = key.set_value("Path", &parent_dir);
            }

            // 3. Register in Applications for Windows Search indexer
            if let Ok((key, _)) =
                hkcu.create_subkey(r"Software\Classes\Applications\synchro.exe")
            {
                let _ = key.set_value("FriendlyAppName", &"Synchro");
                let _ = key.set_value("ApplicationName", &"Synchro");
            }
        }
    }
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

    #[test]
    fn test_native_winverifytrust() {
        #[cfg(target_os = "windows")]
        {
            let non_existent = std::path::PathBuf::from("C:\\synchro_invalid_non_existent_file.exe");
            assert!(!synchro_lib::ffi::verify_embedded_signature(&non_existent));

            if let Ok(sys_root) = std::env::var("SystemRoot") {
                let explorer = std::path::PathBuf::from(sys_root).join("explorer.exe");
                if explorer.exists() {
                    assert!(super::prerequisites_check::is_authenticode_valid(&explorer));
                }
            }
        }
    }
}
