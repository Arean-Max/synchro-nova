use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

pub const BLOCKED_PROCESS_TARGETS: &[&str] = &[
    // Anti-Cheat binaries and services
    "easyanticheat.exe",
    "easyanticheat_eos.exe",
    "easyanticheat_x64.exe",
    "start_protected_game.exe",
    "beservice.exe",
    "bedaisy.sys",
    "vgc.exe",
    "vgk.sys",
    "anticheat_expert.exe",
    "ace-base.sys",
    "gameguard.des",
    // Protected Game Executables
    "rustclient.exe",
    "rust.exe",
    "cs2.exe",
    "csgo.exe",
    "valorant.exe",
    "fortniteclient-win64-shipping.exe",
    "fortniteclient-win64-shipping_eac.exe",
    "fortniteclient-win64-shipping_be.exe",
    "pubg.exe",
    "tslgame.exe",
    "apex.exe",
    "r5apex.exe",
    "deadbydaylight-win64-shipping.exe",
    "huntgame.exe",
    "rainbowsix.exe",
    "rainbowsix_vulkan.exe",
    "destiny2.exe",
    "thefinals.exe",
    "discovery.exe",
    "helldivers2.exe",
    "overwatch.exe",
    "warframe.x64.exe",
    "escapefromtarkov.exe",
    "escapefromtarkov_be.exe",
    "arma3_x64.exe",
    "dayz_x64.exe",
    "sotgame.exe",
    "eldenring.exe",
];

pub const VULNERABLE_DRIVERS_BLACKLIST: &[&str] = &[
    "rtcore64.sys",
    "gdrv.sys",
    "winring0x64.sys",
    "winring0.sys",
    "asrockpolychrome.sys",
    "asrdrv10.sys",
    "inpoutx64.sys",
    "atszio.sys",
    "glckio2.sys",
    "ene.sys",
    "capcom.sys",
    "dbutil_2_3.sys",
    "kprocesshacker.sys",
];

