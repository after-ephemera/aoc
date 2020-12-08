use eyre::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::vec::Vec;

#[derive(Debug)]
struct BagNode {
    color: String,
    neighbors: HashSet<String>,
}

#[derive(Debug)]
struct BagGraph {
    nodes: HashMap<String, BagNode>,
}

impl fmt::Display for BagNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(color:\n{:?},\n neighbors:\n{:?}\n)",
            self.color, self.neighbors
        )
    }
}

impl BagNode {
    fn new(color: String) -> BagNode {
        BagNode {
            color: color,
            neighbors: HashSet::new(),
        }
    }

    fn add_neighbor(&mut self, n: String) {
        self.neighbors.insert(n);
    }
}

impl BagGraph {
    fn new() -> BagGraph {
        BagGraph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, color: String, neighbors: Vec<Option<String>>) {
        let new_node = self
            .nodes
            .entry(color.clone())
            .or_insert(BagNode::new(color.clone()));
        for n in neighbors {
            if let Some(neighbor) = n {
                new_node.add_neighbor(neighbor);
            }
        }
    }

    fn node_can_contain(&self, node_color: &str, search_color: &str) -> bool {
        let parent_node = self.nodes.get(node_color).unwrap();
        // if the search color is in the immediate children we are good.
        if parent_node.neighbors.contains(search_color) {
            return true;
        }

        // recurse on each of the children, searching for the search color
        for neighbor_color in parent_node.neighbors.clone() {
            if self.node_can_contain(&neighbor_color, &search_color) {
                return true;
            }
        }
        // no node in this path has the search color
        false
    }

    fn count_containers_of(&self, color: String) -> usize {
        let mut count = 0;
        for node in self.nodes.values() {
            if self.node_can_contain(&node.color, &color) {
                count += 1;
            }
        }
        count
    }
}

fn gen_graph(input: String) -> Result<BagGraph> {
    let main_re = Regex::new(r"(?m)^(.*) bags contain (.*,?)+\n").unwrap();
    let contains_re = Regex::new(r"(\d+) (.*) bag[s]?[.]?").unwrap();
    let mut graph = BagGraph::new();

    for line in main_re.captures_iter(&input) {
        let primary_color = line[1].to_string();
        //println!("color: {:?}", &line[1]);
        let contains_results: Vec<Option<String>> = line[2]
            .split(',')
            .map(|contains| {
                let contained = contains_re.captures(&contains);
                return if let Some(contained_val) = contained {
                    let child_color = contained_val[2].to_string();
                    Some(child_color)
                } else {
                    None
                };
            })
            .collect();
        graph.add_node(primary_color, contains_results);
    }
    Ok(graph)
}

fn main() -> Result<()> {
    let graph = gen_graph(read_to_string("src/day7/input.txt")?)?;
    println!("graph: {:#?}", graph);

    let final_count = graph.count_containers_of("shiny gold".to_string());
    println!("final count: {}", final_count);
    Ok(())
}
