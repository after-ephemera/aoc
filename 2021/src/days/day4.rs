use super::Day;
use eyre::Result;
use std::fs::read_to_string;

pub struct Day4 {}

#[derive(Debug)]
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

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    numbers: Vec<NumberEntry>,
}

impl Board {
    fn new(raw_board: &str) -> Self {
        let mut entries = vec![];
        let rows = raw_board.split('\n').collect::<Vec<_>>();
        let height = rows.len();
        let width = rows[0].len();
        for row in rows {
            for num in row.trim().split_whitespace() {
                entries.push(NumberEntry::new(num.parse::<u32>().unwrap()));
            }
        }
        Board {
            numbers: entries,
            height,
            width,
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
        for x in 0..self.height {
            let mut col_marked = true;
            for y in 0..self.width {
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
}

impl Day4 {
    fn part_1(&self, raw_input: &str) -> Result<()> {
        let mut entries = raw_input.trim().split("\n\n");
        let drawings = entries.next().unwrap().split(',').collect::<Vec<_>>();
        println!("drawings: {:?}", drawings);
        let raw_boards = entries.collect::<Vec<_>>();
        let mut boards = vec![];
        for raw_board in raw_boards {
            let mut new_board = Board::new(&raw_board);
            boards.push(new_board);
        }
        // println!("boards: {:#?}", boards);

        for drawn_number in drawings
            .iter()
            .map(|num_str| num_str.parse::<u32>().unwrap())
        {
            //println!("drawing: {}", drawn_number);
            for board in &mut boards {
                board.update(drawn_number);
                if board.is_winner() {
                    break;
                }
            }
        }
        Ok(())
    }

    fn part_2(&self, raw_input: &str) -> Result<()> {
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
