pub struct Game {
    pub identifier: usize,
    pub rounds: Vec<Round>
}

impl Game {
    pub fn parse_game(lines: &str) -> Game {
        let mut game_iter = lines.split(":");
        let game_part = game_iter.next().unwrap();
        let rounds_part = game_iter.next().unwrap();

        Game {
            identifier: game_part.chars().skip(5).collect::<String>().parse::<usize>().unwrap(),
            rounds: rounds_part.split(';').map(|r| Round::parse_round(r)).collect::<Vec<Round>>()
        }
    }

}

pub struct Round {
    pub blue_count: usize,
    pub green_count: usize,
    pub red_count: usize
}

impl Round {
    pub fn parse_round(round_def: &str) -> Round {
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
}


#[cfg(test)]
mod tests {
    use crate::problems::day2::game::{Game};

    #[test]
    fn input_parses() {
        let game = Game::parse_game("Game 1: 1 green, 6 red, 4 blue");
        assert_eq!(1, game.identifier);
        assert_eq!(1, game.rounds.len());
        assert_eq!(1, game.rounds[0].green_count);
        assert_eq!(6, game.rounds[0].red_count);
        assert_eq!(4, game.rounds[0].blue_count);
    }

    #[test]
    fn input_parse_compexs() {
        let game = Game::parse_game("Game 1: 1 green, 6 red, 4 blue; 2 blue, 6 green, 7 red; 3 red, 4 blue, 6 green; 3 green; 3 blue, 2 green, 1 red");
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