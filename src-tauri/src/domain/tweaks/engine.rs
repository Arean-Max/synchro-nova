use super::gaming;
use super::network;
use super::power;
use super::runner;
use super::snapshot;
use super::storage;
use super::system;
use super::types::skipped;

pub use snapshot::{collect_tweak_registry_snapshot, restore_tweak_registry_snapshot};
pub use super::types::{TweakApplyResult, TweakRegistrySnapshot, TweakRegistryValue, TweakStatus};

pub fn apply_selected_tweaks(ids: Vec<String>) -> Vec<TweakApplyResult> {
    ids.into_iter().take(80).map(|id| apply_one(&id)).collect()
}

pub fn collect_tweak_statuses() -> Vec<TweakStatus> {
    known_tweak_ids()
        .iter()
        .map(|id| TweakStatus {
            id: (*id).to_string(),
            installed: is_tweak_applied(id),
        })
        .collect()
}

pub(crate) fn is_admin_tweak(id: &str) -> bool {
    matches!(
        id,
        "hags-on"
            | "mpo-disable"
            | "mmcss-games-priority"
            | "system-responsiveness-10"
            | "network-throttle-off"
            | "hibernate-off"
            | "ntfs-last-access-off"
            | "trim-enable"
            | "rss-on"
            | "rsc-off"
            | "ecn-off"
            | "disable-gamedvr"
            | "delivery-optimization-lan"
            | "activity-history-off"
            | "tcp-nodelay-ack"
            | "nic-energy-saving-off"
            | "tcp-heuristics-off"
            | "cpu-unpark-cores"
            | "power-throttling-off"
            | "input-response-fast"
            | "csrss-high-priority"
    )
}

fn apply_one(id: &str) -> TweakApplyResult {
    if is_admin_tweak(id) && !crate::admin::is_running_elevated() {
        return TweakApplyResult {
            id: id.to_string(),
            status: "requiresAdmin".to_string(),
            message: "Requires administrator privileges; restart Synchro as administrator to apply this tweak".to_string(),
        };
    }
    match id {
        // Group 1: Gaming & Latency
        "game-mode-on" => gaming::apply_game_mode(id),
        "modern-flip-model-on" => gaming::apply_modern_flip_model(id),
        "gamedvr-fse-mode" => gaming::apply_gamedvr_fse_mode(id),
        "disable-fso-globally" => gaming::apply_disable_fso_globally(id),
        "hags-on" => gaming::apply_hags(id),
        "mpo-disable" => gaming::apply_mpo_disable(id),
        "pcie-aspm-off" => gaming::apply_pcie_aspm_off(id),

        // Group 2: CPU & Performance
        "power-plan-high" => runner::run_powercfg(
            id,
            &["/setactive", "SCHEME_MIN"],
            "High performance power plan selected",
        ),
        "ultimate-performance-plan" => power::apply_ultimate_performance(id),
        "cpu-unpark-cores" => power::apply_cpu_unpark_cores(id),
        "power-throttling-off" => power::apply_power_throttling_off(id),
        "mmcss-games-priority" => power::apply_mmcss_games_priority(id),
        "system-responsiveness-10" => power::apply_system_responsiveness(id),
        "network-throttle-off" => power::apply_network_throttle_off(id),

        // Group 3: Input & Responsiveness
        "pointer-precision-off" => system::apply_pointer_precision_off(id),
        "sticky-keys-off" => system::apply_sticky_keys_off(id),
        "usb-selective-suspend-off" => system::apply_usb_selective_suspend(id),
        "input-response-fast" => system::apply_input_response_fast(id),
        "csrss-high-priority" => system::apply_csrss_high_priority(id),
        "visual-effects-performance" => system::apply_visual_effects_performance(id),
        "transparency-off" => system::apply_transparency_off(id),
        "menu-show-delay-low" => system::apply_menu_show_delay_low(id),

        // Group 4: Storage & Debloat
        "clean-temp-junk" => storage::apply_clean_temp_junk(id),
        "hibernate-off" => runner::run_command_result(
            id,
            "powercfg",
            &["/hibernate", "off"],
            "Hibernate disabled; Fast Startup also disabled",
        ),
        "ntfs-last-access-off" => runner::run_command_result(
            id,
            "fsutil",
            &["behavior", "set", "disableLastAccess", "1"],
            "NTFS last access updates disabled",
        ),
        "trim-enable" => runner::run_command_result(
            id,
            "fsutil",
            &["behavior", "set", "DisableDeleteNotify", "0"],
            "TRIM notifications enabled",
        ),

        // Group 5: Network Latency
        "tcp-nodelay-ack" => network::apply_tcp_nodelay_ack(id),
        "nic-energy-saving-off" => network::apply_nic_energy_saving_off(id),
        "tcp-heuristics-off" => network::apply_tcp_heuristics_off(id),
        "rss-on" => runner::run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rss=enabled"],
            "Receive-side scaling enabled",
        ),
        "rsc-off" => runner::run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rsc=disabled"],
            "Receive segment coalescing disabled",
        ),
        "ecn-off" => runner::run_netsh(
            id,
            &[
                "interface",
                "tcp",
                "set",
                "global",
                "ecncapability=disabled",
            ],
            "ECN disabled",
        ),
        "dns-cache-flush" => {
            runner::run_command_result(id, "ipconfig", &["/flushdns"], "DNS cache refreshed")
        }

        // Group 6: Background & Privacy
        "disable-gamedvr" => gaming::apply_disable_gamedvr(id),
        "disable-bg-recording" => gaming::apply_disable_bg_recording(id),
        "gamebar-startup-off" => gaming::apply_gamebar_startup_off(id),
        "wer-off" => storage::apply_wer_off(id),
        "start-bing-search-off" => storage::apply_start_bing_search_off(id),
        "delivery-optimization-lan" => storage::apply_delivery_optimization_lan(id),
        "activity-history-off" => storage::apply_activity_history_off(id),
        "advertising-id-off" => storage::apply_advertising_id_off(id),

        _ => skipped(
            id,
            "This tweak has no active action",
        ),
    }
}

