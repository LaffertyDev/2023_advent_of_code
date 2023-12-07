use std::fs;
use crate::problems::day6::race::Race;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let race = Race::parse(&contents);
    println!("Part 1: {}", race.get_number_of_winning_moves());
}
