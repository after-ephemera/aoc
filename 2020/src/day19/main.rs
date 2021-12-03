use eyre::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Rule {
    Ch(String),
    List(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

struct RuleSet {
    rules: HashMap<usize, Rule>,
}

impl RuleSet {
    fn new(rules_str: &str) -> Result<Self> {
        let raw_rules: Vec<&str> = rules_str.split('\n').filter(|x| !x.is_empty()).collect();
        let mut rules = HashMap::new();
        for rule_str in raw_rules {
            let re = Regex::new(
                r#"(\d+): (?:(".*")|(?:((?:[0-9]+ ?)+)$)|(?:((?:[0-9]+ ?)+) \| ((?:[0-9]+ ?)+)))"#,
            )?;
            let caps = re.captures(rule_str).unwrap();
            let rule_number = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            //println!("captured rule number {}: {:?}", rule_number, caps);

            if let Some(str_rule_match) = caps.get(2) {
                let str_rule = str_rule_match.as_str();
                rules.insert(
                    rule_number,
                    Rule::Ch(str_rule.replace("\"", "").to_string()),
                );
                //println!("string rule: {}", str_rule);
            } else if let Some(rule_list_match) = caps.get(3) {
                let rule_list = rule_list_match
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                //println!("rule list: {:?}", rule_list);
                rules.insert(rule_number, Rule::List(rule_list));
            } else if let Some(rule_list_1_match) = caps.get(4) {
                let rule_list_1 = rule_list_1_match
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let rule_list_2 = caps
                    .get(5)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                //println!("rule list or: {:?} or {:?}", rule_list_1, rule_list_2);
                rules.insert(rule_number, Rule::Or(rule_list_1, rule_list_2));
            }
        }
        //println!("finished parsing rules: {:#?}", rules);
        Ok(RuleSet { rules })
    }

    fn _solve_word(&self, word: String, rule: &Rule) -> HashSet<String> {
        let result = match rule {
            Rule::Ch(val) => word
                .strip_prefix(val)
                .map(|w| {
                    let mut h = HashSet::new();
                    h.insert(w.to_string());
                    h
                })
                .unwrap_or_else(HashSet::new),
            Rule::List(rule_list) => {
                let mut result = HashSet::new();
                result.insert(word);
                for rule in rule_list {
                    let mut new_result = HashSet::new();
                    for message in result {
                        new_result = new_result
                            .union(&self._solve_word(message, &self.rules.get(rule).unwrap()))
                            .map(|x| x.to_string())
                            .collect();
                    }
                    result = new_result;
                }
                result
            }
            Rule::Or(rule_list_1, rule_list_2) => {
                let result = self._solve_word(word.clone(), &Rule::List(rule_list_1.clone()));
                let result2 = self._solve_word(word, &Rule::List(rule_list_2.clone()));
                result.union(&result2).cloned().collect()
            }
        };
        result
    }

    fn solve_word(&self, word: String, rule_num: usize) -> HashSet<String> {
        let current_rule = self.rules.get(&rule_num).unwrap();
        self._solve_word(word, &current_rule)
    }
}

fn main() -> Result<()> {
    let input = read_to_string("src/day19/input-pt2.txt")?;
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules = sections[0];
    let messages: Vec<String> = sections[1]
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    let rule_set = RuleSet::new(&rules)?;
    let count = messages
        .iter()
        .flat_map(|message| rule_set.solve_word(message.to_string(), 0))
        .filter(|x| x.is_empty())
        .count();
    println!("{} messages match", count);

    Ok(())
}