pub fn known_tweak_ids() -> &'static [&'static str] {
    &[
        // Group 1: Gaming & Latency
        "game-mode-on",
        "modern-flip-model-on",
        "gamedvr-fse-mode",
        "disable-fso-globally",
        "hags-on",
        "mpo-disable",
        "pcie-aspm-off",

        // Group 2: CPU & Performance
        "power-plan-high",
        "ultimate-performance-plan",
        "cpu-unpark-cores",
        "power-throttling-off",
        "mmcss-games-priority",
        "system-responsiveness-10",
        "network-throttle-off",

        // Group 3: Input & Responsiveness
        "pointer-precision-off",
        "sticky-keys-off",
        "usb-selective-suspend-off",
        "input-response-fast",
        "csrss-high-priority",
        "visual-effects-performance",
        "transparency-off",
        "menu-show-delay-low",

        // Group 4: Storage & Debloat
        "clean-temp-junk",
        "hibernate-off",
        "ntfs-last-access-off",
        "trim-enable",

        // Group 5: Network Latency
        "tcp-nodelay-ack",
        "nic-energy-saving-off",
        "tcp-heuristics-off",
        "rss-on",
        "rsc-off",
        "ecn-off",
        "dns-cache-flush",

        // Group 6: Background & Privacy
        "disable-gamedvr",
        "disable-bg-recording",
        "gamebar-startup-off",
        "wer-off",
        "start-bing-search-off",
        "delivery-optimization-lan",
        "activity-history-off",
        "advertising-id-off",
    ]
}

pub fn is_tweak_applied(id: &str) -> bool {
    match id {
        // Group 1: Gaming & Latency
        "game-mode-on" => gaming::is_game_mode_applied(),
        "modern-flip-model-on" => gaming::is_modern_flip_model_applied(),
        "gamedvr-fse-mode" => gaming::is_gamedvr_fse_mode_applied(),
        "disable-fso-globally" => gaming::is_disable_fso_globally_applied(),
        "hags-on" => gaming::is_hags_applied(),
        "mpo-disable" => gaming::is_mpo_disable_applied(),
        "pcie-aspm-off" => gaming::is_pcie_aspm_off_applied(),

        // Group 2: CPU & Performance
        "power-plan-high" => power::is_power_plan_high_applied(),
        "ultimate-performance-plan" => power::is_ultimate_performance_applied(),
        "cpu-unpark-cores" => power::is_cpu_unpark_cores_applied(),
        "power-throttling-off" => power::is_power_throttling_off_applied(),
        "mmcss-games-priority" => power::is_mmcss_games_priority_applied(),
        "system-responsiveness-10" => power::is_system_responsiveness_applied(),
        "network-throttle-off" => power::is_network_throttle_off_applied(),

        // Group 3: Input & Responsiveness
        "pointer-precision-off" => system::is_pointer_precision_off_applied(),
        "sticky-keys-off" => system::is_sticky_keys_off_applied(),
        "usb-selective-suspend-off" => system::is_usb_selective_suspend_applied(),
        "input-response-fast" => system::is_input_response_fast_applied(),
        "csrss-high-priority" => system::is_csrss_high_priority_applied(),
        "visual-effects-performance" => system::is_visual_effects_performance_applied(),
        "transparency-off" => system::is_transparency_off_applied(),
        "menu-show-delay-low" => system::is_menu_show_delay_low_applied(),

        // Group 4: Storage & Debloat
        "clean-temp-junk" => false,
        "hibernate-off" => storage::is_hibernate_applied(),
        "ntfs-last-access-off" => storage::is_ntfs_last_access_applied(),
        "trim-enable" => storage::is_trim_applied(),

        // Group 5: Network Latency
        "tcp-nodelay-ack" => network::hklm_tcp_nodelay_active(),
        "nic-energy-saving-off" => network::hklm_nic_energy_saving_off(),
        "tcp-heuristics-off" => network::is_tcp_heuristics_off_applied(),
        "rss-on" => network::is_rss_applied(),
        "rsc-off" => network::is_rsc_applied(),
        "ecn-off" => network::is_ecn_applied(),
        "dns-cache-flush" => false,

        // Group 6: Background & Privacy
        "disable-gamedvr" => gaming::is_disable_gamedvr_applied(),
        "disable-bg-recording" => gaming::is_disable_bg_recording_applied(),
        "gamebar-startup-off" => gaming::is_gamebar_startup_off_applied(),
        "wer-off" => storage::is_wer_off_applied(),
        "start-bing-search-off" => storage::is_start_bing_search_off_applied(),
        "delivery-optimization-lan" => storage::is_delivery_optimization_lan_applied(),
        "activity-history-off" => storage::is_activity_history_off_applied(),
        "advertising-id-off" => storage::is_advertising_id_off_applied(),

        _ => false,
    }
}

pub fn create_system_restore_point(description: &str) {
    crate::infra::restore_point::create_system_restore_point(description);
}
