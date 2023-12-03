mod trebuchet;

use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use log::{debug, error, info, log_enabled, Level};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day to run
    #[arg(short, long, default_value_t = 2)]
    day: u8,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let args = Args::parse();
    match args.day {
        1 => trebuchet::run(),
        _ => {
            info!("not done yet");
            Ok(())
        }
    }
}
