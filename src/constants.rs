#[cfg(feature = "prod")]
pub const APP_NAME: &str = "TrackTopia";

#[cfg(not(feature = "prod"))]
pub const APP_NAME: &str = "TrackTopiaDebug";

pub const DEFAULT_CONFIG_NAME: &str = "config.json";
pub const DEFAULT_DB_NAME: &str = "db.sqlite";
pub const DEFAULT_USER_NAME: &str = "root";
