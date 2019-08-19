use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Doing this the hard way, without regular expressions.
// This leads to some interesting algorithms and fun with the borrow checker.

fn at_least_three_vowels(s: &str) -> bool {
    let mut num_vowels = 0;
    for c in s.chars() {
        if "aeiou".contains(c) {
            num_vowels += 1;
        }
    }
    num_vowels >= 3
}

fn repeated_letter(s: &str) -> bool {
    let mut last_char = '\0';
    for c in s.chars() {
        if c == last_char {
            return true;
        }
        last_char = c;
    }
    false
}

// "abc" => ["ab, bc"]
fn into_pairs(s: &str) -> Vec<String> {
    let mut result = Vec::with_capacity(s.len() - 1);
    let mut last_char = '\0';
    for c in s.chars() {
        if last_char != '\0' {
            let mut curr_str = String::with_capacity(2);
            curr_str.push(last_char);
            curr_str.push(c);
            result.push(curr_str);
        }
        last_char = c;
    }
    result
}

fn no_forbidden_pair(s: &str) -> bool {
    !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

fn is_nice(s: &str) -> bool {
    at_least_three_vowels(s) && repeated_letter(s) && no_forbidden_pair(s)
}

fn has_repeated_pair(pairs: &Vec<String>) -> bool {
    let mut seen_pairs = HashMap::<String, i32>::new();
    for (idx, pair) in pairs.iter().enumerate() {
        let idx = idx as i32;
        let pair_opt = seen_pairs.get(pair).copied(); // Option<&i32> -> Option<i32>
        match pair_opt {
            Some(other_idx) => {
                if idx - other_idx > 1 {
                    return true;
                }
            },
            None => {
                seen_pairs.insert(pair.to_string(), idx);
            }
        }
    }
    false
}

fn has_aba_pattern(pairs: &Vec<String>) -> bool {
    let mut last_pair = "";
    for pair in pairs.iter() {
        if last_pair != "" && last_pair.chars().rev().collect::<String>() == *pair {
            return true;
        }
        last_pair = pair;
    }
    false
}

fn is_nice_2(s: &str) -> bool {
    let pairs = into_pairs(s);
    has_repeated_pair(&pairs) && has_aba_pattern(&pairs)
}

fn main() -> Result<()> {
    let file = File::open("5.txt")?;
    let mut num_nice_strings = 0i32;
    let mut num_nice_strings_2 = 0i32;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if is_nice(&line) {
            num_nice_strings += 1;
        }
        if is_nice_2(&line) {
            num_nice_strings_2 += 1;
        }
    }
    println!("{} {}", num_nice_strings, num_nice_strings_2);
    Ok(())
}