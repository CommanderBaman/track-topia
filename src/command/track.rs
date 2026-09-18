use crate::{
    config::AppConfig,
    database::{Database, DatabaseImpl},
    error::AppError,
    model::{Tracker, TrackerKind},
};

pub(super) fn run(name: &str, tracker_kind: &TrackerKind) -> Result<(), AppError> {
    let config = AppConfig::parse()?;
    let db = DatabaseImpl::from_config(&config)?;
    db.add_tracker(&Tracker::new(name, tracker_kind))
}
