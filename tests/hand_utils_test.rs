use poker::card::{Card, Rank, Suit};
use poker::hand_utils::*;
use poker::new_card;

macro_rules! test_better_hand {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (better_hand, worse_hand) = $value;
            assert!(get_hand_value(&better_hand) > get_hand_value(&worse_hand));
        }
    )*
    }
}

#[cfg(test)]
mod valuation_tests {
    use super::*;
    test_better_hand! {
        worst_pair_better_than_best_high_card: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Diamond),
                new_card!(Four, Spade),
                new_card!(Five, Spade),
            ] ,vec![
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
                new_card!(Queen, Diamond),
                new_card!(Jack, Spade),
                new_card!(Nine, Spade),
            ]
        ),
        worst_two_pair_better_than_best_pair: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Diamond),
                new_card!(Three, Spade),
                new_card!(Four, Spade),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
                new_card!(Queen, Spade),
                new_card!(Jack, Spade),
            ]
        ),
        worst_set_better_than_best_two_pair: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Spade),
                new_card!(Four, Spade),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
                new_card!(King, Spade),
                new_card!(Queen, Spade),
            ]
        ),
        worst_straight_better_than_best_set: (
            vec![
                new_card!(Ace, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Diamond),
                new_card!(Four, Spade),
                new_card!(Five, Spade),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(Ace, Diamond),
                new_card!(Ace, Diamond),
                new_card!(King, Spade),
                new_card!(Queen, Spade),
            ]
        ),
        worst_flush_better_than_best_straight: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Three, Diamond),
                new_card!(Four, Diamond),
                new_card!(Five, Diamond),
                new_card!(Seven, Diamond),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
                new_card!(Queen, Diamond),
                new_card!(Jack, Spade),
                new_card!(Ten, Spade),
            ]
        ),
        worst_full_house_better_than_best_flush: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Two, Heart),
                new_card!(Two, Spade),
                new_card!(Three, Diamond),
                new_card!(Three, Spade),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
                new_card!(Queen, Diamond),
                new_card!(Jack, Diamond),
                new_card!(Nine, Diamond),
            ]
        ),
        worst_quads_better_than_best_full_house: (
            vec![
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Spade),
            ],
            vec![
                new_card!(Ace, Diamond),
                new_card!(Ace, Spade),
                new_card!(Ace, Club),
                new_card!(King, Heart),
                new_card!(King, Diamond),
            ]
        ),
        worst_straight_flush_better_than_best_quads: (
            vec![
                new_card!(Ace, Diamond),
                new_card!(Two, Diamond),
                new_card!(Three, Diamond),
                new_card!(Four, Diamond),
                new_card!(Five, Diamond),
            ],
            vec![
                new_card!(Ace, Spade),
                new_card!(Ace, Club),
                new_card!(Ace, Heart),
                new_card!(Ace, Diamond),
                new_card!(King, Diamond),
            ]
        ),
    }
}
