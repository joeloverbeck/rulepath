//! Pure trick-taking selection helpers.
//!
//! This module owns only two behavior-free repeated shapes:
//!
//! - choose the stable held-item indices that follow a caller-projected led suit,
//!   or all held indices when the hand is void in that suit;
//! - choose the stable index of the winning play from caller-projected suit and
//!   rank keys using highest trump when present, otherwise highest led suit.
//!
//! Callers still own cards, seats, phases, ownership, diagnostics, legal leads,
//! first-trick exceptions, trump-breaking policy, winner-leads mutation, dealing,
//! bidding, scoring, effects, visibility, replay, and terminal behavior.

/// Return legal held-item indices under a simple follow-suit rule.
///
/// The returned indices preserve input order. When one or more held items match
/// `led_suit`, only matching indices are returned. When no held item matches,
/// every held index is returned. Empty input returns an empty vector.
///
/// This helper does not decide whether the actor may lead, whether special
/// first-trick restrictions apply, or how diagnostics/effects are emitted.
pub fn follow_suit_indices<T, S: Copy + Eq>(
    held: &[T],
    led_suit: S,
    suit_of: impl Fn(&T) -> S,
) -> Vec<usize> {
    let matching: Vec<usize> = held
        .iter()
        .enumerate()
        .filter_map(|(index, item)| (suit_of(item) == led_suit).then_some(index))
        .collect();

    if matching.is_empty() {
        (0..held.len()).collect()
    } else {
        matching
    }
}

/// Return the stable winning play index for a completed trick-like play list.
///
/// If any play has the caller-projected trump suit, only trump plays can win.
/// Otherwise only led-suit plays can win. Within the winning class, the highest
/// caller-projected rank wins. Equal ranks keep the first occurrence.
///
/// Empty inputs, and inputs with no trump and no led-suit play, return `None`.
/// This helper does not mutate turn order, capture cards, score, emit effects,
/// or define a shared card/trick type.
pub fn winning_play_index<T, S: Copy + Eq, R: Copy + Ord>(
    plays: &[T],
    led_suit: S,
    trump: Option<S>,
    suit_of: impl Fn(&T) -> S,
    rank_of: impl Fn(&T) -> R,
) -> Option<usize> {
    if let Some(trump_suit) = trump {
        if let Some(index) = best_index_in_suit(plays, trump_suit, &suit_of, &rank_of) {
            return Some(index);
        }
    }

    best_index_in_suit(plays, led_suit, &suit_of, &rank_of)
}

