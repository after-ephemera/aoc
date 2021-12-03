use eyre::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn part1(input: Vec<isize>) {
    let mut last = input[0];
    // first adapter always adds one
    let mut one_jolt_diffs = 1;
    let mut three_jolt_diffs = 0;
    for line in input.iter().skip(1) {
        let diff = line - last;
        match diff {
            1 => one_jolt_diffs += 1,
            3 => three_jolt_diffs += 1,
            _ => (),
        }
        last = *line;
    }
    // add the final adapter, always three higher than the highest.
    three_jolt_diffs += 1;
    println!(
        "{} one jolt diffs * {} three jolt diffs = {}",
        one_jolt_diffs,
        three_jolt_diffs,
        one_jolt_diffs * three_jolt_diffs
    );
}

fn find_possible_solutions(
    input: &HashSet<isize>,
    value: isize,
    visited: &mut HashMap<isize, isize>,
) -> isize {
    // memoized
    return if let Some(&v) = visited.get(&value) {
        v
    } else if value == 0 {
        // reached the end
        1
    } else if value < 0 || !input.contains(&value) {
        // not a valid solution
        0
    } else {
        // search a little deeper
        let result = find_possible_solutions(input, value - 1, visited)
            + find_possible_solutions(input, value - 2, visited)
            + find_possible_solutions(input, value - 3, visited);
        // memoize
        visited.insert(value, result);
        result
    };
}

fn part2(input: &HashSet<isize>) -> isize {
    find_possible_solutions(input, *input.iter().max().unwrap(), &mut HashMap::new())
}

fn main() -> Result<()> {
    let mut input: Vec<isize> = read_to_string("src/day10/input.txt")?
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    input.sort_unstable();
    part1(input);
    let input = read_to_string("src/day10/input.txt")?
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let result = part2(&input);
    //println!("final result: {:#?}", result);
    println!("final result: {:?}", result);
    Ok(())
}
