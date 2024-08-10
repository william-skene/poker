use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club];
        SUITS.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        RANKS.iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} of {:?}s", self.rank, self.suit)
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    current_deal: usize,
}

impl Deck {
    pub fn new() -> Self {
        let mut new_deck = Deck {
            cards: vec![],
            current_deal: 0,
        };
        for rank in Rank::iterator() {
            for suit in Suit::iterator() {
                new_deck.cards.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }
        new_deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self.current_deal = 0;
    }

    pub fn get_next(&mut self) -> Card {
        self.current_deal += 1;
        self.cards[(self.current_deal - 1) % self.cards.len()]
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "[");
        if res.is_err() {
            return res;
        }
        for card in &self.cards[0..self.cards.len() - 1] {
            let res = write!(f, "{}, ", card);
            if res.is_err() {
                return res;
            }
        }
        write!(f, "{}]", self.cards[self.cards.len() - 1])
    }
}
