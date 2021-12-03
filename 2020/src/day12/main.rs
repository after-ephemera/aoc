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
        let mut cycle = i.cycle();

        let r = cycle.nth((index as i32 + 4 + other) as usize).unwrap();
        *r
    }
}

impl CardinalDirection {
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
        CardinalDelta(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for CardinalDelta {
    fn add_assign(&mut self, other: CardinalDelta) {
        *self = *self + other;
    }
}

impl CardinalDelta {
    fn rotate(&mut self, direction: char, count: u32) {
        for _ in 0..count {
            let tmp = self.0;
            self.0 = self.1;
            self.1 = tmp;
            match direction {
                'R' => self.0 = -self.0,
                'L' => self.1 = -self.1,
                _ => println!("cannot compute"),
            }
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
            ShipType::Navigational => match dir {
                'L' => self.facing = self.facing + -(ticks as i32),
                'R' => self.facing = self.facing + ticks as i32,
                _ => println!("failure!"),
            },
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
        //println!("{} for {}", *dir, count);
        //println!("before: {:?}", ship);
        ship.move_by(*dir, count);
        //println!("after: {:?}", ship);
    }
    println!("final ship: {:?}", ship);
    println!("final manhattan delta: {}", ship.manhattan_delta());
    Ok(())
}
