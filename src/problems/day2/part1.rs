use std::fs;
use crate::problems::day2::game::Game;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let red_cube_limit = 12;
    let green_cube_limit = 13;
    let blue_cube_limit = 14;

    let mut result = 0;
    for line in contents.lines().filter(|l| !l.is_empty()) {
        let game = Game::parse_game(line);
        let mut game_valid = true;
        for round in game.rounds {
            if round.blue_count > blue_cube_limit || round.red_count > red_cube_limit || round.green_count > green_cube_limit {
                // invalid
                game_valid = false;
                break;
            }
        }

        if game_valid {
            result += game.identifier;
        }
    }
    println!("Part 1: {}", result);
}
