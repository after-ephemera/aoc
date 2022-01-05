use super::Day;
use eyre::Result;
use std::fs::read_to_string;

const NEW_FISH_DAYS: usize = 8;

struct SeaFloor {
    fish: [usize; NEW_FISH_DAYS + 1],
}

impl SeaFloor {
    fn new(init: Option<&[usize]>) -> Self {
        let mut fish = [0; NEW_FISH_DAYS + 1];
        if let Some(items) = init {
            for &item in items {
                fish[item] += 1;
            }
        }
        SeaFloor { fish }
    }

    fn pass_days(&mut self, count: u32) -> Result<()> {
        for _ in 0..count {
            self.fish.rotate_left(1);
            self.fish[6] += self.fish[NEW_FISH_DAYS];
            //println!("day {}, fish: {:?}", i, self.fish);
        }

        Ok(())
    }
}

pub struct Day6 {}

impl Day6 {
    fn parse_input(&self, raw_input: &str) -> Vec<usize> {
        let initial_state = raw_input
            .trim()
            .split(',')
            .map(|i| {
                //println!("{:?}", i);
                i.parse::<usize>().unwrap()
            })
            .collect::<Vec<_>>();
        println!("initial state: {:?}", initial_state);
        initial_state
    }

    fn part_1(&self, raw_input: &str) -> Result<()> {
        let initial_state = self.parse_input(raw_input);
        let mut floor = SeaFloor::new(Some(&initial_state));
        let days = 80;

        floor.pass_days(days)?;
        println!(
            "{} fish after {} days",
            floor.fish.iter().sum::<usize>(),
            days
        );
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let initial_state = self.parse_input(raw_input);
        let mut floor = SeaFloor::new(Some(&initial_state));
        let days = 256;

        floor.pass_days(days)?;
        println!(
            "{} fish after {} days",
            floor.fish.iter().sum::<usize>(),
            days
        );
        Ok(())
    }
}

impl Day for Day6 {
    fn run(&mut self) -> Result<()> {
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
