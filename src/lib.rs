mod config;
pub mod constants;
mod db;
pub mod file;
pub mod time;
mod tracker;
pub mod user;

use clap::{Parser, Subcommand};
use std::error::Error;

use crate::{
    config::initialize_user,
    db::initialize_database,
    tracker::{add_tracker, add_tracker_entry, list_tracker_entries},
};

/// TrackTopia CLI: A personal tracking tool
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    /// command to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new tracking entry
    Add {
        /// Name of the tracker (e.g., Milk, Dinner)
        #[arg(short, long)]
        tracker_name: String,

        /// Value for the entry (boolean, count, or continuous number)
        #[arg(short, long)]
        value: i32,

        /// Date for the entry (YYYY-MM-DD HH:MM:SS). Defaults to today if not provided.
        #[arg(short, long)]
        date: Option<String>,
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
        #[arg(long)]
        tracker: Option<String>,

        /// Sort entries by date, tracker, or value
        #[arg(long)]
        sort: Option<String>,
    },

    /// Show summaries of tracked tracker
    Summary {
        /// Tracker to summarize
        #[arg(long)]
        _tracker: String,

        /// Start date for summary
        #[arg(long)]
        _from: String,

        /// End date for summary
        #[arg(long)]
        _to: Option<String>,
    },

    /// Initialize the database with tables
    Initialize {
        /// User name to store
        #[arg(long)]
        user_name: Option<String>,
    },

    /// Track a new habit
    Track {
        /// Name of Tracker
        #[arg(long)]
        tracker_name: String,

        /// Type of Tracker
        #[arg(long)]
        tracker_type: String,
    },
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    // add -> track -> summary -> list
    match &cli.command {
        Commands::Add {
            tracker_name,
            value,
            date,
        } => {
            add_tracker_entry(tracker_name, value, date)?;
            Ok(())
        }
        Commands::Initialize { user_name } => {
            // TODO: better error handling for this
            initialize_database()?;
            initialize_user(user_name.as_deref())?;
            Ok(())
        }
        Commands::Summary {
            _tracker,
            _from,
            _to,
        } => {
            println!("summary command");
            Ok(())
        }
        Commands::List {
            from,
            to,
            tracker,
            sort,
        } => {
            list_tracker_entries(from, to, tracker, sort)?;
            Ok(())
        }
        Commands::Track {
            tracker_name,
            tracker_type,
        } => {
            add_tracker(tracker_name, tracker_type)?;
            Ok(())
        }
    }
}
