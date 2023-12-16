use std::fs;
use crate::problems::day15::hashmap_box::HashmapBox;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let hash = HashmapBox::execute(&contents);
    println!("Part 2: {}", hash);
}