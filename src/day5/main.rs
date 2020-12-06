use eyre::Result;
use std::cmp::max;
use std::fs::read_to_string;
use std::vec::Vec;

fn part1() -> Result<()> {
    let input = read_to_string("src/day5/input.txt")?;
    let mut max_seat_id = 0;
    for line in input.split_whitespace() {
        let chars = line.chars().map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => c,
        });
        let row_b = chars.clone().take(7).collect::<String>();
        let col_b = chars.clone().skip(7).collect::<String>();
        //println!("{}:{}", row_b, col_b);
        let row = usize::from_str_radix(&row_b, 2).unwrap();
        let col = usize::from_str_radix(&col_b, 2).unwrap();
        //println!("\t{}:{}", row, col);
        let seat_id = row * 8 + col;
        max_seat_id = max(seat_id, max_seat_id);
    }

    println!("max seat id was {}", max_seat_id);

    Ok(())
}

fn part2() -> Result<()> {
    let input = read_to_string("src/day5/input.txt")?;
    let lines: Vec<&str> = input.split_whitespace().collect();
    // provide buffer just in case there are up to 50% missing seats in front + back
    let capacity = lines.len() as f64 * 1.5;
    let mut seats_full = vec![0; capacity as usize];
    for line in lines {
        let chars = line.chars().map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => c,
        });
        let row_b = chars.clone().take(7).collect::<String>();
        let col_b = chars.clone().skip(7).collect::<String>();
        let row = usize::from_str_radix(&row_b, 2).unwrap();
        let col = usize::from_str_radix(&col_b, 2).unwrap();
        let seat_id = row * 8 + col;
        seats_full[seat_id] = 1;
    }
    //println!("seats full {:?}", seats_full);
    let index = seats_full
        .iter()
        .enumerate()
        .skip_while(|(_, v)| v == &&0)
        .skip_while(|(_, v)| v == &&1)
        .take(1)
        .next();
    println!("found my seat! seat id is {:?}", index.unwrap().0);

    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
