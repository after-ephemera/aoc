use super::Day;
use eyre::Result;
use std::cmp;
use std::fs::read_to_string;

pub struct Day7 {}

impl Day7 {
    fn parse_input(&self, raw_input: &str) -> Vec<usize> {
        let initial_state = raw_input
            .trim()
            .split(',')
            .map(|i| {
                //println!("{:?}", i);
                i.parse::<usize>().unwrap()
            })
            .collect::<Vec<_>>();
        let mut fleet_map = vec![0; initial_state.iter().max().unwrap() + 1];
        for position in initial_state {
            fleet_map[position] += 1;
        }
        fleet_map
    }

    fn part_1(&self, raw_input: &str) -> Result<()> {
        let initial_state = self.parse_input(raw_input);
        println!("initial state: {:?}", initial_state);

        let mut minimum_fuel_usage = None;
        for possible_alignment in 0..initial_state.len() {
            println!("trying for possible alignment at {}", possible_alignment);
            // calculate the fuel for that alignment
            let possible_fuel_usage: usize = initial_state
                .iter()
                .enumerate()
                .map(|(i, val)| *val * (i as isize - possible_alignment as isize).abs() as usize)
                .sum::<usize>();
            println!("would cost {}", possible_fuel_usage);
            if let Some(cur_min) = minimum_fuel_usage {
                minimum_fuel_usage = Some(cmp::min(cur_min, possible_fuel_usage));
            } else {
                minimum_fuel_usage = Some(possible_fuel_usage);
            }
        }
        println!("minimum fuel is {}", minimum_fuel_usage.unwrap());

        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let initial_state = self.parse_input(raw_input);
        println!("initial state: {:?}", initial_state);

        let mut minimum_fuel_usage = None;
        for possible_alignment in 0..initial_state.len() {
            println!("trying for possible alignment at {}", possible_alignment);
            // calculate the fuel for that alignment
            let possible_fuel_usage: usize = initial_state
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    let difference = (i as isize - possible_alignment as isize).abs() as usize;
                    let triangle_sum = (difference.pow(2) + difference) / 2;
                    *val * triangle_sum
                })
                .sum::<usize>();
            println!("would cost {}", possible_fuel_usage);
            if let Some(cur_min) = minimum_fuel_usage {
                minimum_fuel_usage = Some(cmp::min(cur_min, possible_fuel_usage));
            } else {
                minimum_fuel_usage = Some(possible_fuel_usage);
            }
        }
        println!("minimum fuel is {}", minimum_fuel_usage.unwrap());
        Ok(())
    }
}

impl Day for Day7 {
    fn run(&self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day7-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day7")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
