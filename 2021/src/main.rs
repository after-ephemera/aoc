mod days;
use days::*;
use eyre::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code, 2021")]
struct Opt {
    /// Challenge day
    #[structopt(short = "d", long = "day", default_value = "1")]
    day: u8,
}
fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("Day {:?}", opt.day);

    match opt.day {
        1 => {
            day1::Day1 {}.run()?;
        }
        2 => {
            day2::Day2 {}.run()?;
        }
        3 => {
            day3::Day3 {}.run()?;
        }
        4 => {
            day4::Day4 {}.run()?;
        }
        5 => {
            day5::Day5 {}.run()?;
        }
        6 => {
            day6::Day6 {}.run()?;
        }
        7 => {
            day7::Day7 {}.run()?;
        }
        8 => {
            day8::Day8 {}.run()?;
        }
        9 => {
            day9::Day9::new().run()?;
        }
        10 => {
            day10::Day10::new().run()?;
        }
        11 => {
            day11::Day11::new().run()?;
        }
        _ => {
            panic!("failed");
        }
    };
    Ok(())
}
