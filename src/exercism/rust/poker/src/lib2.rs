#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: u16,
    pub suit: char,
}

impl Card {
    pub fn new(s: &str) -> Self {
        let (rank, suit) = s.split_at(s.len() - 1);

        let rank = match rank.parse::<u16>() {
            Ok(n) => n,
            _ => "JQKA".find(&rank).unwrap() as u16 + 11,
        };
        let suit = suit.chars().nth(0).unwrap();

        Self { rank, suit }
    }
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    rank: Vec<u16>,
}

fn sort_cards(mut cards: Vec<Card>) -> Vec<Card> {
    // sort cards by rank descending and suit ascending (alphabetically)
    cards.sort_by(|a, b| a.suit.partial_cmp(&b.suit).unwrap());
    cards.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    cards
}

impl Hand {
    pub fn new(s: &str) -> Self {
        let mut cards = s
            .split_whitespace()
            .map(|x| Card::new(x))
            .collect::<Vec<Card>>();
        cards = sort_cards(cards);
        let rank = vec![];
        Self { cards, rank }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands.iter().map(|x| Hand::new(x)).collect::<Vec<Hand>>();
    // hands.sort_by(|a, b| b.partial_cmp(&a).unwrap());

    println!("{hands:?}");
    todo!("Out of {hands:?}, which hand wins?")
}
