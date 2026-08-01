pub mod client;
pub mod installer;

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    pub status: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: u8,
    pub error: Option<String>,
}

impl Default for UpdateProgress {
    fn default() -> Self {
        Self {
            status: "idle".to_string(),
            downloaded_bytes: 0,
            total_bytes: 0,
            percent: 0,
            error: None,
        }
    }
}

static PROGRESS_STATE: OnceLock<Mutex<UpdateProgress>> = OnceLock::new();

fn get_state() -> &'static Mutex<UpdateProgress> {
    PROGRESS_STATE.get_or_init(|| Mutex::new(UpdateProgress::default()))
}

pub fn get_progress() -> UpdateProgress {
    get_state()
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

pub fn set_progress(progress: UpdateProgress) {
    if let Ok(mut guard) = get_state().lock() {
        *guard = progress;
    }
}
