use super::Day;
use eyre::Result;
use std::fs::read_to_string;
use std::str::FromStr;

pub struct Day2 {}

impl Day2 {
    fn part_1(&self, raw_input: &str) -> Result<()> {
        let mut horiz = 0;
        let mut depth = 0;
        for line in raw_input.trim().lines() {
            if let [direction, amount] = line.split(' ').collect::<Vec<_>>().as_slice() {
                let amount_int = u32::from_str(amount)?;
                match *direction {
                    "forward" => horiz += amount_int,
                    "down" => depth += amount_int,
                    "up" => depth -= amount_int,
                    _ => panic!("bad direction"),
                }
            };
        }
        println!("horizontal position: {:?}, depth: {:?}", horiz, depth);
        println!("final result: {}", horiz * depth);
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let mut horiz = 0;
        let mut depth = 0;
        let mut aim = 0;
        for line in raw_input.trim().lines() {
            if let [direction, amount] = line.split(' ').collect::<Vec<_>>().as_slice() {
                let amount_int = u32::from_str(amount)?;
                match *direction {
                    "forward" => {
                        horiz += amount_int;
                        depth += aim * amount_int;
                    }
                    "down" => aim += amount_int,
                    "up" => aim -= amount_int,
                    _ => panic!("bad direction"),
                }
            };
        }
        println!("horizontal position: {:?}, depth: {:?}", horiz, depth);
        println!("final result: {}", horiz * depth);
        Ok(())
    }
}

impl Day for Day2 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day2-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day2-part1")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)
    }
}
