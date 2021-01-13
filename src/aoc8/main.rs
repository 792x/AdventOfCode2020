use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashSet};

struct Command {
    cmd: String,
    value: i32,
    linenr: u32
}

pub fn run(){
    let lines = read_puzzle_input();
    let commands = parse_commands(lines);
    run_commands(&commands);
    find_broken_command(&commands);
}

fn find_broken_command(commands: &Vec<Command>) {
    for x in 0..commands.len() as i32 {
        let mut executed_commands = HashSet::new();
        let mut acc = 0i32;
        let mut i = 0i32;
        let mut terminated = false;
        while i < commands.len() as i32 {
            // casting i to usize here can cause problems if i is negative
            let command = &commands[i as usize];
            match executed_commands.get(&command.linenr) {
                Some(_) => (break),
                None => (executed_commands.insert(command.linenr))
            };
            match &command.cmd {
                cmd if cmd == "acc" => acc += command.value,
                cmd if (cmd == "jmp" && i != x) => i += command.value - 1,
                cmd if (cmd == "jmp" && i == x) => (),
                cmd if (cmd == "nop" && i == x) => i += command.value - 1,
                _ => (),
            };
            if i == commands.len() as i32 - 1 {
                terminated = true;
                println!("TERMINATED");
                break;
            }
            i += 1;
        }
        if terminated {
            println!("Final acc: {}", acc);
            break;
        }
    }
}

fn run_commands(commands: &Vec<Command>) {
    let mut executed_commands = HashSet::new();
    let mut acc = 0i32;
    let mut i = 0;
    while i < commands.len() {
        let command = &commands[i];
        match executed_commands.get(&command.linenr) {
            Some(_) => (break),
            None => (executed_commands.insert(command.linenr))
        };

        match &command.cmd {
            cmd if cmd == "acc" => acc += command.value,
            cmd if cmd == "jmp" => i = (i as i32 + command.value - 1) as usize,
            _ => (),
        };
        i += 1;
    }
    println!("Final acc: {}", acc);
}

fn parse_commands(lines: Lines<BufReader<File>>) -> Vec<Command>{
    let mut commands: Vec<Command> = Vec::new();
    for (i, line) in lines.enumerate() {
        let res = line.unwrap();
        let mut split = res.split(" ");
        commands.push(Command {
            cmd: split.next().unwrap().parse().unwrap(),
            value: split.next().unwrap().parse::<i32>().unwrap(),
            linenr: i as u32
        })
    }
    return commands;
}


fn read_puzzle_input() -> Lines<BufReader<File>> {
    let path = Path::new("./src/aoc8/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    return reader.lines();
}