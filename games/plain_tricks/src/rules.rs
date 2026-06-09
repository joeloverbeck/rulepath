use engine_core::Diagnostic;

use crate::{
    actions::{self, ValidatedAction},
    ids::{PlainTricksSeat, STANDARD_TRICKS_PER_ROUND},
    setup::{deal_round, round_leader},
    state::{
        CompletedTrick, CurrentTrick, Phase, PlainTricksState, TerminalOutcome, TrickCounts,
        TrickPlay,
    },
};

pub fn apply_action(
    state: &mut PlainTricksState,
    action: ValidatedAction,
) -> Result<(), Diagnostic> {
    ensure_action_still_legal(state, action)?;

    let actor_index = action.actor.index();
    let card_index = state.hands[actor_index]
        .iter()
        .position(|card| *card == action.card)
        .expect("validated action card must be in actor hand");
    let card = state.hands[actor_index].remove(card_index);
    let play = TrickPlay {
        seat: action.actor,
        card,
    };

    if state.current_trick.plays.is_empty() {
        state.current_trick.led_suit = Some(card.suit());
        state.current_trick.plays.push(play);
        state.active_seat = Some(action.actor.other());
        state.freshness_token = state.freshness_token.next();
        return Ok(());
    }

    state.current_trick.plays.push(play);
    resolve_current_trick(state)?;
    state.freshness_token = state.freshness_token.next();
    Ok(())
}

pub fn trick_winner(leader_play: TrickPlay, follower_play: TrickPlay) -> PlainTricksSeat {
    let led_suit = leader_play.card.suit();
    if follower_play.card.suit() == led_suit && follower_play.card.rank() > leader_play.card.rank()
    {
        follower_play.seat
    } else {
        leader_play.seat
    }
}

fn ensure_action_still_legal(
    state: &PlainTricksState,
    action: ValidatedAction,
) -> Result<(), Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if state.active_seat != Some(action.actor) {
        return Err(actions::wrong_seat_diagnostic());
    }
    if !state.hands[action.actor.index()].contains(&action.card) {
        return Err(actions::not_in_hand_diagnostic(action.card));
    }
    if !actions::legal_cards(state, action.actor).contains(&action.card) {
        if state.current_trick.led_suit.is_some() {
            return Err(actions::must_follow_suit_diagnostic());
        }
        return Err(actions::unavailable_action_diagnostic());
    }
    Ok(())
}

fn resolve_current_trick(state: &mut PlainTricksState) -> Result<(), Diagnostic> {
    let plays: [TrickPlay; 2] = state
        .current_trick
        .plays
        .as_slice()
        .try_into()
        .map_err(|_| malformed_trick_state_diagnostic())?;
    let winner = trick_winner(plays[0], plays[1]);
    state.round_trick_counts.increment(winner);
    state.completed_tricks.push(CompletedTrick {
        round_index: state.round_index,
        trick_index: state.trick_index,
        leader: plays[0].seat,
        plays,
        winner,
        trick_counts_after: state.round_trick_counts,
    });

    if state.trick_index + 1 >= STANDARD_TRICKS_PER_ROUND {
        close_round(state)?;
    } else {
        state.trick_index += 1;
        state.phase = Phase::Playing {
            round_index: state.round_index,
            trick_index: state.trick_index,
        };
        state.current_leader = winner;
        state.active_seat = Some(winner);
        state.current_trick = CurrentTrick::default();
    }

    Ok(())
}

fn close_round(state: &mut PlainTricksState) -> Result<(), Diagnostic> {
    state.total_trick_counts.seat_0 = state
        .total_trick_counts
        .seat_0
        .saturating_add(state.round_trick_counts.seat_0);
    state.total_trick_counts.seat_1 = state
        .total_trick_counts
        .seat_1
        .saturating_add(state.round_trick_counts.seat_1);

    if state.round_index == 0 {
        let deal = deal_round(&mut state.rng, 1)?;
        state.round_index = 1;
        state.trick_index = 0;
        state.phase = Phase::Playing {
            round_index: 1,
            trick_index: 0,
        };
        state.round_leader = round_leader(1);
        state.current_leader = deal.leader;
        state.active_seat = Some(deal.leader);
        state.hands = deal.hands;
        state.tail = deal.tail;
        state.current_trick = CurrentTrick::default();
        state.round_trick_counts = TrickCounts::default();
        return Ok(());
    }

    state.phase = Phase::Terminal;
    state.active_seat = None;
    state.current_trick = CurrentTrick::default();
    state.terminal_outcome = Some(resolve_terminal(state.total_trick_counts));
    Ok(())
}

fn resolve_terminal(totals: TrickCounts) -> TerminalOutcome {
    if totals.seat_0 > totals.seat_1 {
        TerminalOutcome::TrickWin {
            winner: PlainTricksSeat::Seat0,
            totals,
        }
    } else if totals.seat_1 > totals.seat_0 {
        TerminalOutcome::TrickWin {
            winner: PlainTricksSeat::Seat1,
            totals,
        }
    } else {
        TerminalOutcome::Split {
            each: totals.seat_0,
            totals,
        }
    }
}

