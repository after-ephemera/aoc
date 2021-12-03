use eyre::Result;
use std::fmt;
use std::fs::read_to_string;

enum MapType {
    Visibility,
    Adjacent,
}

struct Map {
    inner: Vec<Vec<char>>,
    map_type: MapType,
}

impl Map {
    fn new(arr: Vec<Vec<char>>, map_type: MapType) -> Map {
        Map {
            inner: arr,
            map_type,
        }
    }

    fn adjacent(&self, x: isize, y: isize) -> Vec<char> {
        //println!("getting adjacent for {} {}", x, y);
        [
            self.inner[x as usize].get((y - 1) as usize),
            self.inner[x as usize].get((y + 1) as usize),
            self.inner
                .get((x + 1) as usize)
                .and_then(|i| i.get((y - 1) as usize)),
            self.inner.get((x + 1) as usize).map(|i| &i[y as usize]),
            self.inner
                .get((x + 1) as usize)
                .and_then(|i| i.get((y + 1) as usize)),
            self.inner
                .get((x - 1) as usize)
                .and_then(|i| i.get((y + 1) as usize)),
            self.inner.get((x - 1) as usize).map(|i| &i[y as usize]),
            self.inner
                .get((x - 1) as usize)
                .and_then(|i| i.get((y - 1) as usize)),
        ]
        .iter()
        .flatten()
        .map(|x| **x)
        .collect()
    }

    fn visible_in_direction(
        &self,
        x: isize,
        y: isize,
        delta_x: isize,
        delta_y: isize,
    ) -> Option<char> {
        let dx = x + delta_x;
        let dy = y + delta_y;

        if dx < 0 || dy < 0 || dx >= self.inner.len() as isize || dy >= self.inner[0].len() as isize
        {
            None
        } else if self.inner[dx as usize][dy as usize] == '#'
            || self.inner[dx as usize][dy as usize] == 'L'
        {
            //println!("{}, {} was occupied", dx, dy);
            Some(self.inner[dx as usize][dy as usize])
        } else {
            self.visible_in_direction(x + delta_x, y + delta_y, delta_x, delta_y)
        }
    }

    fn visible(&self, x: isize, y: isize) -> Vec<char> {
        [
            self.visible_in_direction(x, y, -1, -1),
            self.visible_in_direction(x, y, -1, 0),
            self.visible_in_direction(x, y, -1, 1),
            self.visible_in_direction(x, y, 0, 1),
            self.visible_in_direction(x, y, 1, 1),
            self.visible_in_direction(x, y, 1, 0),
            self.visible_in_direction(x, y, 1, -1),
            self.visible_in_direction(x, y, 0, -1),
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }

    fn adjacent_empty(&self, x: isize, y: isize) -> bool {
        //println!("adjacent to {}, {} is {:?}", x, y, self.adjacent(x, y));
        let empty = match self.map_type {
            MapType::Adjacent => self.adjacent(x, y),
            MapType::Visibility => self.visible(x, y),
        };
        empty.iter().all(|x| *x == 'L' || *x == '.')
    }

    fn some_adjacent_occupied(&self, x: isize, y: isize) -> bool {
        let limit = match self.map_type {
            MapType::Adjacent => 4,
            MapType::Visibility => 5,
        };

        let occupied = match self.map_type {
            MapType::Adjacent => self.adjacent(x, y),
            MapType::Visibility => self.visible(x, y),
        };
        let occupied_count = occupied.iter().filter(|x| **x == '#').count();
        //println!(
        //    "{} occupied near {}, {}: {:?}",
        //    occupied_count, x, y, occupied
        //);
        occupied_count >= limit
    }

    fn update_state(&mut self, result: &mut Vec<Vec<char>>, x: usize, y: usize) {
        let seat = self.inner[x][y];
        if seat == 'L' && self.adjacent_empty(x as isize, y as isize) {
            //println!("{}, {} is now occupied", x, y);
            result[x][y] = '#';
        } else if seat == '#' && self.some_adjacent_occupied(x as isize, y as isize) {
            //println!("{}, {} is now empty", x, y);
            result[x][y] = 'L';
        }
    }

    fn next(&mut self) {
        let mut updated_map = self.inner.clone();
        for (x, v) in self.inner.clone().iter().enumerate() {
            for (y, _v2) in v.iter().enumerate() {
                self.update_state(&mut updated_map, x, y)
            }
        }
        self.inner = updated_map;
    }

    fn run_till_stable(&mut self) -> u32 {
        let mut count = 0;

        loop {
            if count % 10 == 0 {
                println!("iteration {}", count);
            }
            let prev = self.inner.clone();

            self.next();

            if prev == self.inner {
                break;
            }
            count += 1;
        }
        count
    }

    fn count_occupied(&self) -> usize {
        self.inner.iter().flatten().filter(|x| **x == '#').count()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for line in &self.inner {
            writeln!(f, "{:?}", line)?;
        }
        write!(f, "")
    }
}

fn build_map(lines: &[&str], map_type: MapType) -> Map {
    let mut result = vec![];
    for line in lines {
        let mut line_result = vec![];
        for ch in line.chars() {
            line_result.push(ch);
        }
        result.push(line_result);
    }
    Map::new(result, map_type)
}

fn main() -> Result<()> {
    let input = read_to_string("src/day11/input.txt")?;
    //let input = read_to_string("src/day11/input.txt")?;
    let mut map = build_map(
        &input.split_whitespace().collect::<Vec<&str>>(),
        MapType::Adjacent,
    );
    //println!("before: {}", map);
    let iteration_count = map.run_till_stable();
    println!("took {} iterations", iteration_count);
    //println!("after: {}", map);
    println!("there are now {} occupied seats", map.count_occupied());

    println!("part 2!");
    let mut map = build_map(
        &input.split_whitespace().collect::<Vec<&str>>(),
        MapType::Visibility,
    );
    let iteration_count = map.run_till_stable();
    //map.next();
    //println!("after: {}", map);
    //map.next();
    //println!("after: {}", map);
    //map.next();
    println!("after: {}", map);
    println!("took {} iterations", iteration_count);
    println!("there are now {} occupied seats", map.count_occupied());

    Ok(())
}
