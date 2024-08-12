use poker::card::{Card, Rank, Suit};
use poker::game::{GameEngine, GamePlayer};
use poker::hand_utils::*;
use poker::new_card;
use poker::poker::{PassivePokerPlayer, PokerAction, PokerEngine, PokerState};

fn main() {
    let players: Vec<&dyn GamePlayer<PokerState, PokerAction>> =
        vec![&PassivePokerPlayer {}, &PassivePokerPlayer {}];

    let mut engine = PokerEngine::new(&players);
    engine.run();
}
