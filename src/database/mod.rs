use crate::{
    error::AppError,
    model::{Tracker, TrackerEntry, TrackerFilter},
};
use uuid::Uuid;

mod sqlite;

pub trait Database {
    fn add_tracker(&self, tracker: &Tracker) -> Result<(), AppError>;
    fn add_tracker_entry(&self, entry: &TrackerEntry) -> Result<(), AppError>;
    fn get_tracker_entries(
        &self,
        filters: &Vec<TrackerFilter>,
    ) -> Result<Vec<TrackerEntry>, AppError>;
    fn get_tracker_from_name(&self, name: &str) -> Result<Option<Tracker>, AppError>;
    fn get_tracker_from_id(&self, id: &Uuid) -> Result<Option<Tracker>, AppError>;
}

pub use sqlite::SqliteDatabase as DatabaseImpl;
