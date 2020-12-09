use eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::vec::Vec;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"^(.*) ([+-].*)$").unwrap();
}

#[derive(Debug)]
enum RunStatus {
    Running,
    Loop,
    EndOfProgram,
}

#[derive(Debug)]
struct Program {
    instruction_pointer: i32,
    code: Vec<(String, i32)>,
    visited: HashSet<i32>,
    accumulator: i32,
}

impl Program {
    fn from_file(filename: &str) -> Result<Program> {
        let input = read_to_string(filename)?;

        let program_code: Vec<(String, i32)> = input
            .split('\n')
            .filter(|line| *line != "")
            .map(|line| {
                let cap = INSTRUCTION_RE.captures_iter(line).next().unwrap();
                let op = cap[1].to_string();
                let count = cap[2].parse::<i32>().unwrap();
                //println!("op: {}, count: {}", op, count);
                return (op, count);
            })
            .collect();
        Ok(Program::new(program_code))
    }

    fn new(code: Vec<(String, i32)>) -> Program {
        Program {
            instruction_pointer: 0,
            code,
            visited: HashSet::new(),
            accumulator: 0,
        }
    }

    fn run_op(&mut self, op: &str, count: &i32) -> RunStatus {
        if self.visited.contains(&self.instruction_pointer) {
            return RunStatus::Loop;
        } else if self.instruction_pointer >= self.code.len() as i32 {
            return RunStatus::EndOfProgram;
        }

        self.visited.insert(self.instruction_pointer);
        match op {
            "acc" => {
                self.accumulator += count;
                self.instruction_pointer += 1;
            }
            "jmp" => {
                self.instruction_pointer += count;
            }
            _ => self.instruction_pointer += 1,
        }
        RunStatus::Running
    }

    fn run(&mut self) -> RunStatus {
        let result: RunStatus;
        // run program operations until the end of the program is reached.
        loop {
            if self.instruction_pointer >= self.code.len() as i32 {
                return RunStatus::EndOfProgram;
            }
            let (op_ref, count_ref) = &self.code[self.instruction_pointer as usize];
            let op = op_ref.clone();
            let count = *count_ref;
            match self.run_op(&op, &count) {
                RunStatus::Running => {
                    continue;
                }
                RunStatus::Loop => {
                    result = RunStatus::Loop;
                    break;
                }
                RunStatus::EndOfProgram => {
                    result = RunStatus::EndOfProgram;
                    break;
                }
            }
        }
        result
    }

    fn flip_instruction(&self, index: usize) -> Vec<(String, i32)> {
        let mut result = self.code.clone();
        let flipped = match result[index].0.as_str() {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => "fail",
        };
        result[index] = (flipped.to_string(), result[index].1);
        result
    }

    fn get_possible_corrupt_variations(&self) -> Vec<Vec<(String, i32)>> {
        let mut result = vec![];

        for (i, (op, _)) in self.code.iter().enumerate() {
            if op == "jmp" || op == "nop" {
                result.push(self.flip_instruction(i));
            }
        }

        result
    }

    fn fix_and_run(&self) -> Option<Program> {
        for possible_variation in self.get_possible_corrupt_variations() {
            let mut possibly_fixed_program = Program::new(possible_variation);
            match possibly_fixed_program.run() {
                RunStatus::EndOfProgram => {
                    println!("found the end!");
                    return Some(possibly_fixed_program);
                }
                _ => (),
            }
        }
        None
    }
}

fn main() -> Result<()> {
    //let mut program = Program::from_file("src/day8/input-sample.txt")?;
    let mut program = Program::from_file("src/day8/input.txt")?;
    let result = program.run();
    //println!("final program result: {:?}", program);
    println!("final program accumulator value: {}", program.accumulator);
    println!("final result status: {:?}", result);

    println!("part 2");
    let program = Program::from_file("src/day8/input.txt")?;
    let result = program.fix_and_run().unwrap();
    println!("final program accumulator value: {}", result.accumulator);

    Ok(())
}
