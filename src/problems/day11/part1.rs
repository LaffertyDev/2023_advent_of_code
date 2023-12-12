use std::fs;
use crate::problems::day11::galaxy::Universe;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let universe = Universe::parse(&contents);
	println!("Part 1: {}", universe.find_distance_between_pairs(2));
}