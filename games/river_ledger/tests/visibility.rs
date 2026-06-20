use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, Viewer,
};
use river_ledger::state::SeatRoles;
use river_ledger::{
    apply_action, filter_effects_for_viewer, legal_action_tree, project_view, setup_effects,
    setup_match, validate_command, Card, Rank, RiverLedgerEffect, RiverLedgerSeat, SeatLedger,
    SeatStatus, SetupOptions, Suit, TerminalOutcome, Variant,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn viewer(index: Option<usize>) -> Viewer {
    Viewer {
        seat_id: index.map(|value| SeatId(format!("seat_{value}"))),
    }
}

fn actor(index: usize) -> Actor {
    Actor {
        seat_id: SeatId(format!("seat_{index}")),
    }
}

fn command(state: &river_ledger::RiverLedgerState, seat: usize, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_segment(
    state: &mut river_ledger::RiverLedgerState,
    seat: usize,
    segment: &str,
) -> Vec<engine_core::EffectEnvelope<RiverLedgerEffect>> {
    let action = validate_command(state, &command(state, seat, segment)).expect("valid action");
    apply_action(state, action).expect("action applies")
}

fn private_ids_for(state: &river_ledger::RiverLedgerState, seat: usize) -> Vec<String> {
    state
        .private_hand_for_internal(RiverLedgerSeat::from_index(seat).unwrap())
        .unwrap()
        .into_iter()
        .map(|card| card.id())
        .collect()
}

fn unrevealed_public_ids(state: &river_ledger::RiverLedgerState) -> Vec<String> {
    state
        .community_deck_internal()
        .iter()
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

fn seat(index: usize) -> RiverLedgerSeat {
    RiverLedgerSeat::from_index(index).unwrap()
}

fn showdown_state_with_folded_seat(count: usize) -> river_ledger::RiverLedgerState {
    let hands = [
        [
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Diamonds),
        ],
        [
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ],
        [
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        ],
        [
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Clubs),
        ],
        [
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
        ],
        [
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
        ],
    ];
    let board = [
        Card::new(Rank::Ten, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Hearts),
        Card::new(Rank::Queen, Suit::Hearts),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::Ace, Suit::Hearts),
    ];
    let mut state = river_ledger::RiverLedgerState::new_after_setup(
        Variant::river_ledger_standard(),
        seats(count),
        SeatRoles {
            button: seat(0),
            small_blind: seat(1),
            big_blind: seat(2),
            active_seat: seat(0),
        },
        vec![river_ledger::STANDARD_STARTING_STACK; count],
        hands[..count].to_vec(),
        board,
        Vec::new(),
    );
    state.board = board.to_vec();
    state.ledger.seats = (0..count)
        .map(|index| SeatLedger {
            seat: seat(index),
            status: if index == 0 {
                SeatStatus::Folded
            } else {
                SeatStatus::ShowdownEligible
            },
            starting_stack: river_ledger::STANDARD_STARTING_STACK,
            remaining_stack: river_ledger::STANDARD_STARTING_STACK - 3,
            street_contribution: 0,
            total_contribution: 3,
        })
        .collect();
    state.ledger.pot_total = (count as u16) * 3;
    state.terminal_outcome = Some(river_ledger::resolve_showdown(&state));
    state
}

fn assert_absent(text: &str, forbidden: &[String], context: &str) {
    for value in forbidden {
        assert!(
            !text.contains(value),
            "{context} leaked hidden value {value}"
        );
    }
}

#[test]
fn pairwise_seat_views_and_effects_hide_other_seats_private_cards_for_all_counts() {
    for count in 3..=6 {
        let state = setup_match(
            Seed(100 + count as u64),
            &seats(count),
            &SetupOptions::default(),
        )
        .expect("setup");
        let effects = setup_effects(&state);

        for owner in 0..count {
            let owner_private = private_ids_for(&state, owner);
            let owner_seat = RiverLedgerSeat::from_index(owner).unwrap();
            for recipient in 0..count {
                if recipient == owner {
                    continue;
                }

                let projection = project_view(&state, &viewer(Some(recipient)));
                assert_absent(
                    &format!("{projection:?}"),
                    &owner_private,
                    &format!("seat_{recipient} projection for {}", owner_seat.as_str()),
                );

                let scoped_effects = filter_effects_for_viewer(&effects, &viewer(Some(recipient)));
                assert_absent(
                    &format!("{scoped_effects:?}"),
                    &owner_private,
                    &format!("seat_{recipient} effects for {}", owner_seat.as_str()),
                );
            }
        }
    }
}

