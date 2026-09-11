use std::{cmp::Ordering, fmt};

use chrono::{DateTime, Utc};

#[derive(Clone)]
pub enum TrackerKind {
    Value,
    Continuous,
}

const TRACKER_KIND_VALUE: &str = "value";
const TRACKER_KIND_CONTINUOUS: &str = "continuous";

impl TrackerKind {
    fn parse(kind: &str) -> Option<TrackerKind> {
        match kind {
            TRACKER_KIND_CONTINUOUS => Some(TrackerKind::Continuous),
            TRACKER_KIND_VALUE => Some(TrackerKind::Value),
            _ => None,
        }
    }
}

impl fmt::Display for TrackerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continuous => write!(f, "{TRACKER_KIND_CONTINUOUS}"),
            Self::Value => write!(f, "{TRACKER_KIND_VALUE}"),
        }
    }
}

#[derive(Clone)]
pub struct Tracker {
    id: i32,
    name: String,
    kind: TrackerKind,
}

impl Tracker {
    pub fn new(id: i32, name: &str, kind: &TrackerKind) -> Self {
        Self {
            id,
            name: name.to_owned(),
            kind: kind.to_owned(),
        }
    }
    pub fn get_id(&self) -> &i32 {
        &self.id
    }
}

pub struct TrackerEntry {
    tracker_id: i32,
    value: i32,
    timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TrackerEntry {
    pub fn new(tracker_id: &i32, value: &i32, timestamp: &DateTime<Utc>) -> Self {
        TrackerEntry {
            tracker_id: tracker_id.clone(),
            value: value.clone(),
            timestamp: timestamp.clone(),
            created_at: crate::time::get_current_timestamp(),
            updated_at: crate::time::get_current_timestamp(),
        }
    }
}

pub enum TrackerFilter {
    Name(String),
    Time {
        operation: Ordering,
        value: DateTime<Utc>,
    },
    CreationTime {
        operation: Ordering,
        value: DateTime<Utc>,
    },
    UpdateTime {
        operation: Ordering,
        value: DateTime<Utc>,
    },
}
