use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    let lines = read_puzzle_input();
    let numbers = parse_numbers(lines);
    let bad_number = find_bad_number(&numbers, 25);
    println!("Bad number: {}", &bad_number);
    let set = find_sum_set(&numbers, bad_number);
    println!("Contiguous set min and max sum: {}", set);


}

fn find_sum_set(numbers: &Vec<i64>, bad_number: i64) -> i64 {
    let mut first = 0;
    let mut last = 0;
    let mut sum = 0;

    while sum != bad_number {
        while sum < bad_number {
            sum += numbers[last];
            last += 1;
        }

        while sum > bad_number {
            sum -= numbers[first];
            first += 1;
        }
    }

    let slice = &numbers[first..last];
    println!("Contiguous set found: {:?}", slice);
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}

fn find_bad_number(numbers: &Vec<i64>, preamble_size: i64) -> i64{
    let mut window: Vec<i64> = Vec::new();
    for number in numbers {
        if window.len() as i64 > preamble_size {
            window.drain(..1);
        }
        if window.len() as i64 >= preamble_size && !find_two_sum(&window, *number) {
            return *number;
        }
        window.push(*number);
    }
    return -1;
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