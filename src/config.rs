use std::path::PathBuf;

use config::{Config, Environment};

use crate::{error::AppError, file::get_config_path};

pub struct AppConfig {
    pub data_folder: PathBuf,
    pub database_host: Option<String>,
}

const ENV_PREFIX: &str = "track_topia";
const ENV_POSTFIX_DATA_FOLDER: &str = "data_folder";
const ENV_POSTFIX_DATABASE_HOST: &str = "database_host";

const APP_NAME: &str = env!("CARGO_PKG_NAME");

impl AppConfig {
    pub fn parse() -> Result<Self, AppError> {
        let config = Config::builder()
            .add_source(Environment::with_prefix(ENV_PREFIX))
            .build()
            .map_err(|e| AppError::Parse {
                field: module_path!(),
                value: "<None>".to_owned(),
                reason: format!("{:?}", e),
            })?;
        let data_folder = config
            .get_string(ENV_POSTFIX_DATA_FOLDER)
            .map(PathBuf::from)
            .or_else(|_| get_config_path().map(|p| p.join(APP_NAME)))?;
        let database_host = config.get_string(ENV_POSTFIX_DATABASE_HOST).ok();
        Ok(Self {
            data_folder,
            database_host,
        })
    }
}
