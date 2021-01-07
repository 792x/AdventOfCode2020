use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("./src/aoc3/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let columns = 31;
    let rows = 323;

    let mut forest = vec![0u16; columns * rows];
    for (i, line) in lines.enumerate() {
        for (j, char) in line.unwrap().chars().enumerate() {
            if char == '#' {
                forest[i*columns + j] = 1;
            } else {
                forest[i*columns + j] = 0;
            }
        }
    }

    for i in 0..rows {
        for j in 0..columns {
            print!("{}", if forest[i*columns + j] == 1 { "#" } else { "." } );
        }
        println!();
    }

    let slopes: [[u16;2];4] = [[1, 1], [3, 1], [5, 1], [7, 1]];
    let mut total_count = 1u32;
    for slope in slopes.iter() {
        let mut j = 0u16;
        let mut tree_count = 0u16;
        for mut i in 0..rows {
            if forest[i * columns + (j as usize) % columns] == 1 {
                tree_count += 1;
            }
            j += slope[0];
        }
        total_count *= tree_count as u32;
        println!("Trees encountered in slope {:?}: {}", slope, tree_count);
    }

    // Handle [1, 2] case separately
    let mut j = 0u16;
    let mut tree_count = 0u16;
    for mut i in (0..rows).step_by(2) {
        if forest[i * columns + (j as usize) % columns] == 1 {
            tree_count += 1;
        }
        j += 1;
    }
    total_count *= tree_count as u32;
    println!("Trees encountered in slope [1,2]: {}", tree_count);
    println!("Total count (multiplied together): {}", total_count);
}