use anyhow::Context;
#[warn(unused_variables)]
use anyhow::{bail, Ok, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use super::{
    tasks::{self, TodoRecord},
    utils,
};

pub const DB_USER: &str = "root";
pub const DB_DIR_PATH: &str = "./db";

#[derive(Debug)]
pub struct DBConfig {
    pub db_dir: Box<PathBuf>,
    pub db_user: String,
    pub db_csv_data_header: String,
}

impl DBConfig {
    pub fn new<P: AsRef<Path>>(db_path: P, db_user: String, db_csv_data_header: String) -> Self {
        DBConfig {
            db_dir: Box::new(db_path.as_ref().to_path_buf()),
            db_user,
            db_csv_data_header,
        }
    }

    pub fn init(&self) -> Result<fs::File> {
        match self.get_db_path() {
            Result::Ok(path) => {
                if !utils::file_exists(&path) {
                    let mut f = fs::File::create(path)?;
                    f.write(self.db_csv_data_header.as_bytes())?;
                    Ok(f)
                } else {
                    Ok(fs::OpenOptions::new().append(true).open(path)?)
                }
            }
            Result::Err(e) => bail!(e),
        }
    }

    pub fn get_db_path(&self) -> Result<PathBuf, anyhow::Error> {
        let mut db_path = self.db_dir.to_path_buf();
        db_path.push(format!(".{}", self.db_user));
        Ok(db_path)
    }
}

pub struct DBOperator {
    pub db: fs::File,
    pub db_path: PathBuf,
    pub db_csv_data_header: String,
}

impl DBOperator {
    pub fn connect(dbconfig: &DBConfig) -> Result<Self> {
        match dbconfig.init() {
            Result::Ok(db) => Ok(Self {
                db,
                db_path: dbconfig.get_db_path()?,
                db_csv_data_header: dbconfig.db_csv_data_header.clone(),
            }),
            Result::Err(e) => Err(e),
        }
    }

    pub fn add_record(&mut self, record: TodoRecord) -> Result<()> {
        let buf = BufReader::new(File::open(&self.db_path)?);
        match buf.lines().next() {
            Some(Result::Ok(content)) => {
                if content.trim() != self.db_csv_data_header.trim() {
                    self.db.write(self.db_csv_data_header.as_bytes())?;
                }
            }
            Some(Result::Err(err)) => {
                return Err(err.into());
            }
            None => {
                self.db.write(self.db_csv_data_header.as_bytes())?;
            }
        }
        self.db.write(record.to_csv_string().as_bytes())?;
        Ok(())
    }

    pub fn remove_record(&mut self, record: TodoRecord) -> Result<()> {
        let file = fs::OpenOptions::new()
            .write(false)
            .create(false)
            .truncate(false)
            .read(true)
            .open(&self.db_path)?;
        let temp_path = Path::new(&self.db_path).with_extension("tmp");
        let temp_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;

        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(temp_file);

        let mut found = false;
        let id_col = tasks::get_header_index("id").or(Some(0)).unwrap() as usize;
        let title_col = tasks::get_header_index("title").or(Some(1)).unwrap() as usize;

        for (index, line) in reader.lines().enumerate() {
            if index == 0 {
                writer.write(self.db_csv_data_header.as_bytes())?;
                continue;
            }
            let old_line = line?;
            let mut data_array: Vec<&str> = old_line
                .split(',')
                .filter_map(|l| l.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
                .collect();

            if !found {
                if let Some(id_str) = data_array.get(id_col) {
                    if let Result::Ok(id) = id_str.parse::<u32>() {
                        if id == record.id {
                            found = true;
                            continue;
                        }
                    }
                }

                if !found {
                    if let Some(title) = data_array.get(title_col) {
                        if title == &record.title {
                            found = true;
                            continue;
                        }
                    }
                }
            }

            let mut id_string: String = String::new();

            if found {
                if let Some(id_str) = data_array.get_mut(id_col) {
                    if let Result::Ok(id) = id_str.parse::<u32>() {
                        id_string = (id - 1).to_string();
                        *id_str = id_string.as_str();
                    }
                }
            }

            let updated_line = data_array
                .iter()
                .map(|&item| format!("\"{}\"", item))
                .collect::<Vec<String>>()
                .join(",");
            writeln!(writer, "{}", updated_line)?;
        }

        if !found {
            bail!("Record {} - {} not found.", &record.id, &record.title)
        }

        writer.flush()?;
        fs::rename(&temp_path, &self.db_path)?;

        Ok(())
    }

    pub fn done_record(&mut self, record: TodoRecord) -> Result<()> {
        let file = fs::OpenOptions::new()
            .write(false)
            .create(false)
            .truncate(false)
            .read(true)
            .open(&self.db_path)?;
        let temp_path = Path::new(&self.db_path).with_extension("tmp");
        let temp_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)?;

        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(temp_file);

        let mut found = false;
        let now_time = utils::current_time_string();
        let id_col = tasks::get_header_index("id").or(Some(0)).unwrap() as usize;
        let title_col = tasks::get_header_index("title").or(Some(1)).unwrap() as usize;
        let status_col = tasks::get_header_index("status").or(Some(5)).unwrap() as usize;
        let updated_at_col = tasks::get_header_index("updated_at").or(Some(7)).unwrap() as usize;
        let completed_at_col =
            tasks::get_header_index("completed_at").or(Some(8)).unwrap() as usize;

        for (index, line) in reader.lines().enumerate() {
            if index == 0 {
                writer.write(self.db_csv_data_header.as_bytes())?;
                continue;
            }
            let old_line = line?;
            let mut data_array: Vec<&str> = old_line
                .split(',')
                .filter_map(|l| l.strip_prefix('"').and_then(|s| s.strip_suffix('"')))
                .collect();

            if !found {
                if let Some(id_str) = data_array.get(id_col) {
                    if let Result::Ok(id) = id_str.parse::<u32>() {
                        if id == record.id {
                            found = true;
                            if let Some(status) = data_array.get_mut(status_col) {
                                *status = "Done";

                                if let Some(update) = data_array.get_mut(updated_at_col) {
                                    *update = &now_time;
                                }
                                if let Some(completed) = data_array.get_mut(completed_at_col) {
                                    *completed = &now_time;
                                }
                            }
                        }
                    }
                }

                if !found {
                    if let Some(title) = data_array.get(title_col) {
                        if title == &record.title {
                            found = true;
                            if let Some(status) = data_array.get_mut(status_col) {
                                *status = "Done";
                                if let Some(update) = data_array.get_mut(updated_at_col) {
                                    *update = &now_time;
                                }
                                if let Some(completed) = data_array.get_mut(completed_at_col) {
                                    *completed = &now_time;
                                }
                            }
                        }
                    }
                }
            }

            let updated_line = data_array
                .iter()
                .map(|&item| format!("\"{}\"", item))
                .collect::<Vec<String>>()
                .join(",");
            writeln!(writer, "{}", updated_line)?;
        }

        if !found {
            bail!("Record {} - {} not found.", &record.id, &record.title)
        }

        writer.flush()?;
        fs::rename(&temp_path, &self.db_path)?;

        Ok(())
    }

    pub fn list_records(&mut self, mut writer: impl std::io::Write) -> Result<()> {
        let db_path = PathBuf::from(&self.db_path);
        let db = fs::File::open(&db_path).context("Failed to open db file")?;
        let reader = BufReader::new(&db);

        // let mut records: Vec<TodoRecord> = Vec::new();
        let mut parse_errors = Vec::new();
        for line in reader.lines().skip(1) {
            let line = line.context("Failed to read line from db file")?;
            if !line.is_empty() {
                let record = TodoRecord::parse_record_line(&line).map_err(|e| {
                    parse_errors.push(format!("Error parsing record: {}", e));
                    e
                })?;
                // records.push(record);
                writeln!(writer, "{}", record.to_list_fmt())?;
            }
        }
        if !parse_errors.is_empty() {
            writeln!(writer, "{}", parse_errors.join("\n"))?;
        }

        Result::Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.db.flush()?;
        self.db.sync_all()?;
        Ok(())
    }
}

impl DBOperator {
    pub fn get_newest_id(dbconf: &DBConfig) -> Result<u32, anyhow::Error> {
        let db_path = dbconf.get_db_path().context("Failed to find db file")?;
        let db = fs::File::open(&db_path).context("Failed to open db file")?;
        let reader = BufReader::new(&db);

        let mut records: Vec<TodoRecord> = Vec::new();
        let mut parse_errors = Vec::new();
        for line in reader.lines().skip(1) {
            let line = line.context("Failed to read line from db file")?;
            if !line.is_empty() {
                let record = TodoRecord::parse_record_line(&line).map_err(|e| {
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
