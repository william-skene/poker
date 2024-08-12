use card::{Card, Rank, Suit};
use game::{GameEngine, GamePlayer};
use hand_utils::*;
use poker::{PokerAction, PokerState};

pub mod card;
mod game;
pub mod hand_utils;
pub mod poker;

fn main() {
    let players: Vec<&dyn GamePlayer<PokerState, PokerAction>> =
        vec![&poker::PassivePokerPlayer {}, &poker::PassivePokerPlayer {}];

    let mut engine = poker::PokerEngine::new(&players);
    engine.run();
}
