use std::{
    env,
    path::{Path, PathBuf},
};

use rusqlite::{Connection, OptionalExtension};

use crate::{
    database::Database,
    error::AppError,
    file,
    model::{Tracker, TrackerEntry, TrackerFilter, TrackerKind},
};

const ENV_KEY_SQLITE_PATH: &str = "TTSD";
const DATABASE_FILE_NAME: &str = "database.sqlite";
const TABLE_TRACKERS: &str = "trackers";
const TABLE_TRACKER_ENTRIES: &str = "tracker_entries";

// TODO: move to constants
const APP_NAME: &str = "";

pub struct SqliteDatabase {
    connection: Connection,
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        AppError::Database {
            reason: format!("{:?}", error),
        }
    }
}

impl rusqlite::ToSql for TrackerKind {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(self.to_string().into()))
    }
}

impl SqliteDatabase {
    fn new(path: &Path) -> Result<Self, AppError> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }
    fn get_path() -> Result<PathBuf, AppError> {
        if let Ok(dir) = env::var(ENV_KEY_SQLITE_PATH) {
            return Ok(PathBuf::from(dir));
        }
        let path = file::get_config_path()?;
        Ok(path.join(APP_NAME).join(DATABASE_FILE_NAME))
    }
    pub fn from_env() -> Result<Self, AppError> {
        if !Self::is_initialized()? {
            return Err(AppError::Initialize {
                value: "sqlite database",
            });
        }
        let path = Self::get_path()?;
        Self::new(&path)
    }
    pub fn initialize(&self) -> Result<(), AppError> {
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                kind TEXT NOT NULL CHECK(kind IN ('{}', '{}')),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            );
            CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tracker_id INTEGER NOT NULL,
                value INTEGER NOT NULL,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (tracker_id) REFERENCES {}(id) ON DELETE CASCADE,
                UNIQUE (id, tracker_id)
            );
            ",
            TABLE_TRACKERS,
            TrackerKind::Value,
            TrackerKind::Continuous,
            TABLE_TRACKER_ENTRIES,
            TABLE_TRACKERS,
        );
        self.connection.execute_batch(&query).map_err(Into::into)
    }
    fn is_initialized() -> Result<bool, AppError> {
        let db_path = Self::get_path()?;
        if !db_path.exists() {
            return Ok(false);
        }
        let connection = Self::new(&db_path)?.connection;
        if !connection.table_exists(None, TABLE_TRACKERS)? {
            return Ok(false);
        }
        if !connection.table_exists(None, TABLE_TRACKER_ENTRIES)? {
            return Ok(false);
        }

        Ok(db_path.exists())
    }
}

impl Database for SqliteDatabase {
    fn get_tracker_entries(
        &self,
        filters: Vec<TrackerFilter>,
    ) -> Result<Vec<TrackerEntry>, AppError> {
        unimplemented!()
    }
    fn get_tracker_from_name(&self, name: &str) -> Result<Option<Tracker>, AppError> {
        let query = format!(
            "SELECT id, name, kind, created_at, updated_at FROM {TABLE_TRACKERS} WHERE name = ?"
        );
        let tracker: Option<Tracker> = self.connection.query_one(&query, [name], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let kind: String = row.get(2)?;
            Ok(Some(Tracker::new(id, &name, &TrackerKind::Value)))
        })?;
        Ok(None)
    }
    fn add_tracker(&self, name: &str, kind: &TrackerKind) -> Result<(), AppError> {
        if let Ok(Some(_tracker)) = self.get_tracker_from_name(name) {
            return Err(AppError::AlreadyPresent {
                field: "tracker",
                value: name.to_owned(),
            });
        }
        let query = format!("INSERT INTO {TABLE_TRACKERS} (name, kind) VALUES (?1, ?2)");
        let rows_changed = self
            .connection
            .execute(&query, rusqlite::params![name, kind])?;
        assert!(rows_changed == 1);
        Ok(())
    }
    fn add_tracker_entry(&self, entry: TrackerEntry) -> Result<(), AppError> {
        unimplemented!()
    }
}
