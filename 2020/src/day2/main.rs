use eyre::Result;
use std::fs::read_to_string;
use std::vec::Vec;

fn part_1() -> Result<()> {
    let s = read_to_string("src/day2/input.txt")?;
    let lines = s.split('\n');

    let mut valid_count = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        let range: Vec<i32> = parts[0]
            .split('-')
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        let ch: char = parts[1].chars().take(1).next().unwrap();
        let password = parts[2];
        let occurence_count = password.matches(ch).count();
        if (range[0]..range[1] + 1).contains(&(occurence_count as i32)) {
            //println!("found valid");
            valid_count += 1;
        } else {
            println!(
                "{} was invalid. {} not in range {}..{}",
                password, ch, range[0], range[1]
            );
        }
        //println!("occurences: {}", occurence_count);
    }
    println!("{}", valid_count);
    Ok(())
}

fn part_2() -> Result<()> {
    let s = read_to_string("src/day2.2/input2.txt")?;
    let lines = s.split('\n');

    let mut valid_count = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(' ').collect();
        let range: Vec<i32> = parts[0]
            .split('-')
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        let ch: char = parts[1].chars().take(1).next().unwrap();
        let password = parts[2];

        let in_pos_1 = password.chars().take(range[0] as usize).last().unwrap() == ch;
        let in_pos_2 = password.chars().take(range[1] as usize).last().unwrap() == ch;

        if in_pos_1 && !in_pos_2 || !in_pos_1 && in_pos_2 {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
    Ok(())
}

fn main() -> Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}
