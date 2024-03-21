use std::{
    fs,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context};
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
    HEADER_MAP
        .iter()
        .find_map(|(h, i)| if *h == header { Some(*i) } else { None })
}

pub fn current_time_string() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

impl TodoRecord {
    const DEFAULT_PRIORITY: u8 = 3;

    pub fn parse_record_line(line: &str) -> Result<TodoRecord, anyhow::Error> {
        let values: Vec<&str> = line.split(',').collect();

        if values.len() < 9 {
            bail!("Input line does not contain enough fields".to_string());
        }

        let id = values[0].parse::<u32>()?;
        let title = values[1].to_string();
        let description = if values[2].is_empty() {
            None
        } else {
            Some(values[2].to_string())
        };
        let due_date = if values[3].is_empty() {
            None
        } else {
            Some(values[3].to_string())
        };
        let priority = if values[4].is_empty() {
            Self::DEFAULT_PRIORITY
        } else {
            values[4].parse::<u8>()?
        };
        let status = values[5].parse::<bool>()?;
        let created_at = if values[6].is_empty() {
            current_time_string()
        } else {
            values[6].to_string()
        };
        let updated_at = if values[7].is_empty() {
            current_time_string()
        } else {
            values[7].to_string()
        };
        let completed_at = if values[8].is_empty() {
            None
        } else {
            Some(values[8].to_string())
        };

        Ok(TodoRecord {
            id,
            title,
            description,
            due_date,
            priority,
            status,
            created_at,
            updated_at,
            completed_at,
        })
    }

    pub fn get_csv_header() -> String {
        let headers: Vec<String> = HEADER_MAP
            .iter()
            .map(|(h, _)| format!("\"{}\"", *h))
            .collect();
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

    pub fn get_newest_id(dbconf: &DBConfig) -> Result<u32, anyhow::Error> {
        let db_path = dbconf.get_db_path().context("Failed to find db file")?;
        let db = fs::File::open(&db_path).context("Failed to open db file")?;
        let reader = BufReader::new(&db);

        let mut records: Vec<TodoRecord> = Vec::new();
        let mut parse_errors = Vec::new();
        for line in reader.lines() {
            let line = line.context("Failed to read line from db file")?;
            if !line.is_empty() {
                let record = Self::parse_record_line(&line).map_err(|e| {
                    parse_errors.push(format!("Error parsing record: {}", e));
                    e
                })?;
                records.push(record);
            }
        }
        if !parse_errors.is_empty() {
            bail!(parse_errors.join("\n"));
        }

        let next_id = match records.iter().map(|r| r.id).max() {
            Some(max_id) => max_id + 1,
            None => 1, // 如果没有记录，从 1 开始
        };

        Ok(next_id)
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
