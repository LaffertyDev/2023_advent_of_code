use crate::problems::day7::camel_hand::Hand;

pub struct CamelGame {
    hands: Vec<Hand>,
    jokers_wild: bool
}

impl CamelGame {
    pub fn parse(input: &str, jokers_wild: bool) -> CamelGame {
        CamelGame {
            hands: input.lines().filter(|l| !l.is_empty()).map(|l| Hand::parse_input(l, jokers_wild)).collect(),
            jokers_wild
        }
    }

    pub fn compute_game_score(&self) -> u64 {
        let mut hands = self.hands.clone();
        hands.sort(); // higher value hands end up at the bottom of the array

        hands.iter().enumerate().map(|(idx, hand)| hand.bid * ((idx as u64) + 1)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day7::camel_game::CamelGame;

    #[test]
    fn test_input_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let game = CamelGame::parse(input, false);

        assert_eq!(6440, game.compute_game_score());
    }

    #[test]
    fn test_input_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let game = CamelGame::parse(input, true);
        assert_eq!(5905, game.compute_game_score());
    }
}