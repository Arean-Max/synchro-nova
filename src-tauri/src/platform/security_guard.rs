use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

const BLOCKED_PROCESS_TARGETS: &[&str] = &[
    "rustclient.exe",
    "rust.exe",
    "easyanticheat.exe",
    "easyanticheat_eos.exe",
    "beservice.exe",
    "vgc.exe",
    "cs2.exe",
    "valorant.exe",
    "fortniteclient-win64-shipping.exe",
    "pubg.exe",
    "apex.exe",
    "r5apex.exe",
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
    pub blocked_incidents_count: usize,
    pub recent_incidents: Vec<SecurityIncident>,
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

pub fn get_security_guard_status() -> SecurityGuardStatus {
    let incidents = if let Ok(lock) = incidents_store().lock() {
        lock.clone()
    } else {
        Vec::new()
    };

    SecurityGuardStatus {
        memory_shield_active: true,
        anticheat_safe_mode: true,
        injection_blocker_active: true,
        ifeo_tamper_blocker_active: true,
        blocked_incidents_count: incidents.len(),
        recent_incidents: incidents.into_iter().rev().take(10).collect(),
    }
}
