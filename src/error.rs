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
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitCode;

    // Helper function to assert ExitCode equality in stable Rust
    fn assert_exit_code_eq(code: ExitCode, expected_u8: u8) {
        let expected = ExitCode::from(expected_u8);
        assert_eq!(
            format!("{code:?}"),
            format!("{expected:?}"),
            "Exit code did not match expected value {expected_u8}"
        );
    }

    #[test]
    fn test_parse_error_exit_code() {
        let err = AppError::Parse {
            field: "port",
            value: "8080a".to_string(),
            reason: "invalid integer".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 64); // EX_USAGE
    }

    #[test]
    fn test_initialize_error_exit_code() {
        let err = AppError::Initialize { value: "database" };
        assert_exit_code_eq(ExitCode::from(err), 78); // EX_CONFIG
    }

    #[test]
    fn test_unknown_error_exit_code() {
        let err = AppError::Unknown {
            location: "main.rs:42",
            reason: "unexpected failure".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 70); // EX_SOFTWARE
    }

    #[test]
    fn test_database_error_exit_code() {
        let err = AppError::Database {
            reason: "connection timeout".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 69); // EX_UNAVAILABLE
    }

    #[test]
    fn test_already_present_exit_code() {
        let err = AppError::AlreadyPresent {
            field: "username",
            value: "alice".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 65); // EX_DATAERR
    }

    #[test]
    fn test_not_present_exit_code() {
        let err = AppError::NotPresent {
            field: "user_id",
            value: "123".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 65); // EX_DATAERR
    }

    #[test]
    fn test_corrupted_exit_code() {
        let err = AppError::Corrupted {
            field: "config_file",
            value: "settings.json".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 65); // EX_DATAERR
    }

    #[test]
    fn test_file_io_error_exit_code() {
        let err = AppError::FileIo {
            reason: "Permission denied".to_string(),
        };
        assert_exit_code_eq(ExitCode::from(err), 74); // EX_IOERR
    }
}
