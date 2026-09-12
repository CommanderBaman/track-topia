use crate::{config::AppConfig, database::DatabaseImpl, error::AppError};

pub(super) fn run() -> Result<(), AppError> {
    let config = AppConfig::parse()?;
    DatabaseImpl::initialize(&config)
}
