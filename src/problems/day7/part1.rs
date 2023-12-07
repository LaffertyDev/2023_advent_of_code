use std::fs;
use crate::problems::day7::camel_game::CamelGame;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let camel_game = CamelGame::parse(&contents);
	println!("Part 1: {}", camel_game.compute_game_score());
}