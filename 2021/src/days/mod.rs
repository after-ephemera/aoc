use eyre::Result;

pub mod day1;

pub trait Day {
    fn run(&self) -> Result<()>;
}
