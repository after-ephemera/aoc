mod cubes;
mod engine_parts;
mod scratchcards;
mod seeds;
mod trebuchet;

use anyhow::Result;
use clap::Parser;
use log::info;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day to run
    #[arg(short, long, default_value_t = 5)]
    day: u8,
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let args = Args::parse();
    info!("day {}", args.day);
    match args.day {
        1 => trebuchet::run(),
        2 => cubes::run(),
        3 => engine_parts::run(),
        4 => scratchcards::run(),
        5 => seeds::run(),
        _ => {
            info!("not done yet");
            Ok(())
        }
    }
}