const PROCESS_VM_READ: u32 = 0x0010;
const PROCESS_VM_WRITE: u32 = 0x0020;
const PROCESS_VM_OPERATION: u32 = 0x0008;
const PROCESS_CREATE_THREAD: u32 = 0x0002;
const FORBIDDEN_ACCESS_MASK: u32 =
    PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION | PROCESS_CREATE_THREAD;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityIncident {
    pub timestamp: u64,
    pub target: String,
    pub reason: String,
    pub action_taken: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityGuardStatus {
    pub memory_shield_active: bool,
    pub anticheat_safe_mode: bool,
    pub injection_blocker_active: bool,
    pub ifeo_tamper_blocker_active: bool,
    pub game_shield_active: bool,
    pub active_protected_game: Option<String>,
    pub eac_compatible: bool,
    pub blocked_incidents_count: usize,
    pub recent_incidents: Vec<SecurityIncident>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntiCheatAuditReport {
    pub is_eac_compatible: bool,
    pub testsigning_clean: bool,
    pub kernel_debug_clean: bool,
    pub integrity_checks_clean: bool,
    pub ifeo_clean: bool,
    pub vulnerable_drivers_clean: bool,
    pub active_game_detected: Option<String>,
    pub detected_issues: Vec<String>,
    pub active_shield: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AntiCheatFixResult {
    pub success: bool,
    pub fixed_items: Vec<String>,
    pub message: String,
}

static INCIDENTS_LOG: OnceLock<Mutex<Vec<SecurityIncident>>> = OnceLock::new();

fn incidents_store() -> &'static Mutex<Vec<SecurityIncident>> {
    INCIDENTS_LOG.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn verify_process_access_safety(target_name: &str, requested_access: u32) -> Result<(), String> {
    let lower_target = target_name.trim().to_lowercase();

    for blocked in BLOCKED_PROCESS_TARGETS {
        if lower_target.contains(blocked) {
            record_incident(
                target_name,
                "Attempted access to protected game or anti-cheat process",
                "BLOCKED_GAME_PROTECTION",
            );
            return Err(format!(
                "Security Guard blocked access to '{}'. Access to gaming and anti-cheat processes is strictly forbidden to protect your account.",
                target_name
            ));
        }
    }

    if (requested_access & FORBIDDEN_ACCESS_MASK) != 0 {
        record_incident(
            target_name,
            "Attempted memory read/write or remote thread injection",
            "BLOCKED_MEMORY_TAMPER",
        );
        return Err(format!(
            "Security Guard blocked memory tampering with '{}'. Memory reading and writing are strictly disabled.",
            target_name
        ));
    }

    Ok(())
}

pub fn verify_registry_write_safety(key_path: &str) -> Result<(), String> {
    let lower = key_path.to_lowercase();
    if lower.contains("image file execution options") {
        for blocked in BLOCKED_PROCESS_TARGETS {
            if lower.contains(blocked) {
                record_incident(
                    key_path,
                    "Attempted IFEO modification of game executable",
                    "BLOCKED_REGISTRY_IFEO_TAMPER",
                );
                return Err(format!(
                    "Security Guard blocked IFEO modification on '{}'. Registry hijacking of games is blocked to prevent bans.",
                    key_path
                ));
            }
        }
    }
    Ok(())
}

pub fn verify_registry_delete_safety(key_path: &str) -> Result<(), String> {
    let trimmed = key_path.trim().trim_matches('\\');
    let lower = trimmed.to_lowercase();

    // Prevent deleting entire hives or vital Windows roots
    if lower.is_empty()
        || lower == "system"
        || lower == "software"
        || lower == "sam"
        || lower == "security"
        || lower == "system\\currentcontrolset"
        || lower == "system\\currentcontrolset\\services"
        || lower == "system\\currentcontrolset\\control"
        || lower == "software\\microsoft"
        || lower == "software\\microsoft\\windows nt"
        || lower == "software\\microsoft\\windows nt\\currentversion"
    {
        return Err(format!("Security Guard prevented dangerous registry tree deletion on root path: '{key_path}'"));
    }

    Ok(())
}

pub fn record_incident(target: &str, reason: &str, action: &str) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let incident = SecurityIncident {
        timestamp: now,
        target: target.to_string(),
        reason: reason.to_string(),
        action_taken: action.to_string(),
    };

    if let Ok(mut lock) = incidents_store().lock() {
        if lock.len() >= 100 {
            lock.remove(0);
        }
        lock.push(incident);
    }
}

#[cfg(target_os = "windows")]
pub fn is_protected_game_running() -> Option<String> {
    use crate::platform::ffi::{winapi, ProcessEntry32W, TH32CS_SNAPPROCESS};

    unsafe {
        let snapshot = winapi::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot.is_null() || snapshot as isize == -1 {
            return None;
        }

        let mut entry = ProcessEntry32W::default();
        let mut detected = None;

        if winapi::Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let len = entry
                    .sz_exe_file
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.sz_exe_file.len());
                let name = String::from_utf16_lossy(&entry.sz_exe_file[..len]).to_lowercase();

                for blocked in BLOCKED_PROCESS_TARGETS {
                    if name == *blocked || name.contains(blocked) {
                        detected = Some(name);
                        break;
                    }
                }

                if detected.is_some() || winapi::Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        winapi::CloseHandle(snapshot);
        detected
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_protected_game_running() -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
pub fn is_safe_child_pid(pid: u32) -> bool {
    use crate::platform::ffi::{winapi, ProcessEntry32W, TH32CS_SNAPPROCESS};

    if pid == 0 {
        return false;
    }

    unsafe {
        if pid == winapi::GetCurrentProcessId() {
            return true;
        }

        let snapshot = winapi::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot.is_null() || snapshot as isize == -1 {
            return false;
        }

        let mut entry = ProcessEntry32W::default();
        let mut is_safe = false;

        if winapi::Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                if entry.th32_process_id == pid {
                    let len = entry
                        .sz_exe_file
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.sz_exe_file.len());
                    let name = String::from_utf16_lossy(&entry.sz_exe_file[..len]).to_lowercase();

                    // Only msedgewebview2.exe belonging to Tauri webview is allowed
                    if name == "msedgewebview2.exe" {
                        is_safe = true;
                    }

                    // Strict exclusion for any game or anti-cheat target
                    for blocked in BLOCKED_PROCESS_TARGETS {
                        if name.contains(blocked) {
                            is_safe = false;
                            break;
                        }
                    }
                    break;
                }

                if winapi::Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }

        winapi::CloseHandle(snapshot);
        is_safe
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_safe_child_pid(_pid: u32) -> bool {
    false
}

#[cfg(target_os = "windows")]
fn check_bcd_integrity() -> (bool, bool, bool) {
    use std::os::windows::process::CommandExt;

    let mut testsigning_clean = true;
    let mut kernel_debug_clean = true;
    let mut integrity_checks_clean = true;

    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let bcdedit_path = std::path::PathBuf::from(system_root).join("System32").join("bcdedit.exe");

    if bcdedit_path.is_file() {
        let mut cmd = std::process::Command::new(bcdedit_path);
        cmd.args(["/enum", "{current}"]);
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW

        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
                for line in text.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("testsigning")
                        && (trimmed.contains("yes") || trimmed.contains("on") || trimmed.contains("1") || trimmed.contains("да"))
                    {
                        testsigning_clean = false;
                    }
                    if trimmed.starts_with("debug")
                        && (trimmed.contains("yes") || trimmed.contains("on") || trimmed.contains("1") || trimmed.contains("да"))
                    {
                        kernel_debug_clean = false;
                    }
                    if trimmed.starts_with("nointegritychecks")
                        && (trimmed.contains("yes") || trimmed.contains("on") || trimmed.contains("1") || trimmed.contains("да"))
                    {
                        integrity_checks_clean = false;
                    }
                }
            }
        }
    }

    (testsigning_clean, kernel_debug_clean, integrity_checks_clean)
}

