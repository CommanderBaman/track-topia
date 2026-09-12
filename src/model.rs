use chrono::{DateTime, Utc};
use clap::ValueEnum;
use strum::{Display, EnumIter, EnumString, IntoStaticStr};
use tabled::Tabled;
use uuid::Uuid;

use crate::error::AppError;
use crate::time;

impl From<strum::ParseError> for AppError {
    fn from(value: strum::ParseError) -> Self {
        AppError::Parse {
            field: "strum-field",
            value: "<None>".to_owned(),
            reason: value.to_string(),
        }
    }
}

#[derive(Clone, Display, EnumString, EnumIter, IntoStaticStr, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum TrackerKind {
    Value,
    Continuous,
}

pub struct Tracker {
    pub id: Uuid,
    pub name: String,
    pub kind: TrackerKind,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tracker {
    pub fn new(name: &str, kind: &TrackerKind) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: name.to_owned(),
            kind: kind.to_owned(),
            created_at: time::get_current_timestamp(),
            updated_at: time::get_current_timestamp(),
        }
    }
    pub fn build(
        id: &Uuid,
        name: &str,
        kind: &TrackerKind,
        created_at: &DateTime<Utc>,
        updated_at: &DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            kind: kind.to_owned(),
            created_at: created_at.to_owned(),
            updated_at: updated_at.to_owned(),
        }
    }
}

#[derive(Tabled)]
pub struct TrackerEntry {
    pub id: Uuid,
    pub tracker_id: Uuid,
    pub value: f64,
    #[tabled(display("display_datetime", self))]
    pub timestamp: DateTime<Utc>,
    #[tabled(skip)]
    pub created_at: DateTime<Utc>,
    #[tabled(skip)]
    pub updated_at: DateTime<Utc>,
}

fn display_datetime(ts: &DateTime<Utc>, _entry: &TrackerEntry) -> String {
    time::format_timestamp(&time::convert_timestamp_to_local(ts), None)
}

impl TrackerEntry {
    pub fn new(tracker_id: &Uuid, value: &f64, timestamp: &DateTime<Utc>) -> Self {
        TrackerEntry {
            id: Uuid::now_v7(),
            tracker_id: tracker_id.clone(),
            value: value.clone(),
            timestamp: timestamp.clone(),
            created_at: time::get_current_timestamp(),
            updated_at: time::get_current_timestamp(),
        }
    }
    pub fn build(
        id: &Uuid,
        tracker_id: &Uuid,
        value: &f64,
        timestamp: &DateTime<Utc>,
        created_at: &DateTime<Utc>,
        updated_at: &DateTime<Utc>,
    ) -> Self {
        TrackerEntry {
            id: id.to_owned(),
            tracker_id: tracker_id.to_owned(),
            value: value.to_owned(),
            timestamp: timestamp.to_owned(),
            created_at: created_at.to_owned(),
            updated_at: updated_at.to_owned(),
        }
    }
}

pub enum Ordering {
    // Less,
    LessOrEqual,
    // Equal,
    // Greater,
    GreaterOrEqual,
}

pub enum TrackerFilter {
    Name(String),
    Time {
        operation: Ordering,
        value: DateTime<Utc>,
    },
    // CreationTime {
    //     operation: Ordering,
    //     value: DateTime<Utc>,
    // },
    // UpdateTime {
    //     operation: Ordering,
    //     value: DateTime<Utc>,
    // },
}
