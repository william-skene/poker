use game::{GameEngine, GamePlayer};
use poker::{PokerAction, PokerState};

pub mod card;
mod game;
pub mod poker;

fn main() {
    let players: Vec<&dyn GamePlayer<PokerState, PokerAction>> = vec![
        &poker::PassivePokerPlayer {},
        &poker::PassivePokerPlayer {},
        &poker::PassivePokerPlayer {},
        &poker::PassivePokerPlayer {},
    ];

    let mut engine = poker::PokerEngine::new(&players);
    engine.run();
}
