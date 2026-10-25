use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn get_log_dir() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let p = PathBuf::from(local_app_data).join("SynchroNova").join("logs");
        let _ = fs::create_dir_all(&p);
        p
    } else {
        let p = std::env::temp_dir().join("SynchroNova").join("logs");
        let _ = fs::create_dir_all(&p);
        p
    }
}

pub fn get_log_file() -> PathBuf {
    get_log_dir().join("updater.log")
}

pub fn log(stage: &str, message: &str) {
    let now = time::OffsetDateTime::now_utc();
    let timestamp = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let line = format!("[{timestamp}] [{stage}] {message}\n");

    eprint!("{line}");

    let log_path = get_log_file();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = file.write_all(line.as_bytes());
    }
}
