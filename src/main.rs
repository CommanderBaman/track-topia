use clap::Parser;
use track_topia::{Cli, run};

fn main() {
    let cli = Cli::parse();

    let result = run(cli);

    match result {
        Ok(_) => println!("Query complete"),
        Err(e) => println!("some error occurred. details: {}", e),
    }
}
