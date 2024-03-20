use anyhow::{bail, Ok, Result};
use std::{
    fs, io::Write, path::{Path, PathBuf}
};

use super::tasks::TodoRecord;

pub const DB_USER: &str = "root";
pub const DB_DIR_PATH: &str = "./db";

#[derive(Debug)]
pub struct DBConfig {
    pub db_dir: Box<PathBuf>,
    pub db_user: String,
}

impl DBConfig{
    pub fn new<P: AsRef<Path>>(db_path: P, db_user: String) -> Self {
        DBConfig {
            db_dir: Box::new(db_path.as_ref().to_path_buf()),
            db_user,
        }
    }

    pub fn init(&self)-> Result<fs::File> {
        match self.get_db_path() {
            Result::Ok(path) => {
                if !file_exists(&path)
                {
                    Ok(fs::File::create(path)?)
                }else {
                    Ok(fs::File::open(path)?)
                }
            },
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

pub struct DBOperator{
    pub db:fs::File,
}

impl DBOperator {
    fn connect(dbconfig:&DBConfig) -> Result<Self>  {
        match dbconfig.init() {
            Result::Ok(db) => Ok(Self{db}),
            Result::Err(e)=> Err(e),
        }
    }

    fn add_record(&mut self,record:TodoRecord)->Result<()>{
        self.db.write(b"buf")?;
        Ok(())
    }

    fn done_record(&mut self,record:TodoRecord)->Result<()>{
        Ok(())
    }

    fn close(&mut self)->Result<()>{
        self.db.flush()?;
        self.db.sync_all()?;
        Ok(())
    }
}