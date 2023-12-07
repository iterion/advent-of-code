use itertools::Itertools;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

pub(crate) fn run() -> (usize, usize) {
    let input_string = get_input_string();
    (answer_part_1(input_string), answer_part_2(input_string))
}

fn answer_part_1(lines: &str) -> usize {
    let hands: Vec<_> = lines.lines().map(HandBid::<Hand>::parse).collect();
    calculate_total_winnings(hands)
}

fn answer_part_2(lines: &str) -> usize {
    let hands: Vec<_> = lines.lines().map(HandBid::<HandV2>::parse).collect();
    calculate_total_winnings(hands)
}

fn calculate_total_winnings<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>>(
    hands: Vec<HandBid<T>>,
) -> usize {
    let mut sorted_hands = hands.clone();
    sorted_hands.sort();
    for hand in &sorted_hands {
        println!("{hand}");
    }
    sorted_hands
        .iter()
        .enumerate()
        .map(|(i, hand_bid)| (i + 1) * hand_bid.bid)
        .sum()
}

fn get_input_string() -> &'static str {
    include_str!("../inputs/day07.txt")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn parse(letter: char) -> Self {
        match letter {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("unexpected card!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = self.get_base_ranking();
        let other_rank = other.get_base_ranking();
        if self_rank < other_rank {
            Ordering::Less
        } else if self_rank > other_rank {
            Ordering::Greater
        } else if let Some((a, b)) = self
            .cards
            .clone()
            .iter()
            .zip(other.cards.clone())
            .find(|(a, b)| a != &b)
        {
            a.cmp(&b)
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BaseRanking for Hand {
    fn get_base_ranking(&self) -> usize {
        let card_counts = self.cards.iter().counts();
        if card_counts.len() == 5 {
            // must be high card
            0
        } else if card_counts.len() == 4 {
            // must be one pair
            1
        } else if card_counts.len() == 3 {
            // could be two pair or three of a kind
            if card_counts.iter().any(|(_c, count)| *count == 3) {
                3
            } else {
                2
            }
        } else if card_counts.len() == 2 {
            // could be full house pair or four of a kind
            if card_counts.iter().any(|(_c, count)| *count == 4) {
                5
            } else {
                4
            }
        } else {
            // must be five of a kind
            6
        }
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandV2 {
    cards: Vec<Card>,
}

impl Ord for HandV2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = self.get_base_ranking();
        let other_rank = other.get_base_ranking();
        if self_rank < other_rank {
            Ordering::Less
        } else if self_rank > other_rank {
            Ordering::Greater
        } else if let Some((a, b)) = self
            .cards
            .clone()
            .iter()
            .zip(other.cards.clone())
            .find(|(a, b)| a != &b)
        {
            // special case jack as joker
            if a == &Card::Jack {
                Ordering::Less
            } else if b == Card::Jack {
                Ordering::Greater
            } else {
                a.cmp(&b)
            }
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for HandV2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BaseRanking for HandV2 {
    fn get_base_ranking(&self) -> usize {
        let card_counts = self.cards.iter().filter(|c| *c != &Card::Jack).counts();
        let jack_counts = self
            .cards
            .iter()
            .filter(|c| *c == &Card::Jack)
            .collect::<Vec<_>>()
            .len();
        let base_rank = card_counts.len();
        if base_rank == 5 {
            0
        } else if base_rank == 4 {
            // must be one pair
            1
        } else if base_rank == 3 {
            // could be two pair or three of a kind
            if jack_counts > 0 || card_counts.iter().any(|(_c, count)| *count == 3) {
                3
            } else {
                2
            }
        } else if base_rank == 2 {
            // could be full house pair or four of a kind
            if jack_counts > 0 {
                if jack_counts >= 2 || card_counts.iter().any(|(_c, count)| *count == 3) {
                    5
                } else {
                    4
                }
            } else if card_counts.iter().any(|(_c, count)| *count == 4) {
                5
            } else {
                4
            }
        } else {
            // must be five of a kind
            6
        }
    }
}

impl From<Vec<Card>> for HandV2 {
    fn from(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}

trait BaseRanking {
    fn get_base_ranking(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandBid<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>> {
    hand: T,
    bid: usize,
}

impl<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>> Display for HandBid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} - {}",
            self.hand.get_base_ranking(),
            self.hand,
            self.bid
        )
    }
}

impl<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>> Ord for HandBid<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>> PartialOrd for HandBid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Debug + BaseRanking + Clone + From<Vec<Card>>> HandBid<T> {
    fn parse(line: &str) -> HandBid<T> {
        let (cards, bid_str) = line.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(Card::parse).collect();
        Self {
            hand: cards.into(),
            bid: bid_str.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day07::{
        answer_part_1, answer_part_2, calculate_total_winnings, get_input_string, Card, Hand,
        HandBid,
    };
    #[test]
    fn test_all_lines() {
        let lines = get_input_string();

        assert_eq!(answer_part_1(lines), 249726565);
        assert_eq!(answer_part_2(lines), 251135960);
    }

    const SAMPLE_HANDS_STR: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_parse_hands() {
        let hands: Vec<_> = SAMPLE_HANDS_STR
            .lines()
            .map(HandBid::<Hand>::parse)
            .collect();
        assert_eq!(
            hands,
            vec![
                HandBid {
                    hand: Hand {
                        cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
                    },
                    bid: 765
                },
                HandBid {
                    hand: Hand {
                        cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]
                    },
                    bid: 684
                },
                HandBid {
                    hand: Hand {
                        cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]
                    },
                    bid: 28
                },
                HandBid {
                    hand: Hand {
                        cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]
                    },
                    bid: 220
                },
                HandBid {
                    hand: Hand {
                        cards: vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]
                    },
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn test_hand_sort() {
        let h1 = Hand {
            cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
        };
        let h2 = Hand {
            cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
        };

        assert_eq!(h1.cmp(&h2), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_camel_cards_example() {
        let hands: Vec<_> = SAMPLE_HANDS_STR
            .lines()
            .map(HandBid::<Hand>::parse)
            .collect();
        let total_winnings = calculate_total_winnings(hands);
        assert_eq!(total_winnings, 6440);
        assert_eq!(answer_part_2(SAMPLE_HANDS_STR), 5905);
    }
}
