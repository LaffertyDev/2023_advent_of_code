use std::fs;
use crate::problems::day23::scenery::ScenicPark;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let scenic_park = ScenicPark::parse(&contents, false);
	println!("Part 1: {}", scenic_park.find_most_scenic_route());
}