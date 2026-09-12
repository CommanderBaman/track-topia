// timestamp is YYYY-MM-DD HH:MM:SS
// this is ISO-8601 format

// All db elements will be in UTC - sqlite default
// convert to local for ease

use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

pub fn get_current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn convert_timestamp_to_local(timestamp: &DateTime<Utc>) -> DateTime<Local> {
    timestamp.with_timezone(&Local)
}

pub fn convert_millis_to_timestamp(x: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_millis(x)
}

pub fn format_timestamp<Tz>(timestamp: &DateTime<Tz>, format: Option<&str>) -> String
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Display,
{
    let format = format.unwrap_or("%Y-%m-%d %H:%M:%S");
    timestamp.format(format).to_string()
}

// This function is also for validatiaon -> None means not correct
pub fn parse_timestamp_from_string(timestamp: &str, is_local: bool) -> Option<DateTime<Utc>> {
    let naive_datetime = if let Ok(date) = NaiveDate::parse_from_str(timestamp, "%Y-%m-%d") {
        // parse for yyyy-mm-dd
        date.and_hms_opt(12, 0, 0)
            .expect("hardcoded time should be correct")
    } else if let Ok(date) = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S") {
        // parse for yyyy-mm-dd hh:mm:ss
        date
    } else {
        return None;
    };

    if is_local {
        Some(Local.from_local_datetime(&naive_datetime).unwrap().to_utc())
    } else {
        Some(Utc.from_utc_datetime(&naive_datetime))
    }
}

pub fn get_days_difference<Tz>(t1: &DateTime<Tz>, t2: &DateTime<Tz>) -> i64
where
    Tz: TimeZone,
{
    // have to clone because sub between two references not allowed
    let diff = t1.clone() - t2;
    diff.num_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_datetime() {
        let input = "2025-09-13 11:00:00";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:00+00:00");
    }

    #[test]
    fn parse_valid_datetime_utc() {
        let input = "2025-09-13 11:00:00";
        let result = parse_timestamp_from_string(input, false);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:00+00:00");
    }

    #[test]
    fn parse_valid_date() {
        let input = "2025-09-13";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T06:30:00+00:00");
    }

    #[test]
    fn parse_invalid_time_format() {
        let input = "13/09/2025 11:00";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_none());
    }

    #[test]
    fn parse_empty_time_string() {
        let input = "";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_none());
    }

    #[test]
    fn convert_to_local_time() {
        let input = "2025-09-13 11:00:01";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:01+00:00");
        let dt = convert_timestamp_to_local(&dt);
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:01+05:30");
    }

    #[test]
    fn timestamp_conversion_default() {
        let input = "2025-09-13 11:00:01";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:01+00:00");
        let dt = convert_timestamp_to_local(&dt);
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:01+05:30");
        assert_eq!(input, format_timestamp(&dt, None));
    }

    #[test]
    fn timestamp_conversion_custom() {
        let input = "2025-09-13 11:00:01";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:01+00:00");
        let dt = convert_timestamp_to_local(&dt);
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:01+05:30");
        assert_eq!("2025-09-13", format_timestamp(&dt, Some("%Y-%m-%d")));
    }
}
