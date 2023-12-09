use std::cmp::{Ordering};
use std::collections::HashMap;
use crate::problems::day7::camel_cards::CamelCard;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    pub fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    pub fn parse_type(cards: [CamelCard; 5]) -> HandType {
        let mut hashed_hand: HashMap<CamelCard, usize> = HashMap::new();
        for card in cards {
            hashed_hand.entry(card).and_modify(|count| *count = *count + 1).or_insert(1);
        }

        let key_count = hashed_hand.keys().filter(|c| *c != &CamelCard::Joker).count();
        let max_count = hashed_hand
            .iter()
            .filter(|(k, _v)| *k != &CamelCard::Joker)
            .map(|(_k, v)| v)
            .max()
            .unwrap_or(&0);
        let joker_count = hashed_hand
            .iter()
            .filter(|(k, _v)| *k == &CamelCard::Joker)
            .map(|(_k, v)| v)
            .sum();

        if max_count + joker_count >= 5 {
            // 4 joker -- would always make 5 pair, ignore
            return HandType::FiveOfAKind;
        }

        if max_count + joker_count == 4 {
            // 3 joker -- would always make 4 pair, ignore
            return HandType::FourOfAKind;
        }

        // complicated cases, just brute force it
        // match (key_count, max_count) {
        //     key_count == 1 && max_count >= 5 =>
        //     (1, max_count > 5) => HandType::FiveOfAKind,
        //     (2, 4) => HandType::FourOfAKind,
        //     (2, 3) => HandType::FullHouse,
        //     (3, 3) => HandType::ThreeOfAKind,
        //     (3, 2) => HandType::TwoPair,
        //     (4, 2) => HandType::OnePair,
        //     (5, 1) => HandType::HighCard,
        //     _ => panic!("Unknown Hand Type")
        // }
        return match (key_count, max_count, joker_count) {
            (2, 2, 1) => HandType::FullHouse, // J9988
            (2, 3, 0) => HandType::FullHouse,
            (2, 3, 1) => panic!(), // impossible, would be a four-kind
            (3, 1, 2) => HandType::ThreeOfAKind, // JJ987
            (3, 3, 0) => HandType::ThreeOfAKind,
            (3, 3, 1) => panic!(), // impossible, would be a four-kind
            (3, 2, 0) => HandType::TwoPair, // 99887
            (3, 2, 1) => HandType::ThreeOfAKind, // J9987
            (4, 2, 0) => HandType::OnePair, // 99876
            (4, 2, 1) => HandType::ThreeOfAKind, // J9987
            (4, 1, 1) => HandType::OnePair, // J9876
            (5, 1, 0) => HandType::HighCard,
            _ => panic!("{}, {}, {}", key_count, max_count, joker_count)
        };
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day7::camel_cards::CamelCard;
    use crate::problems::day7::hand_type::HandType;

    #[test]
    fn parses_correctly() {
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Ace, CamelCard::Ace, CamelCard::Ace, CamelCard::Ace, CamelCard::Ace]));
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Joker, CamelCard::Joker, CamelCard::Joker, CamelCard::Joker, CamelCard::Joker]));
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Ace, CamelCard::Joker, CamelCard::Joker, CamelCard::Joker, CamelCard::Joker]));
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Ace, CamelCard::Ace, CamelCard::Joker, CamelCard::Joker, CamelCard::Joker]));
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Ace, CamelCard::Ace, CamelCard::Ace, CamelCard::Joker, CamelCard::Joker]));
        assert_eq!(HandType::FiveOfAKind, HandType::parse_type([CamelCard::Ace, CamelCard::Ace, CamelCard::Ace, CamelCard::Ace, CamelCard::Joker]));

    }
}