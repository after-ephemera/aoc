use eyre::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

fn part1() -> Result<()> {
    let input_str = read_to_string("src/day6/input.txt")?;
    let entries = input_str.split("\n\n");
    let mut count = 0;
    for entry in entries {
        let mut questions = HashSet::new();
        for c in entry.chars() {
            if c == '\n' {
                continue;
            }
            let charval = c as u32 - 'a' as u32;
            // println!("charval {}", charval);
            if charval < 26 {
                questions.insert(c);
            }
        }
        count += questions.len();
    }
    println!("total of {}", count);
    Ok(())
}

fn part2() -> Result<()> {
    let input_str = read_to_string("src/day6/input.txt")?;
    let entries = input_str.split("\n\n");
    let mut count = 0;
    for entry in entries {
        let s: Vec<HashSet<char>> = entry
            .split('\n')
            .map(|person_answer| {
                let mut questions = HashSet::new();
                for c in person_answer.chars() {
                    if c == '\n' {
                        continue;
                    }
                    let charval = c as u32 - 'a' as u32;
                    // println!("charval {}", charval);
                    if charval < 26 {
                        questions.insert(c);
                    }
                }
                questions
            })
            .collect();

        let mut answered_by_all: HashSet<char> = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .iter()
        .cloned()
        .collect();
        for set in s {
            //println!("{:?}", set);
            answered_by_all = answered_by_all.intersection(&set).copied().collect();
        }
        count += answered_by_all.len();
        //println!("final: {:?}", answered_by_all.len());
    }
    println!("total of {}", count);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    println!("part 2:");
    part2()?;
    Ok(())
}
