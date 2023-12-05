use std::fs;
use crate::problems::day5::almanac::Almanac;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let almanac = Almanac::parse_input(&contents);
    println!("Part 1: {}", almanac.get_lowest_seed_location());
}
