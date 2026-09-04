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
