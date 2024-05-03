use clap::Parser;
use std::path::PathBuf;

mod nginx_log;
mod stats;

#[derive(Parser)]
struct Cli {
    pub input: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let log = match nginx_log::NginxLog::from_path(&args.input) {
        Ok(log) => log,
        Err(e) => {
            eprintln!("Error reading log file: {}", e);
            std::process::exit(1);
        }
    };

    let stats = stats::LogStats::from_nginx_log(&log);

    println!("{stats}")
}
