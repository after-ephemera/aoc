use anyhow::Context;
use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, error, info};
use std::collections::BinaryHeap;
use std::collections::HashMap;

// 12 red cubes, 13 green cubes, and 14 blue cubes
//
lazy_static! {
    static ref MAP: HashMap<&'static str, usize> = [("red", 12), ("blue", 14), ("green", 13)]
        .iter()
        .copied()
        .collect();
}

pub fn run() -> Result<()> {
    cubes_1(
        "sample",
        &std::fs::read_to_string("src/sampledata/2.sample").unwrap(),
    )?;
    cubes_1(
        "part 1",
        &std::fs::read_to_string("src/sampledata/2.1").unwrap(),
    )?;
    cubes_2(
        "sample",
        &std::fs::read_to_string("src/sampledata/2.sample2").unwrap(),
    )?;
    cubes_2(
        "part 2",
        &std::fs::read_to_string("src/sampledata/2.1").unwrap(),
    )?;
    Ok(())
}

fn cubes_1(name: &str, f: &str) -> Result<()> {
    let sum: usize = f
        .lines()
        .enumerate()
        .filter_map(|(i, game)| {
            let game_not_possible = game
                .trim_start_matches(&format!("Game {}:", i + 1))
                .split(";")
                .any(|round| {
                    round.split(",").any(|cube| {
                        let entry = cube.trim().split(" ").collect::<Vec<_>>();
                        let (count, color) = (
                            entry[0]
                                .parse::<usize>()
                                .context(format!("unwrapping digit for {}", entry[0]))
                                .unwrap(),
                            entry[1],
                        );
                        *MAP.get(color).unwrap() < count
                    })
                });
            if game_not_possible {
                None
            } else {
                Some(i + 1)
            }
        })
        .map(|x| {
            debug!("{}", x);
            x
        })
        .sum();
    info!("{}: sum is {}", name, sum);
    Ok(())
}

fn cubes_2(name: &str, f: &str) -> Result<()> {
    let sum: usize = f
        .lines()
        .enumerate()
        .map(|(i, game)| {
            let mut red_heap = BinaryHeap::new();
            let mut green_heap = BinaryHeap::new();
            let mut blue_heap = BinaryHeap::new();
            game.trim_start_matches(&format!("Game {}:", i + 1))
                .split(";")
                .for_each(|round| {
                    round.split(",").for_each(|cube| {
                        let entry = cube.trim().split(" ").collect::<Vec<_>>();
                        let (count, color) = (
                            entry[0]
                                .parse::<usize>()
                                .context(format!("unwrapping digit for {}", entry[0]))
                                .unwrap(),
                            entry[1],
                        );
                        match color {
                            "red" => {
                                red_heap.push(count);
                            }
                            "green" => {
                                green_heap.push(count);
                            }
                            "blue" => {
                                blue_heap.push(count);
                            }
                            _ => {
                                error!("unknown color {}", color);
                            }
                        }
                    })
                });
            red_heap.pop().unwrap() * green_heap.pop().unwrap() * blue_heap.pop().unwrap()
        })
        .map(|x| {
            debug!("{}", x);
            x
        })
        .sum();
    info!("{}: sum is {}", name, sum);
    Ok(())
}
