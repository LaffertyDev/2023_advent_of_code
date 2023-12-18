use std::fs;
use crate::problems::day17::factory_city::FactoryCity;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let factory = FactoryCity::parse(&contents);
	println!("Part 1: {}", factory.compute_lowest_heat_loss(1, 3));
}