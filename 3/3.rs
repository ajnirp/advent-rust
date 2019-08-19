use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("3.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => (),
    }

    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut seen = HashSet::<(i32, i32)>::new();
    seen.insert((0, 0));
    for c in s.chars() {
        match c {
            '^' => current_y += 1,
            'v' => current_y -= 1,
            '<' => current_x -= 1,
            '>' => current_x += 1,
            _ => (),
        }
        if !seen.contains(&(current_x, current_y)) {
            seen.insert((current_x, current_y));
        }
    }
    println!("{}", seen.len());

    current_x = 0;
    current_y = 0;
    let mut current_robo_x = 0;
    let mut current_robo_y = 0;
    seen.clear();
    seen.insert((0, 0));
    let mut santa_moves = true;
    for c in s.chars() {
        match c {
            '^' => if santa_moves {
                current_y += 1
            } else {
                current_robo_y += 1
            },
            'v' => if santa_moves {
                current_y -= 1
            } else {
                current_robo_y -= 1
            },
            '<' => if santa_moves {
                current_x -= 1
            } else {
                current_robo_x -= 1
            },
            '>' => if santa_moves {
                current_x += 1
            } else {
                current_robo_x += 1
            },
            _ => (),
        }
        if !seen.contains(&(current_x, current_y)) {
            seen.insert((current_x, current_y));
        }
        if !seen.contains(&(current_robo_x, current_robo_y)) {
            seen.insert((current_robo_x, current_robo_y));
        }
        santa_moves = !santa_moves;
    }
    println!("{}", seen.len());
}