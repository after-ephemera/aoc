use eyre::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() -> Result<()> {
    //let input = read_to_string("src/day16/input-sample2.txt")?;
    let input = read_to_string("src/day16/input.txt")?;
    let mut sections = input.split("\n\n");
    let rules: &str = sections.next().unwrap();
    let my_ticket: &str = sections.next().unwrap();
    let nearby_tickets: &str = sections.next().unwrap();
    //println!("{:?}, {:?}, {:?}", rules, my_ticket, nearby_tickets);

    let mut all_fields = HashSet::new();

    // all valid values in one set
    let mut valid_values: HashSet<usize> = HashSet::new();
    // valid values by key (field name)
    let mut field_rules: HashMap<&str, HashSet<usize>> = HashMap::new();

    let rules_re = Regex::new(r"(.*): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)")?;
    let rules_iter = rules.split('\n');
    let rule_count = rules_iter.clone().count();
    for rule in rules_iter {
        let caps = rules_re.captures(rule).unwrap();
        let field_name = caps.get(1).unwrap().as_str();
        all_fields.insert(field_name.to_string());

        //println!("range: {:?}", caps);
        // all ranges
        let range1_low = caps.get(2).unwrap().as_str().parse::<usize>()?;
        let range1_high = caps.get(3).unwrap().as_str().parse::<usize>()?;
        let range2_low = caps.get(4).unwrap().as_str().parse::<usize>()?;
        let range2_high = caps.get(5).unwrap().as_str().parse::<usize>()?;
        let range1 = (range1_low..range1_high + 1).collect::<HashSet<usize>>();
        let range2 = (range2_low..range2_high + 1).collect::<HashSet<usize>>();
        valid_values.extend(range1.clone());
        valid_values.extend(range2.clone());
        field_rules.insert(field_name, range1.union(&range2).copied().collect());
    }
    //println!("all valid values: {:?}", valid_values);

    let mut error_rate = 0;
    let mut entry_field_map = HashMap::new();
    // populate possible fields for each entry
    for i in 0..rule_count {
        entry_field_map.insert(i, all_fields.clone());
    }

    for nearby_ticket in nearby_tickets
        .split('\n')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|tick| {
            tick.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
    {
        for (index, val) in nearby_ticket.iter().enumerate() {
            if !valid_values.contains(&val) {
                error_rate += val;
                continue;
            }
            //ticket may be valid... check if it breaks any rules
            let possible_fields = entry_field_map.get_mut(&index).unwrap();
            for (field_name, possible_values) in field_rules.iter() {
                if !possible_values.contains(&val) {
                    possible_fields.remove(*field_name);
                }
            }
        }
    }
    println!("final error rate: {}", error_rate);

    let mut final_guesses: HashMap<usize, String> = HashMap::new();
    let mut count = 0;

    while final_guesses.len() < entry_field_map.len() {
        //println!("intermediate guesses: {:?}", entry_field_map);
        //println!("final guesses: {:?}", final_guesses);
        for (field_index, possible_fields) in entry_field_map.iter_mut() {
            //println!("checking {} {:?}", field_index, possible_fields);
            // if one of the possible fields has already been guessed,
            // remove it from the list of possible fields.
            for field_name in possible_fields.clone() {
                if final_guesses.values().any(|x| **x == field_name) {
                    //println!(
                    //    "already ffound the answer for {}:{}",
                    //    field_index, field_name
                    //);
                    possible_fields.remove(&field_name);
                }
            }
            // if there is only one guess left, this is the right field!
            if possible_fields.len() == 1 {
                //println!("only one guess left here! {:?}", possible_fields);
                let final_field = &possible_fields.drain().collect::<Vec<String>>()[0];
                final_guesses.insert(*field_index, final_field.to_string());
            }
        }
        count += 1;
    }
    println!(
        "final guesses took {} iterations: {:?}",
        count, final_guesses
    );

    let mut departure_sum = 1;
    let my_ticket_entries = my_ticket.split('\n').nth(1).unwrap().split(',');
    for (index, entry) in my_ticket_entries
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
    {
        let field_name = final_guesses.get(&index).unwrap();
        println!("{}:{:?}", field_name, entry);
        if field_name.contains("departure") {
            departure_sum *= entry;
        }
    }
    println!("final departure sum: {}", departure_sum);

    Ok(())
}
