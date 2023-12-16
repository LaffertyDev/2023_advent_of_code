use std::fs;
use crate::problems::day14::mirror_platform::{Platform, TiltDirection};

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let mut platform = Platform::parse(&contents).unwrap();
    platform.tilt(&TiltDirection::Up);
	println!("Part 1: {}", platform.compute_load());
}