use std::cmp::Ordering;

use crate::cards::Card;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl HandCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighCard => "high_card",
            Self::OnePair => "one_pair",
            Self::TwoPair => "two_pair",
            Self::ThreeOfAKind => "three_of_a_kind",
            Self::Straight => "straight",
            Self::Flush => "flush",
            Self::FullHouse => "full_house",
            Self::FourOfAKind => "four_of_a_kind",
            Self::StraightFlush => "straight_flush",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandEvaluation {
    pub category: HandCategory,
    pub tie_break_vector: Vec<u8>,
    pub used_cards: [Card; 5],
}

pub fn compare_evaluations(left: &HandEvaluation, right: &HandEvaluation) -> Ordering {
    left.category
        .cmp(&right.category)
        .then_with(|| left.tie_break_vector.cmp(&right.tie_break_vector))
}

pub fn evaluate_five(cards: [Card; 5]) -> HandEvaluation {
    let flush = cards.iter().all(|card| card.suit == cards[0].suit);
    let ranks_desc = ranks_desc(&cards);
    let straight_high = straight_high(&cards);
    let counts = rank_counts_desc(&cards);

    let (category, tie_break_vector) = if flush {
        if let Some(high) = straight_high {
            (HandCategory::StraightFlush, vec![high])
        } else {
            (HandCategory::Flush, ranks_desc)
        }
    } else if counts[0].1 == 4 {
        let kicker = counts
            .iter()
            .find(|(_, count)| *count == 1)
            .map(|(rank, _)| *rank)
            .expect("four of a kind leaves one kicker");
        (HandCategory::FourOfAKind, vec![counts[0].0, kicker])
    } else if counts[0].1 == 3 && counts[1].1 == 2 {
        (HandCategory::FullHouse, vec![counts[0].0, counts[1].0])
    } else if let Some(high) = straight_high {
        (HandCategory::Straight, vec![high])
    } else if counts[0].1 == 3 {
        let mut tie_break = vec![counts[0].0];
        tie_break.extend(
            counts
                .iter()
                .filter(|(_, count)| *count == 1)
                .map(|(rank, _)| *rank),
        );
        (HandCategory::ThreeOfAKind, tie_break)
    } else if counts[0].1 == 2 && counts[1].1 == 2 {
        let mut pairs = counts
            .iter()
            .filter(|(_, count)| *count == 2)
            .map(|(rank, _)| *rank)
            .collect::<Vec<_>>();
        pairs.sort_by(|left, right| right.cmp(left));
        let kicker = counts
            .iter()
            .find(|(_, count)| *count == 1)
            .map(|(rank, _)| *rank)
            .expect("two pair leaves one kicker");
        (HandCategory::TwoPair, vec![pairs[0], pairs[1], kicker])
    } else if counts[0].1 == 2 {
        let pair = counts[0].0;
        let mut tie_break = vec![pair];
        tie_break.extend(
            counts
                .iter()
                .filter(|(_, count)| *count == 1)
                .map(|(rank, _)| *rank),
        );
        (HandCategory::OnePair, tie_break)
    } else {
        (HandCategory::HighCard, ranks_desc)
    };

    HandEvaluation {
        category,
        tie_break_vector,
        used_cards: canonical_used_cards(cards),
    }
}

pub fn best_five_from_seven(cards: [Card; 7]) -> HandEvaluation {
    let mut best: Option<HandEvaluation> = None;
    for a in 0..3 {
        for b in (a + 1)..4 {
            for c in (b + 1)..5 {
                for d in (c + 1)..6 {
                    for e in (d + 1)..7 {
                        let candidate =
                            evaluate_five([cards[a], cards[b], cards[c], cards[d], cards[e]]);
                        if best.as_ref().is_none_or(|current| {
                            compare_evaluations(&candidate, current) == Ordering::Greater
                        }) {
                            best = Some(candidate);
                        }
                    }
                }
            }
        }
    }
    best.expect("seven cards always yield at least one five-card subset")
}

fn rank_counts_desc(cards: &[Card; 5]) -> Vec<(u8, u8)> {
    let mut counts = Vec::new();
    for rank in (2..=14).rev() {
        let count = cards
            .iter()
            .filter(|card| card.rank.value() == rank)
            .count() as u8;
        if count > 0 {
            counts.push((rank, count));
        }
    }
    counts.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| right.0.cmp(&left.0)));
    counts
}

