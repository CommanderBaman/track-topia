use std::process::ExitCode;

fn main() -> std::process::ExitCode {
    match track_topia::run() {
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
