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

    fn slide(&mut self, hz: usize, vrt: usize) {
        self.toboggan.row += vrt;
        self.toboggan.col += hz;
        self.toboggan.col %= self.width;

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
        if line.is_empty() {
            continue;
        }
        //println!("line:{}", line);
        let mut sub_v: Vec<char> = vec![];
        for c in line.chars() {
            sub_v.push(c);
            //println!("char [{}][{}]: {:?}", row, col, c);
        }
        locations.push(sub_v);
        width = line.chars().count();
    }
    Map::new(width, height, locations)
}

fn trees_hit(mut map: Map, horizontal_slide: usize, vertical_slide: usize) -> u32 {
    while map.toboggan.row < map.height - 1 {
        map.slide(horizontal_slide, vertical_slide);
    }
    println!(
        "found {} trees for {}, {}",
        map.trees_found, horizontal_slide, vertical_slide
    );
    map.trees_found
}

fn main() -> Result<()> {
    let map_str = read_to_string("src/day3/input.txt")?;
    let map = gen_map(&map_str);
    let t11 = trees_hit(map, 1, 1);
    let map = gen_map(&map_str);
    let t31 = trees_hit(map, 3, 1);
    let map = gen_map(&map_str);
    let t51 = trees_hit(map, 5, 1);
    let map = gen_map(&map_str);
    let t71 = trees_hit(map, 7, 1);
    let map = gen_map(&map_str);
    let t12 = trees_hit(map, 1, 2);
    println!("final result {}", t11 * t31 * t51 * t71 * t12);

    Ok(())
}
