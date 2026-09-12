use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    database::Database,
    error::AppError,
    file::make_file_if_not_exists,
    model::{Ordering, Tracker, TrackerEntry, TrackerFilter, TrackerKind},
    time,
};

const DATABASE_FILE_NAME: &str = "database.sqlite";
const TABLE_TRACKERS: &str = "trackers";
const TABLE_TRACKER_ENTRIES: &str = "tracker_entries";

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

impl SqliteDatabase {
    fn new(path: &Path) -> Result<Self, AppError> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }
    fn database_path(config: &AppConfig) -> PathBuf {
        let database_host = config
            .database_host
            .as_deref()
            .unwrap_or(DATABASE_FILE_NAME);
        config.data_folder.join(database_host).to_owned()
    }
    fn is_initialized(config: &AppConfig) -> Result<bool, AppError> {
        let db_path = Self::database_path(config);
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
        Ok(true)
    }
    pub fn from_config(config: &AppConfig) -> Result<Self, AppError> {
        if !Self::is_initialized(config)? {
            return Err(AppError::Initialize {
                value: "sqlite database",
            });
        }
        let path = Self::database_path(config);
        Self::new(&path)
    }
    // TODO: rethink about making this pub
    // there must be a better way to initialize things from config
    // without exposing the internal connection
    pub fn initialize_tables(&self) -> Result<(), AppError> {
        assert!(TrackerKind::iter().next().is_some());
        let kinds: Vec<String> = TrackerKind::iter().map(|k| format!("'{k}'")).collect();
        let possible_kind_values = format!("({})", kinds.join(", "));
        let query = format!(
            "
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                kind TEXT NOT NULL CHECK(kind IN {}),
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                tracker_id TEXT NOT NULL,
                value REAL NOT NULL,
                timestamp INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (tracker_id) REFERENCES {}(id) ON DELETE CASCADE,
                UNIQUE (id, tracker_id)
            );
            ",
            TABLE_TRACKERS, possible_kind_values, TABLE_TRACKER_ENTRIES, TABLE_TRACKERS,
        );
        self.connection.execute_batch(&query).map_err(Into::into)
    }
    pub fn initialize(config: &AppConfig) -> Result<(), AppError> {
        // ensure file
        let path = Self::database_path(config);
        make_file_if_not_exists(&path)?;
        println!("{path:?}");
        let db = Self::new(&path)?;
        db.initialize_tables()
    }
}

