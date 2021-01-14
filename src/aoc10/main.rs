use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;


pub fn run() {
    let lines = read_puzzle_input();
    let mut numbers = parse_numbers(lines);
    numbers.sort_unstable();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);
    println!("{:?}", &numbers);
    let (one_diffs, three_diffs) = get_jolt_differences(&mut numbers);
    println!("1 diffs: {}, 3 diffs: {}, mult: {}", one_diffs, three_diffs, one_diffs * three_diffs);
    let num_arrangements = get_num_distinct_arrangements(&mut numbers);
    println!("Number of arrangements: {}", num_arrangements);
}

fn get_num_distinct_arrangements(numbers: &mut Vec<i64>) -> i64 {
    let mut slices = Vec::new();
    let mut current_slice = Vec::new();

    // Credit to https://github.com/tudorpavel for elegant solution
    for window in numbers.windows(2) {
        match window[1] - window[0] {
            1 => current_slice.push(window[0]),
            3 => {
                current_slice.push(window[0]);
                slices.push(current_slice);
                current_slice = Vec::new();
            }
            _ => (),
        }
    }

    slices.iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("panic"),
        })
        .product()
}


fn get_jolt_differences(numbers: &mut Vec<i64>) -> (i64, i64) {
    let mut one_diffs = 0i64;
    let mut three_diffs = 0i64;
    for window in numbers.windows(2) {
        match window[1] - window[0] {
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            _ => (),
        }
    }
    return (one_diffs, three_diffs);
}


fn parse_numbers(lines: Lines<BufReader<File>>) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();
    for line in lines {
        vec.push(line.unwrap().parse::<i64>().unwrap())
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