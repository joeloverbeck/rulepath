use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use river_ledger::{
    apply_action, canonical_deck, legal_action_tree, setup_match, validate_command,
    RiverLedgerSeat, SetupOptions, Street, TerminalOutcome, STANDARD_BIG_BLIND,
    STANDARD_CARD_COUNT, STANDARD_SMALL_BLIND,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(state: &river_ledger::RiverLedgerState, seat: &str, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_segment(state: &mut river_ledger::RiverLedgerState, seat: &str, segment: &str) {
    let action = validate_command(state, &command(state, seat, segment)).expect("valid command");
    apply_action(state, action).expect("apply succeeds");
}

fn legal_segments(state: &river_ledger::RiverLedgerState, seat: &str) -> Vec<String> {
    legal_action_tree(state, &actor(seat))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect()
}

fn standard_state(count: usize) -> river_ledger::RiverLedgerState {
    setup_match(Seed(21), &seats(count), &SetupOptions::default()).expect("setup")
}

fn advance_four_player_hand_to_flop() -> river_ledger::RiverLedgerState {
    let mut state = standard_state(4);
    apply_segment(&mut state, "seat_3", "call");
    apply_segment(&mut state, "seat_0", "call");
    apply_segment(&mut state, "seat_1", "call");
    apply_segment(&mut state, "seat_2", "check");
    state
}

#[test]
fn setup_accepts_three_to_six_seats_and_rejects_other_counts() {
    let options = SetupOptions::default();

    for count in 3..=6 {
        setup_match(Seed(100 + count as u64), &seats(count), &options)
            .unwrap_or_else(|err| panic!("{count} seats should be accepted: {err:?}"));
    }

    for count in [0, 1, 2, 7] {
        let err = setup_match(Seed(200 + count as u64), &seats(count), &options)
            .expect_err("seat count should reject");
        assert_eq!(err.code, "invalid_seat_count");
        assert_eq!(err.message, "river_ledger requires between 3 and 6 seats");
    }
}

#[test]
fn setup_is_deterministic_for_same_seed_and_options() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(42), &seats, &options).expect("second setup");

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_varies_shuffle_for_different_seeds() {
    let options = SetupOptions::default();
    let seats = seats(6);

    let first = setup_match(Seed(42), &seats, &options).expect("first setup");
    let second = setup_match(Seed(43), &seats, &options).expect("second setup");

    assert_ne!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
}

#[test]
fn setup_assigns_button_blinds_active_seat_and_initial_ledger() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(7), &seats(4), &options).expect("setup");

    assert_eq!(state.button, RiverLedgerSeat::from_index(0).unwrap());
    assert_eq!(state.small_blind, RiverLedgerSeat::from_index(1).unwrap());
    assert_eq!(state.big_blind, RiverLedgerSeat::from_index(2).unwrap());
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(3));
    assert_eq!(state.betting.street, Street::Preflop);
    assert_eq!(state.betting.current_to_call, u16::from(STANDARD_BIG_BLIND));
    assert_eq!(
        state.ledger.seats[1].total_contribution,
        u16::from(STANDARD_SMALL_BLIND)
    );
    assert_eq!(
        state.ledger.seats[2].total_contribution,
        u16::from(STANDARD_BIG_BLIND)
    );
    assert_eq!(
        state.ledger.pot_total,
        u16::from(STANDARD_SMALL_BLIND + STANDARD_BIG_BLIND)
    );
}

#[test]
fn setup_deals_unique_hole_cards_reserved_board_and_tail() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(9), &seats(6), &options).expect("setup");

    assert_eq!(state.private_hands_internal().len(), 6);
    assert_eq!(state.community_deck_internal().len(), 5);
    assert_eq!(
        state.deck_tail_internal().len(),
        STANDARD_CARD_COUNT as usize - (6 * 2) - 5
    );

    let mut dealt = BTreeSet::new();
    for hand in state.private_hands_internal() {
        dealt.insert(hand[0]);
        dealt.insert(hand[1]);
    }
    for card in state.community_deck_internal() {
        dealt.insert(*card);
    }
    for card in state.deck_tail_internal() {
        dealt.insert(*card);
    }

    assert_eq!(dealt.len(), canonical_deck().len());
}

