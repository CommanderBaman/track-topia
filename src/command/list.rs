use chrono::{DateTime, Utc};

use crate::{cli::SortOptions, error::AppError};

pub(super) fn run(
    from: &DateTime<Utc>,
    to: &DateTime<Utc>,
    sort: &SortOptions,
    tracker: &Option<String>,
) -> Result<(), AppError> {
    Ok(())
}
