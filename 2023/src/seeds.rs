use anyhow::Result;
use lazy_static::lazy_static;
use log::{debug, info};
use rayon::prelude::*;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
    static ref LOCATION_HASH: u64 = calculate_hash("location");
}

/// (dest range, source range)
type RangeMap = (std::ops::Range<usize>, std::ops::Range<usize>);

#[derive(Debug, Clone)]
struct SeedMapping {
    source: String,
    destination: String,
    ranges: Vec<RangeMap>,
}

impl SeedMapping {
    fn new(mapping_raw: &str) -> Self {
        let mut parts = mapping_raw.split('\n');
        let source_dest = parts
            .next()
            .expect("no parts")
            .split(' ')
            .next()
            .expect("no source")
            .split('-')
            .collect::<Vec<_>>();
        let (source, dest) = (source_dest[0], source_dest[2]);

        let mut ranges: Vec<RangeMap> = parts
            .filter(|s| !s.is_empty())
            .map(|s| {
                let range_parts = s.split(' ').collect::<Vec<_>>();
                let (dest_i, source_i, length): (usize, usize, usize) = (
                    range_parts[0].parse().expect("parsing dest"),
                    range_parts[1].parse().expect("parsing source"),
                    range_parts[2].parse().expect("parsing length"),
                );
                (dest_i..dest_i + length, source_i..source_i + length)
            })
            .collect();
        ranges.sort_by(|a, b| a.1.start.cmp(&b.1.start));
        Self {
            source: source.to_owned(),
            destination: dest.to_owned(),
            ranges,
        }
    }

    fn get(&self, source_num: &str) -> String {
        // check if the source is in the source ranges
        let range_result = self.ranges.iter().find_map(|(d, s)| {
            let src = source_num.parse::<usize>().unwrap();
            if s.contains(&src) {
                d.clone().nth(src - s.start)
            } else {
                None
            }
        });
        if let Some(res) = range_result {
            debug!("mapping: {} -> {}", source_num, res);
            res.to_string()
        } else {
            debug!("mapping: {} -> {} (no match)", source_num, source_num);
            source_num.to_owned()
        }
    }
}

struct Almanac {
    /// The seeds that are part of the almanac
    seeds: Vec<String>,
    /// The mappings of various attributes
    /// key: source attribute
    /// value: HashMap<destination attribute, ranges??>
    mappings: HashMap<String, SeedMapping>,
    map_cache: HashMap<String, String>,
}

impl Almanac {
    fn new(raw: &str) -> Self {
        let seeds = raw
            .lines()
            .next()
            .expect("no lines")
            .split(':')
            .map(str::trim)
            .collect::<Vec<_>>()[1]
            .split(' ')
            .map(str::to_string)
            .collect();
        let mappings: HashMap<_, _> =
            raw.split("\n\n")
                .skip(1)
                .fold(HashMap::new(), |mut acc, mapping_raw| {
                    let mapping = SeedMapping::new(mapping_raw);
                    acc.insert(mapping.source.to_owned(), mapping);
                    acc
                });
        debug!("Mappings: {:#?}", mappings);
        Self {
            seeds,
            mappings,
            map_cache: HashMap::new(),
        }
    }

    fn get_mappings(&mut self, source_name: &str, num: &str) -> usize {
        // debug!("Getting mapping from {}: {}", source_name, num);
        let seed_mapping = &self.mappings[source_name];
        // early return if cached
        let map_cache_index = format!("{}:{}", source_name, num);
        let mapping_result = if let Some(mapping) = self.map_cache.get(&map_cache_index) {
            debug!("!!! cache hit on {}", num);
            return mapping.parse().expect("parsing seed mapping result");
        } else {
            seed_mapping.get(num)
        };
        if seed_mapping.destination == "location" {
            self.map_cache
                .insert(map_cache_index, mapping_result.clone());
            mapping_result.parse().expect("parsing seed mapping result")
        } else {
            let dest = seed_mapping.destination.clone();
            self.get_mappings(&dest, &mapping_result)
        }
    }

    fn locations(&mut self) -> Vec<usize> {
        let mut res = vec![];
        let seeds = self.seeds.clone();
        for seed in seeds {
            debug!("Getting location for {}", seed);
            res.push(self.get_mappings("seed", &seed));
        }
        res
    }
}

#[derive(Debug, Clone)]
struct RangeDelta {
    dest_range: std::ops::Range<isize>,
    source_range: std::ops::Range<isize>,
    delta: isize,
}

#[derive(Debug, Clone)]
struct SeedMappingv2 {
    source: u64,
    destination: u64,
    ranges: Vec<RangeDelta>,
    removed_vals: Vec<usize>,
}

fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl SeedMappingv2 {
    fn new(mapping_raw: &str) -> Self {
        let mut parts = mapping_raw.split('\n');
        let source_dest = parts
            .next()
            .expect("no parts")
            .split(' ')
            .next()
            .expect("no source")
            .split('-')
            .collect::<Vec<_>>();
        let (source, dest) = (source_dest[0], source_dest[2]);

        let mut ranges: Vec<_> = parts
            .filter(|s| !s.is_empty())
            .map(|s| {
                let range_parts = s.split(' ').collect::<Vec<_>>();
                let (dest_i, source_i, length): (isize, isize, isize) = (
                    range_parts[0].parse().expect("parsing dest"),
                    range_parts[1].parse().expect("parsing source"),
                    range_parts[2].parse().expect("parsing length"),
                );
                RangeDelta {
                    dest_range: dest_i..dest_i + length,
                    source_range: source_i..source_i + length,
                    delta: source_i as isize - dest_i as isize,
                }
            })
            .collect();
        Self {
            removed_vals: vec![],
            source: calculate_hash(source),
            destination: calculate_hash(dest),
            ranges,
        }
    }

    fn get(&self, source_num: isize) -> isize {
        // check if the source is in the source ranges
        let mut dest = source_num;
        for range_delta in &self.ranges {
            if range_delta.source_range.contains(&source_num) {
                dest = source_num - range_delta.delta;
                break;
            }
        }
        dest
    }
}

struct Almanacv2 {
    /// The seeds that are part of the almanac
    seeds: Vec<std::ops::Range<isize>>,
    /// The mappings of various attributes
    /// key: source attribute
    /// value: HashMap<destination attribute, ranges??>
    mappings: HashMap<u64, SeedMappingv2>,
    map_cache: HashMap<String, String>,
}

impl Almanacv2 {
    fn new(raw: &str) -> Self {
        let seeds: Vec<std::ops::Range<isize>> = raw
            .lines()
            .next()
            .expect("no lines")
            .split(':')
            .map(str::trim)
            .collect::<Vec<_>>()[1]
            // separate into values
            .split(' ')
            // parse to ints
            .map(str::parse::<isize>)
            // unwrap
            .flatten()
            // create ranges from each pairing
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
            .collect();
        info!(
            "{:?} seeds",
            seeds.iter().fold(0, |acc, b| acc + b.end - b.start)
        );
        let mappings: HashMap<_, _> =
            raw.split("\n\n")
                .skip(1)
                .fold(HashMap::new(), |mut acc, mapping_raw| {
                    let mapping = SeedMappingv2::new(mapping_raw);
                    acc.insert(mapping.source.to_owned(), mapping);
                    acc
                });
        debug!("Mappings: {:#?}", mappings);
        Self {
            seeds,
            mappings,
            map_cache: HashMap::new(),
        }
    }

    fn get_mappings(&self, source_name: u64, num: isize, mut depth: usize) -> isize {
        // debug!("Getting mapping from {}: {}", source_name, num);
        // debug!("d: {}", depth);
        let seed_mapping = &self.mappings[&source_name];
        let mapping_result = seed_mapping.get(num);
        if seed_mapping.destination == *LOCATION_HASH {
            mapping_result
        } else {
            self.get_mappings(seed_mapping.destination, mapping_result, depth + 1)
        }
    }

    fn min_location(&mut self) -> usize {
        let mut min = std::isize::MAX;
        for seed_range in self.seeds.clone() {
            info!("starting range {:?}", seed_range);
            let seed_hash = calculate_hash("seed");
            for seed in seed_range {
                let timer = Instant::now();
                min = min.min(self.get_mappings(seed_hash, seed, 0));
                debug!(
                    "Got location for {} in {:?}",
                    seed,
                    timer.elapsed().as_nanos(),
                );
            }
            // info!("finished range {:?}", seed_range);
        }
        min as usize
    }
}

pub fn run() -> Result<()> {
    seeds_1(
        "sample",
        &std::fs::read_to_string("src/sampledata/5.sample").unwrap(),
    )?;
    seeds_1(
        "part 1",
        &std::fs::read_to_string("src/sampledata/5.1").unwrap(),
    )?;
    seeds_2(
        "smaple 2",
        &std::fs::read_to_string("src/sampledata/5.sample").unwrap(),
    )?;
    seeds_2(
        "part 2",
        &std::fs::read_to_string("src/sampledata/5.1").unwrap(),
    )?;
    Ok(())
}

fn seeds_1(name: &str, f: &str) -> Result<()> {
    let mut almanac = Almanac::new(f);
    // find the location for each of the given seeds
    let locations = almanac.locations();
    let min_location = locations.iter().min();
    info!("{}: min location is {:?}", name, min_location);
    Ok(())
}

fn seeds_2(name: &str, f: &str) -> Result<()> {
    let mut almanac = Almanacv2::new(f);
    // find the location for each of the given seeds
    let min_location = almanac.min_location();
    info!("{}: min location is {:?}", name, min_location);
    Ok(())
}
