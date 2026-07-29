use super::runner::{
    hkcu_dword, hkcu_string, run_command, run_command_output,
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