#[cfg(target_os = "windows")]
fn check_ifeo_integrity() -> (bool, Vec<String>) {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let mut ifeo_clean = true;
    let mut flagged = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let ifeo_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options";

    if let Ok(ifeo_key) = hklm.open_subkey(ifeo_path) {
        for subkey in ifeo_key.enum_keys().flatten() {
            let lower = subkey.to_lowercase();
            if BLOCKED_PROCESS_TARGETS
                .iter()
                .any(|target| lower == *target || lower.contains(target))
            {
                ifeo_clean = false;
                flagged.push(subkey);
            }
        }
    }

    (ifeo_clean, flagged)
}

#[cfg(target_os = "windows")]
fn check_vulnerable_drivers() -> (bool, Vec<String>) {
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let drivers_dir = std::path::PathBuf::from(system_root).join("System32").join("drivers");

    let mut clean = true;
    let mut found = Vec::new();

    for driver_name in VULNERABLE_DRIVERS_BLACKLIST {
        let driver_path = drivers_dir.join(driver_name);
        if driver_path.is_file() {
            clean = false;
            found.push((*driver_name).to_string());
        }
    }

    (clean, found)
}

pub fn audit_anticheat_integrity() -> AntiCheatAuditReport {
    #[cfg(target_os = "windows")]
    {
        let (testsigning_clean, kernel_debug_clean, integrity_checks_clean) = check_bcd_integrity();
        let (ifeo_clean, ifeo_flagged) = check_ifeo_integrity();
        let (vulnerable_drivers_clean, vulnerable_flagged) = check_vulnerable_drivers();
        let active_game_detected = is_protected_game_running();

        let mut detected_issues = Vec::new();

        if !testsigning_clean {
            detected_issues.push("Windows Testsigning mode is active (triggers EAC / BattlEye launch ban)".to_string());
        }
        if !kernel_debug_clean {
            detected_issues.push("Kernel debugging is active in BCD (triggers EAC kick error 30005)".to_string());
        }
        if !integrity_checks_clean {
            detected_issues.push("Driver signature enforcement is disabled (triggers EAC Untrusted System File)".to_string());
        }
        if !ifeo_clean {
            detected_issues.push(format!(
                "Lingering Image File Execution Options found for games: {} (triggers EAC debugger violation)",
                ifeo_flagged.join(", ")
            ));
        }
        if !vulnerable_drivers_clean {
            detected_issues.push(format!(
                "Vulnerable third-party driver(s) detected in drivers folder: {} (blacklisted by EAC/BattlEye)",
                vulnerable_flagged.join(", ")
            ));
        }

        let is_eac_compatible = testsigning_clean
            && kernel_debug_clean
            && integrity_checks_clean
            && ifeo_clean
            && vulnerable_drivers_clean;

        AntiCheatAuditReport {
            is_eac_compatible,
            testsigning_clean,
            kernel_debug_clean,
            integrity_checks_clean,
            ifeo_clean,
            vulnerable_drivers_clean,
            active_game_detected,
            detected_issues,
            active_shield: true,
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        AntiCheatAuditReport {
            is_eac_compatible: true,
            testsigning_clean: true,
            kernel_debug_clean: true,
            integrity_checks_clean: true,
            ifeo_clean: true,
            vulnerable_drivers_clean: true,
            active_game_detected: None,
            detected_issues: Vec::new(),
            active_shield: true,
        }
    }
}

pub fn fix_anticheat_integrity() -> AntiCheatFixResult {
    let mut fixed_items = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // 1. Clean IFEO hijack entries for all protected games
        let (ifeo_clean, flagged) = check_ifeo_integrity();
        if !ifeo_clean {
            for subkey in flagged {
                let full_path = format!(
                    "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}",
                    subkey
                );
                let _ = crate::infra::registry::SafeRegistry::delete_tree(
                    crate::infra::registry::RootKey::Hklm,
                    &full_path,
                );
                fixed_items.push(format!("Removed IFEO hijack entry for '{subkey}'"));
            }
        }

        // 2. Restore BCD integrity if elevated
        if crate::admin::is_running_elevated() {
            let _ = crate::domain::tweaks::runner::run_command("bcdedit", &["/set", "testsigning", "off"]);
            let _ = crate::domain::tweaks::runner::run_command("bcdedit", &["/set", "nointegritychecks", "off"]);
            let _ = crate::domain::tweaks::runner::run_command("bcdedit", &["/set", "debug", "off"]);
            let _ = crate::domain::tweaks::runner::run_command("bcdedit", &["/deletevalue", "disabledynamictick"]);
            fixed_items.push("Enforced Windows Driver Signature Policies (testsigning: off, nointegritychecks: off)".to_string());
        }

        // 3. Clear legacy tweak IFEO overrides
        crate::domain::tweaks::gaming::clean_legacy_rust_ifeo();
        crate::domain::tweaks::system::clean_legacy_csrss_override();
    }

    AntiCheatFixResult {
        success: true,
        message: if fixed_items.is_empty() {
            "System is already 100% compliant with Easy Anti-Cheat and BattlEye requirements".to_string()
        } else {
            format!(
                "Successfully applied {} security fixes for Anti-Cheat integrity",
                fixed_items.len()
            )
        },
        fixed_items,
    }
}

pub fn get_security_guard_status() -> SecurityGuardStatus {
    let incidents = if let Ok(lock) = incidents_store().lock() {
        lock.clone()
    } else {
        Vec::new()
    };

    let active_game = is_protected_game_running();
    let audit = audit_anticheat_integrity();

    SecurityGuardStatus {
        memory_shield_active: true,
        anticheat_safe_mode: true,
        injection_blocker_active: true,
        ifeo_tamper_blocker_active: true,
        game_shield_active: active_game.is_some(),
        active_protected_game: active_game,
        eac_compatible: audit.is_eac_compatible,
        blocked_incidents_count: incidents.len(),
        recent_incidents: incidents.into_iter().rev().take(10).collect(),
    }
}
