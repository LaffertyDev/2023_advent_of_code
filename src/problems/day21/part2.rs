use std::fs;
use crate::problems::day21::garden::Garden;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let garden = Garden::parse(&contents);
    // compute most plausible steps
    println!("Part 2: {}", garden.count_garden_plots_reachable_in_steps(64, true));
}