use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use tabled::{
    Table,
    settings::{
        Alignment, Format, Merge, Modify, Style, object::Columns, themes::BorderCorrection,
    },
};
use uuid::Uuid;

use crate::{
    cli::SortOptions,
    config::AppConfig,
    database::{Database, DatabaseImpl},
    error::AppError,
    model::{Ordering, Tracker, TrackerEntry, TrackerFilter},
    time::get_days_difference,
};

pub(super) fn run(
    from: &Option<DateTime<Utc>>,
    to: &Option<DateTime<Utc>>,
    sort: &SortOptions,
    tracker_name: &Option<String>,
) -> Result<(), AppError> {
    let config = AppConfig::parse()?;
    let db = DatabaseImpl::from_config(&config)?;
    let mut filters = Vec::new();
    if let Some(from) = from {
        filters.push(TrackerFilter::Time {
            operation: Ordering::GreaterOrEqual,
            value: from.to_owned(),
        });
    }
    if let Some(to) = to {
        filters.push(TrackerFilter::Time {
            operation: Ordering::LessOrEqual,
            value: to.to_owned(),
        });
    }
    if let Some(name) = tracker_name {
        filters.push(TrackerFilter::Name(name.to_owned()));
    }
    let mut entries: Vec<TrackerEntry> = db.get_tracker_entries(&filters)?;
    // sort
    let tracker_ids: HashSet<Uuid> = entries.iter().map(|e| e.tracker_id).collect();
    let trackers: HashMap<Uuid, Tracker> = tracker_ids
        .iter()
        .map(|id| {
            db.get_tracker_from_id(id)
                .and_then(|opt_tracker| {
                    opt_tracker.ok_or(AppError::Corrupted {
                        field: "tracker id invalid",
                        value: id.to_string(),
                    })
                })
                .map(|tracker| (id.to_owned(), tracker))
        })
        .collect::<Result<HashMap<Uuid, Tracker>, AppError>>()?;
    entries.sort_by(|entry_a, entry_b| match sort {
        SortOptions::Tracker => {
            // WARN: usage of expect
            // we have already ensured that all tracker will be present
            // by doing error bubbling when getting trackers
            let tracker_a = trackers.get(&entry_a.tracker_id).expect(&format!(
                "tracker id = {} to be present in hashmap",
                entry_a.id
            ));
            let tracker_b = trackers.get(&entry_b.tracker_id).expect(&format!(
                "tracker id = {} to be present in hashmap",
                entry_b.id
            ));
            tracker_a.name.cmp(&tracker_b.name)
        }
        SortOptions::Value => entry_a.value.total_cmp(&entry_b.value),
        SortOptions::Time => entry_a.timestamp.cmp(&entry_b.timestamp),
        SortOptions::Id => entry_a.id.cmp(&entry_b.id),
    });
    print_entries_table(&entries, &trackers);
    Ok(())
}

fn print_entries_table(entries: &Vec<TrackerEntry>, trackers: &HashMap<Uuid, Tracker>) {
    if entries.is_empty() {
        println!("No entries present!");
        return;
    }
    // header - i don't believe there is a need anymore
    // main table
    let mut table = Table::new(entries);
    table
        .with(Style::modern_rounded())
        .with(BorderCorrection::span())
        .with(Alignment::center())
        .with(Alignment::center_vertical())
        .with(Merge::vertical())
        .with(
            Modify::new(Columns::one(0)).with(Format::content(|content| match content {
                "tracker_id" => "tracker".to_owned(),
                _ => Uuid::from_str(content)
                    .ok()
                    .and_then(|uuid| trackers.get(&uuid))
                    .map(|e| e.name.as_str())
                    .unwrap_or("?")
                    .to_string(),
            })),
        );
    // footer - better to just print things out than modify the tabled
    let sum: f64 = entries.iter().map(|e| e.value).sum();
    let num_entries = entries.len();
    // WARN: usage of expect
    // assert already should not make it happen
    // already ensured above, done so that we can expect safely
    assert!(num_entries > 0);
    let max_time = entries
        .iter()
        .map(|e| e.timestamp)
        .max()
        .expect("max time can't be calculated");
    let min_time = entries
        .iter()
        .map(|e| e.timestamp)
        .min()
        .expect("min time can't be calculated");
    let scores = format!(
        "Total : {} entries spanning over {} days, Sum: {}, Average: {:.2}",
        num_entries,
        get_days_difference(&max_time, &min_time),
        sum,
        sum / num_entries as f64
    );
    println!("{table}");
    println!("{scores}");
}
