mod cli;
mod command;
mod config;
mod database;
mod error;
mod file;
mod model;
mod time;

pub fn run() -> Result<(), error::AppError> {
    let opts = cli::parse();
    command::run(&opts.command)?;
    Ok(())
}
