use eyre::Result;
use std::fs::read_to_string;

const PREAMBLE_SIZE: usize = 25;

fn find_preceding_sum(v: Vec<&u64>, n: &u64) -> Option<(usize, usize)> {
    //println!("searching for {}", n);
    for (i, num1) in v.iter().enumerate() {
        for (j, num2) in v.iter().skip(i).enumerate() {
            let result = **num1 + **num2;
            //println!("({},{}): {} + {} = {}", i, i + 1 + j, num1, num2, result);
            if result == *n {
                //println!("found it!");
                return Some((i, j));
            }
        }
    }
    println!("Searching for {} failed on {:?}", n, v);
    None
}

fn find_contiguous_sum(v: Vec<u64>, n: &u64) -> Option<u64> {
    for i in 0..v.len() {
        for j in i + 1..v.len() + 1 {
            //println!("{},{}", i, j);
            let range_iter = v.iter().take(j).skip(i).copied();
            let sum = range_iter.clone().sum::<u64>();
            if sum == *n {
                //println!("found it!");
                let min = range_iter.clone().min().unwrap();
                let max = range_iter.clone().max().unwrap();
                return Some(min + max);
            }
        }
    }
    None
}

fn main() -> Result<()> {
    let input = read_to_string("src/day9/input.txt")?;
    let nums: Vec<u64> = input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let last_possible_preamble = nums.len() - PREAMBLE_SIZE - 1;

    let mut preceding_sum_failure: u64 = 0;
    for preamble_index in 0..last_possible_preamble {
        let search_num = &nums[PREAMBLE_SIZE + preamble_index];
        match find_preceding_sum(
            nums.iter()
                .skip(preamble_index)
                .take(PREAMBLE_SIZE)
                .collect(),
            search_num,
        ) {
            Some(_result) => {}
            None => {
                println!(
                    "found first non matching number at index {}: {}",
                    PREAMBLE_SIZE + preamble_index,
                    search_num
                );
                preceding_sum_failure = *search_num;
            }
        }
    }

    println!("part 2");
    let sum = find_contiguous_sum(nums, &preceding_sum_failure);
    println!("final sum: {}", sum.unwrap());

    Ok(())
}
