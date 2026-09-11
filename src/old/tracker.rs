use std::{collections::HashMap, error::Error};

use crate::{
    config::Config,
    db::{Database, DatabaseValue},
    time::{
        convert_timestamp_to_local, format_timestamp, get_days_difference,
        parse_timestamp_from_string,
    },
    user::UserDatabase,
};
use chrono::{DateTime, Utc};
use rusqlite::params;
use tabled::{
    builder::Builder,
    settings::{
        Alignment, Color, Merge, Style,
        object::Rows,
        themes::{BorderCorrection, Colorization},
    },
};

pub struct Tracker {
    id: i32,
    name: String,
    user_id: i32,
    tracker_type: String,
}

pub struct TrackerEntry {
    id: i32,
    tracker_id: i32,
    value: i32,
    date: String,
}

pub trait TrackerDatabase {
    fn intialize_tracker(
        &self,
        tracker_name: &str,
        tracker_type: &str,
        user_id: &i32,
    ) -> Result<(), rusqlite::Error>;
    fn get_tracker_details_by_name(
        &self,
        tracker_name: &str,
        user_id: &i32,
    ) -> Result<Tracker, rusqlite::Error>;
    fn get_tracker_details_by_id(&self, tracker_id: &i32) -> Result<Tracker, rusqlite::Error>;

    fn add_tracker_entry(
        &self,
        tracker_id: &i32,
        value: &i32,
        date: &Option<String>,
    ) -> Result<(), rusqlite::Error>;

    fn get_tracker_entries(
        &self,
        from: &Option<String>,
        to: &Option<String>,
        tracker: &Option<Tracker>,
    ) -> Result<Vec<TrackerEntry>, rusqlite::Error>;
}

impl TrackerDatabase for Database {
    fn intialize_tracker(
        &self,
        tracker_name: &str,
        tracker_type: &str,
        user_id: &i32,
    ) -> Result<(), rusqlite::Error> {
        let conn = self.get_connection();
        conn.execute_batch(
            format!(
                "INSERT INTO trackers (name, type, user_id) VALUES ('{}','{}',{})",
                tracker_name, tracker_type, user_id
            )
            .as_str(),
        )
        .expect("initialize tracker query must be successful");
        Ok(())
    }

    fn get_tracker_details_by_name(
        &self,
        tracker_name: &str,
        user_id: &i32,
    ) -> Result<Tracker, rusqlite::Error> {
        let conn = self.get_connection();
        conn.query_row(
            "SELECT id, user_id, name, type FROM trackers WHERE name = ? AND user_id = ?",
            params![tracker_name, user_id],
            |row| {
                let id: i32 = row.get(0).expect("tracker id should be present");
                let user_id: i32 = row.get(1).expect("tracker user should be present");
                let name: String = row.get(2).expect("tracker name should be present");
                let tracker_type: String = row.get(3).expect("tracker type should be present");

                Ok(Tracker {
                    id,
                    name,
                    user_id,
                    tracker_type,
                })
            },
        )
    }
    fn get_tracker_details_by_id(&self, tracker_id: &i32) -> Result<Tracker, rusqlite::Error> {
        let conn = self.get_connection();
        conn.query_row(
            "SELECT id, user_id, name, type FROM trackers WHERE id = ?",
            params![tracker_id],
            |row| {
                let id: i32 = row.get(0).expect("tracker id should be present");
                let user_id: i32 = row.get(1).expect("tracker user should be present");
                let name: String = row.get(2).expect("tracker name should be present");
                let tracker_type: String = row.get(3).expect("tracker type should be present");

                Ok(Tracker {
                    id,
                    name,
                    user_id,
                    tracker_type,
                })
            },
        )
    }

    fn add_tracker_entry(
        &self,
        tracker_id: &i32,
        value: &i32,
        date: &Option<String>,
    ) -> Result<(), rusqlite::Error> {
        // build query
        let query = match date {
            // date present then check for timestamp validity
            Some(date) => {
                let Some(date) = parse_timestamp_from_string(date, true) else {
                    return Err(rusqlite::Error::InvalidQuery);
                };
                format!(
                    "INSERT INTO tracker_entries (tracker_id, value, date)
                VALUES ({},{},'{}')",
                    tracker_id,
                    value,
                    format_timestamp(&date, Some("%Y-%m-%d %H:%M:%S"))
                )
            }
            // date not present then add with current time
            None => format!(
                "INSERT INTO tracker_entries (tracker_id, value) 
                VALUES ({},{})",
                tracker_id, value,
            ),
        };
        // make query
        let conn = self.get_connection();
        conn.execute_batch(query.as_str())
    }

