use std::{fs, path::Path};

use chrono::{DateTime, Local};

pub fn file_exists<T: AsRef<Path>>(path: &T) -> bool {
    if let Err(_) = fs::metadata(path) {
        return false;
    }
    true
}

pub fn current_time_string() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
