extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// This is noticeably slow when compiled in debug mode, so use `cargo build --release`

type Coord = (u64, u64);

enum CommandType {
    On,
    Off,
    Toggle,
}

struct Command {
    command_type: CommandType,
    bottom_left: Coord,
    top_right: Coord,
}

fn parse_captured_nums(captures: regex::Captures) -> (Coord, Coord) {
    ((captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
      captures.get(2).unwrap().as_str().parse::<u64>().unwrap()),
     (captures.get(3).unwrap().as_str().parse::<u64>().unwrap(),
      captures.get(4).unwrap().as_str().parse::<u64>().unwrap()))
}

fn parse_coordinates_from_line(line: &str) -> (Coord, Coord) {
    let re = regex::Regex::new(r"(\d+),(\d+)[^\d]+(\d+),(\d+)").unwrap();
    parse_captured_nums(re.captures(line).unwrap())
}

fn parse_command_from_line(line: &str) -> Command {
    let line_vec = line.chars().collect::<Vec<char>>();
    // Observation - the 7th character disambiguates the command.
    let command_type: CommandType;
    command_type = match line_vec[6] {
        'n' => CommandType::On,
        'f' => CommandType::Off,
        _ => CommandType::Toggle,
    };
    let (bottom_left, top_right) = parse_coordinates_from_line(&line);
    Command {
        command_type: command_type,
        bottom_left: bottom_left,
        top_right: top_right,
    }
}

fn execute_command(command: &Command, lights: &mut [[u64; 1000]; 1000]) {
    let (x1, y1) = command.bottom_left;
    let (x2, y2) = command.top_right;
    for x in x1..x2+1 {
        for y in y1..y2+1 {
            let x = x as usize;
            let y = y as usize;
            match command.command_type {
                CommandType::On => lights[x][y] = 1,
                CommandType::Off => lights[x][y] = 0,
                CommandType::Toggle => lights[x][y] = 1 - lights[x][y],
            }
        }
    }
}

fn count_total_brightness(lights: &[[u64; 1000]; 1000]) -> u64 {
    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            result += lights[x][y]
        }
    }
    result
}

fn execute_command_2(command: &Command, lights: &mut [[u64; 1000]; 1000]) {
    let (x1, y1) = command.bottom_left;
    let (x2, y2) = command.top_right;
    for x in x1..x2+1 {
        for y in y1..y2+1 {
            let x = x as usize;
            let y = y as usize;
            match command.command_type {
                CommandType::On => lights[x][y] += 1,
                CommandType::Off => lights[x][y] = if lights[x][y] == 0 {
                    0
                } else {
                    lights[x][y] - 1
                },
                CommandType::Toggle => lights[x][y] += 2,
            }
        }
    }
}
fn main() -> Result<()> {
    let file = File::open("src/6.txt")?;
    let mut lights = [[0; 1000]; 1000];

    let mut commands = Vec::new();
    println!("Reading file and parsing commands");
    for line in BufReader::new(file).lines() {
        commands.push(parse_command_from_line(&line?));
    }

    println!("Executing commands");
    for command in commands.iter() {
        execute_command(command, &mut lights);
    }

    println!("Counting lights that are on"); 
    println!("{}", count_total_brightness(&lights));

    println!("Executing commands, round 2");
    lights = [[0; 1000]; 1000]; // don't forget to reset the array
    for command in commands.iter() {
        execute_command_2(command, &mut lights);
    }

    println!("Counting total brightness");
    println!("{}", count_total_brightness(&lights));

    Ok(())
}