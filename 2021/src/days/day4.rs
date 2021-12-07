use super::Day;
use eyre::Result;
use std::fmt;
use std::fs::read_to_string;

pub struct Day4 {}

#[derive(Debug, Clone)]
struct NumberEntry {
    val: u32,
    marked: bool,
}

impl NumberEntry {
    fn new(num: u32) -> Self {
        NumberEntry {
            val: num,
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    width: usize,
    height: usize,
    numbers: Vec<NumberEntry>,
    won: bool,
}

impl Board {
    fn new(raw_board: &str) -> Self {
        let mut entries = vec![];
        let rows = raw_board.split('\n').collect::<Vec<_>>();
        let height = rows.len();
        let width = rows[0].split_whitespace().count();
        for row in rows {
            for num in row.trim().split_whitespace() {
                entries.push(NumberEntry::new(num.parse::<u32>().unwrap()));
            }
        }
        Board {
            numbers: entries,
            height,
            width,
            won: false,
        }
    }

    fn update(&mut self, num: u32) {
        for entry in &mut self.numbers {
            if entry.val == num {
                entry.marked = true;
            }
        }
    }

    fn is_winner(&self) -> bool {
        // rows
        for row in self.numbers.chunks(self.height) {
            if row.iter().all(|e| e.marked) {
                return true;
            }
        }
        // columns
        for x in 0..self.width {
            let mut col_marked = true;
            for y in 0..self.height {
                if !self.numbers[y * self.width + x].marked {
                    col_marked = false;
                }
            }
            if col_marked {
                return true;
            }
        }
        false
    }

    fn final_score(&self, last_num: u32) -> u32 {
        // We assume the board already is a winner.
        let sum: u32 = self
            .numbers
            .iter()
            .filter(|e| !e.marked)
            .map(|e| e.val)
            .sum();
        println!(
            "calculating final score with {} * {} = {}",
            sum,
            last_num,
            sum * last_num
        );
        sum * last_num
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.width {
            for x in 0..self.height {
                let entry = &self.numbers[y * self.height + x];
                write!(f, "{}{} ", entry.val, if entry.marked { "*" } else { "" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Day4 {
    fn part_1(&self, raw_input: &str) -> Result<()> {
        let mut entries = raw_input.trim().split("\n\n");
        let drawings = entries.next().unwrap().split(',').collect::<Vec<_>>();
        //println!("drawings: {:?}", drawings);
        let raw_boards = entries.collect::<Vec<_>>();
        let mut boards = vec![];
        for raw_board in raw_boards {
            let new_board = Board::new(&raw_board);
            boards.push(new_board);
        }
        // println!("boards: {:#?}", boards);

        let mut winning_board = None;
        let mut last_drawn_number = None;
        for drawn_number in drawings
            .iter()
            .map(|num_str| num_str.parse::<u32>().unwrap())
        {
            //println!("drawing: {}", drawn_number);
            for board in &mut boards {
                board.update(drawn_number);
                if board.is_winner() {
                    //println!(
                    //"winning board: {:?}, last_drawn_number: {:?}",
                    //board, drawn_number
                    //);
                    winning_board = Some(board.clone());
                    last_drawn_number = Some(drawn_number);
                    break;
                }
            }
            if winning_board.is_some() {
                break;
            }
        }
        if let Some(winner) = winning_board {
            println!(
                "final score: {}",
                winner.final_score(last_drawn_number.unwrap())
            );
        } else {
            panic!("no winner");
        }
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
        let mut entries = raw_input.trim().split("\n\n");
        let drawings = entries.next().unwrap().split(',').collect::<Vec<_>>();
        println!("drawings: {:?}", drawings);
        let raw_boards = entries.collect::<Vec<_>>();
        let mut boards = vec![];
        for raw_board in raw_boards {
            let new_board = Board::new(&raw_board);
            boards.push(new_board);
        }
        // println!("boards: {:#?}", boards);

        let mut last_winning_board = None;
        let mut last_drawn_number = None;
        let mut winning_boards = 0;
        let mut last_winner_ready = false;
        for drawn_number in drawings
            .iter()
            .map(|num_str| num_str.parse::<u32>().unwrap())
        {
            println!("drawing: {}", drawn_number);

            let board_count = boards.len();
            for board in &mut boards {
                if board.won {
                    continue;
                }
                board.update(drawn_number);
                if board.is_winner() {
                    if winning_boards == board_count - 1 {
                        last_winner_ready = true;
                        last_winning_board = Some(board.clone());
                        last_drawn_number = Some(drawn_number);
                        println!("last winning board: \n{}", board);
                        break;
                    }
                    println!("board won: \n{}", board);
                    winning_boards += 1;
                    board.won = true;
                }
            }
            if last_winner_ready {
                break;
            }
        }
        if let Some(winner) = last_winning_board {
            println!(
                "final score: {}",
                winner.final_score(last_drawn_number.unwrap())
            );
        } else {
            panic!("no winner");
        }
        Ok(())
    }
}

impl Day for Day4 {
    fn run(&self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day4-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day4")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        self.part_2(&sample_raw_input)?;
        self.part_2(&raw_input)
    }
}
