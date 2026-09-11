use std::process::ExitCode;

mod cli;
mod command;
mod error;
mod time;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(_) => {
            println!("Query complete");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("some error occurred. details: {}", e);
            ExitCode::from(e)
        }
    }
}

fn run() -> Result<(), error::AppError> {
    let opts = cli::parse();
    command::run(&opts.command)?;
    Ok(())
}
