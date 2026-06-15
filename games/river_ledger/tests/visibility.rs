use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, Viewer,
};
use river_ledger::state::SeatRoles;
use river_ledger::{
    filter_effects_for_viewer, legal_action_tree, project_view, setup_effects, setup_match,
    validate_command, Card, Rank, RiverLedgerSeat, SeatLedger, SeatStatus, SetupOptions, Suit,
    TerminalOutcome, Variant,
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

            let rationale = projection
                .terminal_rationale
                .as_ref()
                .expect("terminal rationale");
            assert!(rationale
                .headline
                .as_deref()
                .is_some_and(|headline| headline.contains("split the ledger")));
            assert!(rationale.decisive_comparison.is_some());
            assert!(rationale.comparison_basis.is_some());

            let folded = rationale
                .per_seat
                .iter()
                .find(|entry| entry.seat == seat(0))
                .expect("folded seat breakdown");
            assert_eq!(folded.result, "folded");
            assert!(folded.strength.is_none());

            for revealed in rationale
                .per_seat
                .iter()
                .filter(|entry| entry.seat != seat(0))
            {
                let strength = revealed.strength.as_ref().expect("revealed strength");
                assert_eq!(strength.result_label, "Split win");
                assert_eq!(strength.hand_name, "Ace-high straight flush");
                assert_eq!(
                    strength.best_five_accessibility_label,
                    "Best five cards: ace of hearts, king of hearts, queen of hearts, jack of hearts, ten of hearts."
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
