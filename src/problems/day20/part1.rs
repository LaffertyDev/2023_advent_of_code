use std::fs;
use crate::problems::day20::machine_initializer::MachineInitializer;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let mut machine_initializer = MachineInitializer::parse(&contents).unwrap();
	println!("Part 1: {}", machine_initializer.count_pulses(1000));
}