use std::sync::Arc;
use std::time::Duration;

use super::state::TrimmerSync;

pub fn trim_process_memory() {
    crate::platform::ffi::trim_working_set();
}

pub fn spawn_memory_trimmer(trimmer_sync: Arc<TrimmerSync>) {
    std::thread::Builder::new()
        .name("synchro-memory-trimmer".to_string())
        .spawn(move || {
            let mut lock = match trimmer_sync.exited.lock() {
                Ok(l) => l,
                Err(e) => e.into_inner(),
            };
            if *lock {
                return;
            }

            // Post-boot initial trims via wait_timeout so shutdown is instantaneous
            let (l2, _) = match trimmer_sync.condvar.wait_timeout(lock, Duration::from_millis(2000)) {
                Ok(res) => res,
                Err(e) => e.into_inner(),
            };
            lock = l2;
            if *lock {
                return;
            }
            trim_process_memory();

            let (l3, _) = match trimmer_sync.condvar.wait_timeout(lock, Duration::from_millis(3000)) {
                Ok(res) => res,
                Err(e) => e.into_inner(),
            };
            lock = l3;
            if *lock {
                return;
            }
            trim_process_memory();

            // Passive wait until shutdown signal - no continuous periodic trimming to avoid micro-stutters during gameplay
            while !*lock {
                lock = match trimmer_sync.condvar.wait(lock) {
                    Ok(l) => l,
                    Err(e) => e.into_inner(),
                };
            }
        })
        .ok();
}
