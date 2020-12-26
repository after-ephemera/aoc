use eyre::Result;
use std::cmp::{max, min};
use std::fs::read_to_string;

const GRID_SIZE: usize = 360;
type TileGrid = [[HexColor; GRID_SIZE]; GRID_SIZE];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HexColor {
    Black,
    White,
    Void,
}

impl HexColor {
    fn flip(&self) -> HexColor {
        match self {
            HexColor::Black => HexColor::White,
            HexColor::White => HexColor::Black,
            HexColor::Void => HexColor::Void,
        }
    }
}

struct HexGrid {
    tiles: TileGrid,
}

impl HexGrid {
    fn new() -> Self {
        let mut tiles = [[HexColor::Void; GRID_SIZE]; GRID_SIZE];
        for (i, row) in tiles.iter_mut().enumerate() {
            if i % 2 == 0 {
                //println!("even");
                for (j, tile) in row.iter_mut().enumerate() {
                    if j % 2 == 0 {
                        *tile = HexColor::White;
                    }
                }
            } else {
                //println!("odd");
                for (j, tile) in row.iter_mut().enumerate() {
                    if j % 2 != 0 {
                        *tile = HexColor::White;
                    }
                }
            }
        }
        HexGrid { tiles }
    }

    fn get_index_in_direction(
        &self,
        from_index: (usize, usize),
        direction: &str,
    ) -> (usize, usize) {
        let (x, y) = from_index;
        //println!("moving {}", direction);
        match direction {
            "e" => (x + 2, y),
            "se" => (x + 1, y + 1),
            "sw" => (x - 1, y + 1),
            "w" => (x - 2, y),
            "nw" => (x - 1, y - 1),
            "ne" => (x + 1, y - 1),
            _o => panic!("got a bad direction: {}", _o),
        }
    }

    fn flip_tile(&mut self, raw_steps: &str) -> (usize, usize) {
        let mut chars = raw_steps.chars();
        let mut current_index = (GRID_SIZE / 2, GRID_SIZE / 2);
        while let Some(ch) = chars.next() {
            current_index = match ch {
                'e' => self.get_index_in_direction(current_index, "e"),
                's' => {
                    let next = chars.next().unwrap();
                    self.get_index_in_direction(current_index, &format!("s{}", next))
                }
                'w' => self.get_index_in_direction(current_index, "w"),
                'n' => {
                    let next = chars.next().unwrap();
                    self.get_index_in_direction(current_index, &format!("n{}", next))
                }
                _o => panic!(" got a bad flip: {}", _o),
            }
        }
        //println!(
        //    "would flip {},{} to {:?}",
        //    current_index.0,
        //    current_index.1,
        //    self.tiles[current_index.0][current_index.1].flip()
        //);
        self.tiles[current_index.0][current_index.1] =
            self.tiles[current_index.0][current_index.1].flip();
        (current_index.0, current_index.1)
    }

    fn count_neighbors(&self, base_index: (usize, usize), color: HexColor) -> usize {
        let (i, j) = base_index;
        if self.tiles[i][j] == HexColor::Void {
            panic!("void count at {},{}", i, j);
        } else {
            let mut neighbors = vec![];
            if i > 0 {
                if j > 0 {
                    neighbors.push(self.tiles[i - 1][j - 1]);
                }
                if j < self.tiles[0].len() - 1 {
                    neighbors.push(self.tiles[i - 1][j + 1]);
                }
            }
            if i > 1 {
                neighbors.push(self.tiles[i - 2][j]);
            }
            if i < self.tiles.len() - 1 {
                if j > 0 {
                    neighbors.push(self.tiles[i + 1][j - 1]);
                }
                if j < self.tiles[0].len() - 1 {
                    neighbors.push(self.tiles[i + 1][j + 1]);
                }
            }
            if i < self.tiles.len() - 2 {
                neighbors.push(self.tiles[i + 2][j]);
            }
            neighbors.iter().filter(|t| **t == color).count()
        }
    }

    fn pass_day(&mut self) {
        let mut new_tiles = self.tiles;
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row
                .iter()
                .enumerate()
                .filter(|(_, t)| **t != HexColor::Void)
            {
                let neighbor_count = self.count_neighbors((i, j), HexColor::Black);
                match tile {
                    HexColor::Black => {
                        if neighbor_count == 0 || neighbor_count > 2 {
                            new_tiles[i][j] = self.tiles[i][j].flip();
                        }
                    }
                    HexColor::White => {
                        if neighbor_count == 2 {
                            new_tiles[i][j] = self.tiles[i][j].flip();
                        }
                    }
                    HexColor::Void => (),
                }
            }
        }
        self.tiles = new_tiles;
    }

    fn count(&self, color: HexColor) -> usize {
        self.tiles.iter().flatten().filter(|x| **x == color).count()
    }
}

fn main() -> Result<()> {
    let raw_input = read_to_string("src/day24/input.txt")?;
    let mut grid = HexGrid::new();
    //println!("grid: {:?}", grid.tiles);
    for (_i, line) in raw_input.lines().enumerate() {
        //println!("trying line {}, {}", _i, line);
        grid.flip_tile(line);
    }
    let final_count = grid.count(HexColor::Black);
    println!("final black count: {}", final_count);
    assert_eq!(266, final_count);

    //part 2
    let raw_input = read_to_string("src/day24/input.txt")?;
    let mut grid = HexGrid::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = GRID_SIZE;
    let mut min_y = GRID_SIZE;
    for line in raw_input.lines() {
        let (flipped_x, flipped_y) = grid.flip_tile(line);
        max_x = max(max_x, flipped_x);
        max_y = max(max_y, flipped_y);
        min_x = min(min_x, flipped_x);
        min_y = min(min_y, flipped_y);
    }
    println!(
        "max x: {}, max y: {}, min x: {}, min y: {}",
        max_x, max_y, min_x, min_y
    );
    for _day in 1..101 {
        grid.pass_day();
        if _day % 10 == 0 {
            let final_count = grid.count(HexColor::Black);
            println!("day {} black count: {}", _day, final_count);
        }
    }
    let final_count = grid.count(HexColor::Black);
    println!("final black count: {}", final_count);

    Ok(())
}
