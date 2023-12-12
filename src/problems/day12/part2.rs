use std::fs;
use crate::problems::day12::springy::{SpringCollection};

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let spring_collection = SpringCollection::parse(&contents, 5);
    println!("Part 2: {}", spring_collection.get_possible_permutations());
}