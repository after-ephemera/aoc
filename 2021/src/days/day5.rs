use super::Day;
use eyre::Result;
use std::cmp::{max, min};
use std::fmt;
use std::fs::read_to_string;

pub struct Day5 {}

fn get_range_for(x1: i32, x2: i32) -> Box<dyn Iterator<Item = i32>> {
    if x1 > x2 {
        //println!("{}-{}", x2, x1);
        Box::new((x2..x1 + 1).rev())
    } else {
        //println!("{}-{}", x2, x1);
        Box::new(x1..x2 + 1)
    }
}

#[derive(Clone, Copy, Debug)]
struct VentEntry {
    line_count: u32,
}

impl VentEntry {
    fn new() -> Self {
        VentEntry { line_count: 0 }
    }
}

struct VentMap {
    locations: Vec<VentEntry>,
    width: usize,
    height: usize,
    version: u8,
}

impl VentMap {
    fn new(
        segments: Vec<((i32, i32), (i32, i32))>,
        width: usize,
        height: usize,
        version: Option<u8>,
    ) -> Self {
        let mut result = VentMap {
            locations: vec![VentEntry::new(); width * height],
            width,
            height,
            version: version.unwrap_or(1),
        };
        for segment in segments {
            println!("working on {:?}", segment);
            let ((x1, y1), (x2, y2)) = (segment.0, segment.1);
            if x1 == x2 {
                // vertical
                //println!("vertical {},{}-{}", x1, y1, y2);
                for y in min(y1, y2)..max(y1, y2) + 1 {
                    let mut entry = result.at_mut(x1 as usize, y as usize);
                    entry.line_count += 1;
                    //println!("got: {:?}", entry);
                }
            } else if y1 == y2 {
                //horizontal
                //println!("horizontal {}-{},{}", x1, x2, y1);
                for x in min(x1, x2)..max(x1, x2) + 1 {
                    let mut entry = result.at_mut(x as usize, y1 as usize);
                    entry.line_count += 1;
                    //println!("got: {:?}", entry);
                }
            } else if result.version == 2 && (x2 - x1).abs() == (y2 - y1).abs() {
                // 45 degree diagonal line
                for (x, y) in (get_range_for(x1, x2)).zip(get_range_for(y1, y2)) {
                    println!("{},{}", x, y);
                    let mut entry = result.at_mut(x as usize, y as usize);
                    entry.line_count += 1;
                }
            } else {
                //println!("diagonal line");
                // diagonal line, ignore
            }
        }
        result
    }

    fn at(&self, x: usize, y: usize) -> &VentEntry {
        &self.locations[y * self.width + x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut VentEntry {
        //println!("trying to get {},{}", x, y);
        &mut self.locations[y * self.width + x]
    }

    /// Count the number of entries in the map with more
    /// than num lines passing through them.
    fn count_locations_with_at_least(&self, num: u32) -> usize {
        self.locations
            .iter()
            .filter(|e| e.line_count >= num)
            .count()
    }
}

impl fmt::Display for VentMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.at(x, y).line_count > 0 {
                    write!(f, " {} ", self.at(x, y).line_count)?;
                } else {
                    write!(f, " . ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Day5 {
    #[allow(dead_code)]
    fn run(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Day5 {
    fn part_1(&self, raw_input: &str) -> Result<()> {
        let raw_nums = raw_input
            .trim()
            .split_whitespace()
            .filter(|i| i != &"->")
            .flat_map(|i| i.split(',').collect::<Vec<_>>());
        let max_x = raw_nums
            .clone()
            .step_by(2)
            .max()
            .map(|i| i.parse::<usize>().unwrap())
            .unwrap()
            + 1;
        let max_y = raw_nums
            .clone()
            .skip(1)
            .step_by(2)
            .max()
            .map(|i| i.parse::<usize>().unwrap())
            .unwrap()
            + 1;
        println!("x max: {}, y max: {}", max_x, max_y);
        //println!("raw nums: {:?}", raw_nums.collect::<Vec<_>>());
        let entries = raw_input
            .trim()
            .split('\n')
            .map(|e| {
                let items = e
                    .split_whitespace()
                    .filter(|i| i != &"->")
                    .map(|i| {
                        let pair = i
                            .split(',')
                            .map(|n| n.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        (pair[0], pair[1])
                    })
                    .collect::<Vec<(i32, i32)>>();
                //println!("items: {:?}", items);
                (items[0], items[1])
            })
            .collect::<Vec<_>>();

        let map = VentMap::new(entries, max_x, max_y, None);
        //println!("{}", map);
        println!("final result: {}", map.count_locations_with_at_least(2));
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let raw_nums = raw_input
            .trim()
            .split_whitespace()
            .filter(|i| i != &"->")
            .flat_map(|i| i.split(',').collect::<Vec<_>>());
        let max_x = raw_nums
            .clone()
            .step_by(2)
            .max()
            .map(|i| i.parse::<usize>().unwrap())
            .unwrap()
            + 1;
        let max_y = raw_nums
            .clone()
            .skip(1)
            .step_by(2)
            .max()
            .map(|i| i.parse::<usize>().unwrap())
            .unwrap()
            + 1;
        println!("x max: {}, y max: {}", max_x, max_y);
        //println!("raw nums: {:?}", raw_nums.collect::<Vec<_>>());
        let entries = raw_input
            .trim()
            .split('\n')
            .map(|e| {
                let items = e
                    .split_whitespace()
                    .filter(|i| i != &"->")
                    .map(|i| {
                        let pair = i
                            .split(',')
                            .map(|n| n.parse::<i32>().unwrap())
                            .collect::<Vec<_>>();
                        (pair[0], pair[1])
                    })
                    .collect::<Vec<(i32, i32)>>();
                //println!("items: {:?}", items);
                (items[0], items[1])
            })
            .collect::<Vec<_>>();

        let map = VentMap::new(entries, max_x, max_y, Some(2));
        println!("{}", map);
        println!("final result: {}", map.count_locations_with_at_least(2));
        Ok(())
    }
}

impl Day for Day5 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day5-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day5")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)
    }
}
