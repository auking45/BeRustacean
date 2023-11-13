use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: u32,
    pub suit: char,
}

impl Card {
    pub fn new(s: &str) -> Self {
        let (rank, suit) = s.split_at(s.len() - 1);
        let rank = match rank.parse::<u32>() {
            Ok(x) => x,
            _ => "JQKA".find(&rank).unwrap() as u32 + 11,
        };
        let suit = suit.chars().nth(0).unwrap();
        Card { rank, suit }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum Rank {
    // in ascending order
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,  // five in sequence
    Flush,     // five in one suit
    FullHouse, // three of one rank and two of another
    FourOfAKind,
    StraightFlush, // five in sequence in one suit
}
#[derive(Debug)]
pub struct Hand<'a> {
    pub raw: &'a str,
    pub cards: Vec<Card>,
    rank: Vec<u32>,
}
fn count_card_ranks(cards: &Vec<Card>) -> Vec<(u32, usize)> {
    let values = cards.iter().map(|c| c.rank).collect::<Vec<u32>>();
    let mut result: Vec<_> = values
        .iter()
        .map(|&x| (x, values.iter().filter(|&&y| y == x).count()))
        .collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    result
}
fn sort_cards(mut cards: Vec<Card>) -> Vec<Card> {
    // sort cards by rank descending and suit ascending (alphabetically)
    // e.g. 10H, 8C, 8D, 8H, 8S, 7C
    cards.sort_by(|a, b| a.suit.partial_cmp(&b.suit).unwrap());
    cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    cards
}
impl<'a> Hand<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut cards: Vec<Card> = s
            .split_whitespace()
            .map(|x| Card::new(x))
            .collect::<Vec<Card>>();
        cards = sort_cards(cards);
        if cards.iter().map(|c| c.rank).collect::<Vec<u32>>() == vec![14u32, 5, 4, 3, 2] {
            cards[0].rank = 1;
            cards = sort_cards(cards);
        }
        let is_straight = cards.windows(2).all(|x| x[0].rank == x[1].rank + 1);
        let is_flush = cards.windows(2).all(|x| x[0].suit == x[1].suit);
        let card_rank_count = count_card_ranks(&cards);
        let rank = if is_straight && is_flush {
            Rank::StraightFlush
        } else if card_rank_count[0].1 == 4 {
            Rank::FourOfAKind
        } else if card_rank_count[0].1 == 3 && card_rank_count[3].1 == 2 {
            Rank::FullHouse
        } else if is_flush {
            Rank::Flush
        } else if is_straight {
            Rank::Straight
        } else if card_rank_count[0].1 == 3 {
            Rank::ThreeOfAKind
        } else if card_rank_count[0].1 == 2 && card_rank_count[2].1 == 2 {
            Rank::TwoPair
        } else if card_rank_count[0].1 == 2 {
            Rank::OnePair
        } else {
            Rank::HighCard
        };
        let mut rank: Vec<u32> = vec![rank as u32];
        rank.extend(card_rank_count.iter().map(|x| x.0));
        Hand {
            raw: s,
            cards,
            rank,
        }
    }
}
impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (&a, &b) in self.rank.iter().zip(other.rank.iter()) {
            if a != b {
                return a.partial_cmp(&b);
            }
        }
        Some(Ordering::Equal)
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
// pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
//     todo!("Out of {hands:?}, which hand wins?")
// }

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands.iter().map(|x| Hand::new(x)).collect::<Vec<_>>();
    hands.sort_by(|a, b| b.partial_cmp(&a).unwrap());
    hands
        .iter()
        .filter(|x| **x == hands[0])
        .map(|x| x.raw)
        .collect()
}