    fn get_tracker_entries(
        &self,
        from: &Option<String>,
        to: &Option<String>,
        tracker: &Option<Tracker>,
    ) -> Result<Vec<TrackerEntry>, rusqlite::Error> {
        let conn = self.get_connection();
        let mut query = String::from("SELECT id, tracker_id, value, date FROM tracker_entries ");
        let mut query_modified = false;

        if let Some(from_date) = from
            .as_ref()
            .and_then(|ts| parse_timestamp_from_string(ts, true))
        {
            query += format!(
                "WHERE date >= '{}' ",
                format_timestamp(&from_date, Some("%Y-%m-%d %H:%M:%S"))
            )
            .as_str();
            query_modified = true;
        }
        if let Some(to_date) = to
            .as_ref()
            .and_then(|ts| parse_timestamp_from_string(ts, true))
        {
            query += format!(
                "WHERE date <= '{}' ",
                format_timestamp(&to_date, Some("%Y-%m-%d %H:%M:%S"))
            )
            .as_str();
            query_modified = true;
        }

        if let Some(tracker) = tracker {
            query = match query_modified {
                true => query + format!("AND tracker_id = {} ", tracker.id).as_str(),
                false => query + format!("WHERE tracker_id = {} ", tracker.id).as_str(),
            };
        }

        // run the statement
        query += ";";
        let mut statement = conn
            .prepare(&query)
            .expect("tracker entry query to succeed");
        let rows = statement
            .query_map([], |row| {
                Ok(TrackerEntry {
                    id: row.get(0).expect("tracker entry id to be defined"),
                    tracker_id: row.get(1).expect("tracker entry tracker to be defined"),
                    value: row.get(2).expect("tracker entry value to be defined"),
                    date: row.get(3).expect("tracker entry date to be defined"),
                })
            })
            .expect("tracker entry map to succeed");

        // read the entries
        let mut entries: Vec<TrackerEntry> = Vec::new();
        for entry in rows {
            entries.push(entry.expect("tracker entry object to be valid"));
        }

        Ok(entries)
    }
}

pub fn add_tracker(tracker_name: &str, tracker_type: &str) -> Result<(), Box<dyn Error>> {
    // get db and config
    let database = Database::new().expect("database should be initialized");
    let mut config = Config::new().expect("config must be initalized");

    // load config
    config.load().expect("config file should not be corrupted");

    // get user
    let user = config.user.expect("user must be defined in config");

    // create a new tracker
    database
        .intialize_tracker(tracker_name, tracker_type, &user.id)
        .expect("tracker insert query should be successful");

    Ok(())
}

