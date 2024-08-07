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
    pub pot: i64,
    pub player_stacks: HashMap<usize, i64>,
    pub community_cards: Vec<Card>,
    pub player_cards: HashMap<usize, (Card, Card)>,
}

pub struct PokerEngine<'a> {
    state: PokerState,
    deck: Deck,
    players: &'a Vec<&'a dyn GamePlayer<PokerState, PokerAction>>,
}

impl PokerEngine<'_> {
    pub fn new<'a>(
        players: &'a Vec<&'a dyn GamePlayer<PokerState, PokerAction>>,
    ) -> PokerEngine<'a> {
        let mut new_engine = PokerEngine {
            state: PokerState {
                pot: 0,
                player_stacks: HashMap::new(),
                community_cards: vec![],
                player_cards: HashMap::new(),
            },
            deck: Deck::new(),
            players: players,
        };

        // Players start with 200 in chips
        for i in 0..players.len() {
            new_engine.state.player_stacks.insert(i, 200);
        }
        new_engine
    }
}

impl GameEngine<PokerState, PokerAction> for PokerEngine<'_> {
    fn run(&mut self) {
        // Start with 5 rounds.
        for _round_num in 0..5 {
            self.state.community_cards = vec![];
            // Shuffle Deck
            self.deck.shuffle();
            // Deal cards
            for player_id in 0..self.players.len() {
                self.state
                    .player_cards
                    .insert(player_id, (self.deck.get_next(), self.deck.get_next()));
            }

            // Get preflop action
            for _ in 0..3 {
                self.state.community_cards.push(self.deck.get_next());
            }
            // Get flop action
            self.state.community_cards.push(self.deck.get_next());
            // Get turn action
            self.state.community_cards.push(self.deck.get_next());
            // Get river action
        }
    }
}

pub struct PassivePokerPlayer {}

impl GamePlayer<PokerState, PokerAction> for PassivePokerPlayer {
    fn get_action(&self, _: PokerState) -> PokerAction {
        PokerAction::Call
    }
}

pub struct PlayerPokerPlayer {}
