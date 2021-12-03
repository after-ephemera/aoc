use eyre::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Turn {
    num: usize,
    last_indices: (usize, usize),
    times_spoken: usize,
}

impl Turn {
    fn new(num: usize, index: usize) -> Self {
        Turn {
            num,
            last_indices: (0, index),
            times_spoken: 1,
        }
    }

    fn add_new_index(&mut self, index: usize) {
        self.last_indices.0 = self.last_indices.1;
        self.last_indices.1 = index;
    }
}

fn insert_or_update(map: &mut HashMap<usize, Turn>, &num: &usize, &index: &usize) {
    if let Some(new_last_turn) = map.get_mut(&num) {
        new_last_turn.times_spoken += 1;
        new_last_turn.add_new_index(index);
    } else {
        let turn = Turn::new(num, index);
        //println!("{} inserting turn: {:?}", index + 1, turn);
        map.insert(num, turn);
    }
}

fn main() -> Result<()> {
    //let input = read_to_string("src/day15/input-sample.txt")?;
    let input = read_to_string("src/day15/input.txt")?;
    let initial_turns = input.strip_suffix("\n").unwrap().split(',');
    let mut turn_index_map = HashMap::new();

    for (i, turn_str) in initial_turns.clone().enumerate() {
        if turn_str.is_empty() {
            continue;
        }
        let word_said = turn_str.parse::<usize>()?;
        let turn = Turn::new(word_said, i);
        println!("turn: {:?}", turn);
        turn_index_map.insert(word_said, turn);
    }

    let initial_len = initial_turns.clone().count();
    let mut last_spoken_number = initial_turns.clone().last().unwrap().parse::<usize>()?;
    //for i in initial_len..2020 {
    for i in initial_len..30000000 {
        if let Some(last_turn) = turn_index_map.get(&last_spoken_number) {
            let mut word_said = 0;
            if last_turn.times_spoken != 1 {
                // say last index
                let last_index = last_turn.last_indices.1;
                let penultimate_index = last_turn.last_indices.0;
                word_said = last_index - penultimate_index;
                // println!(
                //     "new word for {} is {} - {} = {}",
                //     i + 1,
                //     last_index,
                //     penultimate_index,
                //     word_said
                // );
            }
            insert_or_update(&mut turn_index_map, &word_said, &i);
            last_spoken_number = word_said;
        } else {
            println!("houston, we have a problem with {}", last_spoken_number);
        }
        if i % 500000 == 0 {
            println!("spoken number for {} is {}", i + 1, last_spoken_number);
        }
    }
    println!("last word said: {}", last_spoken_number);
    Ok(())
}
