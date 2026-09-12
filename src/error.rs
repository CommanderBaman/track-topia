use std::process::ExitCode;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("argument parsing error")]
    Parse {
        field: &'static str,
        value: String,
        reason: String,
    },
    #[error("{value} is not yet initialized. please run initialize command before operations")]
    Initialize { value: &'static str },
    #[error("some unknown error occurred. location: {location}, reason: {reason}")]
    Unknown {
        location: &'static str,
        reason: String,
    },
    #[error("database error: {reason}")]
    Database { reason: String },
    #[error("given {field} with value {value} is already present")]
    AlreadyPresent { field: &'static str, value: String },
    #[error("given {field} with value {value} is not present")]
    NotPresent { field: &'static str, value: String },
    #[error("app data is corrupted")]
    Corrupted { field: &'static str, value: String },
    #[error("file io error: {reason}")]
    FileIo { reason: String },
}

impl From<AppError> for ExitCode {
    fn from(e: AppError) -> Self {
        // using convention from:
        // https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
        match e {
            AppError::Parse { .. } => ExitCode::from(64),
            AppError::Initialize { .. } => ExitCode::from(78),
            AppError::Unknown { .. } => ExitCode::from(70),
            AppError::Database { .. } => ExitCode::from(69),
            AppError::AlreadyPresent { .. } => ExitCode::from(65),
            AppError::NotPresent { .. } => ExitCode::from(65),
            AppError::Corrupted { .. } => ExitCode::from(65),
            AppError::FileIo { .. } => ExitCode::from(74),
        }
    }
}
