use std::fs;
use crate::problems::day18::dig_plan::DigPlan;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let plan = DigPlan::parse(&contents).unwrap();
	println!("Part 1: {}", plan.count_dug_depth());
}