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

fn get_rank_counts(cards: &Vec<Card>) -> HashMap<Rank, i64> {
    cards
        .iter()
        .fold(HashMap::<Rank, i64>::new(), |mut acc, x| {
            *acc.entry(x.rank).or_insert(0) += 1;
            acc
        })
}

fn get_highest_n_of_kind(cards: &Vec<Card>, n: i64) -> Rank {
    let rank_counts = get_rank_counts(cards);
    rank_counts.iter().fold(
        Rank::Null,
        |acc, (a, b)| if b == &n { acc.max(*a) } else { acc },
    )
}

fn straight_flush_value(cards: &Vec<Card>) -> i64 {
    if cards.len() < 5 {
        return -1;
    }

    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    // Ace can be high or low
    if cards_sorted[0].rank == Rank::Ace {
        cards_sorted.push(cards_sorted[0].clone());
    }
    for i in 0..cards_sorted.len() - 5 {
        let view = &cards_sorted[i..i + 5];
        let mut straight = true;
        let mut flush = true;
        for (prev_card, card) in view.iter().zip(view[1..].iter()) {
            if (card.rank as i64) - (prev_card.rank as i64) != -1
                && !(prev_card.rank == Rank::Two && card.rank == Rank::Ace)
            {
                straight = false;
                break;
            } else if card.suit != prev_card.suit {
                flush = false;
                break;
            }
        }
        if straight && flush {
            return view[0].rank as i64;
        }
    }
    -1
}

fn quads_value(cards: &Vec<Card>) -> i64 {
    let values: HashMap<Rank, i64> = get_rank_counts(cards);
    let quads = get_highest_n_of_kind(cards, 4);
    let top_card: Rank = cards
        .clone()
        .iter()
        .filter(|a| a.rank != quads)
        .max_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap())
        .unwrap()
        .rank;

    if quads == Rank::Null {
        -1
    } else {
        (quads as i64) * 13 + top_card as i64
    }
}

fn full_house_value(cards: &Vec<Card>) -> i64 {
    let highest_three = get_highest_n_of_kind(cards, 3);
    let highest_two = get_highest_n_of_kind(cards, 2);
    if highest_three == Rank::Null || highest_two == Rank::Null {
        -1
    } else {
        13 * (highest_three as i64) + (highest_two as i64)
    }
}

fn flush_value(cards: &Vec<Card>) -> i64 {
    if cards.len() < 5 {
        return -1;
    }
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

fn straight_value(cards: &Vec<Card>) -> i64 {
    if cards.len() < 5 {
        return -1;
    }

    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    let mut card_ranks: Vec<i64> = cards_sorted.iter().map(|a| a.rank as i64).collect();

    // Ace can be high or low
    if card_ranks[0] == 13 {
        card_ranks.push(-1);
    }
    for i in 0..card_ranks.len() - 5 {
        let view = &card_ranks[i..i + 5];
        let mut straight = true;
        for (prev_card, card) in view.iter().zip(view[1..].iter()) {
            if card - prev_card != -1 {
                straight = false;
                break;
            }
        }
        if straight {
            return view[0];
        }
    }
    -1
}

fn set_value(cards: &Vec<Card>) -> i64 {
    let set = get_highest_n_of_kind(cards, 3);
    if set == Rank::Null {
        return -1;
    }
    let mut total = set as i64 * 13_i64.pow(2);

    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    for (idx, card) in cards[0..2].iter().enumerate() {
        if card.rank == set {
            continue;
        } else {
            total += 13_i64.pow(1 - idx as u32) * card.rank as i64;
        }
    }
    total
}

fn two_pair_value(cards: &Vec<Card>) -> i64 {
    let values: HashMap<Rank, i64> = get_rank_counts(cards);
    let (mut pairs, _): (Vec<&Rank>, Vec<&i64>) =
        values.iter().filter(|(_, count)| **count == 2).unzip();
    pairs.sort();
    pairs.reverse();
    let top_card: Rank = cards
        .clone()
        .iter()
        .filter(|a| a.rank != *pairs[0] && a.rank != *pairs[1])
        .max_by(|a, b| a.rank.partial_cmp(&b.rank).unwrap())
        .unwrap()
        .rank;

    if pairs.len() < 2 {
        -1
    } else {
        (*pairs[0] as i64) * 13 * 13 + (*pairs[1] as i64) * 13 + top_card as i64
    }
}

fn pair_value(cards: &Vec<Card>) -> i64 {
    let pair = get_highest_n_of_kind(cards, 2);
    if pair == Rank::Null {
        return -1;
    }

    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    let mut total = 13_i64.pow(3) * pair as i64;
    for (idx, card) in cards.iter().enumerate() {
        if card.rank == pair {
            continue;
        } else {
            total += 13_i64.pow(2 - idx as u32) * card.rank as i64
        }
    }
    total
}

fn high_card_value(cards: &Vec<Card>) -> i64 {
    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    let mut total = 0;
    for (idx, card) in cards_sorted[0..5].iter().enumerate() {
        total += 13_i64.pow(4 - idx as u32);
    }
    return total;
}

fn get_hand_value(cards: &Vec<Card>) -> i64 {
    let valuations: Vec<fn(&Vec<Card>) -> i64> = vec![
        straight_flush_value,
        quads_value,
        full_house_value,
        flush_value,
        straight_value,
        set_value,
        two_pair_value,
        pair_value,
        high_card_value,
    ];

    let base = 13_i64.pow(5);
    for (idx, val) in valuations.iter().enumerate() {
        let value = val(cards);
        if value != -1 {
            return base * (valuations.len() - 1 - idx) as i64 + value;
        }
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
