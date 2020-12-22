use eyre::Result;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const PATTERN: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    number: usize,
    vals: Vec<Vec<bool>>,
    borders: Vec<Vec<bool>>,
}

impl Tile {
    fn from_raw(raw_input: &str) -> Self {
        let mut lines = raw_input.trim().lines();
        let number = lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap();
        let vals: Vec<Vec<bool>> = lines
            .map(|line| line.trim().chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect();
        Self::new(number, vals)
    }

    fn new(number: usize, vals: Vec<Vec<bool>>) -> Self {
        let top: Vec<_> = vals[0].clone();
        let bottom: Vec<_> = vals.last().unwrap().clone();
        let left: Vec<_> = vals.iter().map(|line| line[0]).collect::<Vec<bool>>();
        let right: Vec<_> = vals
            .iter()
            .map(|line| *line.last().unwrap())
            .collect::<Vec<bool>>();
        Tile {
            number,
            vals,
            borders: vec![top, right, bottom, left],
        }
    }

    fn flip(&self) -> Self {
        let vals = flip_vals(&self.vals);
        Self::new(self.number, vals)
    }

    fn rotate(&self) -> Self {
        let vals = rotate_vals(&self.vals);
        Self::new(self.number, vals)
    }

    fn matched_borders(&self, other: &Tile) -> Option<(usize, usize, bool)> {
        for i in 0..4 {
            for j in 0..4 {
                if self.borders[i] == other.borders[j] {
                    return Some((i, j, false));
                } else if self.borders[i]
                    == other.borders[j].iter().rev().copied().collect::<Vec<_>>()
                {
                    return Some((i, j, true));
                }
            }
        }
        None
    }

    fn matches_with(&self, other: &Tile) -> bool {
        self.matched_borders(other).is_some()
    }

    fn vals_without_border(&self) -> Vec<Vec<bool>> {
        let mut vals = vec![];
        for i in 1..self.vals.len() - 1 {
            let mut row = vec![];
            for j in 1..self.vals[0].len() - 1 {
                row.push(self.vals[i][j]);
            }
            vals.push(row);
        }
        vals
    }
}

fn flip_vals(vals: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut flipped_vals = vec![];
    for row in vals {
        let tmp: Vec<_> = row.iter().cloned().rev().collect();
        flipped_vals.push(tmp);
    }
    flipped_vals
}

fn rotate_vals(vals: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut rotated_vals = vec![];
    for row_i in (0..vals[0].len()).rev() {
        let mut row = vec![];
        for item in vals {
            row.push(item[row_i]);
        }
        rotated_vals.push(row);
    }
    rotated_vals
}

fn compose_image(tile_map: &HashMap<&Tile, HashSet<&Tile>>) -> Vec<Vec<bool>> {
    let structure = compose_board_structure(&tile_map);
    for row in &structure {
        println!();
        for ch in row {
            print!("{} ", ch.number);
        }
    }
    let board = orient_tiles(&structure);

    board.into_iter().fold(vec![], |mut full, row| {
        let n_r = row[0].vals_without_border().len();
        let contents = row
            .into_iter()
            .fold(vec![vec![]; n_r], |mut full_row, tile| {
                let vals = tile.vals_without_border();
                for i in 0..vals.len() {
                    full_row[i].extend(vals[i].clone());
                }
                full_row
            });
        for c in contents {
            full.push(c);
        }
        full
    })
}

fn orient_tiles(structure: &[Vec<&Tile>]) -> Vec<Vec<Tile>> {
    let mut first_tile = structure[0][0].clone();
    if let Some(oriented) = compose_board(&structure, &first_tile) {
        return oriented;
    }

    for _ in 0..2 {
        first_tile = first_tile.flip();
        for _ in 0..4 {
            first_tile = first_tile.rotate();
            if let Some(oriented) = compose_board(&structure, &first_tile) {
                return oriented;
            }
        }
    }
    panic!();
}

fn compose_board(structure: &[Vec<&Tile>], first_tile: &Tile) -> Option<Vec<Vec<Tile>>> {
    let mut board: Vec<Vec<Tile>> = vec![];
    for i in 0..structure.len() {
        let mut row = vec![];
        for j in 0..structure.len() {
            if i == 0 && j == 0 {
                row.push(first_tile.clone());
                continue;
            }
            let mut tile = structure[i][j].clone();
            if i > 0 {
                let mut matched = false;
                for _ in 0..2 {
                    tile = tile.flip();
                    for _ in 0..4 {
                        tile = tile.rotate();
                        let (first_rotation, second_rotation, flip) =
                            board[i - 1][j].matched_borders(&tile).unwrap();
                        if first_rotation == 2 && second_rotation == 0 && !flip {
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        break;
                    }
                }
                if !matched {
                    return None;
                }
            }
            if j > 0 {
                let mut matched = false;
                for _ in 0..2 {
                    tile = tile.flip();
                    for _ in 0..4 {
                        tile = tile.rotate();
                        let (first_rotation, second_rotation, flip) =
                            row[j - 1].matched_borders(&tile).unwrap();
                        if first_rotation == 1 && second_rotation == 3 && !flip {
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        break;
                    }
                }
                if !matched {
                    return None;
                }
            }
            row.push(tile);
        }
        board.push(row);
    }
    Some(board)
}

fn compose_board_structure<'a>(
    tile_map: &HashMap<&'a Tile, HashSet<&'a Tile>>,
) -> Vec<Vec<&'a Tile>> {
    let mut chosen = HashSet::new();
    let mut board = vec![];
    let first_corner = tile_map
        .iter()
        .filter(|(_, m)| m.len() == 2)
        .map(|(t, _)| t)
        .next()
        .unwrap();
    let mut row = vec![];
    let mut current = first_corner;

    // middle pieces on top row
    while let Some(next) = tile_map[current]
        .iter()
        .find(|&tile| !chosen.contains(tile) && tile_map[tile].len() == 3)
    {
        row.push(*current);
        chosen.insert(current);
        current = next;
    }
    row.push(*current);
    chosen.insert(current);

    // final corner of top row
    if let Some(next) = tile_map[current]
        .iter()
        .find(|&tile| !chosen.contains(tile) && tile_map[tile].len() == 2)
    {
        row.push(*next);
    }
    let m = row.len();
    board.push(row);

    // start at the leftmost tile at the bottom of the current board
    while let Some(first_in_row) = tile_map[board[board.len() - 1][0]]
        .iter()
        .find(|&tile| !chosen.contains(tile) && tile_map[tile].len() < 4)
    {
        let mut row = vec![];
        let mut current = first_in_row;
        let mut i = 0;
        while let Some(next) = tile_map[current].iter().find(|&tile| {
            !chosen.contains(tile)
                && i < m - 1
                && tile_map[tile].contains(board[board.len() - 1][i + 1])
        }) {
            row.push(*current);
            chosen.insert(current);
            i += 1;
            current = next;
        }
        row.push(*current);
        chosen.insert(current);

        board.push(row);
    }

    board
}

fn find_pattern(image: &[Vec<bool>], pattern: &[Vec<bool>]) -> usize {
    let mut image = image.to_owned();
    let mut any_matched = false;
    for i in 0..(image.len() - pattern.len()) {
        for j in 0..(image[0].len() - pattern[0].len()) {
            let mut matched = true;

            for k in 0..pattern.len() {
                for l in 0..pattern[0].len() {
                    if !pattern[k][l] {
                        continue;
                    }
                    if !image[i + k][j + l] {
                        matched = false;
                        break;
                    }
                }
                if !matched {
                    break;
                }
            }
            if matched {
                any_matched = true;
                for k in 0..pattern.len() {
                    for l in 0..pattern[0].len() {
                        if !pattern[k][l] {
                            continue;
                        }
                        if image[i + k][j + l] {
                            image[i + k][j + l] = false;
                        }
                    }
                }
            }
        }
    }
    if !any_matched {
        return 0;
    }
    image
        .into_iter()
        .map(|row| row.into_iter().filter(|&c| c).count())
        .sum()
}

fn main() -> Result<()> {
    let input = read_to_string("src/day20/input.txt")?;
    let tiles: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(|raw_str| Tile::from_raw(raw_str))
        .collect();
    //println!("tiles: {:?}", tiles);

    let mut matches = HashMap::new();
    for tile in &tiles {
        for tile2 in tiles.iter().skip_while(|t| *t != tile).skip(1) {
            if tile.matches_with(&tile2) {
                matches
                    .entry(tile)
                    .or_insert_with(HashSet::new)
                    .insert(tile2);
                matches
                    .entry(&tile2)
                    .or_insert_with(HashSet::new)
                    .insert(tile);
            }
        }
    }
    //println!("matches: {:?}", matches.values().collect::<Vec<_>>());
    // part 1
    let corners: usize = matches
        .clone()
        .iter()
        .filter(|(_, m)| {
            //println!("{}:{}", t.number, m.len());
            m.len() == 2
        })
        .map(|(t, _)| t.number)
        .product();
    println!("corners product: {:?}", corners);

    // part 2
    let image = compose_image(&matches);
    for row in &image {
        for &c in row {
            let ch = if c { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }

    let mut pattern: Vec<_> = PATTERN
        .lines()
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    let mut final_answer = 0;
    for _ in 0..2 {
        pattern = flip_vals(&pattern);
        for _ in 0..4 {
            pattern = rotate_vals(&pattern);
            let tmp = find_pattern(&image, &pattern);
            final_answer = cmp::max(final_answer, tmp);
        }
    }
    println!("final answer: {}", final_answer);

    Ok(())
}
