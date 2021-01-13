use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    let lines = read_puzzle_input();
    let numbers = parse_numbers(lines);
    find_bad_numbers(numbers, 25);

}

fn find_bad_numbers(numbers: Vec<i64>, preamble_size: i64) {
    let mut window: Vec<i64> = Vec::new();
    for number in numbers {
        if window.len() as i64 > preamble_size {
            window.drain(..1);
        }
        if window.len() as i64 >= preamble_size && !find_two_sum(&window, number) {
            println!("Bad number: {}", number);
            break;
        }
        window.push(number);
    }
}

fn find_two_sum(window: &Vec<i64>, target: i64) -> bool {
    //O(n^2) but window is relatively small so whatever
    for i in window {
        for j in window {
            if i + j == target {
                println!("{} + {} = {}",i, j, target);
                return true;
            }
        }
    }
    false
}

fn parse_numbers(lines: Lines<BufReader<File>>) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();
    for line in lines {
        vec.push(line.unwrap().parse::<i64>().unwrap())
    }
    vec
}

fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc9/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}