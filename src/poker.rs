use crate::card::{Card, Deck, Rank, Suit};
use crate::game::{GameEngine, GamePlayer};

use std::collections::hash_map::HashMap;

pub enum PokerAction {
    Call,
    Raise(i64),
    Check,
    Fold,
}

pub struct PokerState {
    pot: i64,
    player_stacks: HashMap<i64, i64>,
    player_order: Vec<i64>,
    community_cards: Vec<Card>,
    player_cards: HashMap<i64, (Card, Card)>,
}

pub struct PokerEngine {
    state: PokerState,
    deck: Deck,
}

pub struct RandomPokerPlayer {}
pub struct PlayerPokerPlayer {}
