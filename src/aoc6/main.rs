use std::error::Error;
use std::fs::{read_to_string};
use std::path::Path;
use std::collections::HashSet;

pub fn run() {
    let file = read_puzzle_input();
    let num_unique_answers = get_num_unique_answers(&file);
    let num_unanimous_answers = get_num_unanimous_answers(&file);
    println!("Total unique answers: {}", num_unique_answers);
    println!("Total unanimous answers: {}", num_unanimous_answers);
}

fn get_num_unanimous_answers(file: &String) -> u32 {
    let groups = file.split("\n\n");
    let mut cumsum = 0u32;
    for group in groups {
        let mut unanimous_answers = 0u32;
        let unique_answers = get_unique_answers_in_group(group);
        let people_in_group: Vec<&str> = group.trim().split("\n").collect();
        for char in unique_answers {
            if people_in_group.iter().all(|&s| s.contains(char)) {
                unanimous_answers += 1;
            }
        }
        println!("Unanimous answers: {} for group:\n{}\n", unanimous_answers, group);
        cumsum += unanimous_answers;
    }
    cumsum
}

fn get_num_unique_answers(file: &String) -> u32 {
    let groups = file.split("\n\n");
    let mut cumsum = 0u32;
    for group in groups {
        let unique_answers = get_unique_answers_in_group(group);
        cumsum += unique_answers.len() as u32;
        println!("Unique answers found: {:?}", unique_answers.len());
    }
    cumsum
}

fn get_unique_answers_in_group(group: &str) -> HashSet<char> {
    let answers_in_group = group.trim().replace("\n", "");
    let mut unique_answers = HashSet::new();
    for char in answers_in_group.chars() {
        unique_answers.insert(char);
    }
    return unique_answers;
}

fn read_puzzle_input() -> String {
    let path = Path::new("./src/aoc6/input.txt");
    let display = path.display();
    let file = match read_to_string(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    return file;
}