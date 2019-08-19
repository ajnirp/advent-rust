use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("1.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => (),
    }
    let mut level: i32 = 0;
    let mut entered_basement: bool = false; // for the first time
    let mut entered_basement_at: i32 = 0;
    for (idx, c) in s.chars().enumerate() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }
        if level == -1 && !entered_basement {
            entered_basement = true;
            entered_basement_at = (1 + idx) as i32;
        }
    }
    print!("{} {}", level, entered_basement_at);
}