pub fn add_tracker_entry(
    tracker_name: &str,
    tracker_value: &i32,
    date: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    // get db and config
    let database = Database::new().expect("database should be initialized");
    let mut config = Config::new().expect("config must be initalized");
    config.load().expect("config file should not be corrupted");

    // get user
    let user = config.user.expect("user must be defined in config");

    // get tracker information for user
    // TODO: better handling when no tracker present
    let tracker = match database.get_tracker_details_by_name(tracker_name, &user.id) {
        Ok(tracker) => tracker,
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => {
                return Err(Box::new(e));
            }
            _ => {
                return Err(Box::new(e));
            }
        },
    };

    // add tracker entry
    let result = database.add_tracker_entry(&tracker.id, tracker_value, date);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn print_tracker_entries(
    entries: &Vec<TrackerEntry>,
    database: &Database,
    sort_by: &Option<String>,
) {
    if entries.is_empty() {
        println!("No corresonding tracker entries were found");
        return;
    }

    // form table rows
    let mut trackers: HashMap<i32, Tracker> = HashMap::new();
    let mut rows: Vec<Vec<DatabaseValue>> = Vec::new();
    let mut max_timestamp = DateTime::<Utc>::MIN_UTC;
    let mut min_timestamp = DateTime::<Utc>::MAX_UTC;

    for entry in entries {
        let tracker = trackers.entry(entry.tracker_id).or_insert_with(|| {
            database
                .get_tracker_details_by_id(&entry.tracker_id)
                .expect("tracker to be present if entry present")
        });
        // entry id, tracker name, entry value, time
        let timestamp = parse_timestamp_from_string(&entry.date, false)
            .expect("date stored in database must be correct");
        max_timestamp = max_timestamp.max(timestamp);
        min_timestamp = min_timestamp.min(timestamp);

        rows.push(vec![
            DatabaseValue::Int(entry.id),
            DatabaseValue::String(tracker.name.clone()),
            DatabaseValue::Int(entry.value),
            DatabaseValue::Time(timestamp),
        ]);
    }

    // build table
    let mut builder = Builder::default();
    let show_tracker = trackers.len() > 1;

    // sort rows
    if let Some(sort_option) = sort_by {
        let sort_index = match sort_option.as_str() {
            "tracker" => 1,
            "value" => 2,
            "time" => 3,
            _ => 0, // id
        };
        rows.sort_by(|row1, row2| {
            row1.get(sort_index)
                .expect("every row element should have four elements")
                .cmp(
                    row2.get(sort_index)
                        .expect("every row element should have four elements"),
                )
        });
    }
    let dummy_tracker = trackers
        .get(&entries.first().unwrap().tracker_id)
        .expect("all trackers have been got when building table");
    let user = database
        .get_user_by_id(&dummy_tracker.user_id)
        .expect("if tracker present, user present");

    // define header
    if show_tracker {
        builder.push_record(["", format!("For User {}", user.name).as_str(), "", ""]);
        builder.push_record(["entry_id", "tracker", "value", "time"]);
    } else {
        builder.push_record([
            "",
            format!(
                "For User {} (ID: {})\nTracker: {}\n(type: {})",
                user.name, user.id, dummy_tracker.name, dummy_tracker.tracker_type
            )
            .as_str(),
            "",
        ]);
        builder.push_record(["entry_id", "value", "time"]);
    }
    // define content
    for mut row in rows {
        if !show_tracker {
            row.remove(1);
        }
        builder.push_record(row.iter().map(|element| match element {
            DatabaseValue::Int(value) => value.to_string(),
            DatabaseValue::String(value) => value.to_owned(),
            DatabaseValue::Time(value) => format_timestamp(
                &convert_timestamp_to_local(value),
                Some("%Y-%m-%d %H:%M:%S"),
            ),
        }));
    }

    // define footer
    // print out sum and average too
    let value_sum: i32 = entries.iter().map(|entry| entry.value).sum();
    let value_count: usize = entries.len();
    let mut scores: Vec<String> = Vec::new();
    scores.push(format!("Total: {} entries", value_count));
    scores.push(format!("Sum: {}", value_sum));
    if show_tracker {
        scores.push(format!(
            "Average: {}",
            value_sum as f32 / value_count as f32
        ));
    }
    scores.push(format!(
        "{} days",
        get_days_difference(&max_timestamp, &min_timestamp)
    ));
    builder.push_record(scores);

    let mut table = builder.build();

    // let clr_primary = Color::BG_WHITE | Color::FG_BLACK;
    // let clr_secondary = Color::BG_BRIGHT_BLACK | Color::FG_WHITE;
    let clr_head = Color::BG_CYAN | Color::FG_BLACK | Color::BOLD;
    let clr_footer = Color::BG_BLUE | Color::FG_BLACK | Color::BOLD;
    table
        // .with(Colorization::rows([clr_primary, clr_secondary]))
        .with(Colorization::exact([clr_head.clone()], Rows::first()))
        .with(Colorization::exact([clr_head], Rows::one(1)))
        .with(Colorization::exact([clr_footer], Rows::last()))
        .with(Style::modern_rounded())
        .with(BorderCorrection::span())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Merge::vertical());

    println!("{table}");
}

pub fn list_tracker_entries(
    from: &Option<String>,
    to: &Option<String>,
    tracker_name: &Option<String>,
    sort: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    // get db and config
    let database = Database::new().expect("database should be initialized");
    let mut config = Config::new().expect("config must be initalized");
    config.load().expect("config file should not be corrupted");

    // get user
    let user = config.user.expect("user must be defined in config");

    // get tracker information for user
    let tracker = tracker_name.clone().map(|name| {
        database
            .get_tracker_details_by_name(name.as_ref(), &user.id)
            .expect("tracker to be present in database")
    });

    // get all corresponding tracker entries
    let tracker_entries = match database.get_tracker_entries(from, to, &tracker) {
        Ok(entries) => entries,
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => Vec::new(),
            _ => return Err(Box::new(e)),
        },
    };

    // print all tracker entries
    print_tracker_entries(&tracker_entries, &database, sort);

    Ok(())
}
