use std::collections::HashMap;

pub struct Game {
    cards: Vec<Scratchoff>
}

impl Game {
    pub fn parse_input(contents: &String) -> Game {
        let mut score_cards = vec![];
        for line in contents.lines().filter(|l| !l.is_empty()) {
            let mut line_iter = line.split_whitespace();
            line_iter.next(); // `card`
            // `1:`
            let number_def = line_iter.next().unwrap();
            let card_number = number_def[0..number_def.len() - 1].parse::<u32>().unwrap();

            let mut split_found = false;
            let mut winning_numbers = vec![];
            let mut numbers_present = vec![];
            while let Some(index) = line_iter.next() {
                if index == "|" {
                    // now move to matching solution
                    split_found = true;
                } else {
                    if split_found {
                        numbers_present.push(index.parse::<u32>().unwrap());
                    } else {
                        winning_numbers.push(index.parse::<u32>().unwrap());
                    }
                }
            }

            score_cards.push(Scratchoff {
                card_number: card_number,
                winning_numbers: winning_numbers,
                numbers_present: numbers_present
            })
        }

        Game {
            cards: score_cards
        }
    }

    pub fn score_game(&self) -> u32 {
        self.cards.iter().map(|cards| cards.find_score()).sum()
    }

    pub fn compute_total_scorecards_after_winning(&self) -> u32 {
        let mut scratch_off_card_counts: HashMap<u32, u32> = HashMap::new(); // card_index as key, value is count of cards
        for card in self.cards.iter() {
            let pairs_in_card = card.count_matches();
            let current_extra = scratch_off_card_counts.entry(card.card_number)
                    .and_modify(|current| *current = *current + 1)
                    .or_insert(1)
                    .clone();
            if pairs_in_card > 0 {
                for duplicates_to_set in card.card_number+1..=(card.card_number + pairs_in_card) {
                    let child_extras = scratch_off_card_counts.entry(duplicates_to_set).or_insert(0);
                    *child_extras = *child_extras + current_extra;
                }
            }
        }

        scratch_off_card_counts.values().sum()
    }
}

pub struct Scratchoff {
    card_number: u32,
    winning_numbers: Vec<u32>,
    numbers_present: Vec<u32>
}

impl Scratchoff {
    pub fn find_score(&self) -> u32 {
        let count_of_winning_numbers = self.count_matches();
        if count_of_winning_numbers == 0 {
            return 0;
        }

        // 0,1,2,4,8..
        return 1 << (count_of_winning_numbers - 1)
    }

    pub fn count_matches(&self) -> u32 {
        let mut count_of_winning_numbers = 0;
        for number_present in self.numbers_present.iter() {
            if self.winning_numbers.contains(&number_present) {
                count_of_winning_numbers += 1;
            }
        }

        count_of_winning_numbers
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day4::scratchoff::Game;

    #[test]
    fn test_input_part1() {
        let input: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into();
        let game = Game::parse_input(&input);
        assert_eq!(13, game.score_game())
    }

    #[test]
    fn test_input_part2() {
        let input: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into();
        let game = Game::parse_input(&input);
        assert_eq!(30, game.compute_total_scorecards_after_winning())
    }
}