use super::Day;
use eyre::Result;
use std::fs::read_to_string;
use std::str::FromStr;

const NEW_FISH_DAYS: u8 = 8;
const RESET_FISH_DAYS: u32 = 6;

struct SeaFloor {
    fish: Vec<u8>,
}

impl SeaFloor {
    fn new(init: Option<Vec<u8>>) -> Self {
        SeaFloor {
            fish: init.unwrap_or(vec![]),
        }
    }

    fn pass_days(&mut self, count: u32) -> Result<()> {
        for i in 0..count {
            let mut fish_to_add = 0;
            for f in &mut self.fish {
                match f {
                    0 => {
                        *f = 6;
                        fish_to_add += 1;
                    }
                    1..=NEW_FISH_DAYS => *f -= 1,

                    _ => (),
                };
            }
            // add the new fish
            self.fish.append(&mut vec![NEW_FISH_DAYS; fish_to_add]);
        }

        Ok(())
    }
}

pub struct Day6 {}

impl Day6 {
    fn parse_input(&self, raw_input: &str) -> Vec<u8> {
        let initial_state = raw_input
            .trim()
            .split(',')
            .map(|i| {
                println!("{:?}", i);
                i.parse::<u8>().unwrap()
            })
            .collect::<Vec<_>>();
        println!("initial state: {:?}", initial_state);
        initial_state
    }

    fn part_1(&self, raw_input: &str) -> Result<()> {
        let initial_state = self.parse_input(raw_input);
        let mut floor = SeaFloor::new(Some(initial_state));
        let days = 80;

        floor.pass_days(days);
        println!("{} fish after {} days", floor.fish.len(), days);
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        Ok(())
    }
}

impl Day for Day6 {
    fn run(&self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day6-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day6")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