fn malformed_trick_state_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_trick_state".to_owned(),
        message: "a trick must contain exactly two plays before resolution".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::ValidatedAction,
        ids::{TrickCardId, TrickSuit},
        setup::{setup_match, SetupOptions},
    };
    use engine_core::{SeatId, Seed, SeededRng};

    fn action(actor: PlainTricksSeat, card: TrickCardId) -> ValidatedAction {
        ValidatedAction {
            actor,
            card,
            round_index: 0,
            trick_index: 0,
        }
    }

    fn setup_state(seed: u64) -> PlainTricksState {
        setup_match(
            Seed(seed),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    #[test]
    fn trick_winner_uses_led_suit_and_off_suit_never_wins() {
        let leader = TrickPlay {
            seat: PlainTricksSeat::Seat0,
            card: TrickCardId::Gale2,
        };
        let high_follow = TrickPlay {
            seat: PlainTricksSeat::Seat1,
            card: TrickCardId::Gale6,
        };
        let off_suit_follow = TrickPlay {
            seat: PlainTricksSeat::Seat1,
            card: TrickCardId::Ember6,
        };

        assert_eq!(trick_winner(leader, high_follow), PlainTricksSeat::Seat1);
        assert_eq!(
            trick_winner(leader, off_suit_follow),
            PlainTricksSeat::Seat0
        );
    }

    #[test]
    fn applying_two_cards_resolves_trick_and_winner_leads_next() {
        let mut state = PlainTricksState::new_after_deal(
            crate::Variant::plain_tricks_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            0,
            PlainTricksSeat::Seat0,
            [vec![TrickCardId::Gale2], vec![TrickCardId::Gale6]],
            Vec::new(),
            SeededRng::from_seed(Seed(0)),
        );

        apply_action(
            &mut state,
            action(PlainTricksSeat::Seat0, TrickCardId::Gale2),
        )
        .expect("leader play applies");
        assert_eq!(state.active_seat, Some(PlainTricksSeat::Seat1));
        assert_eq!(state.current_trick.led_suit, Some(TrickSuit::Gale));

        apply_action(
            &mut state,
            action(PlainTricksSeat::Seat1, TrickCardId::Gale6),
        )
        .expect("follower play applies");
        assert_eq!(state.completed_tricks.len(), 1);
        assert_eq!(state.round_trick_counts.seat_1, 1);
        assert_eq!(state.active_seat, Some(PlainTricksSeat::Seat1));
        assert_eq!(state.trick_index, 1);
    }

    #[test]
    fn round_close_rotates_deal_and_second_round_leader() {
        let mut state = setup_state(3);
        let first_summary = state.stable_internal_summary();

        state.round_trick_counts = TrickCounts {
            seat_0: 5,
            seat_1: 0,
        };
        state.trick_index = 5;
        state.current_trick = CurrentTrick {
            led_suit: Some(TrickSuit::Gale),
            plays: vec![
                TrickPlay {
                    seat: PlainTricksSeat::Seat0,
                    card: TrickCardId::Gale6,
                },
                TrickPlay {
                    seat: PlainTricksSeat::Seat1,
                    card: TrickCardId::Gale1,
                },
            ],
        };

        resolve_current_trick(&mut state).expect("round closes");

        assert_eq!(state.round_index, 1);
        assert_eq!(state.trick_index, 0);
        assert_eq!(state.round_leader, PlainTricksSeat::Seat1);
        assert_eq!(state.active_seat, Some(PlainTricksSeat::Seat1));
        assert_eq!(state.total_trick_counts.seat_0, 6);
        assert_eq!(state.total_trick_counts.seat_1, 0);
        assert_eq!(state.round_trick_counts, TrickCounts::default());
        assert_ne!(state.stable_internal_summary(), first_summary);
        assert_eq!(state.hands[0].len(), 6);
        assert_eq!(state.hands[1].len(), 6);
        assert_eq!(state.tail.len(), 6);
    }

    #[test]
    fn terminal_win_and_split_resolve_after_second_round() {
        assert_eq!(
            resolve_terminal(TrickCounts {
                seat_0: 7,
                seat_1: 5
            }),
            TerminalOutcome::TrickWin {
                winner: PlainTricksSeat::Seat0,
                totals: TrickCounts {
                    seat_0: 7,
                    seat_1: 5
                },
            }
        );
        assert_eq!(
            resolve_terminal(TrickCounts {
                seat_0: 6,
                seat_1: 6
            }),
            TerminalOutcome::Split {
                each: 6,
                totals: TrickCounts {
                    seat_0: 6,
                    seat_1: 6
                },
            }
        );
    }

    #[test]
    fn full_match_terminates_in_twenty_four_validated_plays() {
        let mut state = setup_state(19);
        let mut plays = 0;

        while state.terminal_outcome.is_none() {
            let actor = state
                .active_seat
                .expect("active seat exists before terminal");
            let card = crate::legal_cards(&state, actor)[0];
            apply_action(&mut state, action(actor, card)).expect("legal action applies");
            plays += 1;
        }

        assert_eq!(plays, 24);
        assert_eq!(
            state.total_trick_counts.seat_0 + state.total_trick_counts.seat_1,
            12
        );
        assert_eq!(state.phase, Phase::Terminal);
    }
}