#[test]
fn setup_public_summary_exposes_no_hidden_card_identities() {
    let options = SetupOptions::default();
    let state = setup_match(Seed(11), &seats(3), &options).expect("setup");
    let public_summary = state.setup_public_summary();

    assert!(public_summary.contains("hole_counts=seat_0:2 hidden,seat_1:2 hidden,seat_2:2 hidden"));
    assert!(public_summary.contains("reserved_community_count=5"));
    assert!(public_summary.contains("deck_tail_count=41"));

    for card in canonical_deck() {
        assert!(
            !public_summary.contains(&card.id()),
            "public summary leaked {}",
            card.id()
        );
    }
}

#[test]
fn legal_action_generation_uses_active_seat_call_price_and_cap_state() {
    let state = standard_state(4);

    assert_eq!(
        legal_segments(&state, "seat_3"),
        vec!["fold", "call", "raise"]
    );
    assert!(legal_segments(&state, "seat_0").is_empty());

    let tree = legal_action_tree(&state, &actor("seat_3"));
    let call = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "call")
        .expect("call action");
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "required_to_call" && entry.value == "2"));
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "adds_to_pot" && entry.value == "2"));
}

#[test]
fn preflop_calls_and_big_blind_check_advance_to_flop() {
    let mut state = standard_state(4);

    apply_segment(&mut state, "seat_3", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(0));
    apply_segment(&mut state, "seat_0", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    apply_segment(&mut state, "seat_1", "call");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(2));
    assert_eq!(legal_segments(&state, "seat_2"), vec!["check", "raise"]);

    apply_segment(&mut state, "seat_2", "check");

    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Flop
        }
    );
    assert_eq!(state.betting.street, Street::Flop);
    assert_eq!(state.board.len(), 3);
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.betting.current_to_call, 0);
    assert!(state
        .ledger
        .seats
        .iter()
        .all(|entry| entry.street_contribution == 0));
    assert_eq!(state.ledger.pot_total, 8);
}

#[test]
fn flop_fixed_limit_cap_blocks_fourth_raise() {
    let mut state = advance_four_player_hand_to_flop();

    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.betting.current_to_call, 2);
    assert_eq!(state.ledger.pot_total, 10);

    apply_segment(&mut state, "seat_2", "raise");
    apply_segment(&mut state, "seat_3", "raise");
    apply_segment(&mut state, "seat_0", "raise");

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.betting.raises_this_street, 3);
    assert_eq!(state.betting.current_to_call, 8);
    assert_eq!(legal_segments(&state, "seat_1"), vec!["fold", "call"]);

    let diagnostic = validate_command(&state, &command(&state, "seat_1", "raise")).unwrap_err();
    assert_eq!(diagnostic.code, "raise_cap_reached");
}

#[test]
fn turn_and_river_use_big_bet_unit() {
    let mut state = advance_four_player_hand_to_flop();

    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(&mut state, seat, "check");
    }
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Turn
        }
    );
    assert_eq!(state.betting.street.unit(), 4);
    assert_eq!(state.board.len(), 4);

    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.ledger.seats[1].street_contribution, 4);
    assert_eq!(state.betting.current_to_call, 4);

    for seat in ["seat_2", "seat_3", "seat_0"] {
        apply_segment(&mut state, seat, "call");
    }
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::River
        }
    );
    assert_eq!(state.betting.street.unit(), 4);
    assert_eq!(state.board.len(), 5);

    let before = state.ledger.pot_total;
    apply_segment(&mut state, "seat_1", "bet");
    assert_eq!(state.ledger.pot_total, before + 4);
}

