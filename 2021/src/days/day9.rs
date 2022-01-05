use super::Day;
use eyre::Result;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

pub struct Day9 {
    map: Vec<(usize, Vec<(usize, u32)>)>,
}

impl Day9 {
    pub fn new() -> Self {
        Self { map: vec![] }
    }

    fn get_neighbor_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let len_i = self.map.len() - 1;
        let len_j = self.map[0].1.len() - 1;
        let mut res = vec![];

        if i > 0 {
            res.push((i - 1, j));
        }
        if i < len_i {
            res.push((i + 1, j));
        }
        if j > 0 {
            res.push((i, j - 1));
        }
        if j < len_j {
            res.push((i, j + 1));
        }
        res
    }

    fn sum_basin_neighbors(&self, i: usize, j: usize) -> usize {
        let mut visited = vec![];
        self.sum_basin_neighbors_recursive(i, j, &mut visited)
    }

    /// naming is hard and I'm not feeling creative
    fn sum_basin_neighbors_recursive(
        &self,
        i: usize,
        j: usize,
        visited: &mut Vec<(usize, usize)>,
    ) -> usize {
        let mut sum = 1;
        for (neighbor_i, neighbor_j) in self.get_neighbor_indices(i, j) {
            let neighbor = self.map[neighbor_i].1[neighbor_j].1;
            if neighbor != 9
                && neighbor > self.map[i].1[j].1
                && !visited.contains(&(neighbor_i, neighbor_j))
            {
                visited.push((neighbor_i, neighbor_j));
                sum += self.sum_basin_neighbors_recursive(neighbor_i, neighbor_j, visited);
            }
        }
        sum
    }

    fn get_neighbors(&self, i: usize, j: usize) -> Vec<u32> {
        let len_i = self.map.len() - 1;
        let len_j = self.map[0].1.len() - 1;
        let mut res = vec![];
        if i > 0 {
            // i-1, j
            res.push(self.map[i - 1].1[j].1);
            if j > 0 {
                // i-1, j-1
                res.push(self.map[i - 1].1[j - 1].1);
            }
            if j < len_j {
                // i-1, j+1
                res.push(self.map[i - 1].1[j + 1].1);
            }
        };
        if j > 0 {
            // i, j-1
            res.push(self.map[i].1[j - 1].1);
        }

        if i < len_i {
            // i+1, j
            res.push(self.map[i + 1].1[j].1);
            if j > 0 {
                // i+1, j-1
                res.push(self.map[i + 1].1[j - 1].1);
            }
            if j < len_j {
                // i+1, j+1
                res.push(self.map[i + 1].1[j + 1].1);
            }
        }
        if j < len_j {
            // i, j+1
            res.push(self.map[i].1[j + 1].1);
        }

        res
    }

    fn parse_input(&mut self, raw_input: &str) -> Result<()> {
        self.map = raw_input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .enumerate()
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .collect();
        Ok(())
    }

    fn part_1(&mut self, raw_input: &str) -> Result<()> {
        let mut low_points = vec![];
        self.parse_input(raw_input)?;
        for (i, line) in &self.map {
            for (j, height) in line {
                let neighbors = self.get_neighbors(*i, *j);
                //println!("{:?} has neighbors {:?}", height, neighbors);
                if neighbors.iter().all(|i| i > height) {
                    //println!("found low point in {:?}", (height, i, j));
                    low_points.push(height + 1);
                }
            }
        }
        println!("risk level sum is {}", low_points.iter().sum::<u32>());
        Ok(())
    }

    fn part_2(&mut self, raw_input: &str) -> Result<()> {
        let mut low_points = vec![];
        self.parse_input(raw_input)?;
        for (i, line) in &self.map {
            for (j, height) in line {
                let neighbors = self.get_neighbors(*i, *j);
                if neighbors.iter().all(|i| i > height) {
                    low_points.push((height + 1, i, j));
                }
            }
        }

        let mut basins = BinaryHeap::new();
        for (lp_val, lp_i, lp_j) in low_points {
            let basin_size = self.sum_basin_neighbors(*lp_i, *lp_j);
            println!("low point {:?} has {:?} basin vals", lp_val, basin_size);
            basins.push(basin_size);
        }
        println!("final basins: {:?}.", basins,);
        println!(
            "Top 3 product: {}",
            basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
        );

        Ok(())
    }
}

impl Day for Day9 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day9-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day9")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
