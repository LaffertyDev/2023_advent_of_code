use std::fs;
use crate::problems::day8::camel_map::CamelMap;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let camel_map = CamelMap::parse(&contents);
	println!("Part 1: {}", camel_map.get_steps_to_zzz());
}