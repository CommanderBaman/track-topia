use chrono::{DateTime, Utc};

use crate::time;

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
            let timestamp = parse_raw_timestamp(timestamp, parse_as_utc, "timestamp")?;
            add::run(
                tracker,
                value,
                &timestamp.unwrap_or_else(time::get_current_timestamp),
            )
        }
        Command::List {
            from,
            to,
            tracker,
            sort,
            parse_as_utc,
        } => {
            let from = parse_raw_timestamp(from, parse_as_utc, "from")?;
            let to = parse_raw_timestamp(to, parse_as_utc, "to")?;
            list::run(&from, &to, sort, tracker)
        }
        Command::Initialize {} => initialize::run(),
        Command::Track { name, tracker_kind } => track::run(name, tracker_kind),
    }
}

fn parse_raw_timestamp(
    raw_timestamp: &Option<String>,
    parse_as_utc: &bool,
    field_name: &'static str,
) -> Result<Option<DateTime<Utc>>, AppError> {
    raw_timestamp
        .as_deref()
        .map(|t| {
            time::parse_timestamp_from_string(&t, !parse_as_utc).ok_or(AppError::Parse {
                field: field_name,
                value: t.to_owned(),
                reason: "unable to parse ts".to_owned(),
            })
        })
        .transpose()
}
