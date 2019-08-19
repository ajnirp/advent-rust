extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Note - I had already done this puzzle before, so my solution assumes knowledge of
// both parts of the question right from the start.

type Coord = (u8, u8);

enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Command {
    command_type: CommandType,
    bottom_left: Coord,
    top_right: Coord,
}

fn parse_coordinates_from_line(line: &str) -> (Coord, Coord) {
    ((0, 0), (0, 0))
}

fn parse_command_from_line(line: &str) -> Command {
    let line_vec = line.chars().collect::<Vec<char>>();
    // Observation - the 7th character disambiguates the command.
    // Further, once we know the command type, we know the offset of the coordinates.
    let command_type: CommandType;
    let coordinates_offset: usize;
    match line_vec[6] {
        'n' => {
            command_type = CommandType::TurnOn;
            coordinates_offset = 8;
        }
        'f' => {
            command_type = CommandType::TurnOff;
            coordinates_offset = 9;
        }
        _ => {
            command_type = CommandType::Toggle;
            coordinates_offset = 7;
        }
    }
    let (bottom_left, top_right) = parse_coordinates_from_line(&line[coordinates_offset..]);
    Command {
        command_type: command_type,
        bottom_left: bottom_left,
        top_right: top_right,
    }
}

fn execute_command(command: Command, lights: &mut [[u8; 1000]; 1000]) {

}

fn main() -> Result<()> {
    let file = File::open("5.txt")?;
    let mut lights = [[0u8; 1000]; 1000];
    for line in BufReader::new(file).lines() {
        let line = line?;
        let command = parse_command_from_line(&line);
        execute_command(command, &mut lights);
    }
    Ok(())
}