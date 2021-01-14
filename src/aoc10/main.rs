use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;


pub fn run() {
    let lines = read_puzzle_input();
    let mut numbers = parse_numbers(lines);
    get_jolt_differences(&mut numbers);


}

fn get_jolt_differences(numbers: &mut Vec<i32>) {
    numbers.sort_unstable();
    let mut prev = numbers[0];
    let mut num_1_diff = 1i32;
    let mut num_3_diff = 1i32;
    for num in numbers {
        match *num - prev {
            1 => num_1_diff += 1,
            3 => num_3_diff += 1,
            _ => ()
        }
        prev = *num;
    }
    println!("1 diffs: {}, 3 diffs: {}, mult: {}", num_1_diff, num_3_diff, num_1_diff * num_3_diff);
}


fn parse_numbers(lines: Lines<BufReader<File>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for line in lines {
        vec.push(line.unwrap().parse::<i32>().unwrap())
    }
    vec
}

fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc10/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}