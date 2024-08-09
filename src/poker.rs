use crate::card::{Card, Deck, Rank, Suit};
use crate::game::{GameEngine, GamePlayer};

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
            write!(
                f,
                "Player {} has {} chips and last performed {:?}\n",
                player,
                stack,
                self.last_action
                    .get(player)
                    .unwrap_or_else(|| &PokerAction::Fold)
            );
        }

        for (player, (card1, card2)) in &self.player_cards {
            write!(f, "Player {} has {} and {}\n", player, card1, card2);
        }

        write!(f, "Community cards: ");
        for card in &self.community_cards[0..self.community_cards.len() - 1] {
            write!(f, "{}, ", card);
        }
        write!(
            f,
            "{}\n",
            self.community_cards[self.community_cards.len() - 1]
        );

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
            players: players,
            starting_player: 0,
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
                self.state.player_cards.insert(
                    self.starting_player + player_id,
                    (self.deck.get_next(), self.deck.get_next()),
                );
            }

            // Get preflop action
            for _ in 0..3 {
                self.state.community_cards.push(self.deck.get_next());
            }
            println!("{}", self.state);
            // Get flop action
            self.state.community_cards.push(self.deck.get_next());
            println!("{}", self.state);
            // Get turn action
            self.state.community_cards.push(self.deck.get_next());
            println!("{}\n", self.state);
            // Get river action
            // Showdown
            //
            self.starting_player = (self.starting_player + 1) % self.players.len();
        }
    }
}

fn flush_value(cards: &Vec<Card>) -> i64 {
    let mut suit_counts = HashMap::<Suit, Vec<Card>>::new();
    for card in cards {
        suit_counts
            .entry(card.suit)
            .or_insert(vec![card.clone()])
            .push(card.clone());
    }
    for (_, hand) in suit_counts {
        if hand.len() < 5 {
            continue;
        }
        let mut ordered_hand = hand.clone();
        ordered_hand.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
        let mut value = 0;
        for (i, card) in ordered_hand.iter().enumerate() {
            value += 13_i64.pow(4 - i as u32) * (card.rank as i64);
        }
        return value;
    }
    -1
}

pub struct PassivePokerPlayer {}

impl GamePlayer<PokerState, PokerAction> for PassivePokerPlayer {
    fn get_action(&self, _: PokerState) -> PokerAction {
        PokerAction::Call
    }
}

pub struct PlayerPokerPlayer {}
