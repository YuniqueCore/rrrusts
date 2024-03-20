use anyhow::{bail, Ok, Result};
use std::{
    default,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use super::tasks::TodoRecord;

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
                if !file_exists(&path) {
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

fn file_exists<T: AsRef<Path>>(path: &T) -> bool {
    if let Err(_) = fs::metadata(path) {
        return false;
    }
    true
}

pub struct DBOperator {
    pub db: fs::File,
    pub db_path: PathBuf,
    pub db_csv_data_header: String,
}

impl DBOperator {
    fn connect(dbconfig: &DBConfig) -> Result<Self> {
        match dbconfig.init() {
            Result::Ok(db) => Ok(Self {
                db,
                db_path: dbconfig.get_db_path()?,
                db_csv_data_header: dbconfig.db_csv_data_header.clone(),
            }),
            Result::Err(e) => Err(e),
        }
    }

    fn add_record(&mut self, record: TodoRecord) -> Result<()> {
        self.db.write(record.to_csv_string().as_bytes())?;
        Ok(())
    }

    fn done_record(&mut self, record: TodoRecord) -> Result<()> {
        let file = &self.db;
        let temp_path = Path::new(&self.db_path).with_extension("tmp");
        let temp_file = File::create(&temp_path)?;

        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(temp_file);

        let mut found = false;

        let id = &self.col_index_of("id", 0);
        let title = &self.col_index_of("title", 1);

        for line in reader.lines() {
            let old_line = line?;
            let mut data_array: Vec<&str> = old_line.split(',').collect();

            if let Some(id_str) = data_array.get(*id) {
                if let Result::Ok(id) = id_str.parse::<u32>() {
                    if id == record.id {
                        found = true;
                        if let Some(status) = data_array.get_mut(2) {
                            *status = "Done";
                        }
                    }
                }
            }

            if let Some(title) = data_array.get(*title) {
                if title == &record.title {
                    found = true;
                    if let Some(status) = data_array.get_mut(2) {
                        *status = "Done";
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

    fn col_index_of(&self, header_name: &str, default_value: usize) -> usize {
        match self.db_headers().position(|(i, name)| name == header_name) {
            Some(index) => index,
            None => {
                eprintln!("Warning: '{}' column not found in the header.", header_name);
                default_value
            }
        }
    }

    fn db_headers(&self) -> std::iter::Enumerate<std::str::Split<'_, char>> {
        self.db_csv_data_header.trim().split(',').enumerate()
    }

    fn close(&mut self) -> Result<()> {
        self.db.flush()?;
        self.db.sync_all()?;
        Ok(())
    }
}
