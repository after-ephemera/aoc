use super::Day;
use eyre::Result;
use std::cmp::Ordering;
use std::fs::read_to_string;

pub struct Day3 {}

impl Day3 {
    /// Assemble a vec of most common bits, where the
    /// index in the vec corresponds to the left-aligned
    /// bit position.
    ///
    /// A positive value indicates that the mcb is 1
    /// and a negative value indicates that the mcb is
    /// -1.
    fn get_mcbs(&self, lines: &[&str]) -> Vec<Option<i32>> {
        let mut counts = vec![0; lines[0].len()];
        for line in lines {
            for (i, ch) in line.chars().enumerate() {
                //println!("{}, {}", i, ch);
                counts[i] += match ch {
                    '0' => -1,
                    '1' => 1,
                    _ => panic!("unexpected char"),
                };
            }
        }
        let mut result = vec![None; lines[0].len()];
        for (i, count) in counts.into_iter().enumerate() {
            result[i] = match count.cmp(&0) {
                Ordering::Greater => Some(1),
                Ordering::Less => Some(0),
                Ordering::Equal => None,
            };
        }
        result
    }

    fn part_1(&self, raw_input: &str) -> Result<()> {
        let lines = raw_input.trim().lines().collect::<Vec<_>>();
        // set most common bit for each column
        let most_common_bits = self.get_mcbs(&lines);
        let mut gamma_rate = 0;
        println!("most_common_bits: {:?}", most_common_bits);
        for bit in &most_common_bits {
            gamma_rate <<= 1;
            gamma_rate |= bit.unwrap();
        }
        println!("gamma_rate: {} b{:b}", gamma_rate, gamma_rate);
        // create a bitmask to avoid unnecessary ones
        let mut mask = 0;
        for _ in 0..most_common_bits.len() {
            mask <<= 1;
            mask |= 1;
        }
        println!("mask: {:b}", mask);
        let epsilon_rate = !gamma_rate & mask;
        println!("epsilon_rate: {} b{:b}", epsilon_rate, epsilon_rate);
        println!("final result: {}", epsilon_rate * gamma_rate);
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let lines = raw_input.trim().lines().collect::<Vec<_>>();
        println!("lines: {:?}", lines);
        let mut oxygen_generator_rating = None;
        let mut c02_scrubber_rating = None;
        let mut oxygen_generator_vals = lines.clone();
        let mut c02_scrubber_vals = lines.clone();
        for i in 0..lines[0].len() {
            if oxygen_generator_rating.is_some() && c02_scrubber_rating.is_some() {
                // found both values already
                break;
            }

            if oxygen_generator_rating.is_none() {
                let bit = self.get_mcbs(&oxygen_generator_vals)[i];
                oxygen_generator_vals = oxygen_generator_vals
                    .into_iter()
                    .filter(|line| {
                        let mcb = if let Some(bit_val) = bit { bit_val } else { 1 };
                        line.chars().collect::<Vec<_>>()[i]
                            == char::from_digit(mcb as u32, 10).unwrap()
                    })
                    .collect::<Vec<_>>();
                if oxygen_generator_vals.len() == 1 {
                    // found it

                    oxygen_generator_rating =
                        Some(u32::from_str_radix(oxygen_generator_vals[0], 2).unwrap());
                }
                //println!(
                //"ox: {} – {:?}",
                //oxygen_generator_vals.len(),
                //oxygen_generator_vals
                //);
            }

            if c02_scrubber_rating.is_none() {
                let bit = self.get_mcbs(&c02_scrubber_vals)[i];
                c02_scrubber_vals = c02_scrubber_vals
                    .into_iter()
                    .filter(|line| {
                        let mcb = if let Some(bit_val) = bit { bit_val } else { 1 };
                        let not_bit = if mcb == 0 { '1' } else { '0' };
                        line.chars().collect::<Vec<_>>()[i] == not_bit
                    })
                    .collect::<Vec<_>>();
                if c02_scrubber_vals.len() == 1 {
                    // found it
                    c02_scrubber_rating =
                        Some(u32::from_str_radix(c02_scrubber_vals[0], 2).unwrap());
                }
                //println!(
                //"c02 for bit {}: {} – {:?}",
                //i,
                //c02_scrubber_vals.len(),
                //c02_scrubber_vals
                //);
            }
        }
        println!(
            "final result: {}",
            oxygen_generator_rating.unwrap() * c02_scrubber_rating.unwrap()
        );
        Ok(())
    }
}

impl Day for Day3 {
    fn run(&self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day3-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day3")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)
    }
}
