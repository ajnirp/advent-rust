use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn at_least_three_vowels(s: &str) -> bool {
    s.chars().fold(0, |acc, c| if "aeiou".contains(c) { acc + 1 } else { acc }) == 3
}

fn is_nice(s: &str) -> bool {
    at_least_three_vowels(&s)
}

fn main() -> Result<()> {
    let file = File::open("5.txt")?;
    let mut num_nice_strings = 0i32;
    for line in BufReader::new(file).lines() {
        if is_nice(&line?) {
            num_nice_strings += 1;
        }
    }
    println!("{}", num_nice_strings);
    Ok(())
}