use std::fs;
use crate::problems::day15::hash_initializer::determine_hash_sum;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let hash = determine_hash_sum(&contents);
	println!("Part 1: {}", hash);
}