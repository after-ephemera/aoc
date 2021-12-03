use eyre::Result;
use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node4d(isize, isize, isize, isize);

impl Node4d {
    fn get_neighbors(&self) -> Vec<Node4d> {
        let mut result = vec![];
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        if (x, y, z, w) == (0, 0, 0, 0) {
                            // skip the current node
                            continue;
                        }
                        result.push(Node4d(self.0 + x, self.1 + y, self.2 + z, self.3 + w));
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Map4d {
    active_nodes: HashSet<Node4d>,
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
    z_bounds: (isize, isize),
    w_bounds: (isize, isize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node(isize, isize, isize);

impl Node {
    fn get_neighbors(&self) -> Vec<Node> {
        let mut result = vec![];
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if (x, y, z) == (0, 0, 0) {
                        // skip the current node
                        continue;
                    }
                    result.push(Node(self.0 + x, self.1 + y, self.2 + z));
                }
            }
        }
        result
    }
}

/// A struct holding the entire 3d space with active nodes.
///
/// x increases to the left, y increases downward, and z increases forward and backward.
/// This is also how the structure will be displayed.
/// ---> x
/// |  z
/// | /
/// v/
/// y
#[derive(Debug)]
struct Map {
    active_nodes: HashSet<Node>,
    x_bounds: (isize, isize),
    y_bounds: (isize, isize),
    z_bounds: (isize, isize),
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // show all ranges
        writeln!(f)?;
        writeln!(
            f,
            "x{{{:?}}} y{{{:?}}} z{{{:?}}}",
            self.x_bounds, self.y_bounds, self.z_bounds
        )?;
        // show x,y grid for each z index
        for z in self.z_bounds.0..self.z_bounds.1 + 1 {
            writeln!(f, " Z == {} ", z)?;
            writeln!(f)?;
            for y in self.y_bounds.0..self.y_bounds.1 + 1 {
                for x in self.x_bounds.0..self.x_bounds.1 + 1 {
                    if self.active_nodes.contains(&Node(x, y, z)) {
                        write!(f, " # ")?;
                    } else {
                        write!(f, " . ")?;
                    }
                    //write!(f, "{},{},{}", x, y, z);
                }
                writeln!(f)?;
            }
        }
        write!(f, "---")
    }
}

impl Map4d {
    fn from_str(input: &str) -> Self {
        let mut active_nodes = HashSet::new();
        let rows: Vec<(usize, &str)> = input.split('\n').enumerate().collect();
        let row_len = rows[0].1.chars().count();
        let mut x_bounds = (-(rows.len() as isize), rows.len() as isize);
        let mut y_bounds = (-(row_len as isize), row_len as isize);
        let z_bounds = (-1, 1);
        let w_bounds = (-1, 1);
        let z = 0isize;
        let w = 0isize;
        for (y, row) in rows {
            // make sure the bounds are always right.
            y_bounds = (
                cmp::min(y_bounds.0, y as isize),
                cmp::max(y_bounds.1, y as isize),
            );
            for (x, ch) in row.chars().enumerate() {
                //println!("{},{}: {}", i, j, ch);
                match ch {
                    '#' => {
                        // make sure the bounds are always right.
                        x_bounds = (
                            cmp::min(x_bounds.0, x as isize),
                            cmp::max(x_bounds.1, x as isize),
                        );
                        active_nodes.insert(Node4d(x as isize, y as isize, z as isize, w as isize));
                    }
                    // empty node, no op.
                    '.' => (),
                    // something else, error
                    _e => eprintln!("bad character input: {}", _e),
                }
            }
        }
        Map4d {
            active_nodes,
            x_bounds,
            y_bounds,
            z_bounds,
            w_bounds,
        }
    }

    fn insert_active_node(&mut self, active_nodes: &mut HashSet<Node4d>, new_active_node: Node4d) {
        // update all bounds first
        self.x_bounds = (
            cmp::min(self.x_bounds.0, new_active_node.0),
            cmp::max(self.x_bounds.1, new_active_node.0),
        );
        self.y_bounds = (
            cmp::min(self.y_bounds.0, new_active_node.1),
            cmp::max(self.y_bounds.1, new_active_node.1),
        );
        self.z_bounds = (
            cmp::min(self.z_bounds.0, new_active_node.2),
            cmp::max(self.z_bounds.1, new_active_node.2),
        );
        self.w_bounds = (
            cmp::min(self.w_bounds.0, new_active_node.3),
            cmp::max(self.w_bounds.1, new_active_node.3),
        );
        active_nodes.insert(new_active_node);
    }

    fn len_active(&self) -> usize {
        self.active_nodes.len()
    }

    /// run six consecutive cycles
    fn boot(&mut self, cycle_count: usize) {
        for i in 0..cycle_count {
            println!("cycle {}", i + 1);
            self.cycle();
            //println!("{:?}", self);
        }
    }

