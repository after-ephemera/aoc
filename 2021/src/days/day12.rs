use super::Day;
use eyre::Result;
use itertools::Itertools;
use std::fs::read_to_string;

fn parse_input(raw_input: &str) -> Result<Vec<Vec<&str>>> {
    Ok(raw_input
        .lines()
        .map(|l| l.split('-').collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

pub struct Day12 {}

impl Day12 {
    pub fn new() -> Self {
        Self {}
    }

    fn should_revisit(&self, visited: &[String], destination: &str, small_cave_rule: bool) -> bool {
        if destination == "start" {
            false
        } else if small_cave_rule {
            // no duplicates in visited
            let duplicates_in_visited = visited.iter().unique().count() != visited.len();
            if duplicates_in_visited {
                // another cave was visited twice, thus
                // this one cannot be visited twice.
                !visited.contains(&destination.to_string())
            } else {
                // this lowercase cave is the first one to
                // be visited twice.
                true
            }
        } else {
            !visited.contains(&destination.to_string())
        }
    }

    fn get_distinct_paths<'a>(
        &self,
        current_location: &'a str,
        cave_map: &[Vec<&'a str>],
        visited: Vec<String>,
        depth: usize,
        small_cave_rule: bool,
    ) -> Vec<Vec<&'a str>> {
        let new_depth = depth + 1;
        //println!(
        //"checking {:?} at {} with visited: {:?}",
        //current_location, new_depth, visited
        //);
        return if current_location == "end" {
            vec![vec!["end"]]
        } else {
            let mut paths_from_here: Vec<Vec<&str>> = vec![];
            for link in cave_map {
                let mut new_visited = visited.clone();
                if link[0] == current_location
                    && self.should_revisit(&visited, link[1], small_cave_rule)
                {
                    if !link[1].chars().all(|c| c.is_uppercase()) {
                        new_visited.push(link[1].to_string());
                    }
                    let distinct_paths = self.get_distinct_paths(
                        link[1],
                        cave_map,
                        new_visited.clone(),
                        new_depth,
                        small_cave_rule,
                    );
                    //println!("found match: {:?}", distinct_paths);
                    paths_from_here.extend(distinct_paths);
                }
            }
            let reverse_links: Vec<Vec<&str>> = cave_map
                .iter()
                .map(|link| link.iter().rev().copied().collect())
                .collect();
            for link in reverse_links {
                let mut new_visited = visited.clone();
                if link[0] == current_location
                    && self.should_revisit(&visited, link[1], small_cave_rule)
                {
                    if !link[1].chars().all(|c| c.is_uppercase()) {
                        new_visited.push(link[1].to_string());
                    }
                    let distinct_paths = self.get_distinct_paths(
                        link[1],
                        cave_map,
                        new_visited.clone(),
                        new_depth,
                        small_cave_rule,
                    );
                    //println!("found match: {:?}", distinct_paths);
                    paths_from_here.extend(distinct_paths);
                }
            }
            paths_from_here
                .iter_mut()
                .for_each(|p| p.insert(0, current_location));
            paths_from_here
        };
    }

    fn count_distinct_paths(&self, cave_map: &[Vec<&str>]) -> usize {
        let visited = vec!["start".to_string()];
        let depth = 0;
        let paths = self.get_distinct_paths("start", cave_map, visited, depth, false);
        //println!("paths were {:#?}", paths);
        paths.iter().unique().count()
    }

    fn count_distinct_paths_2(&self, cave_map: &[Vec<&str>]) -> usize {
        let visited = vec!["start".to_string()];
        let depth = 0;
        let paths = self.get_distinct_paths("start", cave_map, visited, depth, true);
        //println!("paths were {:#?}", paths);
        paths.iter().unique().count()
    }

    fn part_1(&mut self, raw_input: &str) -> Result<()> {
        let cave_map = parse_input(raw_input)?;
        let distinct_path_count = self.count_distinct_paths(&cave_map);
        println!("there are {:?} distinct paths", distinct_path_count);

        Ok(())
    }

    fn part_2(&mut self, raw_input: &str) -> Result<()> {
        let cave_map = parse_input(raw_input)?;
        let distinct_path_count = self.count_distinct_paths_2(&cave_map);
        println!("there are {:?} distinct paths", distinct_path_count);

        Ok(())
    }
}

impl Day for Day12 {
    fn run(&mut self) -> Result<()> {
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day12-sample")?;
        self.part_1(&sample_raw_input)?;
        println!("sample2");
        let sample_raw_input = read_to_string("src/data/day12-sample2")?;
        self.part_1(&sample_raw_input)?;
        println!("sample3");
        let sample_raw_input = read_to_string("src/data/day12-sample3")?;
        self.part_1(&sample_raw_input)?;
        println!("part 1");
        let raw_input = read_to_string("src/data/day12")?;
        self.part_1(&raw_input)?;

        println!("part 2!");
        println!("sample");
        let sample_raw_input = read_to_string("src/data/day12-sample")?;
        self.part_2(&sample_raw_input)?;
        println!("sample2");
        let sample_raw_input = read_to_string("src/data/day12-sample2")?;
        self.part_2(&sample_raw_input)?;
        println!("sample3");
        let sample_raw_input = read_to_string("src/data/day12-sample3")?;
        self.part_2(&sample_raw_input)?;
        println!("final");
        self.part_2(&raw_input)?;
        Ok(())
    }
}
