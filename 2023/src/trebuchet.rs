use anyhow::Context;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info, log_enabled, Level};
use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<&'static str, char> = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]
    .iter()
    .copied()
    .collect();
}

pub fn run() -> anyhow::Result<()> {
    trebuchet_1(
        "sample",
        &std::fs::read_to_string("src/sampledata/1.sample").unwrap(),
    );
    trebuchet_1(
        "part 1",
        &std::fs::read_to_string("src/sampledata/1.1").unwrap(),
    );
    trebuchet_2(
        "sample",
        &std::fs::read_to_string("src/sampledata/1.sample2").unwrap(),
    );
    trebuchet_2(
        "part 2",
        &std::fs::read_to_string("src/sampledata/1.1").unwrap(),
    );
    Ok(())
}

fn trebuchet_2(name: &str, f: &str) {
    let res: i32 = f
        .lines()
        .map(|line| {
            let mut first_digit = '!';
            for (i, ch) in line.chars().enumerate() {
                let line_remainder = &line.clone()[i..];
                if ch.is_digit(10) {
                    first_digit = ch;
                    break;
                } else {
                    for (k, v) in MAP.iter() {
                        if line_remainder.starts_with(k) {
                            first_digit = *v;
                        }
                    }
                    if first_digit != '!' {
                        break;
                    }
                }
            }

            let mut last_digit = '!';

            for (i, ch) in line.chars().rev().enumerate() {
                let i = line.len() - 1 - i;
                let line_remainder = &line.clone()[i..];
                if ch.is_digit(10) {
                    last_digit = ch;
                    break;
                } else {
                    for (k, v) in MAP.iter() {
                        if line_remainder.starts_with(k) {
                            last_digit = *v;
                        }
                    }
                    if last_digit != '!' {
                        break;
                    }
                }
            }
            let line_result = format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .context(format!("{}: {} and {}", name, first_digit, last_digit))
                .unwrap();
            debug!("{}: {} and {}", line_result, first_digit, last_digit);
            return line_result;
        })
        .sum();

    info!("{}: sum is {}", name, res);
}

fn trebuchet_1(name: &str, f: &str) {
    let res: i32 = f
        .lines()
        .map(|line| {
            let mut digit_chars = line.chars().filter(|ch| ch.is_digit(10));
            let line_num = vec![
                digit_chars.clone().nth(0).unwrap(),
                digit_chars.last().unwrap(),
            ]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
            debug!("{}", line_num);
            line_num
        })
        .sum();
    info!("{}: sum is {}", name, res);
}
