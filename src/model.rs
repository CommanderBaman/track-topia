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
    #[tabled(skip)]
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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn are_within_ten_seconds<Tz: TimeZone>(dt1: &DateTime<Tz>, dt2: &DateTime<Tz>) -> bool {
        let diff = (dt1.timestamp_millis() - dt2.timestamp_millis()).abs();
        diff <= 10_000
    }

    #[test]
    fn tracker_new() {
        let name = "fizz";
        let t = Tracker::new(name, &TrackerKind::Continuous);
        let now = Utc::now();

        assert_eq!(t.name, name);

        assert!(matches!(t.kind, TrackerKind::Continuous));
        assert!(are_within_ten_seconds(&t.created_at, &now));
        assert!(are_within_ten_seconds(&t.updated_at, &now));
    }

    #[test]
    fn tracker_build() {
        let expected_id = Uuid::now_v7();
        let expected_name = "Fitness Tracker";
        let expected_created_at = Utc::now();
        let expected_updated_at = Utc::now();

        let tracker = Tracker::build(
            &expected_id,
            expected_name,
            &TrackerKind::Continuous,
            &expected_created_at,
            &expected_updated_at,
        );

        assert_eq!(tracker.id, expected_id);
        assert_eq!(tracker.name, expected_name);
        assert!(matches!(tracker.kind, TrackerKind::Continuous));
        assert_eq!(tracker.created_at, expected_created_at);
        assert_eq!(tracker.updated_at, expected_updated_at);
    }

    #[test]
    fn tracker_entry_new() {
        let expected_tracker_id = Uuid::now_v7();
        let expected_value = 75.5;
        let expected_timestamp = Utc::now();
        let now = Utc::now();

        let entry = TrackerEntry::new(&expected_tracker_id, &expected_value, &expected_timestamp);

        assert_eq!(entry.tracker_id, expected_tracker_id);
        assert_eq!(entry.value, expected_value);
        assert_eq!(entry.timestamp, expected_timestamp);
        assert!(are_within_ten_seconds(&entry.created_at, &now));
        assert!(are_within_ten_seconds(&entry.updated_at, &now));
    }

    #[test]
    fn tracker_entry_build() {
        let expected_id = Uuid::now_v7();
        let expected_tracker_id = Uuid::now_v7();
        let expected_value = 75.5;
        let expected_timestamp = Utc::now();
        let expected_created_at = Utc::now();
        let expected_updated_at = Utc::now();

        let entry = TrackerEntry::build(
            &expected_id,
            &expected_tracker_id,
            &expected_value,
            &expected_timestamp,
            &expected_created_at,
            &expected_updated_at,
        );

        assert_eq!(entry.id, expected_id);
        assert_eq!(entry.tracker_id, expected_tracker_id);
        assert_eq!(entry.value, expected_value);
        assert_eq!(entry.timestamp, expected_timestamp);
        assert_eq!(entry.created_at, expected_created_at);
        assert_eq!(entry.updated_at, expected_updated_at);
    }
}
