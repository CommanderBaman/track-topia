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

pub fn format_timestamp<Tz: TimeZone>(timestamp: &DateTime<Tz>, format: Option<&str>) -> String
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    let format = format.unwrap_or("%Y-%m-%d %H:%M:%S");
    timestamp.format(format).to_string()
}

// This function is also for validatiaon -> None means not correct
pub fn parse_timestamp_from_string(timestamp: &str, is_local: bool) -> Option<DateTime<Utc>> {
    NaiveDate::parse_from_str(timestamp, "%Y-%m-%d")
        .map(|t| {
            t.and_hms_opt(0, 0, 0)
                .expect("hardcoded time should be correct")
        })
        .or_else(|_| NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S"))
        .ok()
        .map(|t| {
            if is_local {
                Local
                    .from_local_datetime(&t)
                    .latest()
                    .expect("local datetime conversion to not fail")
                    .to_utc()
            } else {
                Utc.from_utc_datetime(&t)
            }
        })
}

pub fn get_days_difference<Tz: TimeZone>(t1: &DateTime<Tz>, t2: &DateTime<Tz>) -> i64 {
    // have to clone because sub between two references not allowed
    let diff = t1.clone() - t2;
    diff.num_days()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, Utc};
    use std::{env, thread, time::Duration};

    fn set_timezone_as_ist() {
        unsafe {
            env::set_var("TZ", "Asia/Kolkata");
        }
    }

    #[test]
    fn parse_valid_datetime() {
        set_timezone_as_ist();
        let input = "2025-09-13 11:00:00";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:00+00:00");
    }

    #[test]
    fn parse_valid_datetime_utc() {
        set_timezone_as_ist();
        let input = "2025-09-13 11:00:00";
        let result = parse_timestamp_from_string(input, false);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:00+00:00");
    }

    #[test]
    fn parse_valid_date() {
        set_timezone_as_ist();
        let input = "2025-09-13";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T18:29:59+00:00");
    }

    #[test]
    fn parse_invalid_time_format() {
        set_timezone_as_ist();
        let input = "13/09/2025 11:00";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_none());
    }

    #[test]
    fn parse_empty_time_string() {
        set_timezone_as_ist();
        let input = "";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_none());
    }

    #[test]
    fn convert_to_local_time() {
        set_timezone_as_ist();
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
        set_timezone_as_ist();
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
        set_timezone_as_ist();
        let input = "2025-09-13 11:00:01";
        let result = parse_timestamp_from_string(input, true);
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-09-13T05:30:01+00:00");
        let dt = convert_timestamp_to_local(&dt);
        assert_eq!(dt.to_rfc3339(), "2025-09-13T11:00:01+05:30");
        assert_eq!("2025-09-13", format_timestamp(&dt, Some("%Y-%m-%d")));
    }

    #[test]
    fn days_diff_same_datetime() {
        let t1 = Utc::now();
        assert_eq!(get_days_difference(&t1, &t1), 0);
    }

    #[test]
    fn days_diff_negative_difference() {
        let t1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2023, 1, 10, 0, 0, 0).unwrap();
        assert_eq!(get_days_difference(&t1, &t2), -9);
    }

    #[test]
    fn days_diff_positive_difference() {
        let t1 = Utc.with_ymd_and_hms(2023, 1, 10, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(get_days_difference(&t1, &t2), 9);
    }

    #[test]
    fn days_diff_partial_days_and_truncation() {
        let t1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        // 23 hours later (less than 1 full 24-hour day)
        let t2 = Utc.with_ymd_and_hms(2023, 1, 1, 23, 0, 0).unwrap();

        // `signed_duration_since().num_days()` truncates towards zero
        assert_eq!(get_days_difference(&t1, &t2), 0);
    }

    #[test]
    fn days_diff_leap_year() {
        let t1 = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2024, 2, 28, 0, 0, 0).unwrap();
        // 2024 is a leap year, so Feb 29 exists (2 days total)
        assert_eq!(get_days_difference(&t1, &t2), 2);
    }

    #[test]
    fn days_diff_different_timezones() {
        let est = FixedOffset::west_opt(5 * 3600).unwrap(); // UTC-5
        let pst = FixedOffset::west_opt(8 * 3600).unwrap(); // UTC-8

        // Same instant in time across different timezones
        let t1 = est.with_ymd_and_hms(2023, 5, 1, 12, 0, 0).unwrap();
        let t2 = pst.with_ymd_and_hms(2023, 5, 1, 9, 0, 0).unwrap();

        assert_eq!(get_days_difference(&t1, &t2), 0);
    }

    #[test]
    fn get_current_timestamp_increases_every_ms() {
        let before = Utc::now();
        thread::sleep(Duration::from_millis(1));
        let ts = get_current_timestamp();
        thread::sleep(Duration::from_millis(1));
        let after = Utc::now();
        assert!(ts > before);
        assert!(after > ts);
    }

    #[test]
    fn format_timestamp_custom_format() {
        let dt = Utc.with_ymd_and_hms(2026, 9, 16, 14, 30, 0).unwrap();
        let formatted = format_timestamp(&dt, Some("%Y-%m-%d %H::%S"));
        assert_eq!(formatted, "2026-09-16 14::00");
    }

    #[test]
    fn format_timestamp_default_format() {
        let dt = Utc.with_ymd_and_hms(2026, 9, 16, 14, 30, 0).unwrap();
        let formatted = format_timestamp(&dt, None);
        assert_eq!(formatted, "2026-09-16 14:30:00");
    }

    #[test]
    fn convert_local_prints_differently() {
        let ts = Utc.with_ymd_and_hms(2026, 9, 16, 14, 30, 0).unwrap();
        let ts_local = convert_timestamp_to_local(&ts);

        let ts_format = format_timestamp(&ts, None);
        let ts_local_format = format_timestamp(&ts_local, None);
        assert_eq!(ts_format, "2026-09-16 14:30:00");
        assert_eq!(ts_local_format, "2026-09-16 20:00:00");
        assert_ne!(ts_format, ts_local_format);
    }

    #[test]
    fn convert_millis_on_invalid_millis() {
        // too big of a date ~ 31709792 years
        let t = convert_millis_to_timestamp(1_000_000_000_000_000_000);
        assert!(t.is_none())
    }

    #[test]
    fn convert_millis_on_valid_millis() {
        // epoch + 1s
        let millis = 1000;
        let expected_ts = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 1).unwrap();
        let ts = convert_millis_to_timestamp(millis);
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts, expected_ts);

        // epoch - 1s
        let millis = -1000;
        let expected_ts = Utc.with_ymd_and_hms(1969, 12, 31, 23, 59, 59).unwrap();
        let ts = convert_millis_to_timestamp(millis);
        assert!(ts.is_some());
        let ts = ts.unwrap();
        assert_eq!(ts, expected_ts);
    }
}
