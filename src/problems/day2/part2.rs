use std::fs;
use crate::problems::day2::game::Game;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
    let mut result = 0;
    for line in contents.split("\n").filter(|l| !l.is_empty()) {
        let game = Game::parse_game(line);

        let mut green_max = 0;
        let mut red_max = 0;
        let mut blue_max = 0;
        for round in game.rounds {
            green_max = std::cmp::max(round.green_count, green_max);
            blue_max = std::cmp::max(round.blue_count, blue_max);
            red_max = std::cmp::max(round.red_count, red_max);
        }

        let power = green_max * red_max * blue_max;
        result += power;
    }
    println!("Part 2: {:}", result);
}