fn ranks_desc(cards: &[Card; 5]) -> Vec<u8> {
    let mut ranks = cards
        .iter()
        .map(|card| card.rank.value())
        .collect::<Vec<_>>();
    ranks.sort_by(|left, right| right.cmp(left));
    ranks
}

fn straight_high(cards: &[Card; 5]) -> Option<u8> {
    let mut present = [false; 15];
    for card in cards {
        present[card.rank.value() as usize] = true;
    }
    if present[14] {
        present[1] = true;
    }

    (5..=14).rev().find(|high| {
        let low = high - 4;
        (low..=*high).all(|rank| present[rank as usize])
    })
}

fn canonical_used_cards(cards: [Card; 5]) -> [Card; 5] {
    let mut sorted = cards.to_vec();
    sorted.sort_by(|left, right| {
        right
            .rank
            .value()
            .cmp(&left.rank.value())
            .then_with(|| left.suit.cmp(&right.suit))
    });
    sorted
        .try_into()
        .expect("five input cards remain five sorted cards")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Rank, Suit};

    fn c(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    fn eval(cards: [Card; 5]) -> HandEvaluation {
        evaluate_five(cards)
    }

    #[test]
    fn five_card_evaluator_covers_all_categories_in_order() {
        let hands = [
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::King, Suit::Diamonds),
                c(Rank::Nine, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Nine, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Nine, Suit::Spades),
                c(Rank::Nine, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Ace, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Nine, Suit::Clubs),
                c(Rank::Eight, Suit::Diamonds),
                c(Rank::Seven, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Five, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::King, Suit::Clubs),
                c(Rank::Nine, Suit::Clubs),
                c(Rank::Six, Suit::Clubs),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Ace, Suit::Spades),
                c(Rank::King, Suit::Hearts),
                c(Rank::King, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Ace, Suit::Spades),
                c(Rank::Ace, Suit::Hearts),
                c(Rank::King, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::King, Suit::Clubs),
                c(Rank::Queen, Suit::Clubs),
                c(Rank::Jack, Suit::Clubs),
                c(Rank::Ten, Suit::Clubs),
            ]),
        ];

        assert_eq!(
            hands.iter().map(|hand| hand.category).collect::<Vec<_>>(),
            vec![
                HandCategory::HighCard,
                HandCategory::OnePair,
                HandCategory::TwoPair,
                HandCategory::ThreeOfAKind,
                HandCategory::Straight,
                HandCategory::Flush,
                HandCategory::FullHouse,
                HandCategory::FourOfAKind,
                HandCategory::StraightFlush,
            ]
        );
        for pair in hands.windows(2) {
            assert_eq!(compare_evaluations(&pair[0], &pair[1]), Ordering::Less);
            assert_eq!(compare_evaluations(&pair[1], &pair[0]), Ordering::Greater);
        }
    }

    #[test]
    fn ace_low_straight_uses_five_high_tie_break() {
        let wheel = eval([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Two, Suit::Diamonds),
            c(Rank::Three, Suit::Spades),
            c(Rank::Four, Suit::Hearts),
            c(Rank::Five, Suit::Clubs),
        ]);
        let six_high = eval([
            c(Rank::Two, Suit::Clubs),
            c(Rank::Three, Suit::Diamonds),
            c(Rank::Four, Suit::Spades),
            c(Rank::Five, Suit::Hearts),
            c(Rank::Six, Suit::Clubs),
        ]);

        assert_eq!(wheel.category, HandCategory::Straight);
        assert_eq!(wheel.tie_break_vector, vec![5]);
        assert_eq!(compare_evaluations(&wheel, &six_high), Ordering::Less);
    }

    #[test]
    fn kicker_order_breaks_pair_and_flush_ties() {
        let pair_king_kicker = eval([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::King, Suit::Spades),
            c(Rank::Six, Suit::Hearts),
            c(Rank::Three, Suit::Clubs),
        ]);
        let pair_queen_kicker = eval([
            c(Rank::Ace, Suit::Hearts),
            c(Rank::Ace, Suit::Spades),
            c(Rank::Queen, Suit::Diamonds),
            c(Rank::Six, Suit::Clubs),
            c(Rank::Three, Suit::Hearts),
        ]);
        assert_eq!(
            compare_evaluations(&pair_king_kicker, &pair_queen_kicker),
            Ordering::Greater
        );

        let flush_jack = eval([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::King, Suit::Clubs),
            c(Rank::Jack, Suit::Clubs),
            c(Rank::Nine, Suit::Clubs),
            c(Rank::Three, Suit::Clubs),
        ]);
        let flush_ten = eval([
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::King, Suit::Diamonds),
            c(Rank::Ten, Suit::Diamonds),
            c(Rank::Nine, Suit::Diamonds),
            c(Rank::Three, Suit::Diamonds),
        ]);
        assert_eq!(
            compare_evaluations(&flush_jack, &flush_ten),
            Ordering::Greater
        );
    }

    #[test]
    fn full_house_tie_break_uses_trip_rank_before_pair_rank() {
        let aces_full = eval([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::Ace, Suit::Spades),
            c(Rank::King, Suit::Hearts),
            c(Rank::King, Suit::Clubs),
        ]);
        let kings_full = eval([
            c(Rank::King, Suit::Diamonds),
            c(Rank::King, Suit::Spades),
            c(Rank::King, Suit::Clubs),
            c(Rank::Ace, Suit::Hearts),
            c(Rank::Ace, Suit::Spades),
        ]);

        assert_eq!(aces_full.tie_break_vector, vec![14, 13]);
        assert_eq!(
            compare_evaluations(&aces_full, &kings_full),
            Ordering::Greater
        );
    }

    #[test]
    fn seven_card_search_returns_exact_best_five_cards() {
        let cards = [
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::Ace, Suit::Hearts),
            c(Rank::Ace, Suit::Spades),
            c(Rank::King, Suit::Clubs),
            c(Rank::Queen, Suit::Diamonds),
            c(Rank::Two, Suit::Hearts),
        ];

        let best = best_five_from_seven(cards);

        assert_eq!(best.category, HandCategory::FourOfAKind);
        assert_eq!(best.tie_break_vector, vec![14, 13]);
        assert_eq!(
            best.used_cards,
            [
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Ace, Suit::Hearts),
                c(Rank::Ace, Suit::Spades),
                c(Rank::King, Suit::Clubs),
            ]
        );
    }

    #[test]
    fn suits_do_not_break_hand_strength_ties() {
        let clubs = eval([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::King, Suit::Clubs),
            c(Rank::Queen, Suit::Diamonds),
            c(Rank::Jack, Suit::Hearts),
            c(Rank::Nine, Suit::Spades),
        ]);
        let spades = eval([
            c(Rank::Ace, Suit::Spades),
            c(Rank::King, Suit::Spades),
            c(Rank::Queen, Suit::Hearts),
            c(Rank::Jack, Suit::Diamonds),
            c(Rank::Nine, Suit::Clubs),
        ]);

        assert_eq!(compare_evaluations(&clubs, &spades), Ordering::Equal);
    }

    #[test]
    fn comparator_is_antisymmetric_and_transitive_for_sweep() {
        let sweep = [
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::King, Suit::Diamonds),
                c(Rank::Nine, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::Ace, Suit::Diamonds),
                c(Rank::Nine, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Three, Suit::Clubs),
            ]),
            eval([
                c(Rank::Nine, Suit::Clubs),
                c(Rank::Eight, Suit::Diamonds),
                c(Rank::Seven, Suit::Spades),
                c(Rank::Six, Suit::Hearts),
                c(Rank::Five, Suit::Clubs),
            ]),
            eval([
                c(Rank::Ace, Suit::Clubs),
                c(Rank::King, Suit::Clubs),
                c(Rank::Queen, Suit::Clubs),
                c(Rank::Jack, Suit::Clubs),
                c(Rank::Ten, Suit::Clubs),
            ]),
        ];

        for left in &sweep {
            for right in &sweep {
                assert_eq!(
                    compare_evaluations(left, right),
                    compare_evaluations(right, left).reverse()
                );
            }
        }
        assert_eq!(compare_evaluations(&sweep[0], &sweep[1]), Ordering::Less);
        assert_eq!(compare_evaluations(&sweep[1], &sweep[2]), Ordering::Less);
        assert_eq!(compare_evaluations(&sweep[0], &sweep[2]), Ordering::Less);
    }
}
