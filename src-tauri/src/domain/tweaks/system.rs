use super::runner::{
    delete_hklm_value, hkcu_dword, hkcu_string, run_command, run_command_output,
    set_hkcu_dword, set_hkcu_string, set_hklm_dword,
};
use super::types::{collect_result, TweakApplyResult};

pub fn apply_pointer_precision_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_string("Control Panel\\Mouse", "MouseSpeed", "0"),
            set_hkcu_string("Control Panel\\Mouse", "MouseThreshold1", "0"),
            set_hkcu_string("Control Panel\\Mouse", "MouseThreshold2", "0"),
        ],
        "Pointer precision disabled",
    )
}

pub fn is_pointer_precision_off_applied() -> bool {
    hkcu_string("Control Panel\\Mouse", "MouseSpeed").as_deref() == Some("0")
        && hkcu_string("Control Panel\\Mouse", "MouseThreshold1").as_deref() == Some("0")
        && hkcu_string("Control Panel\\Mouse", "MouseThreshold2").as_deref() == Some("0")
}

pub fn apply_sticky_keys_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [
            set_hkcu_string("Control Panel\\Accessibility\\StickyKeys", "Flags", "506"),
            set_hkcu_string("Control Panel\\Accessibility\\Keyboard Response", "Flags", "98"),
            set_hkcu_string("Control Panel\\Accessibility\\ToggleKeys", "Flags", "58"),
        ],
        "Sticky Keys and accessibility gaming popups disabled",
    )
}

pub fn is_sticky_keys_off_applied() -> bool {
    hkcu_string("Control Panel\\Accessibility\\StickyKeys", "Flags").as_deref() == Some("506")
}

pub fn apply_usb_selective_suspend(id: &str) -> TweakApplyResult {
    let results = [
        run_command(
            "powercfg",
            &[
                "/setacvalueindex",
                "SCHEME_CURRENT",
                "SUB_USB",
                "USBSELECTIVE",
                "0",
            ],
        ),
        run_command(
            "powercfg",
            &[
                "/setdcvalueindex",
                "SCHEME_CURRENT",
                "SUB_USB",
                "USBSELECTIVE",
                "0",
            ],
        ),
        run_command("powercfg", &["/setactive", "SCHEME_CURRENT"]),
    ];
    collect_result(
        id,
        results,
        "USB selective suspend disabled for current power plan",
    )
}

pub fn is_usb_selective_suspend_applied() -> bool {
    run_command_output("powercfg", &["/query", "SCHEME_CURRENT", "SUB_USB", "USBSELECTIVE"])
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("0x00000000"))
        .unwrap_or(false)
}

pub fn apply_input_response_fast(id: &str) -> TweakApplyResult {
    let r1 = set_hkcu_string("Control Panel\\Keyboard", "KeyboardDelay", "0");
    let r2 = set_hkcu_string("Control Panel\\Keyboard", "KeyboardSpeed", "31");
    let r3 = set_hkcu_string("Control Panel\\Mouse", "MouseHoverTime", "8");
    let r4 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\mouclass\\Parameters",
        "MouseDataQueueSize",
        100,
    );
    let r5 = set_hklm_dword(
        "SYSTEM\\CurrentControlSet\\Services\\kbdclass\\Parameters",
        "KeyboardDataQueueSize",
        100,
    );
    collect_result(
        id,
        [r1, r2, r3, r4, r5],
        "Input response latency optimized (keyboard delay 0, instant mouse hover, high queue sizes)",
    )
}

pub fn is_input_response_fast_applied() -> bool {
    hkcu_string("Control Panel\\Keyboard", "KeyboardDelay").as_deref() == Some("0")
        && hkcu_string("Control Panel\\Keyboard", "KeyboardSpeed").as_deref() == Some("31")
}

pub fn clean_legacy_csrss_override() {
    let _ = delete_hklm_value(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\csrss.exe\\PerfOptions",
        "CpuPriorityClass",
    );
}

pub fn apply_visual_effects_performance(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects",
            "VisualFXSetting",
            2,
        )],
        "Visual effects set to performance profile",
    )
}

pub fn is_visual_effects_performance_applied() -> bool {
    hkcu_dword("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VisualEffects", "VisualFXSetting")
        == Some(2)
}

pub fn apply_transparency_off(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_dword(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
            "EnableTransparency",
            0,
        )],
        "Windows transparency disabled",
    )
}

pub fn is_transparency_off_applied() -> bool {
    hkcu_dword(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        "EnableTransparency",
    ) == Some(0)
}

pub fn apply_menu_show_delay_low(id: &str) -> TweakApplyResult {
    collect_result(
        id,
        [set_hkcu_string(
            "Control Panel\\Desktop",
            "MenuShowDelay",
            "100",
        )],
        "Desktop menu delay reduced",
    )
}

pub fn is_menu_show_delay_low_applied() -> bool {
    hkcu_string("Control Panel\\Desktop", "MenuShowDelay").as_deref() == Some("100")
}
