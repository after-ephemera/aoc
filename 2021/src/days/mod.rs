use eyre::Result;

pub mod day1;
pub mod day2;

pub trait Day {
    fn run(&self) -> Result<()>;
}
