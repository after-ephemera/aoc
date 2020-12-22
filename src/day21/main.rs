use eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

lazy_static! {
    static ref ING_REGEX: Regex = Regex::new(r"((?:\w+ )+)(?:\(contains (.*)+\))").unwrap();
}

#[derive(Debug)]
struct AllergenListing {
    name: String,
    entry_count: usize,
    // name -> count (for this ingredient)
    ingredient_lists: HashMap<String, usize>,
}

impl AllergenListing {
    fn new(name: &str) -> Self {
        AllergenListing {
            name: name.to_string(),
            entry_count: 0,
            ingredient_lists: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct IngredientListing {
    names: HashSet<String>,
    allergens: HashSet<String>,
}

impl IngredientListing {
    fn new(names: HashSet<String>, allergens: HashSet<String>) -> Self {
        IngredientListing { names, allergens }
    }
}

fn read_raw_to_ingredient_list(
    ingredient_listings: &mut Vec<IngredientListing>,
    raw_listing: &str,
) {
    let caps = ING_REGEX.captures(raw_listing).unwrap();
    let names: HashSet<String> = caps
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    let potential_allergens: HashSet<String> = caps
        .get(2)
        .unwrap()
        .as_str()
        .trim()
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();

    ingredient_listings.push(IngredientListing::new(names, potential_allergens));
}
fn read_raw_to_allergen_map(
    allergens_map: &mut HashMap<String, AllergenListing>,
    raw_listing: &str,
) {
    let caps = ING_REGEX.captures(raw_listing).unwrap();
    let names: Vec<&str> = caps
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .split_whitespace()
        .collect();
    let potential_allergens: Vec<&str> = caps
        .get(2)
        .unwrap()
        .as_str()
        .trim()
        .split(',')
        .map(|x| x.trim())
        .collect();
    //println!("names: {:?}, allergens: {:?}", names, potential_allergens);

    for allergen in &potential_allergens {
        let allergen_entry = allergens_map
            .entry(allergen.to_string())
            .or_insert_with(|| AllergenListing::new(allergen));
        allergen_entry.entry_count += 1;
        for ingredient_name in &names {
            *allergen_entry
                .ingredient_lists
                .entry(ingredient_name.to_string())
                .or_insert(0) += 1;
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("src/day21/input.txt")?;
    //let input = read_to_string("src/day21/input-sample.txt")?;

    let mut all_allergens = HashMap::new();
    for line in input.trim().lines() {
        read_raw_to_allergen_map(&mut all_allergens, &line);
    }

    let all_ingredients = all_allergens
        .iter()
        .map(|(_, entry)| entry.ingredient_lists.keys().collect::<HashSet<&String>>())
        .fold(HashSet::new(), |res, ing_list| {
            res.union(&ing_list).copied().collect()
        });

    //println!("all ingredients: {:?}", all_ingredients);
    let not_possibly_allergenic = all_allergens
        .iter()
        .map(|(i, entry)| {
            let mut filtered_ingredients = all_ingredients
                .clone()
                .iter()
                .copied()
                .collect::<HashSet<_>>();
            // remove any ingredients that aren't in all entries.
            for (ingredient_name, &count) in &entry.ingredient_lists {
                if count == entry.entry_count {
                    filtered_ingredients.remove(&ingredient_name);
                }
            }
            (i, filtered_ingredients)
        })
        .fold(all_ingredients.clone(), |res, (_, set)| {
            res.intersection(&set).copied().collect()
        });
    //println!("not possible: {:?}", not_possibly_allergenic);

    let non_allergenic_appearances: usize = not_possibly_allergenic
        .iter()
        .map(|ingredient_name| {
            let count: usize = input
                .split_whitespace()
                .filter(|s| s == *ingredient_name)
                .count();
            count
        })
        .sum();
    println!(
        "total nonallergenic appearances: {}",
        non_allergenic_appearances
    );

    // part 2
    // approach:
    // - take original entries (ingredients -> allergens)
    // - find an entry with a single allergen (the ingredient must be in the list)
    // - search all other entries with that allergen and find a common ingredient
    // - search for a second common ingredient. if one exists then move to the next
    // single-allergen entry.
    // - if there is only one common ingredient, we've found a match. Mark the allergen as found and
    // remove it from all other allergen lists.
    // - repeat

    let mut ingredient_listings = vec![];
    for line in input.trim().lines() {
        read_raw_to_ingredient_list(&mut ingredient_listings, &line);
    }
    let mut final_ingredient_map = HashMap::new();
    let mut all_ingredient_listings = ingredient_listings.clone();
    while let Some((single_index, single_allergen_listing)) = all_ingredient_listings
        .iter()
        .enumerate()
        .find(|(_, l)| l.allergens.len() == 1)
    {
        let allergen_to_match = single_allergen_listing.allergens.iter().next().unwrap();
        let common_ingredients: HashSet<String> = ingredient_listings
            .iter()
            .filter(|l| l.allergens.contains(allergen_to_match))
            .map(|s| {
                s.names
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<HashSet<_>>()
            })
            .fold(single_allergen_listing.names.clone(), |res, s| {
                res.intersection(&s).map(|i| i.to_string()).collect()
            });

        if common_ingredients.len() == 1 {
            // found a match!
            let common_ingredient = common_ingredients.iter().next().unwrap().to_string();
            final_ingredient_map.insert(common_ingredient.clone(), allergen_to_match.clone());
            // remove this entry
            ingredient_listings.remove(single_index);
            // remove this allergen from all other entries
            ingredient_listings.iter_mut().for_each(|l| {
                l.allergens.remove(allergen_to_match);
                l.names.remove(&common_ingredient);
            });
        } else {
            // this listing can be revisited
            ingredient_listings.remove(single_index);
            ingredient_listings.push(single_allergen_listing.clone());
        }
        all_ingredient_listings = ingredient_listings.clone();
    }
    let mut matched_allergens = final_ingredient_map.values().collect::<Vec<_>>();
    matched_allergens.sort();
    let ordered_ingredients_str = matched_allergens
        .iter()
        .map(|a| {
            final_ingredient_map
                .iter()
                .find(|(_, i)| i == a)
                .map(|(k, _)| k)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(",");
    println!("ordered: {}", ordered_ingredients_str);
    Ok(())
}
