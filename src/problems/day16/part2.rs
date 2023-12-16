use std::fs;
use crate::problems::day16::lava_factory::LavaFactory;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let factory = LavaFactory::parse(&contents);
    println!("Part 2: {}", factory.unwrap().compute_maximum_energy());
}