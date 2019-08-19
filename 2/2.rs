use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// Sort a vector of exactly 3 elements
fn sort_vec3(vec: &mut [i32]) {
    assert!(vec.len() == 3);
    if vec[0] > vec[1] {
        vec.swap(0, 1);
    }
    if vec[1] > vec[2] {
        vec.swap(1, 2);
    }
    if vec[0] > vec[1] {
        vec.swap(0, 1);
    }
}

// Compute wrapping paper, assuming the vec is sorted
fn compute_wrapping_paper_for(vec: &[i32]) -> i32 {
    3*vec[0]*vec[1] + 2*vec[1]*vec[2] + 2*vec[2]*vec[0]
}

// Compute ribbon length, assuming the vec is sorted
fn compute_ribbon_length_for(vec: &[i32]) -> i32 {
    vec[0]*vec[1]*vec[2] + 2*(vec[0] + vec[1])
}

fn main() -> Result<()> {
    let file = File::open("2.txt")?;
    let mut paper_needed = 0;
    let mut ribbon_needed = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut dimensions: Vec<i32> = line.split("x").map(|s| s.parse::<i32>().unwrap()).collect();
        sort_vec3(&mut dimensions);
        paper_needed += compute_wrapping_paper_for(&dimensions);
        ribbon_needed += compute_ribbon_length_for(&dimensions);
    }
    println!("{} {}", paper_needed, ribbon_needed);
    Ok(())
}