fn best_index_in_suit<T, S: Copy + Eq, R: Copy + Ord>(
    plays: &[T],
    target_suit: S,
    suit_of: &impl Fn(&T) -> S,
    rank_of: &impl Fn(&T) -> R,
) -> Option<usize> {
    let mut best: Option<(usize, R)> = None;

    for (index, play) in plays.iter().enumerate() {
        if suit_of(play) != target_suit {
            continue;
        }

        let rank = rank_of(play);
        match best {
            None => best = Some((index, rank)),
            Some((_, best_rank)) if rank > best_rank => best = Some((index, rank)),
            Some(_) => {}
        }
    }

    best.map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Suit {
        Reed,
        Shell,
        Star,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Play {
        suit: Suit,
        rank: u8,
        id: u8,
    }

    fn play(suit: Suit, rank: u8, id: u8) -> Play {
        Play { suit, rank, id }
    }

    fn suit_of(play: &Play) -> Suit {
        play.suit
    }

    fn rank_of(play: &Play) -> u8 {
        play.rank
    }

    #[test]
    fn follow_suit_empty_input_returns_empty() {
        let held: [Play; 0] = [];

        assert_eq!(
            follow_suit_indices(&held, Suit::Reed, suit_of),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn follow_suit_returns_matching_indices_in_input_order() {
        let held = [
            play(Suit::Shell, 2, 0),
            play(Suit::Reed, 5, 1),
            play(Suit::Star, 9, 2),
            play(Suit::Reed, 3, 3),
        ];

        assert_eq!(follow_suit_indices(&held, Suit::Reed, suit_of), [1, 3]);
    }

    #[test]
    fn follow_suit_returns_all_indices_when_void() {
        let held = [
            play(Suit::Shell, 2, 0),
            play(Suit::Star, 9, 1),
            play(Suit::Shell, 3, 2),
        ];

        assert_eq!(follow_suit_indices(&held, Suit::Reed, suit_of), [0, 1, 2]);
    }

    #[test]
    fn follow_suit_property_indices_are_ordered_valid_and_complete() {
        let suits = [Suit::Reed, Suit::Shell, Suit::Star];
        let held = [
            play(Suit::Reed, 2, 0),
            play(Suit::Shell, 7, 1),
            play(Suit::Reed, 4, 2),
            play(Suit::Star, 8, 3),
            play(Suit::Shell, 3, 4),
        ];

        for led in suits {
            let indices = follow_suit_indices(&held, led, suit_of);

            assert!(indices.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(indices.iter().all(|&index| index < held.len()));

            let matching: Vec<usize> = held
                .iter()
                .enumerate()
                .filter_map(|(index, card)| (card.suit == led).then_some(index))
                .collect();
            if matching.is_empty() {
                assert_eq!(indices, (0..held.len()).collect::<Vec<_>>());
            } else {
                assert_eq!(indices, matching);
            }
        }
    }

    #[test]
    fn winning_play_empty_input_returns_none() {
        let plays: [Play; 0] = [];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, Some(Suit::Shell), suit_of, rank_of),
            None
        );
    }

    #[test]
    fn winning_play_uses_highest_led_suit_without_trump() {
        let plays = [
            play(Suit::Reed, 8, 0),
            play(Suit::Shell, 14, 1),
            play(Suit::Reed, 12, 2),
            play(Suit::Star, 13, 3),
        ];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, None, suit_of, rank_of),
            Some(2)
        );
    }

    #[test]
    fn winning_play_ignores_off_suit_non_trumps() {
        let plays = [play(Suit::Shell, 14, 0), play(Suit::Star, 13, 1)];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, None, suit_of, rank_of),
            None
        );
    }

    #[test]
    fn winning_play_uses_highest_trump_before_led_suit() {
        let plays = [
            play(Suit::Reed, 14, 0),
            play(Suit::Shell, 2, 1),
            play(Suit::Shell, 9, 2),
            play(Suit::Reed, 13, 3),
        ];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, Some(Suit::Shell), suit_of, rank_of),
            Some(2)
        );
    }

    #[test]
    fn winning_play_falls_back_to_led_suit_when_no_trump_is_played() {
        let plays = [
            play(Suit::Reed, 7, 0),
            play(Suit::Star, 14, 1),
            play(Suit::Reed, 10, 2),
        ];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, Some(Suit::Shell), suit_of, rank_of),
            Some(2)
        );
    }

    #[test]
    fn winning_play_keeps_first_occurrence_on_equal_rank() {
        let plays = [
            play(Suit::Reed, 9, 0),
            play(Suit::Reed, 12, 1),
            play(Suit::Reed, 12, 2),
        ];

        assert_eq!(
            winning_play_index(&plays, Suit::Reed, None, suit_of, rank_of),
            Some(1)
        );
    }

    #[test]
    fn winning_play_property_result_is_maximal_in_winning_class() {
        let suit_sets = [
            [Suit::Reed, Suit::Reed, Suit::Shell, Suit::Star],
            [Suit::Shell, Suit::Star, Suit::Shell, Suit::Reed],
            [Suit::Star, Suit::Shell, Suit::Star, Suit::Shell],
        ];
        let ranks = [[2, 3, 4, 5], [9, 9, 8, 7], [14, 2, 13, 12]];

        for suits in suit_sets {
            for ranks in ranks {
                let plays = [
                    play(suits[0], ranks[0], 0),
                    play(suits[1], ranks[1], 1),
                    play(suits[2], ranks[2], 2),
                    play(suits[3], ranks[3], 3),
                ];

                for led in [Suit::Reed, Suit::Shell, Suit::Star] {
                    for trump in [None, Some(Suit::Reed), Some(Suit::Shell), Some(Suit::Star)] {
                        let result = winning_play_index(&plays, led, trump, suit_of, rank_of);
                        let winning_suit = trump
                            .filter(|&trump_suit| plays.iter().any(|play| play.suit == trump_suit))
                            .unwrap_or(led);
                        let eligible: Vec<(usize, u8)> = plays
                            .iter()
                            .enumerate()
                            .filter_map(|(index, play)| {
                                (play.suit == winning_suit).then_some((index, play.rank))
                            })
                            .collect();

                        match (result, eligible.iter().map(|(_, rank)| rank).max()) {
                            (None, None) => {}
                            (Some(index), Some(max_rank)) => {
                                assert_eq!(plays[index].suit, winning_suit);
                                assert_eq!(plays[index].rank, *max_rank);
                                assert!(eligible
                                    .iter()
                                    .take_while(|(eligible_index, _)| *eligible_index != index)
                                    .all(|(_, rank)| rank < max_rank));
                            }
                            _ => panic!("result and eligible set disagree"),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn helpers_do_not_mutate_caller_input() {
        let held = [
            play(Suit::Shell, 3, 0),
            play(Suit::Reed, 8, 1),
            play(Suit::Shell, 10, 2),
        ];
        let before = held;

        let _ = follow_suit_indices(&held, Suit::Shell, suit_of);
        let _ = winning_play_index(&held, Suit::Shell, Some(Suit::Reed), suit_of, rank_of);

        assert_eq!(held, before);
    }
}
