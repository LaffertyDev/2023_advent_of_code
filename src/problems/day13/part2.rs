use std::fs;
use crate::problems::day13::mirror::Observation;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let observations = Observation::parse(&contents);
    println!("Part 2: {}", observations.find_mirror_values(1));
}