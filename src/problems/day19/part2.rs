use std::fs;
use crate::problems::day19::rules_engine::RulesEngine;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let engine = RulesEngine::parse(&contents).unwrap();
    println!("Part 1: {}", engine.apply_rules_for_ranges_and_count_uniques(1, 4000));
}