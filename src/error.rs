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
    #[error("cli is not yet initialize. please run initialize command before operations")]
    Initialize,
}

impl From<AppError> for ExitCode {
    fn from(e: AppError) -> Self {
        // using convention from:
        // https://man.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html
        match e {
            AppError::Parse { .. } => ExitCode::from(64),
            AppError::Initialize => ExitCode::from(78),
        }
    }
}
