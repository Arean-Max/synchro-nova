use std::time::{Duration, Instant};
use tauri::State;

use crate::app::state::{DriverInfo, RuntimeState, SystemCharacteristics};
use crate::browser::open_driver_search_url;
use crate::driver_info;
use crate::platform::live_metrics::{collect_live_metrics, LiveMetrics};
use crate::system_info::collect_system_characteristics;

#[tauri::command]
pub fn get_system_characteristics(
    state: State<'_, RuntimeState>,
) -> Result<SystemCharacteristics, String> {
    const CACHE_TTL: Duration = Duration::from_secs(8);

    let live = collect_system_live_metrics(&state)?;

    if let Ok(cache) = state.system_cache.lock() {
        if let (Some(value), Some(updated_at)) = (&cache.value, cache.updated_at) {
            if updated_at.elapsed() < CACHE_TTL {
                let mut snapshot = value.clone();
                apply_live_metrics(&mut snapshot, &live);
                return Ok(snapshot);
            }
        }
    }

    let mut collected = collect_system_characteristics();
    apply_live_metrics(&mut collected, &live);
    let mut cache = state
        .system_cache
        .lock()
        .map_err(|_| "System cache lock poisoned".to_string())?;
    cache.value = Some(collected.clone());
    cache.updated_at = Some(Instant::now());
    Ok(collected)
}

#[tauri::command]
pub fn get_system_live_metrics(state: State<'_, RuntimeState>) -> Result<LiveMetrics, String> {
    collect_system_live_metrics(&state)
}

pub fn collect_system_live_metrics(state: &State<'_, RuntimeState>) -> Result<LiveMetrics, String> {
    let mut sample = state
        .cpu_sample
        .lock()
        .map_err(|_| "CPU sample lock poisoned".to_string())?;
    Ok(collect_live_metrics(&mut sample))
}

pub fn apply_live_metrics(snapshot: &mut SystemCharacteristics, live: &LiveMetrics) {
    snapshot.cpu_usage_percent = live.cpu_usage_percent.clone();
    snapshot.gpu_usage_percent = live.gpu_usage_percent.clone();
    snapshot.ram_available_gb = live.ram_available_gb.clone();
    snapshot.ram_available = format!("{} GB", live.ram_available_gb);
    snapshot.ram_used_gb = live.ram_used_gb.clone();
    snapshot.ram_used_percent = live.ram_used_percent.clone();
    snapshot.vram_total_gb = live.vram_total_gb.clone();
    snapshot.vram_used_gb = live.vram_used_gb.clone();
    snapshot.vram_used_percent = live.vram_used_percent.clone();
}

#[tauri::command]
pub fn open_driver_search(query: String) -> Result<(), String> {
    open_driver_search_url(&query)
}

#[tauri::command]
pub fn scan_drivers() -> Vec<DriverInfo> {
    driver_info::collect_drivers()
}
