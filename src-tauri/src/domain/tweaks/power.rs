use super::runner::{
    hklm_dword, output_guid, run_command, run_command_output, set_hklm_dword, set_hklm_string,
};
use super::types::{applied, collect_result, failed, TweakApplyResult};

pub fn active_power_scheme_text() -> String {
    run_command_output("powercfg", &["/getactivescheme"])
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

pub fn is_power_plan_high_applied() -> bool {
    let scheme = active_power_scheme_text();
    scheme.contains("8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c")
        || scheme.to_lowercase().contains("high performance")
        || scheme.contains("Высокая производительность")
}

pub fn apply_ultimate_performance(id: &str) -> TweakApplyResult {
    const ULTIMATE_GUID: &str = "e9a42b02-d5df-448d-aa00-03f14749eb61";

    let duplicate = run_command_output("powercfg", &["/duplicatescheme", ULTIMATE_GUID]);
    let mut candidates = Vec::new();
    let mut last_error = None;

    match duplicate {
        Ok(output) => {
            if let Some(guid) = output_guid(&output) {
                candidates.push(guid);
            }
        }
        Err(error) => {
            last_error = Some(error);
        }
    }

    if !candidates
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(ULTIMATE_GUID))
    {
        candidates.push(ULTIMATE_GUID.to_string());
    }

    for guid in candidates {
        let args = ["/setactive", guid.as_str()];
        match run_command("powercfg", &args) {
            Ok(_) => return applied(id, "Ultimate Performance power plan selected"),
            Err(error) => last_error = Some(error),
        }
    }

    failed(
        id,
        &last_error.unwrap_or_else(|| {
            "Unable to activate Ultimate Performance power plan; run Synchro as administrator"
                .to_string()
        }),
    )
}

pub fn is_ultimate_performance_applied() -> bool {
    let scheme = active_power_scheme_text();
    scheme.contains("e9a42b02-d5df-448d-aa00-03f14749eb61")
        || scheme.to_lowercase().contains("ultimate performance")
        || scheme.contains("Максимальная производительность")
}

pub fn apply_cpu_unpark_cores(id: &str) -> TweakApplyResult {
    let _ = run_command("powercfg", &["-setacvalueindex", "SCHEME_CURRENT", "SUB_PROCESSOR", "CPMINCORES", "100"]);
    let _ = run_command("powercfg", &["-setacvalueindex", "SCHEME_CURRENT", "SUB_PROCESSOR", "CPMAXCORES", "100"]);
    let _ = run_command("powercfg", &["-setactive", "SCHEME_CURRENT"]);
    collect_result(
        id,
        [
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
                "ValueMax",
                0,
            ),
            set_hklm_dword(
                "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
                "ValueMin",
                0,
            ),
        ],
        "All CPU cores unparked (100% active, zero wake-up stutter)",
    )
}

pub fn is_cpu_unpark_cores_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\54533251-82be-4824-96c1-47b60b740d00\\0cc5b647-6429-45d6-8e05-69d96c744b5c",
        "ValueMax",
    ) == Some(0)
}

pub fn apply_power_throttling_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling",
            "PowerThrottlingOff",
            1,
        )],
        "Windows Power Throttling globally disabled (sustained maximum CPU clocks)",
    )
}

pub fn is_power_throttling_off_applied() -> bool {
    hklm_dword(
        "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling",
        "PowerThrottlingOff",
    ) == Some(1)
}

pub fn apply_mmcss_games_priority(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "GPU Priority",
                8,
            ),
            set_hklm_dword(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "Priority",
                6,
            ),
            set_hklm_string(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "Scheduling Category",
                "High",
            ),
            set_hklm_string(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
                "SFIO Priority",
                "High",
            ),
        ],
        "MMCSS games priority updated; reboot recommended",
    )
}

pub fn is_mmcss_games_priority_applied() -> bool {
    hklm_dword(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        "GPU Priority",
    ) == Some(8)
}

pub fn apply_system_responsiveness(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
            "SystemResponsiveness",
            10,
        )],
        "System responsiveness reserve set to 10; reboot recommended",
    )
}

pub fn is_system_responsiveness_applied() -> bool {
    let val = hklm_dword(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "SystemResponsiveness",
    );
    val == Some(10) || val == Some(0)
}

pub fn apply_network_throttle_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hklm_dword(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
            "NetworkThrottlingIndex",
            0xffff_ffff,
        )],
        "MMCSS network throttling disabled",
    )
}

pub fn is_network_throttle_off_applied() -> bool {
    hklm_dword(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        "NetworkThrottlingIndex",
    ) == Some(0xffff_ffff)
}
