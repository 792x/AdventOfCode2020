use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;

// x * height + y
// Part 1 at 16ms (excluding i/o)

const WIDTH: i32 = 92;
const HEIGHT: i32 = 99;
const DIRECTIONS: [(i32, i32); 8] = [(-1, 0), (0, -1), (0, 1), (1, 0), (-1, 1), (-1, -1), (1, -1), (1, 1)];

pub fn run() {
    let lines = read_puzzle_input();

    let mut seats = parse_lines(lines);
    loop {
        let (new_seats, changed) = update_seats(&mut seats);
        seats = new_seats;
        if !changed { break; };
    }

    println!("Occupied seats after iterations: {}", seats.iter().filter(|&c| *c == '#').count())
}

fn update_seats(seats: &mut Vec<char>) -> (Vec<char>, bool) {
    let mut new_seats = seats.clone();
    let mut changed = false;
    for x in 0..WIDTH as i32 {
        for y in 0..HEIGHT as i32 {
            // let new_seat_state = get_new_seat_state_part_1(&seats, x, y);
            let new_seat_state = get_new_seat_state_part_2(&seats, x, y);
            if seats[(x * HEIGHT + y) as usize] != new_seat_state {
                new_seats[(x * HEIGHT + y) as usize] = new_seat_state;
                changed = true;
            }
        }
    }
    return (new_seats, changed);
}

fn get_new_seat_state_part_1(seats: &Vec<char>, x: i32, y: i32) -> char {
    let mut neighbors_occupied = 0;
    let seat = seats[(x * HEIGHT + y) as usize];
    // Check all 8 neighbors around seat
    for xd in x-1..x+2 {
        for yd in y-1..y+2 {
            // Clamp to width and height
            if !(xd == x && yd == y) && xd >= 0 && xd < WIDTH && yd >= 0 && yd < HEIGHT {
                match seats[(xd * HEIGHT + yd) as usize] {
                    '#' => { neighbors_occupied += 1; },
                    'L' => (),
                    '.' => (),
                    _ => panic!("Unknown character")
                }
            }
        }
    }

    match seat {
        '#' => return if neighbors_occupied >= 4 { 'L' } else { '#' },
        'L' => return if neighbors_occupied == 0 { '#' } else { 'L' },
        '.' => return '.',
        _ => panic!("Unknown character")
    }
}


fn get_new_seat_state_part_2(seats: &Vec<char>, x: i32, y: i32) -> char {
    let mut visible_occupied = 0;
    let seat = seats[(x * HEIGHT + y) as usize];
    // Check all 8 directions around seat
    for dir in DIRECTIONS.iter() {
        let mut i = 1i32;
        loop {
            let mut xd = x + (dir.0 * i);
            let mut yd = y + (dir.1 * i);

            if !(xd >= 0 && xd < WIDTH && yd >= 0 && yd < HEIGHT) {
                // Out of bounds
                break;
            }

            match seats[(xd * HEIGHT + yd) as usize] {
                '#' => { visible_occupied += 1; break;},
                'L' => { break; },
                '.' => { i += 1;},
                _ => panic!("Unknown character")
            }
        }
    }

    match seat {
        '#' => return if visible_occupied >= 5 { 'L' } else { '#' },
        'L' => return if visible_occupied == 0 { '#' } else { 'L' },
        '.' => return '.',
        _ => panic!("Unknown character")
    }
}


fn parse_lines(lines: Lines<BufReader<File>>) -> Vec<char> {
    let lines: Vec<String> = lines.map(|s| s.unwrap()).collect();
    let width: usize = lines[0].len();
    let height: usize = lines.len();
    let mut seats = vec!['.'; width * height];
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            seats[x*height + y] = character;
        }
    }
    seats
}

fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc11/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}