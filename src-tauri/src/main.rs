#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
mod prerequisites_check {
    use std::path::{Path, PathBuf};
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };


    use synchro_lib::ffi::{
        show_message_box, IDOK, MB_ICONINFORMATION, MB_OKCANCEL, MB_SETFOREGROUND, MB_TOPMOST,
    };

    pub fn ensure_runtime_prerequisites() {
        let missing_webview2 = !is_webview2_installed();
        let missing_vc_redist = !is_vc_redist_installed();

        if !missing_webview2 && !missing_vc_redist {
            return;
        }

        let mut missing_list = Vec::new();
        let mut target_url = "https://developer.microsoft.com/microsoft-edge/webview2/";
        if missing_webview2 {
            missing_list.push("• Microsoft Edge WebView2 Runtime");
            target_url = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";
        }
        if missing_vc_redist {
            missing_list.push("• Microsoft Visual C++ 2015–2022 Redistributable (x64)");
            if !missing_webview2 {
                target_url = "https://aka.ms/vs/17/release/vc_redist.x64.exe";
            }
        }

        let title = "Synchro Nova";
        let prompt = format!(
            "Для запуска приложения требуются компоненты Microsoft:\n\n{}\n\n\
            Открыть страницу загрузки на официальном сайте Microsoft?",
            missing_list.join("\n")
        );

        let choice = show_message_box(
            title,
            &prompt,
            MB_OKCANCEL | MB_ICONINFORMATION | MB_SETFOREGROUND | MB_TOPMOST,
        );

        if choice == IDOK {
            let _ = synchro_lib::ffi::open_path_or_url(target_url);
        }

        std::process::exit(0);
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

    #[allow(dead_code)]
    pub(crate) fn is_authenticode_valid(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        // Native WinVerifyTrust in-process verification (clean heuristics, zero child processes)
        synchro_lib::ffi::verify_embedded_signature(path)
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

        // Optimize WebView2 runtime: eliminate telemetry, background network chatter, and minimize RAM safely
        if std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS").is_err() {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                "--disable-background-networking \
                 --disable-component-update \
                 --disable-domain-reliability \
                 --disable-sync \
                 --no-pings \
                 --disable-client-side-phishing-detection \
                 --safebrowsing-disable-auto-update \
                 --disable-default-apps \
                 --disable-extensions \
                 --disable-breakpad \
                 --no-crash-upload \
                 --disable-speech-api \
                 --disable-speech-synthesis-api \
                 --disable-notifications \
                 --disable-wake-on-wifi \
                 --disable-features=Translate,OptimizationHints,MediaRouter,DialMediaRouteProvider,InterestFeedContentSuggestions,SpeechSynthesis \
                 --disk-cache-size=16777216 \
                 --media-cache-size=8388608",
            );
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

    #[test]
    fn test_native_winverifytrust() {
        #[cfg(target_os = "windows")]
        {
            let non_existent = std::path::PathBuf::from("C:\\synchro_invalid_non_existent_file.exe");
            assert!(!synchro_lib::ffi::verify_embedded_signature(&non_existent));

            // Test on installed WebView2 or Edge binaries which are signed with embedded Authenticode
            let candidates = [
                r"C:\Program Files (x86)\Microsoft\EdgeWebView\Application",
                r"C:\Program Files\Microsoft\EdgeWebView\Application",
                r"C:\Program Files (x86)\Microsoft\Edge\Application",
            ];
            for candidate in candidates {
                if let Ok(entries) = std::fs::read_dir(candidate) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let exe = entry.path().join("msedgewebview2.exe");
                            if exe.is_file() {
                                assert!(super::prerequisites_check::is_authenticode_valid(&exe));
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
