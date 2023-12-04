use anyhow::{Result};

use log::{debug, info};

use std::collections::HashSet;

#[derive(Debug)]
struct Part {
    number: String,
    location: (usize, usize),
}

struct Schematic {
    cells: Vec<Vec<char>>,
    parts: Vec<Part>,
    gears: Vec<(usize, usize)>,
}

/// Convert a line into parts with corresponding locations
fn to_parts(y: usize, mut line: &mut str) -> Vec<Part> {
    let mut parts = vec![];
    let mut x = 0;
    while !line.is_empty() {
        if line.starts_with(|c: char| c.is_digit(10)) {
            if let Some(end_index) = line.find(|c: char| !c.is_digit(10)) {
                parts.push(Part {
                    number: line[..end_index].to_string(),
                    location: (y, x),
                });
                x += end_index;
                line = &mut line[end_index..];
            } else if line.chars().all(|c| c.is_digit(10)) {
                parts.push(Part {
                    number: line.to_owned(),
                    location: (y, x),
                });
                let len = line.len();
                line = &mut line[len..];
            }
        } else {
            x += 1;
            line = &mut line[1..];
        }
    }
    parts
}

fn to_gears(y: usize, line: &mut str) -> Vec<(usize, usize)> {
    line.chars()
        .enumerate()
        .flat_map(|(x, c)| if c == '*' { Some((y, x)) } else { None })
        .collect()
}

impl Schematic {
    fn new(s: &str) -> Self {
        let parts = s
            .clone()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                debug!("line {}", y);
                to_parts(y, &mut l.to_owned()).into_iter()
            })
            .collect();

        let gears = s
            .clone()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| to_gears(y, &mut l.to_owned()).into_iter())
            .collect();
        debug!("gears: {:#?}", gears);
        Self {
            cells: s.lines().map(|l| l.chars().collect()).collect(),
            parts,
            gears,
        }
    }

    fn adjacent_parts_gear_ratio(&self, y: usize, x: usize) -> Option<usize> {
        let adjacent_parts: HashSet<_> = self
            .parts
            .iter()
            .flat_map(|p| {
                (p.location.1..p.location.1 + p.number.len()).filter_map(move |dx| {
                    // check if the part is adjacent, within one cell of the given coordinate
                    if (p.location.0 as isize - y as isize).abs() <= 1
                        && (dx as isize - x as isize).abs() <= 1
                    {
                        debug!("{:?} is adjacent to {:?}", p, (y, x));
                        Some(p.number.parse::<usize>().unwrap())
                    } else {
                        debug!("{:?} too far from {:?}", (p.location.0, dx), (y, x));
                        None
                    }
                })
            })
            .collect();
        if adjacent_parts.len() == 2 {
            Some(adjacent_parts.iter().product())
        } else {
            debug!("adjacent_parts_gear_ratio: {:?}", adjacent_parts);
            None
        }
    }
    fn is_adjacent_to_symbol(&self, x: isize, y: isize) -> bool {
        (-1..=1).any(|dy| {
            (-1..=1).any(|dx| {
                if (dx == 0 && dy == 0)
                    || x + dx < 0
                    || x + dx >= self.cells[0].len() as isize
                    || y + dy < 0
                    || y + dy >= self.cells.len() as isize
                {
                    false
                } else if self.cells[(y + dy) as usize][(x + dx) as usize] != '.'
                    && !self.cells[(y + dy) as usize][(x + dx) as usize].is_digit(10)
                {
                    // debug!(
                    //     "is_adjacent_to_symbol: {} at ({}, {})",
                    //     self.cells[y as usize][x as usize], y, x
                    // );
                    true
                } else {
                    false
                }
            })
        })
    }
}

pub fn run() -> Result<()> {
    engine_parts_1(
        "sample",
        &std::fs::read_to_string("src/sampledata/3.sample").unwrap(),
    )?;
    engine_parts_1(
        "part 1",
        &std::fs::read_to_string("src/sampledata/3.1").unwrap(),
    )?;
    engine_parts_2(
        "sample",
        &std::fs::read_to_string("src/sampledata/3.sample2").unwrap(),
    )?;
    engine_parts_2(
        "part 2",
        &std::fs::read_to_string("src/sampledata/3.1").unwrap(),
    )?;
    Ok(())
}

fn engine_parts_1(name: &str, f: &str) -> Result<()> {
    let schematic = Schematic::new(f);
    let sum: usize = schematic
        .parts
        .iter()
        .filter_map(|part| {
            // debug!("checking {:?} with {}", part, part.number.len());
            if (0..part.number.len()).any(|i| {
                schematic.is_adjacent_to_symbol(
                    part.location.1 as isize + i as isize,
                    part.location.0 as isize,
                )
            }) {
                // debug!("Adding {}", part.number);
                Some(part.number.parse::<usize>().unwrap())
            } else {
                None
            }
        })
        .sum();
    info!("{}: sum is {}", name, sum);
    Ok(())
}

fn engine_parts_2(name: &str, f: &str) -> Result<()> {
    let schematic = Schematic::new(f);
    let sum: usize = schematic
        .gears
        .iter()
        .filter_map(|gear| {
            let ratio = schematic.adjacent_parts_gear_ratio(gear.0, gear.1);
            debug!("part ratio for gear {:?}, {:?}", gear, ratio);
            ratio
        })
        .sum();
    info!("{}: sum is {}", name, sum);
    Ok(())
}
