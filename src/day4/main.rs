use eyre::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::vec::Vec;

fn part1() -> Result<()> {
    let mut valid_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    valid_fields.sort();
    let input = read_to_string("src/day4/input.txt")?;
    let lines: Vec<&str> = input.split("\n\n").collect();
    let mut valid_count = 0;
    let mut invalid_count = 0;

    for line in lines {
        // println!("");
        //println!("\n{}\n", line);
        let mut passport_map = HashMap::new();
        for l in line.split(|c| c == ' ' || c == '\n') {
            if l == "" {
                continue;
            }
            let k_v: Vec<&str> = l.split(':').collect();
            let k = k_v[0];
            let v = k_v[1];
            passport_map.insert(k.to_string(), v);
        }
        passport_map.remove("cid");
        let keys_found: Vec<String> = passport_map.keys().cloned().collect();
        // convert to &str for easier comparison
        let mut keys_found_str: Vec<&str> = keys_found.iter().map(|s| s as &str).collect();
        keys_found_str.sort();
        let matching = keys_found_str
            .iter()
            .zip(&valid_fields)
            // .map(|(a, b)| {
            // println!("{:?}, {:?}. Equal? {}", a, b, a == b);
            // (a, b)
            // })
            .filter(|&(a, b)| a == b)
            .count();
        let valid_passport = matching == valid_fields.len();
        if valid_passport {
            valid_count += 1;
        } else {
            invalid_count += 1;
        }
    }
    println!(
        "{} valid passports, {} invalid passports",
        valid_count, invalid_count
    );
    Ok(())
}

fn validate(k: &str, v: &str) -> bool {
    match k {
        "byr" => {
            v.len() == 4 && 1920 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2002
        }
        "iyr" => {
            v.len() == 4 && 2010 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2020
        }
        "eyr" => {
            v.len() == 4 && 2020 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2030
        }
        "hgt" => {
            let is_cm = v.contains("cm");
            let is_in = v.contains("in");
            if is_cm && is_in {
                return false;
            }
            return if is_cm {
                let res: Vec<&str> = v.split("cm").collect();
                let val = res[0].parse::<i32>().unwrap();
                // println!("{} cm ", val);
                150 <= val && val <= 193
            } else if is_in {
                let res: Vec<&str> = v.split("in").collect();
                let val = res[0].parse::<i32>().unwrap();
                // println!("{} in ", val);
                59 <= val && val <= 76
            } else {
                false
            };
        }
        "hcl" => {
            let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
            re.is_match(v)
        }
        "ecl" => {
            if let Some(_) = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .find(|s| s == &&v)
            {
                true
            } else {
                false
            }
        }
        "pid" => {
            let re = Regex::new(r"[0-9]{9}").unwrap();
            re.is_match(v)
        }
        "cid" => true,
        _ => {
            println!("unknown value {}", k);
            false
        }
    }
}

fn part2() -> Result<()> {
    let mut valid_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    valid_fields.sort();
    //let input = read_to_string("src/day4/input-sample-invalid.txt")?;
    let input = read_to_string("src/day4/input.txt")?;
    let lines: Vec<&str> = input.split("\n\n").collect();
    let mut valid_count = 0;
    let mut invalid_count = 0;

    let mut invalid_count_map = HashMap::new();
    for line in lines {
        println!("");
        //println!("\n{}\n", line);
        let mut passport_map = HashMap::new();
        for l in line.split(|c| c == ' ' || c == '\n') {
            if l == "" {
                continue;
            }
            let k_v: Vec<&str> = l.split(':').collect();
            let k = k_v[0];
            let v = k_v[1];
            if validate(k, v) {
                passport_map.insert(k.to_string(), v);
            } else {
                println!("invalid {}", k);
                let counter = invalid_count_map.entry(k).or_insert(0);
                *counter += 1;
            }
        }
        passport_map.remove("cid");
        let keys_found: Vec<String> = passport_map.keys().cloned().collect();
        // convert to &str for easier comparison
        let mut keys_found_str: Vec<&str> = keys_found.iter().map(|s| s as &str).collect();
        keys_found_str.sort();
        let matching = keys_found_str
            .iter()
            .zip(&valid_fields)
            // .map(|(a, b)| {
            // println!("{:?}, {:?}. Equal? {}", a, b, a == b);
            // (a, b)
            // })
            .filter(|&(a, b)| a == b)
            .count();
        let valid_passport = matching == valid_fields.len();
        if valid_passport {
            valid_count += 1;
        } else {
            invalid_count += 1;
        }
    }
    println!(
        "{} valid passports, {} invalid passports",
        valid_count, invalid_count
    );
    println!("{:?}", invalid_count_map);
    Ok(())
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
