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
    system::clean_legacy_csrss_override();
    gaming::clean_legacy_rust_ifeo();
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
        "win32-priority-rust"
            | "system-timer-resolution"
            | "disable-paging-executive"
            | "disable-memory-compression"
            | "disable-page-combining"
            | "adaptive-io-page-lock"
            | "kill-gamebar-presence"
            | "directx-thread-priority"
            | "gpu-adaptive-power"
            | "network-udp-buffers"
            | "large-system-cache-off"
            | "hags-on"
            | "mpo-disable"
            | "mmcss-games-priority"
            | "system-responsiveness-0"
            | "system-worker-threads"
            | "cpu-adaptive-scheduling"
            | "network-throttle-off"
            | "hibernate-off"
            | "ntfs-last-access-off"
            | "trim-enable"
            | "rss-adaptive-cores"
            | "rsc-off"
            | "disable-gamedvr"
            | "tcp-nodelay-ack"
            | "nic-energy-saving-off"
            | "tcp-heuristics-off"
            | "cpu-unpark-cores"
            | "power-throttling-off"
            | "input-response-fast"
    )
}

fn apply_one(id: &str) -> TweakApplyResult {
    if is_tweak_applied(id) && id != "clean-temp-junk" {
        return TweakApplyResult {
            id: id.to_string(),
            status: "skipped".to_string(),
            message: "Tweak is already applied in the system".to_string(),
        };
    }
    if is_admin_tweak(id) && !crate::admin::is_running_elevated() {
        return TweakApplyResult {
            id: id.to_string(),
            status: "requiresAdmin".to_string(),
            message: "Requires administrator privileges; restart Synchro as administrator to apply this tweak".to_string(),
        };
    }
    match id {
        "win32-priority-rust" => gaming::apply_win32_priority_rust(id),
        "system-timer-resolution" => gaming::apply_system_timer_resolution(id),
        "kill-gamebar-presence" => gaming::apply_kill_gamebar_presence(id),
        "directx-thread-priority" => gaming::apply_directx_thread_priority(id),
        "gpu-adaptive-power" => gaming::apply_gpu_adaptive_power(id),
        "game-mode-on" => gaming::apply_game_mode(id),
        "modern-flip-model-on" => gaming::apply_modern_flip_model(id),
        "gamedvr-fse-mode" => gaming::apply_gamedvr_fse_mode(id),
        "disable-fso-globally" => gaming::apply_disable_fso_globally(id),
        "hags-on" => gaming::apply_hags(id),
        "mpo-disable" => gaming::apply_mpo_disable(id),
        "pcie-aspm-off" => gaming::apply_pcie_aspm_off(id),

        "power-plan-high" => runner::run_powercfg(
            id,
            &["/setactive", "SCHEME_MIN"],
            "High performance power plan selected",
        ),
        "ultimate-performance-plan" => power::apply_ultimate_performance(id),
        "cpu-unpark-cores" => power::apply_cpu_unpark_cores(id),
        "power-throttling-off" => power::apply_power_throttling_off(id),
        "cpu-adaptive-scheduling" => power::apply_cpu_adaptive_scheduling(id),
        "system-worker-threads" => power::apply_system_worker_threads(id),
        "mmcss-games-priority" => power::apply_mmcss_games_priority(id),
        "system-responsiveness-0" => power::apply_system_responsiveness(id),
        "network-throttle-off" => power::apply_network_throttle_off(id),

        "pointer-precision-off" => system::apply_pointer_precision_off(id),
        "usb-selective-suspend-off" => system::apply_usb_selective_suspend(id),
        "input-response-fast" => system::apply_input_response_fast(id),
        "visual-effects-performance" => system::apply_visual_effects_performance(id),
        "transparency-off" => system::apply_transparency_off(id),

        "clean-temp-junk" => storage::apply_clean_temp_junk(id),
        "disable-paging-executive" => storage::apply_disable_paging_executive(id),
        "disable-memory-compression" => storage::apply_disable_memory_compression(id),
        "disable-page-combining" => storage::apply_disable_page_combining(id),
        "adaptive-io-page-lock" => storage::apply_adaptive_io_page_lock(id),
        "large-system-cache-off" => storage::apply_large_system_cache_off(id),
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

        "network-udp-buffers" => network::apply_network_udp_buffers(id),
        "tcp-nodelay-ack" => network::apply_tcp_nodelay_ack(id),
        "nic-energy-saving-off" => network::apply_nic_energy_saving_off(id),
        "tcp-heuristics-off" => network::apply_tcp_heuristics_off(id),
        "rss-adaptive-cores" => network::apply_rss_adaptive_cores(id),
        "rsc-off" => runner::run_netsh(
            id,
            &["interface", "tcp", "set", "global", "rsc=disabled"],
            "Receive segment coalescing disabled",
        ),

        "disable-gamedvr" => gaming::apply_disable_gamedvr(id),
        "disable-bg-recording" => gaming::apply_disable_bg_recording(id),
        "wer-off" => storage::apply_wer_off(id),

        _ => skipped(
            id,
            "This tweak has no active action",
        ),
    }
}
