use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const DB_USER: &str = "root";
pub const DB_DIR_PATH: &str = "./db";

#[derive(Debug)]
pub struct DBConfig<'a> {
    pub db_dir: &'a Path,
    pub db_user: String,
}

impl<'a> DBConfig<'a> {
    pub fn new(db_path: &'a Path, db_user: String) -> Self {
        DBConfig {
            db_dir: db_path,
            db_user,
        }
    }

    pub fn init(&self) {
        if !file_exists(&self.db_dir) {
            let _ = create_file(&self.db_dir)
                .with_context(|| format!("Failed to create file: {:?}", &self.db_dir));
        }
    }

    pub fn get_db_path(&self) -> Result<PathBuf, anyhow::Error> {
        let mut db_path = self.db_dir.to_path_buf();
        db_path.push(format!(".{}", self.db_user));
        Ok(db_path)
    }
}

fn file_exists<T: AsRef<Path>>(path: T) -> bool {
    if let Err(e) = fs::metadata(path) {
        return false;
    }
    true
}

fn create_file<T: AsRef<Path>>(path: T) -> Result<(), anyhow::Error> {
    if !file_exists(path) {
        fs::File::create(path)?;
    }
    Ok(())
}
