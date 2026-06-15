use std::cmp::Ordering;

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use river_ledger::{
    apply_action, compare_evaluations, evaluate_five, legal_action_tree, setup_match,
    validate_command, Card, Phase, Rank, SetupOptions, Suit, TerminalOutcome,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn actor(seat_index: usize) -> Actor {
    Actor {
        seat_id: SeatId(format!("seat_{seat_index}")),
    }
}

fn command(
    state: &river_ledger::RiverLedgerState,
    seat_index: usize,
    segment: String,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat_index),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn next_choice(seed: &mut u64, bound: usize) -> usize {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    (*seed as usize) % bound
}

fn assert_accounting(state: &river_ledger::RiverLedgerState) {
    let total = state
        .ledger
        .seats
        .iter()
        .map(|seat| seat.total_contribution)
        .sum::<u16>();
    assert_eq!(state.ledger.pot_total, total);
    assert!(state
        .ledger
        .seats
        .iter()
        .all(|seat| seat.total_contribution >= seat.street_contribution));

    if state.betting.actors_to_respond.is_empty() && matches!(state.phase, Phase::Betting { .. }) {
        assert!(state
            .ledger
            .seats
            .iter()
            .filter(|seat| seat.status == river_ledger::SeatStatus::Live)
            .all(|seat| seat.street_contribution == state.betting.current_to_call));
    }

    if let Some(TerminalOutcome::Showdown {
        pot_total,
        allocations,
        ..
    }) = &state.terminal_outcome
    {
        assert_eq!(
            allocations.iter().map(|share| share.amount).sum::<u16>(),
            *pot_total
        );
    }
}

#[test]
fn random_legal_action_sequences_preserve_accounting_invariants() {
    for count in 3..=6 {
        for seed in 0..30 {
            let mut state =
                setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("setup");
            let mut choice_seed = seed ^ ((count as u64) << 32);

            for _ in 0..80 {
                assert_accounting(&state);
                let Some(active) = state.active_seat else {
                    break;
                };
                let choices = legal_action_tree(&state, &actor(active.index()))
                    .root
                    .choices;
                if choices.is_empty() {
                    break;
                }
                let segment = choices[next_choice(&mut choice_seed, choices.len())]
                    .segment
                    .clone();
                let action = validate_command(&state, &command(&state, active.index(), segment))
                    .expect("generated legal action validates");
                apply_action(&mut state, action).expect("generated legal action applies");
            }

            assert_accounting(&state);
        }
    }
}

#[test]
fn setup_and_stable_serialization_are_deterministic_for_random_seed_sweep() {
    for count in 3..=6 {
        for seed in 0..50 {
            let left =
                setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("left");
            let right =
                setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("right");

            assert_eq!(
                left.stable_internal_summary(),
                right.stable_internal_summary()
            );
            assert_eq!(
                river_ledger::project_view(&left, &engine_core::Viewer { seat_id: None })
                    .stable_summary(),
                river_ledger::project_view(&right, &engine_core::Viewer { seat_id: None })
                    .stable_summary()
            );
        }
    }
}

#[test]
fn evaluator_comparison_is_antisymmetric_and_transitive_for_sweep() {
    let c = Card::new;
    let hands = [
        evaluate_five([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::King, Suit::Diamonds),
            c(Rank::Nine, Suit::Spades),
            c(Rank::Six, Suit::Hearts),
            c(Rank::Three, Suit::Clubs),
        ]),
        evaluate_five([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::Nine, Suit::Spades),
            c(Rank::Six, Suit::Hearts),
            c(Rank::Three, Suit::Clubs),
        ]),
        evaluate_five([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::Ace, Suit::Diamonds),
            c(Rank::Nine, Suit::Spades),
            c(Rank::Nine, Suit::Hearts),
            c(Rank::Three, Suit::Clubs),
        ]),
        evaluate_five([
            c(Rank::Nine, Suit::Clubs),
            c(Rank::Eight, Suit::Diamonds),
            c(Rank::Seven, Suit::Spades),
            c(Rank::Six, Suit::Hearts),
            c(Rank::Five, Suit::Clubs),
        ]),
        evaluate_five([
            c(Rank::Ace, Suit::Clubs),
            c(Rank::King, Suit::Clubs),
            c(Rank::Queen, Suit::Clubs),
            c(Rank::Jack, Suit::Clubs),
            c(Rank::Ten, Suit::Clubs),
        ]),
    ];

    for left in &hands {
        for right in &hands {
            assert_eq!(
                compare_evaluations(left, right),
                compare_evaluations(right, left).reverse()
            );
        }
    }

    for a in 0..hands.len() {
        for b in a..hands.len() {
            for c in b..hands.len() {
                if compare_evaluations(&hands[a], &hands[b]) != Ordering::Greater
                    && compare_evaluations(&hands[b], &hands[c]) != Ordering::Greater
                {
                    assert_ne!(compare_evaluations(&hands[a], &hands[c]), Ordering::Greater);
                }
            }
        }
    }
}
