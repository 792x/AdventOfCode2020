use std::error::Error;
use std::fs::{read_to_string};
use std::path::Path;
use std::collections::HashMap;

pub fn run() {
    let file = read_puzzle_input();
    let passports = parse_passports(&file);
    let invalid_count = verify_passports(&passports);
    println!("Found {} invalid passsports", invalid_count);
    println!("Found {} valid passports", passports.len() - invalid_count as usize);
}

fn verify_passports(passports: &Vec<HashMap<String, String>>) -> u32 {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut invalid_count = 0;
    for passport in passports {
        for key in keys.iter() {
            if !passport.contains_key(*key) || !is_passport_field_valid(*key, passport.get(*key).unwrap()) {
                println!("Invalid passport found: {:?}", passport);
                invalid_count += 1;
                break;
            }
        }
    }
    return invalid_count;
}

fn is_passport_field_valid(key: &str, value: &String) -> bool {
    return match key {
        ("byr") =>
            value.chars().count() == 4
                && value.parse::<u16>().unwrap() >= 1920
                && value.parse::<u16>().unwrap() <= 2002,
        ("iyr") =>
            value.chars().count() == 4
                && value.parse::<u16>().unwrap() >= 2010
                && value.parse::<u16>().unwrap() <= 2020,
        ("eyr") =>
            value.chars().count() == 4
                && value.parse::<u16>().unwrap() >= 2020
                && value.parse::<u16>().unwrap() <= 2030,
        ("hgt") => (
            value.contains("in")
                && value.replace("in", "").parse::<u16>().unwrap() >= 59
                && value.replace("in", "").parse::<u16>().unwrap() <= 76
        ) || (
            value.contains("cm")
                && value.replace("cm", "").parse::<u16>().unwrap() >= 150
                && value.replace("cm", "").parse::<u16>().unwrap() <= 193
        ),
        ("hcl") => value.chars().count() == 7
            && value.chars().nth(0).unwrap() == '#'
            && value.replace("#", "").chars().all(char::is_alphanumeric),
        ("ecl") => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .contains(&&**value),
        ("pid") => value.chars().count() == 9
            && value.chars().all(char::is_numeric),
        _ => false
    };
}

fn parse_passports(file: &String) -> Vec<HashMap<String, String>> {
    let mut result = Vec::new();
    let passports = file.split("\n\n");
    for passport in passports {
        let pretty_passport = passport.trim().replace("\n", " ");
        let mut passport_map = HashMap::new();
        let passport_entries = pretty_passport.split(" ");
        for entry in passport_entries {
            let mut pair = entry.split(":");
            let key = pair.next().unwrap();
            let value = pair.last().unwrap();
            passport_map.insert(String::from(key), String::from(value));
        }
        println!("{:?}", passport_map);
        result.push(passport_map);
    }
    return result;
}

fn read_puzzle_input() -> String {
    let path = Path::new("./src/aoc4/input.txt");
    let display = path.display();
    let file = match read_to_string(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    return file;
}