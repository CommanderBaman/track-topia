use std::path::PathBuf;

use config::{Config, Environment};

use crate::{error::AppError, file::get_local_data_path};

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
            .or_else(|_| get_local_data_path().map(|p| p.join(APP_NAME)))?;
        let database_host = config.get_string(ENV_POSTFIX_DATABASE_HOST).ok();
        Ok(Self {
            data_folder,
            database_host,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use serial_test::serial;

    use super::*;

    fn clear_env() {
        unsafe {
            env::remove_var(format!("{ENV_PREFIX}_{ENV_POSTFIX_DATABASE_HOST}").to_uppercase());
            env::remove_var(format!("{ENV_PREFIX}_{ENV_POSTFIX_DATA_FOLDER}").to_uppercase());
        }
    }

    fn set_env_var_database_host(value: &str) {
        unsafe {
            env::set_var(
                format!("{ENV_PREFIX}_{ENV_POSTFIX_DATABASE_HOST}").to_uppercase(),
                value,
            );
        }
    }

    fn set_env_var_data_folder(value: &str) {
        unsafe {
            env::set_var(
                format!("{ENV_PREFIX}_{ENV_POSTFIX_DATA_FOLDER}").to_uppercase(),
                value,
            );
        }
    }

    fn get_config() -> AppConfig {
        AppConfig::parse().expect("config can be initialized safely in test")
    }

    fn assert_default_data_path(cfg: &AppConfig) {
        let data_path =
            get_local_data_path().expect("config path can be initialized safely in tests");
        assert_eq!(cfg.data_folder, data_path.join(env!("CARGO_PKG_NAME")));
    }

    fn assert_default_database_host(cfg: &AppConfig) {
        assert!(cfg.database_host.is_none());
    }

    #[test]
    fn constants() {
        assert_eq!(ENV_PREFIX, "track_topia");
        assert_eq!(ENV_POSTFIX_DATABASE_HOST, "database_host");
        assert_eq!(ENV_POSTFIX_DATA_FOLDER, "data_folder");
    }

    #[test]
    #[serial]
    fn default_value() {
        clear_env();
        let cfg = get_config();
        assert_default_data_path(&cfg);
        assert_default_database_host(&cfg);
    }

    #[test]
    #[serial]
    fn value_extraction_for_database_host_and_other_unchanged() {
        clear_env();
        let host_name = "memory";
        set_env_var_database_host(host_name);
        let cfg = get_config();
        assert_default_data_path(&cfg);
        assert_eq!(cfg.database_host, Some(host_name.to_owned()));
    }

    #[test]
    #[serial]
    fn value_extraction_for_data_folder_and_other_unchanged() {
        clear_env();
        let folder = "/someplace";
        set_env_var_data_folder(folder);
        let cfg = get_config();
        assert_default_database_host(&cfg);
        assert_eq!(cfg.data_folder, PathBuf::from(folder));
    }

    #[test]
    #[serial]
    fn both_values_extracted() {
        clear_env();
        let folder = "/someplace";
        let host_name = "memory";
        set_env_var_database_host(host_name);
        set_env_var_data_folder(folder);
        let cfg = get_config();
        assert_eq!(cfg.data_folder, PathBuf::from(folder));
        assert_eq!(cfg.database_host, Some(host_name.to_owned()));
    }
}
