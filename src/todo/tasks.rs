use chrono::{DateTime, Local};

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

impl TodoRecord {
    pub fn get_csv_header() -> String {
        "id,title,description,due_date,priority,status,created_at,updated_at,completed_at\n"
            .to_string()
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
}

impl TodoRecord {
    pub fn new(id: u32, title: String, description: Option<String>, priority: u8) -> TodoRecord {
        let now = Self::current_time_string();
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

    fn current_time_string() -> String {
        let now: DateTime<Local> = Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