#[test]
fn observer_projection_effects_and_action_tree_hide_all_private_and_future_cards() {
    for count in 3..=6 {
        let state = setup_match(
            Seed(200 + count as u64),
            &seats(count),
            &SetupOptions::default(),
        )
        .expect("setup");
        let mut hidden = unrevealed_public_ids(&state);
        for seat in 0..count {
            hidden.extend(private_ids_for(&state, seat));
        }

        let observer = viewer(None);
        let projection = project_view(&state, &observer);
        assert_eq!(projection.ui.seat_labels.len(), 6);
        assert_eq!(projection.active_seat_labels.len(), count);
        assert_eq!(
            projection
                .active_seat_labels
                .iter()
                .map(|label| label.seat.as_str())
                .collect::<Vec<_>>(),
            (0..count)
                .map(|index| format!("seat_{index}"))
                .collect::<Vec<_>>()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        );
        assert_eq!(projection.ui.seat_labels[0].seat, "seat_0");
        assert_eq!(projection.ui.seat_labels[0].label, "Seat 1");
        assert_eq!(projection.ui.seat_labels[5].seat, "seat_5");
        assert_eq!(projection.ui.seat_labels[5].label, "Seat 6");
        assert_absent(
            &format!("{projection:?}"),
            &hidden,
            &format!("{count}-seat observer projection"),
        );

        let scoped_effects = filter_effects_for_viewer(&setup_effects(&state), &observer);
        assert_absent(
            &format!("{scoped_effects:?}"),
            &hidden,
            &format!("{count}-seat observer effects"),
        );

        let active = state.active_seat.expect("setup has active seat").index();
        let tree = legal_action_tree(&state, &actor(active));
        assert_absent(
            &format!("{tree:?}"),
            &hidden,
            &format!("{count}-seat action tree"),
        );
    }
}

#[test]
fn active_seat_labels_are_viewer_safe_and_identical_for_all_viewers() {
    for count in 3..=6 {
        let state = setup_match(
            Seed(700 + count as u64),
            &seats(count),
            &SetupOptions::default(),
        )
        .expect("setup");
        let observer = project_view(&state, &viewer(None));
        let observer_labels = observer.active_seat_labels.clone();
        let expected = (0..count)
            .map(|index| (format!("seat_{index}"), format!("Seat {}", index + 1)))
            .collect::<Vec<_>>();

        assert_eq!(
            observer_labels
                .iter()
                .map(|label| (label.seat.clone(), label.label.clone()))
                .collect::<Vec<_>>(),
            expected
        );

        for viewer_index in 0..count {
            let seat_view = project_view(&state, &viewer(Some(viewer_index)));
            assert_eq!(seat_view.active_seat_labels, observer_labels);

            let labels_debug = format!("{:?}", seat_view.active_seat_labels);
            for private_id in private_ids_for(&state, viewer_index) {
                assert!(!labels_debug.contains(&private_id));
            }
            for future_id in unrevealed_public_ids(&state) {
                assert!(!labels_debug.contains(&future_id));
            }
        }
    }
}

#[test]
fn stack_pot_projection_and_effects_are_public_accounting_only() {
    let mut state = setup_match(
        Seed(91),
        &seats(3),
        &SetupOptions {
            starting_stacks: Some(vec![8, 3, 2]),
            ..SetupOptions::default()
        },
    )
    .expect("setup");

    let raise_effects = apply_segment(&mut state, 0, "raise");
    assert!(raise_effects.iter().any(|effect| matches!(
        effect.payload,
        RiverLedgerEffect::StackChanged { seat: changed_seat, .. } if changed_seat == seat(0)
    )));

    let call_effects = apply_segment(&mut state, 1, "call");
    let effect_names = call_effects
        .iter()
        .map(|effect| match &effect.payload {
            RiverLedgerEffect::StackChanged { .. } => "stack",
            RiverLedgerEffect::SeatBecameAllIn { .. } => "all_in",
            RiverLedgerEffect::UncalledContributionReturned { .. } => "return",
            RiverLedgerEffect::PotResolved { .. } => "pot_resolved",
            RiverLedgerEffect::PotAwarded { .. } => "pot_awarded",
            RiverLedgerEffect::ShowdownResolved { .. } => "showdown",
            _ => "other",
        })
        .collect::<Vec<_>>();

    let stack_index = effect_names
        .iter()
        .position(|name| *name == "stack")
        .expect("stack effect");
    let all_in_index = effect_names
        .iter()
        .position(|name| *name == "all_in")
        .expect("all-in effect");
    let return_index = effect_names
        .iter()
        .position(|name| *name == "return")
        .expect("return effect");
    let pot_index = effect_names
        .iter()
        .position(|name| *name == "pot_resolved")
        .expect("pot resolved effect");
    assert!(stack_index < all_in_index);
    assert!(all_in_index < return_index);
    assert!(return_index < pot_index);

    let view = project_view(&state, &viewer(None));
    assert_eq!(view.seats[0].starting_stack, 8);
    assert_eq!(view.seats[0].remaining_stack, 5);
    assert!(!view.seats[1].is_all_in);
    assert_eq!(view.pot_tiers.len(), 2);
    assert_eq!(
        view.pot_tiers.iter().map(|tier| tier.amount).sum::<u16>(),
        8
    );
    assert!(view.uncalled_returns.is_empty());

    let accounting_effects = call_effects
        .iter()
        .filter(|effect| {
            !matches!(
                effect.payload,
                RiverLedgerEffect::ShowdownResolved { .. }
                    | RiverLedgerEffect::PrivateCardsDealt { .. }
            )
        })
        .collect::<Vec<_>>();
    let public_text = format!("{accounting_effects:?}");
    for id in unrevealed_public_ids(&state) {
        assert!(
            !public_text.contains(&id),
            "public accounting projection leaked {id}"
        );
    }
}

