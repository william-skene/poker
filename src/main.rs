use crate::card::Deck;

pub mod card;
mod game;
pub mod poker;

fn main() {
    let mut deck = Deck::new();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}
