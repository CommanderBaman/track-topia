use clap::{Parser, Subcommand, ValueEnum};

use crate::model::TrackerKind;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "A simple program to help you keep track of things",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/CommanderBaman/track-topia/issues"
)]
pub struct Opts {
    /// command to run
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, ValueEnum)]
pub enum SortOptions {
    Tracker,
    Time,
    Value,
    Id,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new tracking entry
    Add {
        /// name of the tracker (e.g., milk, dinner)
        #[arg(long, short = 't')]
        tracker: String,
        /// value for the entry (boolean, count, or continuous number)
        #[arg(long, short = 'v')]
        value: f64,
        /// timestamp for the entry (YYYY-MM-DD HH:MM:SS). Defaults to now if not provided.
        #[arg(long, short = 'd')]
        timestamp: Option<String>,
        /// pass true if the timestamp is utc instead of local
        #[arg(long, default_value_t = false)]
        parse_as_utc: bool,
    },
    /// List all entries with optional filters
    List {
        /// Start date filter (inclusive)
        #[arg(long)]
        from: Option<String>,
        /// End date filter (inclusive)
        #[arg(long)]
        to: Option<String>,
        /// Filter by tracked tracker name
        #[arg(long, short = 't')]
        tracker: Option<String>,
        /// Sort entries by date, tracker, or value
        #[arg(long, short = 's', value_enum, default_value_t = SortOptions::Id)]
        sort: SortOptions,
        /// pass true if the timestamp is utc instead of local
        #[arg(long, default_value_t = false)]
        parse_as_utc: bool,
    },
    /// Initialize the database with tables
    Initialize {},
    /// Track a new habit
    Track {
        /// Name of Tracker
        #[arg(long, short = 'n')]
        name: String,
        /// Type of Tracker
        #[arg(long, value_enum)]
        tracker_kind: TrackerKind,
    },
}

pub fn parse() -> Opts {
    Opts::parse()
}
