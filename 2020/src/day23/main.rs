use std::{thread, time};

//const INPUT: &str = "389125467"; // sample
const INPUT: &str = "219347865"; // my input
const DEBUG: bool = false;
const MAGIC: usize = 5558672960;

#[derive(Debug)]
struct CupCircle {
    vals: Vec<usize>,
    current: usize,
    limits: (usize, usize),
}

impl CupCircle {
    fn from_raw(initial: usize, raw_input: &str, len: usize) -> Self {
        let mut vals = vec![MAGIC; len + 1];
        let mut next_char_circle = raw_input.chars().cycle().skip(1);
        raw_input.chars().for_each(|ch| {
            vals[ch.to_digit(10).unwrap() as usize] =
                next_char_circle.next().unwrap().to_digit(10).unwrap() as usize
        });

        let max_input: usize = raw_input
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .max()
            .unwrap();
        let first_label = raw_input.chars().next().unwrap().to_digit(10).unwrap() as usize;
        let last_label = raw_input.chars().last().unwrap().to_digit(10).unwrap() as usize;
        if len > raw_input.len() {
            // generate the remaining items.
            let mut current_max = max_input + 1;
            vals[last_label] = current_max;
            for _ in max_input..len {
                vals[current_max] = current_max + 1;
                current_max += 1;
            }
            vals[current_max - 1] = first_label;
        }
        let &lowest = vals.iter().filter(|v| **v != MAGIC).min().unwrap();
        let &highest = vals.iter().filter(|v| **v != MAGIC).max().unwrap();
        CupCircle {
            vals,
            current: initial,
            limits: (lowest, highest),
        }
    }

    fn next(&self, initial: usize) -> usize {
        self.vals[initial] as usize
    }

    fn get_circle_repr(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut ptr = self.current;
        loop {
            result.push(ptr);
            ptr = self.vals[ptr];
            if ptr == self.current {
                break;
            }
        }
        result
    }

    fn get_final_circle_repr(&self) -> String {
        let repr = self.get_circle_repr();
        let len = repr.len() - 1;
        repr.iter()
            .cycle()
            .skip_while(|v| **v != 1)
            .skip(1) // skip the actual 1
            .take(len)
            .copied()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn print_round_state(&self, round_num: usize, destination: usize, next_three: &[usize; 3]) {
        println!();
        println!("-- round {} --", round_num);
        println!("circle: {:?}", self.get_circle_repr());
        println!("current: {}", self.current);
        println!("next three: {:?}", next_three);
        println!("destination: {}", destination);
    }

    fn find_destination(&self, next_three: &[usize; 3]) -> usize {
        let mut target_destination: usize = self.current - 1;
        let (lowest, highest) = self.limits;
        loop {
            if target_destination < lowest {
                target_destination = highest;
            } else if !next_three.contains(&target_destination)
                && (lowest..highest + 1).contains(&target_destination)
            {
                break;
            } else {
                if DEBUG {
                    println!(
                        "{} not in circle or in next three, skipping..",
                        target_destination
                    );
                }
                target_destination -= 1;
            }
        }
        target_destination
    }

    fn play(&mut self, rounds: usize) {
        let mut next_three = [0; 3]; // next three indices
        let mut destination: usize; // destination index
        for round_num in 0..rounds {
            if DEBUG && round_num % 100_000 == 0 {
                println!("{}", round_num);
            }
            next_three[0] = self.next(self.current);
            next_three[1] = self.next(next_three[0]);
            next_three[2] = self.next(next_three[1]);

            destination = self.find_destination(&next_three);
            if DEBUG {
                let _sleep_time = time::Duration::from_millis(1000);
                thread::sleep(_sleep_time);
                self.print_round_state(round_num + 1, destination, &next_three);
            }

            // rearrange
            let tmp_next = self.next(next_three[2]);
            self.vals[next_three[2]] = self.vals[destination];
            self.vals[destination] = self.next(self.current);
            self.vals[self.current] = tmp_next;

            self.current = self.next(self.current);
        }
    }
}

fn main() {
    let mut circle = CupCircle::from_raw(
        INPUT.chars().next().unwrap().to_digit(10).unwrap() as usize,
        INPUT,
        INPUT.len(),
    );
    circle.play(100);
    let final_circle = circle.get_final_circle_repr();
    println!("{:?}", final_circle);
    assert_eq!(final_circle, "36472598");

    // part2
    let mut circle = CupCircle::from_raw(
        INPUT.chars().next().unwrap().to_digit(10).unwrap() as usize,
        INPUT,
        1_000_000,
    );
    circle.play(10_000_000);
    let final_circle = circle.get_circle_repr();
    let final_product: usize = final_circle
        .iter()
        .skip_while(|x| **x != 1)
        .skip(1)
        .take(2)
        .product();
    println!("final product: {:?}", final_product);
}
