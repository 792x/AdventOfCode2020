use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;


const MAX_ROWS: u32 = 128;
const MAX_COLUMNS: u32 = 8;

pub fn run() {
    let lines = read_puzzle_input();
    let mut max_seat_id: u32 = 0;
    let mut row_frequency: HashMap<u32, u32> = HashMap::new();
    let mut col_frequency: HashMap<u32, u32> = HashMap::new();
    let mut filled_seats_row_70 = Vec::new();

    for line in lines {
        if let Ok(input) = line {
            let rows = &input[..7];
            let columns = &input[7..];
            println!("{} - {}", rows, columns);
            let row_number = get_row_number(rows);
            *row_frequency.entry(row_number).or_insert(0) += 1;
            let column_number = get_column_number(columns);
            *col_frequency.entry(column_number).or_insert(0) += 1;
            let seat_id = row_number * 8 + column_number;
            if row_number == 70 {filled_seats_row_70.push(column_number)};
            max_seat_id = if seat_id > max_seat_id { seat_id } else { max_seat_id }
        }
    }

    for (&key, &value) in row_frequency.iter() {
        if value == 7 {
            println!("Empty seat on row found: {}", key);
        }
    }
    println!("Filled seats on row 70: {:?}", filled_seats_row_70);
    println!("Maximum seatID found: {}", max_seat_id);
    println!("My seat is row 70, column 5; seat ID: {}", 70 * 8 + 5);
}


fn get_row_number(rows: &str) -> u32 {
    let mut range = MAX_ROWS;
    let mut start = 0u32;
    let mut final_row = 0;

    for (i, char) in rows.chars().enumerate() {
        match char {
            'B' => {
                start += range / 2;
            }
            'F' => (),
            _ => panic!("Unknown char: {}", char)
        }
        if i == 6 {
            final_row = start;
        }
        range /= 2;
    }

    println!("start: {}, range: {}, final: {}", start, range, final_row);
    return final_row;
}


fn get_column_number(columns: &str) -> u32 {
    let mut range = MAX_COLUMNS;
    let mut start = 0u32;
    let mut final_col = 0;

    for (i, char) in columns.chars().enumerate() {
        match char {
            'R' => {
                start += range / 2;
            }
            'L' => (),
            _ => panic!("Unknown char: {}", char)
        }
        if i == 2 {
            final_col = start;
        }
        range /= 2;
    }
    println!("start: {}, range: {}, final: {}", start, range, final_col);
    return final_col;
}


fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc5/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}