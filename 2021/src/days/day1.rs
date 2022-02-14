use super::Day;
use eyre::Result;
use std::fs::read_to_string;
use std::str::FromStr;

pub struct Day1 {}

impl Day1 {
    fn part_1(&self, raw_input: &str) -> Result<()> {
        let mut times_increased = 0;
        let mut last: Option<u32> = None;
        for line in raw_input.trim().lines().map(|l| u32::from_str(l).unwrap()) {
            times_increased += if last.unwrap_or(line) < line { 1 } else { 0 };
            last = Some(line);
        }
        println!("times increased: {:?}", times_increased);
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let mut times_increased = 0;
        let mut last_sum: Option<u32> = None;
        for window in raw_input
            .trim()
            .lines()
            .map(|l| u32::from_str(l).unwrap())
            .collect::<Vec<u32>>()
            .windows(3)
        {
            let window_sum = window[0] + window[1] + window[2];
            times_increased += if last_sum.unwrap_or(window_sum) < window_sum {
                1
            } else {
                0
            };
            last_sum = Some(window_sum);
        }
        println!("times increased: {:?}", times_increased);
        Ok(())
    }
}

impl Day for Day1 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day1-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day1-part1")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)
    }
}
