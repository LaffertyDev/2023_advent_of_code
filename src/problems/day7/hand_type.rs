use std::cmp::Ordering;
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

        let key_count = hashed_hand.keys().count();
        let max_count = hashed_hand.values().max().unwrap();
        match (key_count, max_count) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            (5, 1) => HandType::HighCard,
            _ => panic!("Unknown Hand Type")
        }
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