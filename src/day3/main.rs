use eyre::Result;
use std::fs::read_to_string;
use std::vec::Vec;

struct Map {
    width: usize,
    height: usize,
    locations: Vec<Vec<char>>,
    toboggan: Toboggan,
    trees_found: u32,
}

struct Toboggan {
    row: usize,
    col: usize,
}

impl Map {
    fn new(width: usize, height: usize, locations: Vec<Vec<char>>) -> Map {
        println!("new map with width:{}, height:{}", width, height);
        Map {
            width,
            height,
            locations,
            toboggan: Toboggan { row: 0, col: 0 },
            trees_found: 0,
        }
    }

    fn slide(&mut self) {
        self.toboggan.row += 1;
        self.toboggan.col += 3;
        self.toboggan.col = self.toboggan.col % self.width;

        if self.toboggan.row > self.height - 1 {
            return;
        }
        if self.locations[self.toboggan.row][self.toboggan.col] == '#' {
            //println!("found tree! {},{}", self.toboggan.row, self.toboggan.col);
            self.trees_found += 1;
        }
    }
}

fn gen_map(map_str: &str) -> Map {
    let mut locations: Vec<Vec<char>> = vec![];
    let lines: Vec<&str> = map_str.split('\n').collect();
    let height = lines.len() - 1;
    let mut width = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        println!("line:{}", line);
        let mut sub_v: Vec<char> = vec![];
        for c in line.chars() {
            sub_v.push(c);
            //println!("char [{}][{}]: {:?}", row, col, c);
        }
        locations.push(sub_v);
        width = line.chars().count();
    }
    //println!("{:?} x {:?}", locations.len(), locations[0].len());
    //println!("{:?}", locations[0]);
    //println!("{:?}", locations[322]);
    Map::new(width, height, locations)
}

fn main() -> Result<()> {
    let map_str = read_to_string("src/day3/input.txt")?;
    let mut map = gen_map(&map_str);

    while map.toboggan.row < map.height - 1 {
        map.slide();
    }
    println!("found {} trees", map.trees_found);

    Ok(())
}
