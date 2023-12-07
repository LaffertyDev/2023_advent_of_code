use std::cmp::Ordering;
use crate::problems::day7::camel_cards::CamelCard;
use crate::problems::day7::hand_type::HandType;

#[derive(Clone, Copy, Debug)]
pub struct Hand {
    pub cards: [CamelCard; 5],
    pub parsed_hand_type: HandType,
    pub bid: u64
}

impl Hand {
    pub fn parse_input(hand: &str) -> Hand {
        let mut lines = hand.split(' ');
        let mut cards = lines.next().unwrap().chars().map(|c| CamelCard::from_char(c));
        let cards = [cards.next().unwrap(), cards.next().unwrap(), cards.next().unwrap(), cards.next().unwrap(), cards.next().unwrap()];
        let hand_type = HandType::parse_type(cards);
        Hand {
            cards: cards,
            parsed_hand_type: hand_type,
            bid: lines.next().unwrap().parse().unwrap()
        }
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.parsed_hand_type.cmp(&other.parsed_hand_type) {
            Ordering::Equal => {
                // now check the ordering
                for index in 0..5 {
                    match self.cards[index].cmp(&other.cards[index]) {
                        Ordering::Equal => {
                            // continue checking
                        },
                        card_order => {
                            return card_order;
                        }
                    }
                }

                // its actually the same, so order doesn't matter
                // I don't think that happens ever
                Ordering::Equal
            },
            hand_type_order => hand_type_order
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}


#[cfg(test)]
mod tests {
    use crate::problems::day7::camel_hand::Hand;

    #[test]
    fn orders_correctly() {
        assert!(Hand::parse_input("AAAAA 1") == Hand::parse_input("AAAAA 1"));
        assert!(Hand::parse_input("AAAAJ 1") < Hand::parse_input("AAAAA 1"));
        assert!(Hand::parse_input("AAAAA 1") > Hand::parse_input("AAAAJ 1"));
        assert!(Hand::parse_input("AAAAA 1") > Hand::parse_input("AAAJJ 1"));
    }
}