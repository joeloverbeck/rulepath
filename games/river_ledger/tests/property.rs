use std::cmp::Ordering;

use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use river_ledger::{
    apply_action, compare_evaluations, evaluate_five, legal_action_tree, setup_match,
    validate_command, Card, Phase, Rank, RiverLedgerSeat, SeatLedger, SeatStatus, SetupOptions,
    Suit, TerminalOutcome,
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

fn seat(index: usize) -> RiverLedgerSeat {
    RiverLedgerSeat::from_index(index).unwrap()
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

fn assert_setup_stack_conservation(state: &river_ledger::RiverLedgerState) {
    let starting = state
        .ledger
        .seats
        .iter()
        .map(|seat| u32::from(seat.starting_stack))
        .sum::<u32>();
    let remaining_plus_contributed = state
        .ledger
        .seats
        .iter()
        .map(|seat| u32::from(seat.remaining_stack) + u32::from(seat.total_contribution))
        .sum::<u32>();
    assert_eq!(starting, remaining_plus_contributed);
    assert!(state
        .ledger
        .seats
        .iter()
        .filter(|seat| seat.status == river_ledger::SeatStatus::AllIn)
        .all(|seat| seat.remaining_stack == 0));
}

fn assert_layer_properties(ledgers: &[SeatLedger]) {
    for ledger in ledgers {
        assert!(ledger.total_contribution <= ledger.starting_stack);
        assert_eq!(
            ledger.remaining_stack + ledger.total_contribution,
            ledger.starting_stack
        );
    }

    let layers = river_ledger::pot::construct_contribution_layers(ledgers);
    let input_total = ledgers
        .iter()
        .map(|ledger| ledger.total_contribution)
        .sum::<u16>();
    let output_total = layers.pots.iter().map(|pot| pot.amount).sum::<u16>()
        + layers
            .returns
            .iter()
            .map(|returned| returned.amount)
            .sum::<u16>();

    assert_eq!(input_total, output_total);
    assert!(layers
        .pots
        .windows(2)
        .all(|pair| pair[0].upper_cap < pair[1].upper_cap));
    assert!(layers
        .pots
        .windows(2)
        .all(|pair| pair[0].eligible != pair[1].eligible));

    for (index, pot) in layers.pots.iter().enumerate() {
        assert_eq!(
            pot.id,
            if index == 0 {
                "main_pot".to_owned()
            } else {
                format!("side_pot_{index}")
            }
        );
        assert!(pot.lower_cap < pot.upper_cap);
        assert!(pot.contributors.len() > 1);
        assert!(!pot.eligible.is_empty());
        assert!(pot.contributors.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(pot.eligible.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(pot.eligible.iter().all(|eligible| {
            let ledger = &ledgers[eligible.index()];
            ledger.total_contribution > pot.lower_cap && ledger.status != SeatStatus::Folded
        }));
        assert!(pot.contributors.iter().all(|contributor| {
            ledgers[contributor.index()].total_contribution > pot.lower_cap
        }));
    }

    for returned in &layers.returns {
        assert!(ledgers[returned.seat.index()].total_contribution > 0);
        assert!(returned.amount > 0);
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
            assert_setup_stack_conservation(&left);
            assert_setup_stack_conservation(&right);

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

#[test]
fn single_pot_allocation_preserves_canonical_winners_and_button_remainders() {
    for seat_count in 3..=6 {
        let all_seats = (0..seat_count).map(seat).collect::<Vec<_>>();
        for mask in 1usize..(1usize << seat_count) {
            let winners = all_seats
                .iter()
                .enumerate()
                .filter_map(|(index, seat)| ((mask & (1usize << index)) != 0).then_some(*seat))
                .collect::<Vec<_>>();
            for button_index in 0..seat_count {
                for pot_total in 1..=31 {
                    let button = seat(button_index);
                    let allocation = river_ledger::pot::allocate_single_pot(
                        pot_total,
                        &winners,
                        button,
                        seat_count as u8,
                    );
                    let button_order = river_ledger::pot::winners_in_button_order(
                        &winners,
                        button,
                        seat_count as u8,
                    );
                    let remainder_recipients = button_order
                        .iter()
                        .take(allocation.remainder as usize)
                        .copied()
                        .collect::<Vec<_>>();

                    assert_eq!(allocation.winners, winners);
                    assert_eq!(allocation.remainder_order, button_order);
                    assert_eq!(allocation.shares.len(), winners.len());
                    assert_eq!(
                        allocation
                            .shares
                            .iter()
                            .map(|share| share.seat)
                            .collect::<Vec<_>>(),
                        winners
                    );
                    assert_eq!(
                        allocation
                            .shares
                            .iter()
                            .map(|share| share.amount)
                            .sum::<u16>(),
                        pot_total
                    );
                    for share in &allocation.shares {
                        assert!(winners.contains(&share.seat));
                        assert_eq!(
                            share.amount,
                            pot_total / winners.len() as u16
                                + u16::from(remainder_recipients.contains(&share.seat))
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn contribution_layer_constructor_conserves_and_orders_generated_profiles() {
    for seat_count in 3..=6 {
        for profile in 0..64u16 {
            let ledgers = (0..seat_count)
                .map(|index| {
                    let total_contribution = 2 + ((profile + (index as u16 * 3)) % 5) * 2;
                    SeatLedger {
                        seat: seat(index),
                        status: if index + 1 == seat_count && profile % 2 == 0 {
                            SeatStatus::Folded
                        } else {
                            SeatStatus::ShowdownEligible
                        },
                        starting_stack: 24,
                        remaining_stack: 24 - total_contribution,
                        street_contribution: 0,
                        total_contribution,
                    }
                })
                .collect::<Vec<_>>();

            assert_layer_properties(&ledgers);
        }
    }
}

#[test]
fn contribution_layer_properties_cover_bounded_validity_classes() {
    for seat_count in 3..=6 {
        for profile in 0..96u16 {
            let ledgers = (0..seat_count)
                .map(|index| {
                    let starting_stack = 6 + ((profile + index as u16) % 19);
                    let total_contribution =
                        1 + ((profile * 3 + index as u16 * 5) % starting_stack);
                    let status = if index == seat_count - 1 && profile % 4 == 0 {
                        SeatStatus::Folded
                    } else if total_contribution == starting_stack {
                        SeatStatus::AllIn
                    } else {
                        SeatStatus::ShowdownEligible
                    };
                    SeatLedger {
                        seat: seat(index),
                        status,
                        starting_stack,
                        remaining_stack: starting_stack - total_contribution,
                        street_contribution: 0,
                        total_contribution,
                    }
                })
                .collect::<Vec<_>>();

            assert_layer_properties(&ledgers);
        }
    }
}

#[test]
fn layered_pot_allocation_conserves_each_pot_and_aggregate_total() {
    for seat_count in 3..=6 {
        for profile in 0..32u16 {
            let ledgers = (0..seat_count)
                .map(|index| {
                    let total_contribution = 2 + ((profile + (index as u16 * 5)) % 6);
                    SeatLedger {
                        seat: seat(index),
                        status: if index == seat_count - 1 && profile % 3 == 0 {
                            SeatStatus::Folded
                        } else {
                            SeatStatus::ShowdownEligible
                        },
                        starting_stack: 24,
                        remaining_stack: 24 - total_contribution,
                        street_contribution: 0,
                        total_contribution,
                    }
                })
                .collect::<Vec<_>>();
            let layers = river_ledger::pot::construct_contribution_layers(&ledgers);
            let winners_by_pot = layers
                .pots
                .iter()
                .map(|pot| (pot.id.clone(), vec![pot.eligible[0]]))
                .collect::<Vec<_>>();
            let allocation = river_ledger::pot::allocate_layered_pots(
                layers,
                &winners_by_pot,
                seat(0),
                seat_count as u8,
            );

            for pot in &allocation.per_pot {
                assert_eq!(
                    pot.shares.iter().map(|share| share.amount).sum::<u16>(),
                    pot.amount
                );
                assert!(pot
                    .winners
                    .iter()
                    .all(|winner| pot.eligible.contains(winner)));
            }

            let aggregate_total = allocation
                .aggregate_shares
                .iter()
                .map(|share| share.amount)
                .sum::<u16>();
            let returned_total = allocation
                .returns
                .iter()
                .map(|returned| returned.amount)
                .sum::<u16>();
            assert_eq!(aggregate_total, allocation.pot_total + returned_total);
        }
    }
}
