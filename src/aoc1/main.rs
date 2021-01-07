use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

pub fn run() {
    // Create a path to the desired file
    let path = Path::new("./src/aoc1/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines= reader.lines();

    // Parse lines as i16 and load into HashSet for O(1) containment queries
    let set: HashSet<_> = lines.map(|x| x.unwrap().parse::<i32>().unwrap()).collect();

    for key in &set {
        for key2 in &set {
            let complement = &set.get(&(2020 - key - key2));
            match complement {
                Some(x) => {
                    println!("Found sum to 2020: {} + {} + {}", x, key, key2);
                    println!("Multiplied: {}", *x * key * key2)
                },
                None => ()
            }
        }
    }
}
