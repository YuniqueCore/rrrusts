use std::{collections::{hash_map, HashMap}, fs};

use chrono::{DateTime, Local};

use super::dbguard::DBConfig;

#[derive(Debug)]
pub struct TodoRecord {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub priority: u8,
    pub status: bool,
    created_at: String,
    updated_at: String,
    completed_at: Option<String>,
}
const HEADER_MAP: &[(&str, u8)] = &[
    ("id", 0),
    ("title", 1),
    ("description", 2),
    ("due_date", 3),
    ("priority", 4),
    ("status", 5),
    ("created_at", 6),
    ("updated_at", 7),
    ("completed_at", 8),
];

pub fn get_header_index(header: &str) -> Option<u8> {
    HEADER_MAP.iter().find_map(|(h, i)| if *h == header { Some(*i) } else { None })
}

pub fn current_time_string() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

impl TodoRecord {
    pub fn get_csv_header() -> String {
        let headers: Vec<String> = HEADER_MAP.iter().map(|(h, _)| format!("\"{}\"", *h)).collect();
        let header_str = headers.join(",");
        format!("{}\n", header_str)
    }

    pub fn to_csv_string(&self) -> String {
        format!(
            "\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            self.id,
            self.title.trim(),
            self.description.clone().unwrap_or(String::new()).trim(),
            self.due_date.clone().unwrap_or(String::new()).trim(),
            self.priority,
            if self.status { "Done" } else { "Todo" },
            self.created_at.trim(),
            self.updated_at.trim(),
            self.completed_at.clone().unwrap_or(String::new()).trim(),
        )
    }

    pub fn get_newest_id(dbconf:&DBConfig){
        let mut reader = fs::OpenOptions::read(true)
        .open(&dbconf.get_db_path().t);
    }
}

impl TodoRecord {
    pub fn new(id: u32, title: String, description: Option<String>, priority: u8) -> TodoRecord {
        let now = current_time_string();
        TodoRecord {
            id,
            title,
            description,
            priority,
            due_date: None,
            status: false,
            created_at: now.clone(),
            updated_at: now.clone(),
            completed_at: None,
        }
    }
}
