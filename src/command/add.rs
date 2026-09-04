use chrono::{DateTime, Utc};

use crate::error::AppError;

pub(super) fn run(tracker_name: &str, value: &i32, date: &DateTime<Utc>) -> Result<(), AppError> {
    Ok(())
}
