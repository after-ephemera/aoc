use super::Day;
use eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub struct Day8 {}

impl Day8 {
    fn get_digit_by_segment_count(&self, segment_count: usize) -> Option<usize> {
        match segment_count {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }

    fn get_digit_from_five_chars(
        &self,
        input: &HashSet<char>,
        corner: &HashSet<char>,
        signal_map: &HashMap<usize, HashSet<char>>,
    ) -> usize {
        if corner.is_subset(input) {
            5
        } else if signal_map.get(&7).unwrap().is_subset(input) {
            3
        } else {
            2
        }
    }

    fn get_digit_from_six_chars(
        &self,
        input: &HashSet<char>,
        corner: &HashSet<char>,
        signal_map: &HashMap<usize, HashSet<char>>,
    ) -> usize {
        if signal_map.get(&4).unwrap().is_subset(input) {
            9
        } else if corner.is_subset(input) {
            6
        } else {
            0
        }
    }

    fn part_1(&self, raw_input: &str) -> Result<()> {
        let unique_segment_count = raw_input
            .trim()
            .lines()
            .map(|line| line.split('|').nth(1).unwrap())
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|i| i.len())
                    .filter(|&i| self.get_digit_by_segment_count(i).is_some())
                    .collect::<Vec<_>>()
            })
            .count();
        println!(
            "There are {} uniquely identifiable outputs.",
            unique_segment_count
        );

        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let mut running_sum = 0;
        for line in raw_input.trim().lines() {
            let mut signal_map = HashMap::new();
            // find 1,4,7
            line.split_whitespace()
                .map(|i| (i, self.get_digit_by_segment_count(i.len())))
                // remove the bar and non-identifiable patterns
                .filter(|&(i, digit)| i != "|" && digit.is_some() && digit != Some(8))
                .for_each(|(signal, digit)| {
                    signal_map.insert(digit.unwrap(), signal.chars().collect::<HashSet<_>>());
                });
            //println!("map for line: {:?}", signal_map);

            // calculate the "corner" - the difference between 4 and 1
            let corner = signal_map
                .get(&4)
                .unwrap()
                .difference(signal_map.get(&1).unwrap())
                .copied()
                .collect::<HashSet<_>>();

            let mut output_for_line = vec![];
            for output_val in line.split('|').nth(1).unwrap().split_whitespace() {
                let digit = match self.get_digit_by_segment_count(output_val.len()) {
                    Some(digit) => digit,
                    None => match output_val.len() {
                        5 => self.get_digit_from_five_chars(
                            &output_val.chars().collect::<HashSet<_>>(),
                            &corner,
                            &signal_map,
                        ),
                        6 => self.get_digit_from_six_chars(
                            &output_val.chars().collect::<HashSet<_>>(),
                            &corner,
                            &signal_map,
                        ),
                        _ => panic!("unexpected output val {}", output_val),
                    },
                };

                output_for_line.push(digit);
            }
            let folded_val = output_for_line.iter().fold(0, |acc, elem| acc * 10 + elem);
            running_sum += folded_val;
        }
        println!("final sum is {}", running_sum);

        Ok(())
    }
}

impl Day for Day8 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day8-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day8")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
