use eyre::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::vec::Vec;

#[derive(Debug)]
struct BagNode {
    color: String,
    contents: HashMap<String, u32>,
}

#[derive(Debug)]
struct BagGraph {
    nodes: HashMap<String, BagNode>,
}

impl fmt::Display for BagNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(color:\n{:?},\n contents:\n{:?}\n)",
            self.color, self.contents
        )
    }
}

impl BagNode {
    fn new(color: String) -> BagNode {
        BagNode {
            color: color,
            contents: HashMap::new(),
        }
    }

    fn add_contents(&mut self, n: String, count: u32) {
        self.contents.insert(n, count);
    }
}

impl BagGraph {
    fn new() -> BagGraph {
        BagGraph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, color: String, contents: Vec<(Option<String>, u32)>) {
        let new_node = self
            .nodes
            .entry(color.clone())
            .or_insert(BagNode::new(color.clone()));
        for n in contents {
            if let Some(content) = n.0 {
                new_node.add_contents(content, n.1);
            }
        }
    }

    fn node_can_contain(&self, node_color: &str, search_color: &str) -> bool {
        let parent_node = self.nodes.get(node_color).unwrap();
        // if the search color is in the immediate children we are good.
        if parent_node.contents.contains_key(search_color) {
            return true;
        }

        // recurse on each of the children, searching for the search color
        for (contents_color, _contents_count) in parent_node.contents.clone() {
            if self.node_can_contain(&contents_color, &search_color) {
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

    fn count_contents(&self, color: &str) -> u32 {
        let mut count = 1;
        let current_root = self.nodes.get(color).unwrap();
        for (content, content_count) in current_root.contents.clone() {
            count += content_count * self.count_contents(&content);
        }
        return count;
    }
}

fn gen_graph(input: String) -> Result<BagGraph> {
    let main_re = Regex::new(r"(?m)^(.*) bags contain (.*,?)+\n").unwrap();
    let contains_re = Regex::new(r"(\d+) (.*) bag[s]?[.]?").unwrap();
    let mut graph = BagGraph::new();

    for line in main_re.captures_iter(&input) {
        let primary_color = line[1].to_string();
        //println!("color: {:?}", &line[1]);
        let contains_results: Vec<(Option<String>, u32)> = line[2]
            .split(',')
            .map(|contains| {
                let contained = contains_re.captures(&contains);
                return if let Some(contained_val) = contained {
                    let child_color = contained_val[2].to_string();
                    (Some(child_color), contained_val[1].parse::<u32>().unwrap())
                } else {
                    (None, 0)
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

    println!("part 2*****");
    // subtract one because the shiny gold bag itself doesn't count
    let final_count_2 = graph.count_contents("shiny gold") - 1;
    println!("final count is {}", final_count_2);
    Ok(())
}
