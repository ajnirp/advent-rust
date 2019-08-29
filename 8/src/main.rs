use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn count_difference_1(s: &str) -> u32 {
    let vec = s.chars().collect::<Vec<char>>();
    let result_1 = vec.len();
    let mut result_2 = vec.len() - 2;
    let mut counter = 0;
    while counter < vec.len()-2 {
        if vec[counter] == '\\' {
            if vec[counter+1] == '\\' || vec[counter+1] == '"' {
                result_2 -= 1;
                counter += 2
            } else if vec[counter+1] == 'x' {
                result_2 -= 3;
                counter += 3;
            }
        } else {
            counter += 1;
        }
    }
    result_1 as u32 - result_2 as u32
}

fn count_difference_2(s: &str) -> u32 {
    let mut result = 2; // this is the minimal increase
    // well technically the minimal increase is 4, but then
    // we'd have to skip the first and last char while iterating
    for c in s.chars() {
        if c == '\\' || c == '"' {
            result += 1
        }
    }
    result
}

fn main() -> Result<()> {
    let file = File::open("src/8.txt")?;
    let mut total_difference_1 = 0;
    let mut total_difference_2 = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        total_difference_1 += count_difference_1(&line.clone());
        total_difference_2 += count_difference_2(&line);
    }
    println!("{} {}", total_difference_1, total_difference_2);
    Ok(())
}