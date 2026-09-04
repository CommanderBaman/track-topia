use chrono::{DateTime, Utc};

use crate::time::{get_current_timestamp, parse_timestamp_from_string};

use crate::{cli::Command, error::AppError};

mod add;
mod initialize;
mod list;
mod track;

pub fn run(command: &Command) -> Result<(), AppError> {
    match command {
        Command::Add {
            tracker,
            value,
            timestamp,
            parse_as_utc,
        } => {
            ensure_setup()?;
            let timestamp = parse_raw_timestamp(timestamp, parse_as_utc, "timestamp")?;
            add::run(tracker, value, &timestamp)
        }
        Command::List {
            from,
            to,
            tracker,
            sort,
            parse_as_utc,
        } => {
            ensure_setup()?;
            let from = parse_raw_timestamp(from, parse_as_utc, "from")?;
            let to = parse_raw_timestamp(to, parse_as_utc, "to")?;
            list::run(&from, &to, sort, tracker)
        }
        Command::Initialize { user_name } => initialize::run(user_name),
        Command::Track { name, tracker_type } => {
            ensure_setup()?;
            track::run(name, tracker_type)
        }
    }
}

fn ensure_setup() -> Result<(), AppError> {
    if !initialize::is_setup_complete() {
        return Err(AppError::Initialize);
    }
    Ok(())
}

fn parse_raw_timestamp(
    raw_timestamp: &Option<String>,
    parse_as_utc: &bool,
    field_name: &'static str,
) -> Result<DateTime<Utc>, AppError> {
    let timestamp = match raw_timestamp {
        None => get_current_timestamp(),
        Some(s) => {
            let Some(t) = parse_timestamp_from_string(s, !parse_as_utc) else {
                return Err(AppError::Parse {
                    field: field_name,
                    value: s.to_owned(),
                    reason: "not provided".to_owned(),
                });
            };
            t
        }
    };
    Ok(timestamp)
}
