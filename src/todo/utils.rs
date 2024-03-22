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

pub fn get_priority_emoji(priority: u8) -> &'static str {
    match priority {
        ..=0 => "⚪",
        1..=2 => "🟢",
        3..=4 => "🟡",
        5..=6 => "🟣",
        7..=8 => "🔵",
        9..=10 => "🟠",
        11.. => "🔴🔴",
    }
}
