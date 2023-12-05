
use anyhow::Result;

use log::{debug, info};
use regex::Regex;
use std::collections::HashMap;
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
    scratchcards_2(
        "sample 2",
        &std::fs::read_to_string("src/sampledata/4.sample").unwrap(),
    );
    scratchcards_2(
        "part 2",
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
    debug!("card: {}", card.clone().split(':').next().unwrap());
    let card_vals = Regex::new("Card [0-9]+:").unwrap().replace_all(card, "");
    let parts = card_vals
        .split('|')
        .map(str::trim)
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    // debug!("parts: {:?}", parts);
    let winning_numbers: Vec<_> = parts[0].intersection(&parts[1]).collect();
    // debug!("winning numbers: {:?}", winning_numbers);
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

fn get_points_from_card_2(card: &str) -> usize {
    debug!("card: {}", card.clone().split(':').next().unwrap());
    let card_vals = Regex::new("Card [0-9]+:").unwrap().replace_all(card, "");
    let parts = card_vals
        .split('|')
        .map(str::trim)
        .map(|s| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    // debug!("parts: {:?}", parts);
    let winning_numbers: Vec<_> = parts[0].intersection(&parts[1]).collect();
    // debug!("winning numbers: {:?}", winning_numbers);
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
        winning_number_count
    }
}

fn scratchcards_2(name: &str, f: &str) -> Result<()> {
    // create a mapping to track count by card number
    let mut card_count_map = HashMap::new();
    let original_card_count = f.lines().count();
    // initialize the map
    for i in 0..original_card_count {
        card_count_map.insert(i, 1);
    }
    let final_sum: usize = original_card_count
        + f.lines()
            .enumerate()
            .map(|(i, card)| {
                let count = *card_count_map.get(&i).unwrap();
                let card_sum = get_points_from_card_2(card);
                // for each of the next card_sum cards, increment the count
                for x in i + 1..=i + card_sum {
                    if x >= f.lines().count() {
                        continue;
                    }
                    *card_count_map.get_mut(&x).unwrap() += count;
                }
                debug!("{}: {} cards, adding {} ", i + 1, count, card_sum,);
                std::cmp::min(card_sum, original_card_count) * count
            })
            .sum::<usize>();
    info!("{}: sum is {}", name, final_sum);

    Ok(())
}