fn get_uuid_from_row(row: &rusqlite::Row, index: usize) -> Result<Uuid, rusqlite::Error> {
    let input: String = row.get(index)?;
    Uuid::parse_str(&input).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn get_tracker_kind_from_row(
    row: &rusqlite::Row,
    index: usize,
) -> Result<TrackerKind, rusqlite::Error> {
    let input: String = row.get(index)?;
    input.parse().map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(index, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn get_datetime_from_row(
    row: &rusqlite::Row,
    index: usize,
    field_name: &'static str,
) -> Result<DateTime<Utc>, rusqlite::Error> {
    let time_ms: i64 = row.get(index)?;
    time::convert_millis_to_timestamp(time_ms).ok_or(rusqlite::Error::FromSqlConversionFailure(
        index,
        rusqlite::types::Type::Integer,
        Box::from(format!(
            "incorrect value received for {field_name} = {time_ms}"
        )),
    ))
}

fn get_tracker_from_row(row: &rusqlite::Row) -> Result<Tracker, rusqlite::Error> {
    let id = get_uuid_from_row(row, 0)?;
    let name: String = row.get(1)?;
    let kind = get_tracker_kind_from_row(row, 2)?;
    let created_at = get_datetime_from_row(row, 3, "created_at")?;
    let updated_at = get_datetime_from_row(row, 4, "updated_at")?;
    Ok(Tracker::build(&id, &name, &kind, &created_at, &updated_at))
}

fn get_tracker_entry_from_row(row: &rusqlite::Row) -> Result<TrackerEntry, rusqlite::Error> {
    let id = get_uuid_from_row(row, 0)?;
    let tracker_id = get_uuid_from_row(row, 1)?;
    let value: f64 = row.get(2)?;
    let timestamp = get_datetime_from_row(row, 3, "timestamp")?;
    let created_at = get_datetime_from_row(row, 4, "created_at")?;
    let updated_at = get_datetime_from_row(row, 5, "updated_at")?;
    Ok(TrackerEntry::build(
        &id,
        &tracker_id,
        &value,
        &timestamp,
        &created_at,
        &updated_at,
    ))
}

fn get_sign_from_ordering(ord: &Ordering) -> &'static str {
    match ord {
        // Ordering::Equal => "=",
        // Ordering::Less => "<",
        Ordering::LessOrEqual => "<=",
        // Ordering::Greater => ">",
        Ordering::GreaterOrEqual => ">=",
    }
}

impl Database for SqliteDatabase {
    fn get_tracker_from_name(&self, name: &str) -> Result<Option<Tracker>, AppError> {
        let query = format!(
            "SELECT id, name, kind, created_at, updated_at FROM {TABLE_TRACKERS} WHERE name = ?"
        );
        let tracker: Option<Tracker> = self
            .connection
            .query_one(&query, [name], get_tracker_from_row)
            .optional()?;
        Ok(tracker)
    }
    fn get_tracker_from_id(&self, id: &Uuid) -> Result<Option<Tracker>, AppError> {
        let query = format!(
            "SELECT id, name, kind, created_at, updated_at FROM {TABLE_TRACKERS} WHERE id = ?"
        );
        let tracker: Option<Tracker> = self
            .connection
            .query_one(&query, [id.to_string()], get_tracker_from_row)
            .optional()?;
        Ok(tracker)
    }
    fn get_tracker_entries(
        &self,
        filters: &Vec<TrackerFilter>,
    ) -> Result<Vec<TrackerEntry>, AppError> {
        let mut query = format!(
            "SELECT id, tracker_id, value, timestamp, created_at, updated_at FROM {TABLE_TRACKER_ENTRIES} "
        );
        if filters.is_empty() {
            query.push_str(" WHERE 1 = 1 ");
            for filter in filters {
                let condition_query: String = match filter {
                    TrackerFilter::Name(name) => {
                        let Some(filter_tracker) = self.get_tracker_from_name(name)? else {
                            return Err(AppError::NotPresent {
                                field: "tracker (name)",
                                value: name.to_owned(),
                            });
                        };
                        format!("tracker_id = {}", filter_tracker.id)
                    }
                    // TrackerFilter::UpdateTime { operation, value } => {
                    //     format!(
                    //         "updated_at {} {}",
                    //         get_sign_from_ordering(operation),
                    //         value.timestamp_millis()
                    //     )
                    // }
                    // TrackerFilter::CreationTime { operation, value } => {
                    //     format!(
                    //         "created_at {} {}",
                    //         get_sign_from_ordering(operation),
                    //         value.timestamp_millis()
                    //     )
                    // }
                    TrackerFilter::Time { operation, value } => {
                        format!(
                            "timestamp {} {}",
                            get_sign_from_ordering(operation),
                            value.timestamp_millis()
                        )
                    }
                };
                query.push_str(&format!(" AND {condition_query} "));
            }
        }
        let mut statement = self.connection.prepare(&query)?;
        statement
            .query_map([], get_tracker_entry_from_row)?
            .map(|res| res.map_err(AppError::from))
            .collect()
    }

    fn add_tracker(&self, tracker: &Tracker) -> Result<(), AppError> {
        if let Ok(Some(tracker)) = self.get_tracker_from_name(&tracker.name) {
            return Err(AppError::AlreadyPresent {
                field: "tracker (name)",
                value: tracker.name,
            });
        }
        let query = format!(
            "INSERT INTO {TABLE_TRACKERS} (id, name, kind, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)"
        );
        let rows_changed = self.connection.execute(
            &query,
            rusqlite::params![
                tracker.id.to_string(),
                tracker.name,
                tracker.kind.to_string(),
                tracker.created_at.timestamp_millis(),
                tracker.updated_at.timestamp_millis(),
            ],
        )?;
        assert!(rows_changed == 1);
        Ok(())
    }
    fn add_tracker_entry(&self, entry: &TrackerEntry) -> Result<(), AppError> {
        let Some(_) = self.get_tracker_from_id(&entry.tracker_id)? else {
            return Err(AppError::NotPresent {
                field: "tracker (id)",
                value: entry.tracker_id.to_string(),
            });
        };
        let query = format!(
            "INSERT INTO {TABLE_TRACKER_ENTRIES} (id, tracker_id, value, timestamp, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        );
        let rows_changed = self.connection.execute(
            &query,
            rusqlite::params![
                entry.id.to_string(),
                entry.tracker_id.to_string(),
                entry.value,
                entry.timestamp.timestamp_millis(),
                entry.created_at.timestamp_millis(),
                entry.updated_at.timestamp_millis()
            ],
        )?;
        assert!(rows_changed == 1);
        Ok(())
    }
}
