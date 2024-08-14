use crate::card::{Card, Deck, Rank, Suit};
use crate::game::{GameEngine, GamePlayer};
use crate::hand_utils::get_hand_value;

use std::collections::hash_map::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum PokerAction {
    Call,
    Raise(u64),
    Check,
    Fold,
}

#[derive(Clone)]
pub struct PokerState {
    pub pot: u64,
    pub player_stacks: HashMap<usize, u64>,
    pub community_cards: Vec<Card>,
    pub player_cards: HashMap<usize, (Card, Card)>,
    pub last_action: HashMap<usize, PokerAction>,
    pub required_call: u64,
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
                    .unwrap_or_else(|| &PokerAction::Check)
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
                required_call: 0,
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

    fn get_betting_action(&mut self) {
        let mut last_bet = self.starting_player;
        let mut current_player = self.starting_player;
        let mut pot_commitment = HashMap::<usize, u64>::new();
        let mut big_blind = false;
        // Preflop
        if self.state.community_cards.len() == 0 {
            // Player 1 in for small blind
            println!("{}", current_player);
            *self
                .state
                .last_action
                .entry(current_player)
                .or_insert(PokerAction::Check) = PokerAction::Raise(1);
            *pot_commitment.entry(current_player).or_insert(0) = 1;
            current_player = (current_player + 1) % self.players.len();

            println!("{}", current_player);
            // Player 2 in for big blind
            *self
                .state
                .last_action
                .entry(current_player)
                .or_insert(PokerAction::Check) = PokerAction::Raise(1);
            *pot_commitment.entry(current_player).or_insert(0) = 2;
            current_player = (current_player + 1) % self.players.len();
            last_bet = current_player;
            big_blind = true;
            self.state.required_call = 2;
        }

        let mut available_actions: Vec<PokerAction> = vec![];
        while current_player != last_bet || big_blind {
            if current_player == last_bet {
                big_blind = false;
            }
            if self.state.last_action[&current_player] == PokerAction::Fold {
                current_player = (current_player + 1) % self.players.len();
                continue;
            }

            if self.state.required_call > 0 {
                available_actions =
                    vec![PokerAction::Fold, PokerAction::Call, PokerAction::Raise(0)];
            } else {
                available_actions = vec![PokerAction::Check, PokerAction::Raise(0)];
            }
            let mut player_view = self.state.clone();
            player_view.player_cards.clear();
            player_view
                .player_cards
                .insert(current_player, self.state.player_cards[&current_player]);

            let action = self.players[current_player].get_action(&player_view, &available_actions);
            match action {
                PokerAction::Call => {
                    *pot_commitment.entry(current_player).or_insert(0) = self.state.required_call;
                }
                PokerAction::Raise(value) => {
                    self.state.required_call += value;
                    *pot_commitment.entry(current_player).or_insert(0) = self.state.required_call;
                    last_bet = current_player;
                }
                _ => (),
            }
            *self
                .state
                .last_action
                .entry(current_player)
                .or_insert(PokerAction::Check) = action;
            current_player = (current_player + 1) % self.players.len();
        }

        for (player, value) in pot_commitment {
            self.state.pot += value;
            *self.state.player_stacks.get_mut(&player).unwrap() -= value;
        }
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
            self.get_betting_action();
            for _ in 0..3 {
                self.state.community_cards.push(self.deck.get_next());
            }
            // Get flop action
            self.get_betting_action();
            self.state.community_cards.push(self.deck.get_next());
            // Get turn action
            self.get_betting_action();
            self.state.community_cards.push(self.deck.get_next());
            println!("{}", self.state);
            // Get river action
            self.get_betting_action();
            // Showdown

            let mut max_value = 0;
            let mut max_player: Vec<usize> = vec![];
            for i in 0..self.players.len() {
                if self.state.last_action.get(&i).unwrap() == &PokerAction::Fold {
                    continue;
                }
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
            for player in &max_player {
                *self.state.player_stacks.get_mut(&player).unwrap() +=
                    self.state.pot / (max_player.len() as u64);
            }
            println!("Player {:?} wins {}!\n", max_player, self.state.pot);

            self.starting_player = (self.starting_player + 1) % self.players.len();
            self.state.pot = 0;
            self.state.required_call = 0;
            self.state.last_action.clear();
            self.state.community_cards = vec![];
            self.state.player_cards.clear();
        }
    }
}

pub struct PassivePokerPlayer {}

impl GamePlayer<PokerState, PokerAction> for PassivePokerPlayer {
    fn get_action(&self, _: &PokerState, actions: &Vec<PokerAction>) -> PokerAction {
        if actions.contains(&PokerAction::Check) {
            PokerAction::Check
        } else {
            PokerAction::Call
        }
    }
}

pub struct PlayerPokerPlayer {}