#[test]
fn seat_private_projection_exposes_own_cards_but_not_future_public_cards() {
    let state = setup_match(Seed(309), &seats(6), &SetupOptions::default()).expect("setup");
    let future = unrevealed_public_ids(&state);

    for seat in 0..6 {
        let projection = project_view(&state, &viewer(Some(seat)));
        let text = format!("{projection:?}");

        for private_id in private_ids_for(&state, seat) {
            assert!(text.contains(&private_id));
        }
        assert_absent(&text, &future, &format!("seat_{seat} future-card view"));
    }
}

#[test]
fn wrong_seat_and_stale_diagnostics_are_public_only() {
    let state = setup_match(Seed(410), &seats(4), &SetupOptions::default()).expect("setup");
    let mut hidden = unrevealed_public_ids(&state);
    for seat in 0..4 {
        hidden.extend(private_ids_for(&state, seat));
    }

    let wrong = validate_command(&state, &command(&state, 0, "call")).unwrap_err();
    assert_eq!(wrong.code, "wrong_seat");
    assert_absent(&format!("{wrong:?}"), &hidden, "wrong-seat diagnostic");

    let mut stale = command(&state, state.active_seat.unwrap().index(), "call");
    stale.freshness_token = FreshnessToken(99);
    let stale = validate_command(&state, &stale).unwrap_err();
    assert_eq!(stale.code, "stale_action");
    assert_absent(&format!("{stale:?}"), &hidden, "stale diagnostic");
}

#[test]
fn showdown_explanation_projection_hides_folded_private_cards_for_all_viewers() {
    for count in 3..=6 {
        let state = showdown_state_with_folded_seat(count);
        let folded_private = private_ids_for(&state, 0);

        for recipient in None.into_iter().chain((1..count).map(Some)) {
            let projection = project_view(&state, &viewer(recipient));
            let text = format!("{projection:?}");
            assert_absent(
                &text,
                &folded_private,
                &format!("{count}-seat showdown explanation projection"),
            );
            let river_ledger::visibility::TerminalView::Showdown {
                presentation_v2, ..
            } = &projection.terminal
            else {
                panic!("showdown terminal projection expected");
            };
            assert!(presentation_v2
                .standings
                .iter()
                .all(|standing| standing.seat != seat(0)));
            assert_eq!(presentation_v2.folded_rows.len(), 1);
            assert_eq!(presentation_v2.folded_rows[0].seat, seat(0));
            assert!(presentation_v2.folded_rows[0]
                .redaction_label
                .contains("hand remains hidden"));

            let rationale = projection
                .terminal_rationale
                .as_ref()
                .expect("terminal rationale");
            assert!(rationale
                .headline
                .as_deref()
                .is_some_and(|headline| headline.contains("split the ledger")));
            assert!(rationale
                .headline
                .as_deref()
                .is_some_and(|headline| !headline.contains("seat_")));
            assert!(rationale.decisive_comparison.is_some());
            assert!(rationale
                .decisive_comparison
                .as_deref()
                .is_some_and(|comparison| !comparison.contains("seat_")));
            assert!(rationale.comparison_basis.is_some());

            let folded = rationale
                .per_seat
                .iter()
                .find(|entry| entry.seat == seat(0))
                .expect("folded seat breakdown");
            assert_eq!(folded.result, "folded");
            assert!(folded.strength.is_none());
            let folded_seat = projection
                .seats
                .iter()
                .find(|entry| entry.seat == seat(0))
                .expect("folded public seat");
            assert_eq!(
                folded_seat.ledger_display.hole_card_summary.value,
                "2 hidden"
            );

            for revealed in rationale
                .per_seat
                .iter()
                .filter(|entry| entry.seat != seat(0))
            {
                let strength = revealed.strength.as_ref().expect("revealed strength");
                assert_eq!(strength.result_label, "Split win");
                assert_eq!(strength.hand_name, "Ace-high straight flush");
                assert_eq!(strength.category_ladder_position.position, 1);
                assert_eq!(strength.category_ladder_position.total, 9);
                assert_eq!(
                    strength.category_ladder_position.description,
                    "Straight flush is category 1 of 9 from strongest to weakest."
                );
                assert_eq!(
                    strength.best_five_accessibility_label,
                    "Best five cards: ace of hearts, king of hearts, queen of hearts, jack of hearts, ten of hearts."
                );
                let revealed_seat = projection
                    .seats
                    .iter()
                    .find(|entry| entry.seat == revealed.seat)
                    .expect("revealed public seat");
                assert_eq!(
                    revealed_seat.ledger_display.hole_card_summary.value,
                    "2 revealed"
                );
            }

            if let Some(TerminalOutcome::Showdown { explanations, .. }) =
                state.terminal_outcome.as_ref()
            {
                assert!(explanations
                    .iter()
                    .find(|entry| entry.seat == seat(0))
                    .expect("folded explanation")
                    .revealed
                    .is_none());
            }
        }
    }
}
