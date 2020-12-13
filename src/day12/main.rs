use eyre::Result;
use std::fs::read_to_string;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Copy, Clone, Debug, PartialEq)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl Add<i32> for CardinalDirection {
    type Output = CardinalDirection;

    fn add(self, other: i32) -> CardinalDirection {
        let i = [
            CardinalDirection::North,
            CardinalDirection::East,
            CardinalDirection::South,
            CardinalDirection::West,
        ]
        .iter();
        let index: usize = i.clone().position(|x| *x == self).unwrap();
        //println!("index is {} for {:?}", index, self);
        let cycle = i.cycle();

        //println!(
        //    "trying to take {} from {} with other {}",
        //    (index as i32 + 4 + other),
        //    (index as i32 + 4 + other) as usize,
        //    other
        //);
        let r = cycle
            .skip((index as i32 + 4 + other) as usize)
            .next()
            .unwrap();
        //println!("direction was {:?} + {} = {:?}", self, other, r);
        *r
    }
}

impl CardinalDirection {
    //fn from(value: char) -> Self {
    //match value {
    //'N' => Self::North,
    //'E' => Self::East,
    //'S' => Self::South,
    //'W' => Self::West,
    //_ => Self::North,
    //}
    //}

    fn char(&self) -> char {
        match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W',
        }
    }
}

/// n/s,e/w
#[derive(Copy, Clone, Debug, PartialEq)]
struct CardinalDelta(i32, i32);

impl Add<CardinalDelta> for CardinalDelta {
    type Output = CardinalDelta;

    fn add(self, other: CardinalDelta) -> CardinalDelta {
        //println!("adding {:?} to {:?}", self, other);
        CardinalDelta(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for CardinalDelta {
    fn add_assign(&mut self, other: CardinalDelta) {
        *self = *self + other;
    }
}

impl CardinalDelta {
    fn quadrant(&self) -> u8 {
        return if self.0 >= 0 && self.1 <= 0 {
            0
        } else if self.0 >= 0 && self.1 > 0 {
            1
        } else if self.0 < 0 && self.1 > 0 {
            2
        } else if self.0 < 0 && self.1 <= 0 {
            3
        } else {
            println!("quadrant not in known spacetime continuum");
            42
        };
    }

    fn rotate(&mut self, direction: char, count: u32) {
        for _ in 0..count {
            println!("rotating from quadrant {}", self.quadrant());
            let tmp = self.0;
            self.0 = self.1;
            self.1 = tmp;
            match direction {
                'R' => self.0 = -self.0,
                'L' => self.1 = -self.1,
                _ => println!("cannot compute"),
            }
            //match (direction, self.quadrant()) {
            //('L', 0) | ('R', 1) | ('L', 2) | ('R', 3) => self.0 = -self.0,
            //('R', 0) | ('L', 1) | ('R', 2) | ('L', 3) => self.1 = -self.1,
            //_ => (),
            //}
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ShipType {
    Navigational,
    Waypoint,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Ship {
    facing: CardinalDirection,
    delta: CardinalDelta,
    waypoint_delta: CardinalDelta,
    ship_type: ShipType,
}

impl Ship {
    fn new(ship_type: ShipType) -> Ship {
        Ship {
            facing: CardinalDirection::East,
            delta: CardinalDelta(0, 0),
            waypoint_delta: CardinalDelta(1, 10),
            ship_type,
        }
    }

    fn rotate(&mut self, dir: char, ticks: u32) {
        match self.ship_type {
            ShipType::Navigational => {
                //println!("turning {}", dir);
                match dir {
                    'L' => self.facing = self.facing + (-1 * (ticks as i32)),
                    'R' => self.facing = self.facing + 1 * ticks as i32,
                    _ => println!("failure!"),
                }
            }
            ShipType::Waypoint => self.waypoint_delta.rotate(dir, ticks),
        }
    }

    fn move_in_dir(&mut self, direction: char, count: i32) {
        match direction {
            'N' => self.delta += CardinalDelta(count, 0),
            'E' => self.delta += CardinalDelta(0, count),
            'S' => self.delta += CardinalDelta(-count, 0),
            'W' => self.delta += CardinalDelta(0, -count),
            _ => (),
        }
    }

    fn move_waypoint_in_dir(&mut self, direction: char, count: i32) {
        match direction {
            'N' => self.waypoint_delta += CardinalDelta(count, 0),
            'E' => self.waypoint_delta += CardinalDelta(0, count),
            'S' => self.waypoint_delta += CardinalDelta(-count, 0),
            'W' => self.waypoint_delta += CardinalDelta(0, -count),
            _ => (),
        }
    }

    fn move_by(&mut self, direction: char, count: u32) {
        match self.ship_type {
            ShipType::Navigational => match direction {
                'N' | 'E' | 'S' | 'W' => self.move_in_dir(direction, count as i32),
                'L' | 'R' => self.rotate(direction, count / 90),
                'F' => self.move_in_dir(self.facing.char(), count as i32),
                _ => println!("failure! bad command"),
            },
            ShipType::Waypoint => match direction {
                'N' | 'E' | 'S' | 'W' => self.move_waypoint_in_dir(direction, count as i32),
                'L' | 'R' => self.rotate(direction, count / 90),
                'F' => {
                    // direction is built into the sign of the waypoint delta, so we can force N and E
                    self.move_in_dir('N', self.waypoint_delta.0 * count as i32);
                    self.move_in_dir('E', self.waypoint_delta.1 * count as i32);
                }
                _ => println!("failure! bad command"),
            },
        }
    }

    fn manhattan_delta(&self) -> i32 {
        self.delta.0.abs() + self.delta.1.abs()
    }
}

fn main() -> Result<()> {
    let input = read_to_string("src/day12/input.txt")?;
    // part 1
    let mut ship = Ship::new(ShipType::Navigational);
    for command in input.split('\n').filter(|c| !c.is_empty()) {
        let chars: Vec<char> = command.chars().collect();
        let dir = &chars[0];
        let count: u32 = chars[1..].iter().collect::<String>().parse()?;
        //println!("{} for {}", *dir, count);
        //println!("before: {:?}", ship);
        ship.move_by(*dir, count);
        //println!("after: {:?}", ship);
    }
    println!("final ship: {:?}", ship);
    println!("final manhattan delta: {}", ship.manhattan_delta());

    // part 2
    let mut ship = Ship::new(ShipType::Waypoint);
    for command in input.split('\n').filter(|c| !c.is_empty()) {
        let chars: Vec<char> = command.chars().collect();
        let dir = &chars[0];
        let count: u32 = chars[1..].iter().collect::<String>().parse()?;
        println!("{} for {}", *dir, count);
        println!("before: {:?}", ship);
        ship.move_by(*dir, count);
        println!("after: {:?}", ship);
    }
    println!("final ship: {:?}", ship);
    println!("final manhattan delta: {}", ship.manhattan_delta());
    Ok(())
}
