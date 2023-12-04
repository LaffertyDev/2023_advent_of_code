use std::fs;
use crate::problems::day4::scratchoff::Game;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let game = Game::parse_input(&contents);
    println!("Part 1: {}", game.score_game());
}
