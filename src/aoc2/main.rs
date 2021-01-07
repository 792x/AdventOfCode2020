use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn run(){
    // Create a path to the desired file
    let path = Path::new("./src/aoc2/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut count: u16 = 0;
    let mut count2: u16 = 0;
    for line in lines.by_ref() {
        let string = line.unwrap();
        let mut iter = string.split_whitespace();
        let mut policy = iter.next().unwrap().split("-");
        let min = &policy.next().unwrap().parse::<u16>().unwrap();
        let max = &policy.last().unwrap().parse::<u16>().unwrap();
        let key = &iter.next().unwrap()[..1];
        let password = iter.next().unwrap();
        let occurrences = password.matches(key).count() as u16;
        if &occurrences >= min  && &occurrences <= max {
            count += 1;
        }
        let first_char = password.chars().nth((min - 1) as usize).unwrap();
        let second_char = password.chars().nth((max - 1) as usize).unwrap();
        if first_char != second_char {
            let char_key = key.chars().nth(0).unwrap();
            if first_char == char_key || second_char == char_key {
                count2 += 1;
            }
        }

    }
    println!("Total valid passwords with first policy: {}", count);
    println!("Total valid passwords with second policy: {}", count2);
}