#[test]
fn six_seat_preflop_wraparound_order_is_stable() {
    let mut state = standard_state(6);

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(3));
    for expected in [3, 4, 5, 0, 1] {
        let seat = format!("seat_{expected}");
        apply_segment(&mut state, &seat, "call");
        assert_eq!(
            state.active_seat,
            RiverLedgerSeat::from_index((expected + 1) % 6)
        );
    }

    apply_segment(&mut state, "seat_2", "check");
    assert_eq!(
        state.phase,
        river_ledger::Phase::Betting {
            street: Street::Flop
        }
    );
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.ledger.pot_total, 12);
}

#[test]
fn foldout_terminal_awards_pot_without_showdown_reveal() {
    let mut state = standard_state(3);

    apply_segment(&mut state, "seat_0", "fold");
    apply_segment(&mut state, "seat_1", "fold");

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::LastLiveHand {
            winner: RiverLedgerSeat::from_index(2).unwrap(),
            pot_total: 3,
        })
    );

    let terminal = format!("{:?}", state.terminal_outcome);
    for card in canonical_deck() {
        assert!(
            !terminal.contains(&card.id()),
            "terminal foldout leaked {}",
            card.id()
        );
    }
}

#[test]
fn diagnostics_are_fail_closed_and_public_only() {
    let state = standard_state(4);

    let wrong_seat = validate_command(&state, &command(&state, "seat_0", "call")).unwrap_err();
    assert_eq!(wrong_seat.code, "wrong_seat");

    let mut stale = command(&state, "seat_3", "call");
    stale.freshness_token = FreshnessToken(99);
    let stale = validate_command(&state, &stale).unwrap_err();
    assert_eq!(stale.code, "stale_action");

    let mut capped = advance_four_player_hand_to_flop();
    apply_segment(&mut capped, "seat_1", "bet");
    apply_segment(&mut capped, "seat_2", "raise");
    apply_segment(&mut capped, "seat_3", "raise");
    apply_segment(&mut capped, "seat_0", "raise");
    let cap = validate_command(&capped, &command(&capped, "seat_1", "raise")).unwrap_err();
    assert_eq!(cap.code, "raise_cap_reached");

    for diagnostic in [wrong_seat, stale, cap] {
        let rendered = format!("{diagnostic:?}");
        for forbidden in ["hidden", "private", "deck", "hole", "rank", "suit"] {
            assert!(!rendered.contains(forbidden), "{rendered}");
        }
        for card in canonical_deck() {
            assert!(
                !rendered.contains(&card.id()),
                "diagnostic leaked {} in {rendered}",
                card.id()
            );
        }
    }
}

#[test]
fn identical_command_stream_is_deterministic() {
    fn run() -> river_ledger::RiverLedgerState {
        let mut state = standard_state(4);
        for (seat, segment) in [
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
            ("seat_2", "check"),
            ("seat_1", "bet"),
            ("seat_2", "raise"),
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
        ] {
            apply_segment(&mut state, seat, segment);
        }
        state
    }

    let first = run();
    let second = run();

    assert_eq!(
        first.stable_internal_summary(),
        second.stable_internal_summary()
    );
    assert_eq!(first.ledger, second.ledger);
    assert_eq!(first.betting, second.betting);
}

#[test]
fn legal_action_tree_metadata_remains_public_and_stable() {
    let state = standard_state(4);
    let allowed = [
        "action_family",
        "street",
        "street_unit",
        "actor_seat",
        "required_to_call",
        "adds_to_pot",
        "pot_after",
        "raises_remaining",
        "accessibility_copy",
    ];

    for choice in legal_action_tree(&state, &actor("seat_3")).root.choices {
        assert!(choice
            .metadata
            .iter()
            .all(|entry| allowed.contains(&entry.key.as_str())));
        let serialized = format!("{choice:?}");
        for forbidden in ["hidden", "private", "deck", "hole", "rank", "suit"] {
            assert!(!serialized.contains(forbidden), "{serialized}");
        }
    }
}
