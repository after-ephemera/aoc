use super::Day;
use eyre::Result;
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

const OPEN: &str = "({[<";
lazy_static! {
    static ref CLOSE_MAP: HashMap<char, char> =
        hashmap! {'{'=> '}', '('=> ')', '['=> ']', '<'=> '>'};
    /// ): 3 points.
    /// ]: 57 points.
    /// }: 1197 points.
    /// >: 25137 points.
    static ref POINT_MAP: HashMap<char, usize> =
        hashmap! {'}'=> 1197, ')'=> 3, ']'=> 57, '>'=> 25137};
    /// ): 1 point.
    /// ]: 2 points.
    /// }: 3 points.
    /// >: 4 points.
    static ref COMPLETION_POINT_MAP: HashMap<char, usize> =
        hashmap! {'('=> 1, '['=> 2, '{'=> 3, '<'=> 4};
}

#[derive(Debug, Clone)]
struct MatchError {
    culprit: char,
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error matching")
    }
}

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_input(&mut self, raw_input: &str) -> Result<Vec<Vec<char>>> {
        Ok(raw_input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    fn process_line(&self, stack: &mut Vec<char>, line: &[char]) -> Result<(), MatchError> {
        for &ch in line {
            if OPEN.contains(ch) {
                stack.push(ch);
            } else {
                let matchable = stack.pop().unwrap();
                //println!("trying for {:?}{:?}", matchable, ch);
                if CLOSE_MAP[&matchable] != ch {
                    // corrupt line
                    //println!(
                    //    "corrupt on char {:?} ({:?}) doesn't match {:?}",
                    //    matchable, CLOSE_MAP[&matchable], ch
                    //);
                    return Err(MatchError { culprit: ch });
                } else {
                    //println!("matched {}", ch);
                }
            }
        }
        Ok(())
    }

    fn score_completion(&self, unmatched_chars: &[char]) -> usize {
        let mut score = 0;
        for ch in unmatched_chars.iter().rev() {
            score *= 5;
            score += COMPLETION_POINT_MAP[ch];
        }
        score
    }

    fn part_1(&mut self, raw_input: &str) -> Result<()> {
        let input = self.parse_input(raw_input)?;
        let mut stack = vec![];
        let mut error_score = 0;
        for (_i, line) in input.iter().enumerate() {
            match self.process_line(&mut stack, line) {
                Ok(_) => (),
                Err(match_err) => {
                    //println!("err on line {}", i);
                    error_score += POINT_MAP[&match_err.culprit];
                }
            }
        }
        println!("final score: {}", error_score);

        Ok(())
    }

    fn part_2(&mut self, raw_input: &str) -> Result<()> {
        let mut completion_scores = vec![];
        let input = self.parse_input(raw_input)?;
        for (i, line) in input.iter().enumerate() {
            let mut stack = vec![];
            if self.process_line(&mut stack, line).is_ok() {
                //println!("line {} is incomplete. Remaining items: {:?}", i, stack);
                let completion_score = self.score_completion(&stack);
                println!("completion score for {} is {}", i, completion_score);
                completion_scores.push(completion_score);
            }
        }
        completion_scores.sort_unstable();
        println!(
            "middle score is {}",
            completion_scores[completion_scores.len() / 2]
        );
        Ok(())
    }
}

impl Day for Day10 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day10-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day10")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)?;
        Ok(())
    }
}
