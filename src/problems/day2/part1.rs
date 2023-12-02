use std::fs;

pub fn execute(input_path: &std::path::PathBuf) {
    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let red_cube_limit = 12;
    let green_cube_limit = 13;
    let blue_cube_limit = 14;

    let mut result = 0;
    for line in contents.split("\n").filter(|l| !l.is_empty()) {
        let game = ParseInput(line);
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
    println!("Part 1: {:}", result);
}

fn ParseInput(lines: &str) -> Game {
    let mut game_iter = lines.split(":");
    let game_part = game_iter.next().unwrap();
    let rounds_part = game_iter.next().unwrap();

    Game {
        identifier: game_part.chars().skip(5).collect::<String>().parse::<usize>().unwrap(),
        rounds: rounds_part.split(';').map(|r| ParseRound(r)).collect::<Vec<Round>>()
    }
}

fn ParseRound(round_def: &str) -> Round {
    let mut red_cubes = 0usize;
    let mut blue_cubes = 0usize;
    let mut green_cubes = 0usize;
    for entry in round_def.split(',') {
        if entry.ends_with("green") {
            green_cubes = entry.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        } else if entry.ends_with("red") {
            red_cubes = entry.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        } else if entry.ends_with("blue") {
            blue_cubes = entry.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        } else {
            panic!();
        }
    }

    Round {
        red_count: red_cubes,
        blue_count: blue_cubes,
        green_count: green_cubes
    }
}

struct Game {
    identifier: usize,
    rounds: Vec<Round>
}

struct Round {
    blue_count: usize,
    green_count: usize,
    red_count: usize
}


#[cfg(test)]
mod tests {
    use crate::problems::day2::part1::ParseInput;

    #[test]
    fn input_parses() {
        let game = ParseInput("Game 1: 1 green, 6 red, 4 blue");
        assert_eq!(1, game.identifier);
        assert_eq!(1, game.rounds.len());
        assert_eq!(1, game.rounds[0].green_count);
        assert_eq!(6, game.rounds[0].red_count);
        assert_eq!(4, game.rounds[0].blue_count);
    }

    #[test]
    fn input_parse_compexs() {
        let game = ParseInput("Game 1: 1 green, 6 red, 4 blue; 2 blue, 6 green, 7 red; 3 red, 4 blue, 6 green; 3 green; 3 blue, 2 green, 1 red");
        assert_eq!(1, game.identifier);
        assert_eq!(5, game.rounds.len());
        assert_eq!(1, game.rounds[0].green_count);
        assert_eq!(6, game.rounds[0].red_count);
        assert_eq!(4, game.rounds[0].blue_count);
        assert_eq!(6, game.rounds[1].green_count);
        assert_eq!(7, game.rounds[1].red_count);
        assert_eq!(2, game.rounds[1].blue_count);
        assert_eq!(6, game.rounds[2].green_count);
        assert_eq!(3, game.rounds[2].red_count);
        assert_eq!(4, game.rounds[2].blue_count);
        assert_eq!(3, game.rounds[3].green_count);
        assert_eq!(0, game.rounds[3].red_count);
        assert_eq!(0, game.rounds[3].blue_count);
        assert_eq!(2, game.rounds[4].green_count);
        assert_eq!(1, game.rounds[4].red_count);
        assert_eq!(3, game.rounds[4].blue_count);
    }
}