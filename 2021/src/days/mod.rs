use eyre::Result;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub trait Day {
    fn run(&self) -> Result<()>;
}
