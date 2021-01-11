use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashMap};

pub fn run() {
    let lines = read_puzzle_input();
    let ruleset = parse_rules(lines);
    println!("Ruleset: {:?}", &ruleset);
    get_potential_bag_count(&ruleset);
}

fn get_potential_bag_count(ruleset: &HashMap<String, HashMap<String, i32>>) {
    let mut count = 0i32;
    for (bag_type, can_contain) in ruleset {
        if can_contain_shiny_gold_bag(&ruleset, &can_contain) {
            println!("Valid bag found: {}", bag_type);
            count += 1;
        }
    }
    println!("Final count: {}", count);
}

fn can_contain_shiny_gold_bag(ruleset: &HashMap<String, HashMap<String, i32>>, can_contain: &HashMap<String, i32>) -> bool {
    if can_contain.keys().any(|s| s == "shiny gold") {
        return true;
    } else if can_contain.keys().any(|s| can_contain_shiny_gold_bag(&ruleset, &ruleset.get(s).unwrap())) {
        return true;
    }
    return false;
}

fn parse_rules(lines: Lines<BufReader<File>>) -> HashMap<String, HashMap<String, i32>> {
    let mut ruleset: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in lines {
        let res = line.unwrap();
        let mut split = res.split("contain");
        let key = split.next().unwrap().trim_end_matches(" bags ");
        let rules = split.next().unwrap().split(",");
        let mut contains: HashMap<String, i32> = HashMap::new();
        for rule in rules {
            let mut rule_split = rule.trim().split(" ");
            let value = match rule_split.next() {
                Some(str) if str == "no" => -1i32,
                Some(str) => str.parse::<i32>().unwrap(),
                _ => 0
            };
            let nominator = [rule_split.next().unwrap(), rule_split.next().unwrap()].join(" ");
            if !nominator.contains("other") {
                contains.insert(nominator, value);
            }
        }
        ruleset.insert(key.parse().unwrap(), contains);
    }
    return ruleset;
}


fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc7/test2.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}