use std::path::PathBuf;

use directories::BaseDirs;

use crate::error::AppError;

pub fn get_config_path() -> Result<PathBuf, AppError> {
    if let Some(base_dirs) = BaseDirs::new() {
        return Ok(base_dirs.data_local_dir().to_owned());
    }
    Err(AppError::Unknown {
        location: concat!(module_path!(), ":", line!()),
        reason: "generating config path failed".to_owned(),
    })
}
