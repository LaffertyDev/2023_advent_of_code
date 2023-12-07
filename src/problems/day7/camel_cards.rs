use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CamelCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl CamelCard {
    pub fn value(&self) -> u64 {
        match self {
            CamelCard::Ace => 14,
            CamelCard::King => 13,
            CamelCard::Queen => 12,
            CamelCard::Jack => 11,
            CamelCard::Ten => 10,
            CamelCard::Nine => 9,
            CamelCard::Eight => 8,
            CamelCard::Seven => 7,
            CamelCard::Six => 6,
            CamelCard::Five => 5,
            CamelCard::Four => 4,
            CamelCard::Three => 3,
            CamelCard::Two => 2,
        }
    }
    pub fn from_char(c: char) -> CamelCard {
        match c {
            'A' => CamelCard::Ace,
            'K' => CamelCard::King,
            'Q' => CamelCard::Queen,
            'J' => CamelCard::Jack,
            'T' => CamelCard::Ten,
            '9' => CamelCard::Nine,
            '8' => CamelCard::Eight,
            '7' => CamelCard::Seven,
            '6' => CamelCard::Six,
            '5' => CamelCard::Five,
            '4' => CamelCard::Four,
            '3' => CamelCard::Three,
            '2' => CamelCard::Two,
            unknown => panic!("Unknown input for card: {}", unknown)
        }
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day7::camel_cards::CamelCard;

    #[test]
    fn orders_correctly() {
        assert!(CamelCard::Ace == CamelCard::Ace);
        assert!(CamelCard::King < CamelCard::Ace);
        assert!(CamelCard::Ace > CamelCard::King);
    }
}