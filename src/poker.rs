use crate::card::{Card, Deck, Rank, Suit};
use crate::game::{GameEngine, GamePlayer};
use crate::hand_utils::get_hand_value;

use std::collections::hash_map::HashMap;
use std::fmt::Display;

#[derive(Debug)]
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
    pub last_action: HashMap<usize, PokerAction>,
}

impl Display for PokerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (player, stack) in &self.player_stacks {
            let res = write!(
                f,
                "Player {} has {} chips and last performed {:?}\n",
                player,
                stack,
                self.last_action
                    .get(player)
                    .unwrap_or_else(|| &PokerAction::Fold)
            );
            if res.is_err() {
                return res;
            }
        }

        for (player, (card1, card2)) in &self.player_cards {
            let res = write!(f, "Player {} has {} and {}\n", player, card1, card2);
            if res.is_err() {
                return res;
            }
        }

        let res = write!(f, "Community cards: ");
        if res.is_err() {
            return res;
        }
        for card in &self.community_cards[0..self.community_cards.len() - 1] {
            let res = write!(f, "{}, ", card);
            if res.is_err() {
                return res;
            }
        }
        let res = write!(
            f,
            "{}\n",
            self.community_cards[self.community_cards.len() - 1]
        );

        if res.is_err() {
            return res;
        }

        write!(f, "Pot: {}", self.pot)
    }
}

pub struct PokerEngine<'a> {
    state: PokerState,
    deck: Deck,
    players: &'a Vec<&'a dyn GamePlayer<PokerState, PokerAction>>,
    starting_player: usize,
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
                last_action: HashMap::new(),
            },
            deck: Deck::new(),
            players,
            starting_player: 0,
        };

        // Players start with 200 in chips
        for i in 0..players.len() {
            new_engine.state.player_stacks.insert(i, 200);
        }
        println!("{}", new_engine.players.len());
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
                self.state.player_cards.insert(
                    (self.starting_player + player_id) % self.players.len(),
                    (self.deck.get_next(), self.deck.get_next()),
                );
            }

            // Get preflop action
            for _ in 0..3 {
                self.state.community_cards.push(self.deck.get_next());
            }
            // Get flop action
            self.state.community_cards.push(self.deck.get_next());
            // Get turn action
            self.state.community_cards.push(self.deck.get_next());
            println!("{}", self.state);
            // Get river action
            // Showdown
            self.starting_player = (self.starting_player + 1) % self.players.len();

            let mut max_value = 0;
            let mut max_player: Vec<usize> = vec![];
            for i in 0..self.players.len() {
                let (player_card1, player_card2) = &self.state.player_cards[&i];
                let mut cards = vec![*player_card1, *player_card2];
                cards.append(&mut self.state.community_cards.clone());
                let value = get_hand_value(&cards);
                println!("Player {} of {}: Score {}", i, self.players.len(), value);
                if value > max_value {
                    max_value = value;
                    max_player = vec![i];
                } else if value == max_value {
                    max_player.push(i);
                }
            }
            println!("Player {:?} wins!\n", max_player);
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
