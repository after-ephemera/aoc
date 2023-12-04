use anyhow::Context;
use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn run() -> Result<()> {
    scratchcards_1(
        "sample",
        &std::fs::read_to_string("src/sampledata/4.sample").unwrap(),
    );
    scratchcards_1(
        "part 1",
        &std::fs::read_to_string("src/sampledata/4.1").unwrap(),
    );
    Ok(())
}

fn scratchcards_1(name: &str, f: &str) -> Result<()> {
    let sum: usize = f.lines().map(get_points_from_card).sum();
    info!("{}: sum is {}", name, sum);
    Ok(())
}

fn get_points_from_card(card: &str) -> usize {
    debug!("card: {}", card.clone().split(":").next().unwrap());
    let card_vals = Regex::new("Card [0-9]+:").unwrap().replace_all(&card, "");
    let parts = card_vals
        .split('|')
        .map(str::trim)
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    debug!("parts: {:?}", parts);
    let winning_numbers: Vec<_> = parts[0].intersection(&parts[1]).collect();
    debug!("winning numbers: {:?}", winning_numbers);
    let winning_number_count = winning_numbers.len();
    if winning_number_count == 0 {
        //debug!("{}:{}", 0, 0);
        0
    } else {
        //debug!(
        //    "{}:{}",
        //    winning_number_count,
        //    usize::pow(2, winning_number_count as u32 - 1)
        //);
        usize::pow(2, winning_number_count as u32 - 1)
    }
}
