use std::sync::Arc;

use super::state::TrimmerSync;

pub fn trim_process_memory() {
    crate::platform::ffi::trim_working_set();
}

pub fn spawn_memory_trimmer(trimmer_sync: Arc<TrimmerSync>) {
    std::thread::Builder::new()
        .name("synchro-memory-trimmer".to_string())
        .stack_size(64 * 1024)
        .spawn(move || {
            let mut lock = match trimmer_sync.exited.lock() {
                Ok(l) => l,
                Err(e) => e.into_inner(),
            };
            while !*lock {
                let next_lock = match trimmer_sync.condvar.wait(lock) {
                    Ok(res) => res,
                    Err(e) => e.into_inner(),
                };
                lock = next_lock;
            }
        })
        .ok();
}