    /// Execute one iteration, working through all nodes adjacent to existing nodes.
    fn cycle(&mut self) {
        // avoid visiting the same node twice by keeping a list of visited nodes.
        let mut visited: HashSet<Node4d> = HashSet::new();
        //println!("active nodes going into cycle: {:?}", self.active_nodes);
        let mut new_active_nodes = self.active_nodes.clone();
        for active_node in &self.active_nodes.clone() {
            // check all adjacent nodes to this one, plus this one
            for edge_node in active_node.get_neighbors().iter().chain(vec![active_node]) {
                //println!("checking edge node {:?}", edge_node);
                if visited.contains(&edge_node) {
                    continue;
                }
                visited.insert(edge_node.clone());

                let mut active_neighbors = 0;
                for neighbor in edge_node.get_neighbors() {
                    if self.active_nodes.contains(&neighbor) {
                        active_neighbors += 1;
                    }
                }
                //println!("{} active_neighbors", active_neighbors);
                if self.active_nodes.contains(&edge_node) && !(2..4).contains(&active_neighbors) {
                    new_active_nodes.remove(&edge_node);
                } else if !self.active_nodes.contains(&edge_node) && active_neighbors == 3 {
                    self.insert_active_node(&mut new_active_nodes, edge_node.clone());
                }
            }
        }
        self.active_nodes = new_active_nodes;
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut active_nodes = HashSet::new();
        let rows: Vec<(usize, &str)> = input.split('\n').enumerate().collect();
        let row_len = rows[0].1.chars().count();
        let mut x_bounds = (-(rows.len() as isize), rows.len() as isize);
        let mut y_bounds = (-(row_len as isize), row_len as isize);
        let z_bounds = (-1, 1);
        let z = 0isize;
        for (y, row) in rows {
            // make sure the bounds are always right.
            y_bounds = (
                cmp::min(y_bounds.0, y as isize),
                cmp::max(y_bounds.1, y as isize),
            );
            for (x, ch) in row.chars().enumerate() {
                //println!("{},{}: {}", i, j, ch);
                match ch {
                    '#' => {
                        // make sure the bounds are always right.
                        x_bounds = (
                            cmp::min(x_bounds.0, x as isize),
                            cmp::max(x_bounds.1, x as isize),
                        );
                        active_nodes.insert(Node(x as isize, y as isize, z as isize));
                    }
                    // empty node, no op.
                    '.' => (),
                    // something else, error
                    _e => eprintln!("bad character input: {}", _e),
                }
            }
        }
        Map {
            active_nodes,
            x_bounds,
            y_bounds,
            z_bounds,
        }
    }

    fn insert_active_node(&mut self, active_nodes: &mut HashSet<Node>, new_active_node: Node) {
        // update all bounds first
        self.x_bounds = (
            cmp::min(self.x_bounds.0, new_active_node.0),
            cmp::max(self.x_bounds.1, new_active_node.0),
        );
        self.y_bounds = (
            cmp::min(self.y_bounds.0, new_active_node.1),
            cmp::max(self.y_bounds.1, new_active_node.1),
        );
        self.z_bounds = (
            cmp::min(self.z_bounds.0, new_active_node.2),
            cmp::max(self.z_bounds.1, new_active_node.2),
        );
        active_nodes.insert(new_active_node);
    }

    fn len_active(&self) -> usize {
        self.active_nodes.len()
    }

    /// run six consecutive cycles
    fn boot(&mut self, cycle_count: usize) {
        for i in 0..cycle_count {
            println!("cycle {}", i + 1);
            self.cycle();
            println!("{}", self);
        }
    }

    /// Execute one iteration, working through all nodes adjacent to existing nodes.
    fn cycle(&mut self) {
        // avoid visiting the same node twice by keeping a list of visited nodes.
        let mut visited: HashSet<Node> = HashSet::new();
        //println!("active nodes going into cycle: {:?}", self.active_nodes);
        let mut new_active_nodes = self.active_nodes.clone();
        for active_node in &self.active_nodes.clone() {
            // check all adjacent nodes to this one, plus this one
            for edge_node in active_node.get_neighbors().iter().chain(vec![active_node]) {
                //println!("checking edge node {:?}", edge_node);
                if visited.contains(&edge_node) {
                    continue;
                }
                visited.insert(edge_node.clone());

                let mut active_neighbors = 0;
                for neighbor in edge_node.get_neighbors() {
                    if self.active_nodes.contains(&neighbor) {
                        active_neighbors += 1;
                    }
                }
                //println!("{} active_neighbors", active_neighbors);
                if self.active_nodes.contains(&edge_node) && !(2..4).contains(&active_neighbors) {
                    new_active_nodes.remove(&edge_node);
                } else if !self.active_nodes.contains(&edge_node) && active_neighbors == 3 {
                    self.insert_active_node(&mut new_active_nodes, edge_node.clone());
                }
            }
        }
        self.active_nodes = new_active_nodes;
    }
}

fn part1(input: &str) {
    let mut map = Map::from_str(input);
    println!("starting with: {}", map.len_active());
    map.boot(6);
    println!("result: {}", map.len_active());
}

fn part2(input: &str) {
    let mut map = Map4d::from_str(input);
    println!("starting with: {}", map.len_active());
    map.boot(6);
    println!("result: {}", map.len_active());
}

fn main() -> Result<()> {
    let input = read_to_string("src/day17/input.txt")?;
    part1(&input);
    part2(&input);
    Ok(())
}
