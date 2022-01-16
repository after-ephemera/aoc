use super::Day;
use eyre::Result;
use std::fmt;
use std::fs::read_to_string;

struct OctoGrid {
    grid: Vec<Vec<u32>>,
}

impl fmt::Display for OctoGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.grid {
            for val in line {
                write!(f, " {:?}", val);
            }
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl OctoGrid {
    fn parse_input(raw_input: &str) -> Result<Vec<Vec<u32>>> {
        Ok(raw_input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>())
    }

    fn new(raw_input: &str) -> Self {
        Self {
            grid: Self::parse_input(raw_input).unwrap(),
        }
    }

    fn flash(&mut self, x: isize, y: isize) {
        //println!("flashing {:?}", (x, y));
        for dx in (x - 1)..=(x + 1) {
            for dy in (y - 1)..=(y + 1) {
                if dx < 0
                    || dy < 0
                    || dy >= self.grid.len() as isize
                    || dx >= self.grid[0].len() as isize
                {
                    // out of bounds
                    // println!("skipping {:?}", (dx, dy));
                    continue;
                }
                //increase
                self.grid[dx as usize][dy as usize] += 1;
                //println!(
                //"increased {:?} to {}",
                //(dx as usize, dy as usize),
                //self.grid[dx as usize][dy as usize]
                //);
            }
        }
    }

    fn step(&mut self) -> usize {
        let mut flashed = vec![];
        let mut total_flashes = 0;
        // add one to each item
        self.grid.iter_mut().flatten().for_each(|i| *i = *i + 1);
        //println!("after step 1:\n{}", self);
        // flash until no more flashes are possible
        let mut attempt_num = 0;
        while self.grid.iter().flatten().any(|&i| i >= 10) {
            let mut flashed_in_attempt = 0;
            //println!("on attempt {}", attempt_num);
            attempt_num += 1;
            let grid_copy = self.grid.clone();
            for x in 0..grid_copy.len() {
                for y in 0..grid_copy[0].len() {
                    if grid_copy[x][y] >= 10 {
                        if flashed.contains(&(x, y)) {
                            // flash at most once per step.
                            continue;
                        }
                        // flash
                        flashed_in_attempt += 1;
                        flashed.push((x, y));
                        total_flashes += 1;
                        self.flash(x as isize, y as isize);
                    }
                }
            }
            if flashed_in_attempt == 0 {
                // done flashing
                break;
            }
        }
        // set all flashes back to zero
        flashed.iter().for_each(|(x, y)| self.grid[*x][*y] = 0);
        //self.grid
        //.iter_mut()
        //.flatten()
        //.filter(|i| *i > &mut 9)
        //.for_each(|i| *i = 0);
        total_flashes
    }

    fn step_n(&mut self, step_count: usize) -> usize {
        let mut total_flashes = 0;
        for i in 0..step_count {
            // println!("step {}\n{}", i, self);
            let flash_count = self.step();
            total_flashes += flash_count;
            // println!("{} flashes", flash_count);
        }
        total_flashes
    }

    fn step_until_all_flash(&mut self) -> usize {
        let mut step = 0;
        loop {
            step += 1;
            let flash_count = self.step();
            if flash_count == self.grid.len() * self.grid[0].len() {
                // all flashed
                println!("all flashed on {}", step);
                break;
            }
        }
        step
    }
}

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self {}
    }

    fn part_1(&mut self, raw_input: &str) -> Result<()> {
        let mut grid = OctoGrid::new(raw_input);
        //let total_flashes = grid.step_n(3);
        let total_flashes = grid.step_n(100);
        println!("total flashes: {}", total_flashes);

        Ok(())
    }

    fn part_2(&mut self, raw_input: &str) -> Result<()> {
        let mut grid = OctoGrid::new(raw_input);
        //let total_flashes = grid.step_n(3);
        let all_flashed_step = grid.step_until_all_flash();
        println!("all flashes after : {}", all_flashed_step);

        Ok(())
    }
}

impl Day for Day11 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day11-sample")?;
        //let sample_raw_input = read_to_string("src/data/day11-test")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day11")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
