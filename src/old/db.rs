use chrono::{DateTime, Utc};
use directories::BaseDirs;
use rusqlite::Connection;

use crate::constants::{APP_NAME, DEFAULT_DB_NAME};
use crate::file::make_file_if_not_exists;
use std::cmp::Ordering;
use std::io::{self, ErrorKind};
use std::{error::Error, path::PathBuf};

#[derive(Eq, PartialEq)]
pub enum DatabaseValue {
    Int(i32),
    String(String),
    Time(DateTime<Utc>),
}
impl PartialOrd for DatabaseValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DatabaseValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (DatabaseValue::Int(a), DatabaseValue::Int(b)) => a.cmp(b),
            (DatabaseValue::String(a), DatabaseValue::String(b)) => a.cmp(b),
            (DatabaseValue::Time(a), DatabaseValue::Time(b)) => a.cmp(b),
            (_, _) => Ordering::Equal,
        }
    }
}

pub struct Database {
    path: PathBuf,
}

impl Database {
    fn get_database_path() -> Option<PathBuf> {
        BaseDirs::new().map(|base_dirs| {
            base_dirs
                .data_local_dir()
                .join(APP_NAME)
                .join(DEFAULT_DB_NAME)
        })
    }

    pub fn new() -> Result<Database, io::Error> {
        if let Some(database_path) = Database::get_database_path() {
            make_file_if_not_exists(&database_path).expect("able to make database file");
            Ok(Database {
                path: database_path,
            })
        } else {
            Err(io::Error::new(
                ErrorKind::NotFound,
                "couldn't find a place to store database",
            ))
        }
    }

    pub fn get_connection(&self) -> Connection {
        Connection::open(&self.path).expect("database file must be present")
    }

    fn initialize_tables(&self) -> Result<(), rusqlite::Error> {
        let conn = self.get_connection();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS trackers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                type TEXT NOT NULL CHECK(type IN ('continuous', 'value', 'enumerated')),
                created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                UNIQUE (id, user_id)
            );

            CREATE TABLE IF NOT EXISTS tracker_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tracker_id INTEGER NOT NULL,
                value INTEGER NOT NULL,
                date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (tracker_id) REFERENCES trackers(id) ON DELETE CASCADE,
                UNIQUE (id, tracker_id)
            );
            ",
        )
        .expect("db query should be successful");
        Ok(())
    }
}

pub fn initialize_database() -> Result<(), Box<dyn Error>> {
    // initialize database
    let db = Database::new().expect("database to be formed");
    // add tables
    let result = db.initialize_tables();
    // TODO: handle various rusqlite error
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
