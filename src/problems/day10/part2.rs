use std::fs;
use crate::problems::day10::pipe_grid::PipeGrid;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let grid = PipeGrid::parse(&contents);
    println!("Part 2: {}", grid.find_area_enclosed_by_loop());
}