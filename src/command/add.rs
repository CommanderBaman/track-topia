use chrono::{DateTime, Utc};

use crate::{
    config::AppConfig,
    database::{Database, DatabaseImpl},
    error::AppError,
    model::TrackerEntry,
};

pub(super) fn run(
    tracker_name: &str,
    value: &f64,
    occurrence_time: &DateTime<Utc>,
) -> Result<(), AppError> {
    let config = AppConfig::parse()?;
    let db = DatabaseImpl::from_config(&config)?;
    let Some(tracker) = db.get_tracker_from_name(tracker_name)? else {
        return Err(AppError::NotPresent {
            field: "tracker (name)",
            value: tracker_name.to_owned(),
        });
    };
    let entry = TrackerEntry::new(&tracker.id, value, occurrence_time);
    db.add_tracker_entry(&entry)
}
