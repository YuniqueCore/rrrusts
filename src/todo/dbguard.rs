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
                    Ok(fs::File::open(path)?)
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
        self.db.write(record.to_csv_string().as_bytes())?;
        Ok(())
    }

    pub fn done_record(&mut self, record: TodoRecord) -> Result<()> {
        let file = &self.db;
        let temp_path = Path::new(&self.db_path).with_extension("tmp");
        let temp_file = File::create(&temp_path)?;

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

        for line in reader.lines() {
            let old_line = line?;
            let mut data_array: Vec<&str> = old_line.split(',').collect();

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

            if let Some(title) = data_array.get(title_col as usize) {
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

            let updated_line = data_array.join(",");
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
        let reader = BufReader::new(&self.db);
        for line in reader.lines() {
            match line {
                Result::Ok(l) => {
                    writeln!(writer, "{}", l)?;
                }
                Result::Err(e) => bail!(
                    "Fail to read lines from db: {:?}/nError: {}",
                    self.db_path,
                    e
                ),
            }
        }
        Result::Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.db.flush()?;
        self.db.sync_all()?;
        Ok(())
    }
}
