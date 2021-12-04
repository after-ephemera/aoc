mod days;
use days::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code, 2021")]
struct Opt {
    /// Challenge day
    #[structopt(short = "d", long = "day", default_value = "1")]
    day: u8,
}
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match opt.day {
        1 => {
            day1::Day1 {}.run();
        }
        2 => {
            day2::Day2 {}.run();
        }
        _ => {
            panic!("failed");
        }
    };
}
