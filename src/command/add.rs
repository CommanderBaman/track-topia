use chrono::{DateTime, Utc};

use crate::{
    database::{Database, DatabaseImpl},
    error::AppError,
    model::TrackerEntry,
};

pub(super) fn run(
    tracker_name: &str,
    value: &i32,
    occurrence_time: &DateTime<Utc>,
) -> Result<(), AppError> {
    let db = DatabaseImpl::from_env();
    let tracker = db.get_tracker_from_name(tracker_name)?;
    let entry = TrackerEntry::new(tracker.get_id(), value, occurrence_time);
    db.add_tracker_entry(entry)
}
