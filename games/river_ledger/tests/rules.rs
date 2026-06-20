use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use river_ledger::state::SeatRoles;
use river_ledger::{
    apply_action, canonical_deck, legal_action_tree, setup_match, validate_command,
    BettingRoundState, Card, PotShare, Rank, RiverLedgerSeat, SeatLedger, SeatStatus, SetupOptions,
    Street, Suit, TerminalOutcome, Variant, MAX_STARTING_STACK, STANDARD_BIG_BLIND,
    STANDARD_CARD_COUNT, STANDARD_SMALL_BLIND, STANDARD_STARTING_STACK,
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

fn flop_state_with_remaining(remaining: [u16; 4]) -> river_ledger::RiverLedgerState {
    let mut state = standard_state(4);
    state.phase = river_ledger::Phase::Betting {
        street: Street::Flop,
    };
    state.active_seat = RiverLedgerSeat::from_index(1);
    for (index, seat_ledger) in state.ledger.seats.iter_mut().enumerate() {
        seat_ledger.street_contribution = 0;
        seat_ledger.remaining_stack = remaining[index];
        seat_ledger.starting_stack = seat_ledger.total_contribution + remaining[index];
        seat_ledger.status = if remaining[index] == 0 {
            SeatStatus::AllIn
        } else {
            SeatStatus::Live
        };
    }
    state.betting =
        BettingRoundState::for_street(Street::Flop, vec![seat(1), seat(2), seat(3), seat(0)]);
    state
}

fn state_with_stacks(stacks: Vec<u16>) -> river_ledger::RiverLedgerState {
    let count = stacks.len();
    setup_match(
        Seed(21),
        &seats(count),
        &SetupOptions {
            starting_stacks: Some(stacks),
            ..SetupOptions::default()
        },
    )
    .expect("setup")
}

fn seeded_state(seed: u64, count: usize) -> river_ledger::RiverLedgerState {
    setup_match(Seed(seed), &seats(count), &SetupOptions::default()).expect("setup")
}

fn advance_four_player_hand_to_flop() -> river_ledger::RiverLedgerState {
    let mut state = standard_state(4);
    apply_segment(&mut state, "seat_3", "call");
    apply_segment(&mut state, "seat_0", "call");
    apply_segment(&mut state, "seat_1", "call");
    apply_segment(&mut state, "seat_2", "check");
    state
}

fn check_down_from_flop_to_terminal(state: &mut river_ledger::RiverLedgerState) {
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
    for seat in ["seat_1", "seat_2", "seat_3", "seat_0"] {
        apply_segment(state, seat, "check");
    }
}

fn royal_board() -> Vec<Card> {
    vec![
        Card::new(Rank::Ace, Suit::Clubs),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Clubs),
        Card::new(Rank::Jack, Suit::Clubs),
        Card::new(Rank::Ten, Suit::Clubs),
    ]
}

fn seat(index: usize) -> RiverLedgerSeat {
    RiverLedgerSeat::from_index(index).unwrap()
}

fn custom_showdown_state(
    private_hands: Vec<[Card; 2]>,
    board: [Card; 5],
) -> river_ledger::RiverLedgerState {
    let mut state = river_ledger::RiverLedgerState::new_after_setup(
        Variant::river_ledger_standard(),
        seats(private_hands.len()),
        SeatRoles {
            button: seat(0),
            small_blind: seat(1),
            big_blind: seat(2),
            active_seat: seat(0),
        },
        vec![STANDARD_STARTING_STACK; private_hands.len()],
        private_hands,
        board,
        Vec::new(),
    );
    state.board = board.to_vec();
    state.ledger.seats = (0..state.seats.len())
        .map(|index| SeatLedger {
            seat: seat(index),
            status: SeatStatus::ShowdownEligible,
            starting_stack: STANDARD_STARTING_STACK,
            remaining_stack: STANDARD_STARTING_STACK - 3,
            street_contribution: 0,
            total_contribution: 3,
        })
        .collect();
    state.ledger.pot_total = 12;
    state
}

fn public_label(seat: RiverLedgerSeat) -> String {
    format!("Seat {}", seat.index() + 1)
}

fn assert_showdown_surface_agreement(outcome: &TerminalOutcome) {
    let TerminalOutcome::Showdown {
        winners,
        pot_total,
        allocations,
        headline,
        decisive_comparison: _,
        comparison_basis,
        explanations,
        presentation_v2,
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert!(!winners.is_empty(), "showdown winner set is nonempty");
    assert_eq!(
        allocations
            .iter()
            .map(|share| share.seat)
            .collect::<Vec<_>>(),
        *winners,
        "allocations serialize in canonical winner order"
    );
    assert_eq!(
        allocations.iter().map(|share| share.amount).sum::<u16>(),
        *pot_total,
        "allocations conserve the ledger"
    );
    assert!(allocations.iter().all(|share| share.amount > 0));
    assert_eq!(presentation_v2.result_banner.headline, *headline);
    assert_eq!(
        presentation_v2.decisive_reason.short_text,
        *comparison_basis
    );

    for standing in &presentation_v2.standings {
        let winner = winners.contains(&standing.seat);
        assert_eq!(standing.default_expanded, winner);
        assert_eq!(
            standing.result_label,
            if winner {
                if winners.len() > 1 {
                    "Split win"
                } else {
                    "Win"
                }
            } else {
                "Showdown loss"
            }
        );
        assert_eq!(standing.seat_label, public_label(standing.seat));
    }

    if winners.len() == 1 {
        let winner_label = public_label(winners[0]);
        assert!(headline.contains(&winner_label));
        assert!(presentation_v2
            .result_banner
            .accessibility_label
            .contains(&winner_label));
    } else {
        assert!(headline.contains("split the ledger"));
        assert!(presentation_v2
            .result_banner
            .accessibility_label
            .contains("split the ledger"));
        for winner in winners {
            assert!(headline.contains(&public_label(*winner)));
        }
        for explanation in explanations {
            if !winners.contains(&explanation.seat) {
                assert!(!headline.contains(&public_label(explanation.seat)));
            }
        }
    }
}

fn assert_stack_conservation(state: &river_ledger::RiverLedgerState) {
    let starting = state
        .ledger
        .seats
        .iter()
        .map(|ledger| ledger.starting_stack)
        .sum::<u16>();
    let remaining_plus_contributed = state
        .ledger
        .seats
        .iter()
        .map(|ledger| ledger.remaining_stack + ledger.total_contribution)
        .sum::<u16>();
    assert_eq!(starting, remaining_plus_contributed);
    assert!(state
        .ledger
        .seats
        .iter()
        .filter(|ledger| ledger.status == SeatStatus::AllIn)
        .all(|ledger| ledger.remaining_stack == 0));
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
        state.ledger.seats[1].starting_stack,
        STANDARD_STARTING_STACK
    );
    assert_eq!(
        state.ledger.seats[1].remaining_stack,
        STANDARD_STARTING_STACK - u16::from(STANDARD_SMALL_BLIND)
    );
    assert_eq!(
        state.ledger.seats[2].total_contribution,
        u16::from(STANDARD_BIG_BLIND)
    );
    assert_eq!(
        state.ledger.seats[2].starting_stack,
        STANDARD_STARTING_STACK
    );
    assert_eq!(
        state.ledger.seats[2].remaining_stack,
        STANDARD_STARTING_STACK - u16::from(STANDARD_BIG_BLIND)
    );
    assert_eq!(
        state.ledger.pot_total,
        u16::from(STANDARD_SMALL_BLIND + STANDARD_BIG_BLIND)
    );
    assert_stack_conservation(&state);
}

#[test]
fn setup_uses_equal_default_starting_stacks() {
    let state = standard_state(3);

    assert_eq!(
        state
            .ledger
            .seats
            .iter()
            .map(|ledger| (ledger.seat.as_str(), ledger.starting_stack))
            .collect::<Vec<_>>(),
        vec![
            ("seat_0".to_owned(), STANDARD_STARTING_STACK),
            ("seat_1".to_owned(), STANDARD_STARTING_STACK),
            ("seat_2".to_owned(), STANDARD_STARTING_STACK),
        ]
    );
    assert!(state
        .stable_internal_summary()
        .contains("stacks=seat_0:24:24,seat_1:24:23,seat_2:24:22"));
    assert_stack_conservation(&state);
}

#[test]
fn setup_accepts_ordered_asymmetric_starting_stacks() {
    let options = SetupOptions {
        starting_stacks: Some(vec![8, 16, 24]),
        ..SetupOptions::default()
    };
    let state = setup_match(Seed(33), &seats(3), &options).expect("asymmetric setup");

    assert_eq!(
        state
            .ledger
            .seats
            .iter()
            .map(|ledger| (
                ledger.seat.as_str(),
                ledger.starting_stack,
                ledger.remaining_stack
            ))
            .collect::<Vec<_>>(),
        vec![
            ("seat_0".to_owned(), 8, 8),
            ("seat_1".to_owned(), 16, 15),
            ("seat_2".to_owned(), 24, 22),
        ]
    );
    assert_stack_conservation(&state);
}

#[test]
fn setup_accepts_six_seat_asymmetric_acceptance_vector() {
    let options = SetupOptions {
        starting_stacks: Some(vec![4, 8, 12, 16, 20, 24]),
        ..SetupOptions::default()
    };
    let state = setup_match(Seed(36), &seats(6), &options).expect("6p asymmetric setup");

    assert_eq!(
        state
            .ledger
            .seats
            .iter()
            .map(|ledger| ledger.starting_stack)
            .collect::<Vec<_>>(),
        vec![4, 8, 12, 16, 20, 24]
    );
}

#[test]
fn setup_rejects_malformed_starting_stacks() {
    let wrong_length = SetupOptions {
        starting_stacks: Some(vec![8, 16]),
        ..SetupOptions::default()
    };
    let err = setup_match(Seed(40), &seats(3), &wrong_length).expect_err("wrong length rejects");
    assert_eq!(err.code, "invalid_starting_stack_count");

    let zero_stack = SetupOptions {
        starting_stacks: Some(vec![8, 0, 24]),
        ..SetupOptions::default()
    };
    let err = setup_match(Seed(41), &seats(3), &zero_stack).expect_err("zero rejects");
    assert_eq!(err.code, "invalid_starting_stack");

    let out_of_range = SetupOptions {
        starting_stacks: Some(vec![8, MAX_STARTING_STACK + 1, 24]),
        ..SetupOptions::default()
    };
    let err = setup_match(Seed(42), &seats(3), &out_of_range).expect_err("range rejects");
    assert_eq!(err.code, "invalid_starting_stack");
}

#[test]
fn setup_caps_short_forced_posts_and_marks_all_in() {
    let short_big_blind = SetupOptions {
        starting_stacks: Some(vec![8, 16, 1]),
        ..SetupOptions::default()
    };
    let state = setup_match(Seed(43), &seats(3), &short_big_blind).expect("short post setup");
    let big_blind = &state.ledger.seats[2];
    assert_eq!(big_blind.starting_stack, 1);
    assert_eq!(big_blind.street_contribution, 1);
    assert_eq!(big_blind.total_contribution, 1);
    assert_eq!(big_blind.remaining_stack, 0);
    assert_eq!(big_blind.status, SeatStatus::AllIn);
    assert_eq!(state.ledger.pot_total, 2);
    assert_eq!(state.betting.current_to_call, 1);
    assert_stack_conservation(&state);

    let short_small_blind = SetupOptions {
        starting_stacks: Some(vec![8, 1, 24]),
        ..SetupOptions::default()
    };
    let state = setup_match(Seed(44), &seats(3), &short_small_blind).expect("short small setup");
    let small_blind = &state.ledger.seats[1];
    assert_eq!(small_blind.starting_stack, 1);
    assert_eq!(small_blind.street_contribution, 1);
    assert_eq!(small_blind.remaining_stack, 0);
    assert_eq!(small_blind.status, SeatStatus::AllIn);
    assert_eq!(state.ledger.pot_total, 3);
    assert_stack_conservation(&state);
}

#[test]
fn setup_exact_blind_exhaustion_is_all_in_without_underflow() {
    let exact_blinds = SetupOptions {
        starting_stacks: Some(vec![8, 1, 2]),
        ..SetupOptions::default()
    };
    let state = setup_match(Seed(45), &seats(3), &exact_blinds).expect("exact blind setup");

    for index in [1, 2] {
        let ledger = &state.ledger.seats[index];
        assert_eq!(ledger.remaining_stack, 0);
        assert_eq!(ledger.status, SeatStatus::AllIn);
        assert_eq!(
            ledger.starting_stack, ledger.total_contribution,
            "forced post consumes the exact stack"
        );
    }
    assert_eq!(state.ledger.pot_total, 3);
    assert_eq!(state.betting.actors_to_respond, vec![seat(0)]);
    assert!(legal_segments(&state, "seat_1").is_empty());
    assert!(legal_segments(&state, "seat_2").is_empty());
    assert_stack_conservation(&state);
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
    assert!(call
        .metadata
        .iter()
        .any(|entry| entry.key == "cap_remaining" && entry.value == "3"));
    assert_eq!(
        presentation_rows(call),
        vec![("Call price", "2"), ("Adds", "2")]
    );

    let fold = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "fold")
        .expect("fold action");
    assert_eq!(presentation_rows(fold), vec![("Adds", "0")]);

    let mut capped = advance_four_player_hand_to_flop();
    apply_segment(&mut capped, "seat_1", "bet");
    apply_segment(&mut capped, "seat_2", "raise");
    apply_segment(&mut capped, "seat_3", "raise");
    apply_segment(&mut capped, "seat_0", "raise");
    let capped_tree = legal_action_tree(&capped, &actor("seat_1"));
    assert!(capped_tree.root.choices.iter().all(|choice| choice
        .metadata
        .iter()
        .any(|entry| entry.key == "cap_remaining" && entry.value == "0")));
}

#[test]
fn short_stack_facing_call_can_fold_or_call_all_in_for_remaining_stack() {
    let mut state = state_with_stacks(vec![1, 16, STANDARD_STARTING_STACK]);

    assert_eq!(legal_segments(&state, "seat_0"), vec!["fold", "call"]);
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let call = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "call")
        .expect("call action");
    assert_eq!(metadata_value(call, "amount_owed"), Some("2"));
    assert_eq!(metadata_value(call, "adds_to_pot"), Some("1"));
    assert_eq!(metadata_value(call, "stack_before"), Some("1"));
    assert_eq!(metadata_value(call, "stack_after"), Some("0"));
    assert_eq!(metadata_value(call, "is_all_in"), Some("true"));
    assert_eq!(metadata_value(call, "is_full_raise"), Some("false"));

    apply_segment(&mut state, "seat_0", "call");

    assert_eq!(state.ledger.seats[0].remaining_stack, 0);
    assert_eq!(state.ledger.seats[0].total_contribution, 1);
    assert_eq!(state.ledger.seats[0].status, SeatStatus::AllIn);
    assert_eq!(state.ledger.pot_total, 4);
}

#[test]
fn exact_stack_call_is_a_full_call_all_in() {
    let mut state = state_with_stacks(vec![2, 16, STANDARD_STARTING_STACK]);
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let call = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "call")
        .expect("call action");

    assert_eq!(metadata_value(call, "amount_owed"), Some("2"));
    assert_eq!(metadata_value(call, "adds_to_pot"), Some("2"));
    assert_eq!(metadata_value(call, "stack_after"), Some("0"));
    assert_eq!(metadata_value(call, "is_all_in"), Some("true"));

    apply_segment(&mut state, "seat_0", "call");

    assert_eq!(state.ledger.seats[0].remaining_stack, 0);
    assert_eq!(state.ledger.seats[0].total_contribution, 2);
    assert_eq!(state.ledger.seats[0].status, SeatStatus::AllIn);
}

#[test]
fn stack_short_of_call_plus_unit_keeps_call_and_short_raise_all_in() {
    let mut state = state_with_stacks(vec![3, 16, STANDARD_STARTING_STACK]);

    assert_eq!(
        legal_segments(&state, "seat_0"),
        vec!["fold", "call", "raise"]
    );
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let raise = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "raise")
        .expect("raise action");
    assert_eq!(metadata_value(raise, "amount_owed"), Some("2"));
    assert_eq!(metadata_value(raise, "adds_to_pot"), Some("3"));
    assert_eq!(metadata_value(raise, "stack_before"), Some("3"));
    assert_eq!(metadata_value(raise, "stack_after"), Some("0"));
    assert_eq!(metadata_value(raise, "is_all_in"), Some("true"));
    assert_eq!(metadata_value(raise, "is_full_raise"), Some("false"));
    assert_eq!(metadata_value(raise, "raise_right_open"), Some("true"));

    apply_segment(&mut state, "seat_0", "raise");

    assert_eq!(state.ledger.seats[0].remaining_stack, 0);
    assert_eq!(state.ledger.seats[0].street_contribution, 3);
    assert_eq!(state.ledger.seats[0].status, SeatStatus::AllIn);
    assert_eq!(state.betting.current_to_call, 3);
}

#[test]
fn exact_stack_raise_is_full_raise_all_in() {
    let mut state = state_with_stacks(vec![4, 16, STANDARD_STARTING_STACK]);
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let raise = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "raise")
        .expect("raise action");

    assert_eq!(metadata_value(raise, "adds_to_pot"), Some("4"));
    assert_eq!(metadata_value(raise, "stack_after"), Some("0"));
    assert_eq!(metadata_value(raise, "is_all_in"), Some("true"));
    assert_eq!(metadata_value(raise, "is_full_raise"), Some("true"));

    apply_segment(&mut state, "seat_0", "raise");

    assert_eq!(state.ledger.seats[0].remaining_stack, 0);
    assert_eq!(state.ledger.seats[0].street_contribution, 4);
    assert_eq!(state.betting.current_to_call, 4);
}

#[test]
fn short_stack_opening_bet_all_in_remains_a_bet_action() {
    let mut state = state_with_stacks(vec![STANDARD_STARTING_STACK, 2, STANDARD_STARTING_STACK]);
    state.phase = river_ledger::Phase::Betting {
        street: Street::Flop,
    };
    state.active_seat = RiverLedgerSeat::from_index(1);
    for seat in &mut state.ledger.seats {
        seat.street_contribution = 0;
    }
    state.betting = BettingRoundState::for_street(Street::Flop, vec![seat(1), seat(2), seat(0)]);

    assert_eq!(state.ledger.seats[1].remaining_stack, 1);
    assert_eq!(legal_segments(&state, "seat_1"), vec!["check", "bet"]);
    let tree = legal_action_tree(&state, &actor("seat_1"));
    let bet = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "bet")
        .expect("bet action");
    assert_eq!(metadata_value(bet, "amount_owed"), Some("0"));
    assert_eq!(metadata_value(bet, "adds_to_pot"), Some("1"));
    assert_eq!(metadata_value(bet, "stack_before"), Some("1"));
    assert_eq!(metadata_value(bet, "stack_after"), Some("0"));
    assert_eq!(metadata_value(bet, "is_all_in"), Some("true"));
    assert_eq!(metadata_value(bet, "is_full_raise"), Some("false"));

    apply_segment(&mut state, "seat_1", "bet");

    assert_eq!(state.ledger.seats[1].remaining_stack, 0);
    assert_eq!(state.ledger.seats[1].street_contribution, 1);
    assert_eq!(state.ledger.seats[1].status, SeatStatus::AllIn);
    assert_eq!(state.betting.current_to_call, 1);
}

#[test]
fn all_all_in_action_sequence_runs_out_to_showdown_without_more_actors() {
    let mut state = state_with_stacks(vec![4, 4, 2]);

    apply_segment(&mut state, "seat_0", "raise");
    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(state.betting.actors_to_respond, vec![seat(1)]);

    apply_segment(&mut state, "seat_1", "call");

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    assert!(state.betting.actors_to_respond.is_empty());
    assert_eq!(state.board.len(), 5);
    assert!(matches!(
        state.terminal_outcome,
        Some(TerminalOutcome::Showdown { .. })
    ));
    assert!(state
        .ledger
        .seats
        .iter()
        .all(|entry| entry.status == SeatStatus::ShowdownEligible));
    assert_eq!(state.ledger.pot_total, 10);
    assert_stack_conservation(&state);
}

#[test]
fn one_live_seat_gets_unmatched_excess_back_before_runout() {
    let mut state = state_with_stacks(vec![8, 3, 2]);

    apply_segment(&mut state, "seat_0", "raise");
    apply_segment(&mut state, "seat_1", "call");

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    assert!(state.betting.actors_to_respond.is_empty());
    assert_eq!(state.board.len(), 5);
    assert!(matches!(
        state.terminal_outcome,
        Some(TerminalOutcome::Showdown { .. })
    ));
    assert_eq!(state.ledger.seats[0].total_contribution, 3);
    assert_eq!(state.ledger.seats[0].remaining_stack, 5);
    assert_eq!(state.ledger.seats[1].total_contribution, 3);
    assert_eq!(state.ledger.seats[2].total_contribution, 2);
    assert_eq!(state.ledger.pot_total, 8);
    assert_stack_conservation(&state);
}

#[test]
fn one_short_all_in_increase_does_not_reopen_already_acted_seat() {
    let mut state = flop_state_with_remaining([STANDARD_STARTING_STACK, 20, 20, 3]);

    apply_segment(&mut state, "seat_1", "bet");
    apply_segment(&mut state, "seat_2", "call");
    apply_segment(&mut state, "seat_3", "raise");
    assert_eq!(state.ledger.seats[3].status, SeatStatus::AllIn);
    assert_eq!(state.betting.current_to_call, 3);
    assert_eq!(state.betting.raises_this_street, 0);

    apply_segment(&mut state, "seat_0", "call");

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(1));
    assert_eq!(legal_segments(&state, "seat_1"), vec!["fold", "call"]);
    let tree = legal_action_tree(&state, &actor("seat_1"));
    let call = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "call")
        .expect("call action");
    assert_eq!(metadata_value(call, "amount_owed"), Some("1"));
    assert_eq!(metadata_value(call, "raise_right_open"), Some("false"));
}

#[test]
fn cumulative_short_all_in_increases_reopen_after_full_unit_pressure() {
    let mut state = flop_state_with_remaining([4, 20, 20, 3]);

    apply_segment(&mut state, "seat_1", "bet");
    apply_segment(&mut state, "seat_2", "call");
    apply_segment(&mut state, "seat_3", "raise");
    apply_segment(&mut state, "seat_0", "raise");
    assert_eq!(state.ledger.seats[0].status, SeatStatus::AllIn);
    assert_eq!(state.betting.current_to_call, 4);
    assert_eq!(state.betting.raises_this_street, 0);

    apply_segment(&mut state, "seat_1", "call");

    assert_eq!(state.active_seat, RiverLedgerSeat::from_index(2));
    assert_eq!(
        legal_segments(&state, "seat_2"),
        vec!["fold", "call", "raise"]
    );
    let tree = legal_action_tree(&state, &actor("seat_2"));
    let raise = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == "raise")
        .expect("raise action");
    assert_eq!(metadata_value(raise, "amount_owed"), Some("2"));
    assert_eq!(metadata_value(raise, "raise_right_open"), Some("true"));

    apply_segment(&mut state, "seat_2", "raise");
    assert_eq!(state.betting.raises_this_street, 1);
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
    state.ledger.seats[1].remaining_stack = 7;
    state.ledger.seats[1].starting_stack = state.ledger.seats[1].total_contribution + 7;
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
fn river_checkdown_resolves_showdown_terminal_and_conserves_pot() {
    let mut state = advance_four_player_hand_to_flop();

    check_down_from_flop_to_terminal(&mut state);

    assert_eq!(state.phase, river_ledger::Phase::Terminal);
    assert_eq!(state.active_seat, None);
    let Some(TerminalOutcome::Showdown {
        winners,
        pot_total,
        allocations,
        explanations,
        presentation_v2,
        ..
    }) = &state.terminal_outcome
    else {
        panic!("showdown terminal expected");
    };

    assert!(!winners.is_empty());
    assert_eq!(*pot_total, state.ledger.pot_total);
    assert_eq!(
        allocations.iter().map(|share| share.amount).sum::<u16>(),
        *pot_total
    );
    assert_eq!(explanations.len(), 4);
    assert!(explanations
        .iter()
        .filter(|explanation| explanation.revealed.is_some())
        .all(|explanation| explanation.status == river_ledger::SeatStatus::ShowdownEligible));
    assert_eq!(presentation_v2.board_cards.len(), 5);
    assert_eq!(presentation_v2.standings.len(), 4);
    assert!(presentation_v2.folded_rows.is_empty());
    assert!(presentation_v2
        .standings
        .iter()
        .any(|standing| standing.default_expanded));
    assert!(presentation_v2
        .standings
        .iter()
        .all(|standing| standing.best_five.len() == 5 && standing.board_cards.len() == 5));
}

#[test]
fn checkdown_can_produce_single_winner_showdown_from_seeded_state() {
    let mut single_winner = None;
    for seed in 0..200 {
        let mut state = seeded_state(seed, 4);
        apply_segment(&mut state, "seat_3", "call");
        apply_segment(&mut state, "seat_0", "call");
        apply_segment(&mut state, "seat_1", "call");
        apply_segment(&mut state, "seat_2", "check");
        check_down_from_flop_to_terminal(&mut state);
        if let Some(TerminalOutcome::Showdown {
            winners,
            allocations,
            ..
        }) = &state.terminal_outcome
        {
            if winners.len() == 1 {
                single_winner = Some((winners[0], allocations.clone(), state.ledger.pot_total));
                break;
            }
        }
    }

    let Some((winner, allocations, pot_total)) = single_winner else {
        panic!("expected a deterministic seed with one showdown winner");
    };
    assert_eq!(
        allocations,
        vec![PotShare {
            seat: winner,
            amount: pot_total,
        }]
    );
}

#[test]
fn seed_10018_showdown_labels_unique_winner_consistently() {
    let mut state = seeded_state(10018, 4);
    apply_segment(&mut state, "seat_3", "call");
    apply_segment(&mut state, "seat_0", "call");
    apply_segment(&mut state, "seat_1", "call");
    apply_segment(&mut state, "seat_2", "check");
    check_down_from_flop_to_terminal(&mut state);

    let Some(TerminalOutcome::Showdown {
        winners,
        pot_total,
        allocations,
        headline,
        decisive_comparison,
        comparison_basis,
        presentation_v2,
        ..
    }) = &state.terminal_outcome
    else {
        panic!("showdown terminal expected");
    };
    assert_showdown_surface_agreement(state.terminal_outcome.as_ref().expect("terminal outcome"));

    assert_eq!(winners, &vec![seat(0)]);
    assert_eq!(
        allocations,
        &vec![PotShare {
            seat: seat(0),
            amount: *pot_total,
        }]
    );
    assert_eq!(headline, "Seat 1 wins with Two pair, Queens and Fives.");
    assert_eq!(
        decisive_comparison,
        "Two pair, Queens and Fives beats Pair of Fives."
    );
    assert_eq!(comparison_basis, "Two pair outranks One pair.");
    assert_eq!(
        presentation_v2
            .decisive_reason
            .contrast_seat_label
            .as_deref(),
        Some("Seat 3")
    );
    assert_eq!(presentation_v2.standings[0].seat, seat(0));
    assert_eq!(presentation_v2.standings[0].seat_label, "Seat 1");
    assert_eq!(presentation_v2.standings[0].result_label, "Win");
}

#[test]
fn tied_showdown_splits_pot_and_reveals_only_showdown_seats() {
    let mut state = standard_state(4);
    state.board = royal_board();
    state.ledger.pot_total = 12;
    for entry in &mut state.ledger.seats {
        entry.status = river_ledger::SeatStatus::ShowdownEligible;
    }
    state.ledger.seats[0].status = river_ledger::SeatStatus::Folded;

    let outcome = river_ledger::resolve_showdown(&state);
    assert_showdown_surface_agreement(&outcome);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        explanations,
        presentation_v2,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(
        winners,
        vec![
            RiverLedgerSeat::from_index(1).unwrap(),
            RiverLedgerSeat::from_index(2).unwrap(),
            RiverLedgerSeat::from_index(3).unwrap(),
        ]
    );
    assert_eq!(
        allocations,
        vec![
            PotShare {
                seat: RiverLedgerSeat::from_index(1).unwrap(),
                amount: 4,
            },
            PotShare {
                seat: RiverLedgerSeat::from_index(2).unwrap(),
                amount: 4,
            },
            PotShare {
                seat: RiverLedgerSeat::from_index(3).unwrap(),
                amount: 4,
            },
        ]
    );
    assert!(explanations
        .iter()
        .find(|explanation| explanation.seat == RiverLedgerSeat::from_index(0).unwrap())
        .expect("folded seat explanation")
        .revealed
        .is_none());
    assert!(explanations
        .iter()
        .filter(|explanation| explanation.seat != RiverLedgerSeat::from_index(0).unwrap())
        .all(|explanation| explanation.revealed.is_some()));
    assert_eq!(presentation_v2.standings.len(), 3);
    assert_eq!(presentation_v2.folded_rows.len(), 1);
    assert_eq!(presentation_v2.folded_rows[0].seat, seat(0));
    assert!(presentation_v2.folded_rows[0]
        .redaction_label
        .contains("hand remains hidden"));
    assert!(presentation_v2
        .standings
        .iter()
        .all(|standing| standing.seat != seat(0)));
    assert!(presentation_v2
        .decisive_reason
        .rule_refs
        .contains(&"RL-SCORE-SPLIT".to_owned()));
}

#[test]
fn side_pot_showdown_allocates_each_pot_to_its_eligible_winner() {
    let board = [
        Card::new(Rank::Two, Suit::Clubs),
        Card::new(Rank::Seven, Suit::Diamonds),
        Card::new(Rank::Nine, Suit::Hearts),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Queen, Suit::Clubs),
    ];
    let mut state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Spades),
            ],
            [
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::King, Suit::Spades),
            ],
            [
                Card::new(Rank::Ten, Suit::Hearts),
                Card::new(Rank::Ten, Suit::Spades),
            ],
        ],
        board,
    );
    for (index, total) in [4, 8, 8].into_iter().enumerate() {
        state.ledger.seats[index].starting_stack = 24;
        state.ledger.seats[index].remaining_stack = 24 - total;
        state.ledger.seats[index].total_contribution = total;
        state.ledger.seats[index].street_contribution = 0;
        state.ledger.seats[index].status = SeatStatus::ShowdownEligible;
    }
    state.ledger.pot_total = 20;

    let TerminalOutcome::Showdown {
        winners,
        allocations,
        pot_total,
        ..
    } = river_ledger::resolve_showdown(&state)
    else {
        panic!("showdown expected");
    };

    assert_eq!(pot_total, 20);
    assert_eq!(winners, vec![seat(0), seat(1)]);
    assert_eq!(
        allocations,
        vec![
            PotShare {
                seat: seat(0),
                amount: 12,
            },
            PotShare {
                seat: seat(1),
                amount: 8,
            },
        ]
    );
}

#[test]
fn showdown_explanation_names_pair_of_queens_beating_pair_of_eights() {
    let state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Ten, Suit::Clubs),
            ],
            [
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::Ten, Suit::Spades),
            ],
            [
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Clubs),
            ],
            [
                Card::new(Rank::King, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
            ],
        ],
        [
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
        ],
    );

    let outcome = river_ledger::resolve_showdown(&state);
    assert_showdown_surface_agreement(&outcome);
    let TerminalOutcome::Showdown {
        winners,
        headline,
        decisive_comparison,
        comparison_basis,
        explanations,
        presentation_v2,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(winners, vec![seat(1)]);
    assert_eq!(headline, "Seat 2 wins with Pair of Queens.");
    assert!(!headline.contains("seat_"));
    assert_eq!(decisive_comparison, "Pair of Queens beats Pair of Eights.");
    assert_eq!(
        comparison_basis,
        "Both hands are one pair, so the pair rank decides first: Queens > Eights."
    );

    let winner = explanations
        .iter()
        .find(|explanation| explanation.seat == seat(1))
        .and_then(|explanation| explanation.revealed.as_ref())
        .expect("winner reveal");
    assert_eq!(winner.category, "one_pair");
    assert_eq!(winner.tie_break_vector, vec![12, 10, 8, 6]);
    assert_eq!(winner.result_label, "Win");
    assert_eq!(winner.hand_name, "Pair of Queens");
    assert_eq!(
        winner.rank_explanation,
        "pair rank Queen; kickers Ten, Eight, Six"
    );
    assert_eq!(
        winner.comparison_note,
        "Pair of Queens beats Pair of Eights."
    );
    assert_eq!(
        winner.best_five_accessibility_label,
        "Best five cards: queen of clubs, queen of hearts, ten of spades, eight of hearts, six of hearts."
    );

    let closest_loser = explanations
        .iter()
        .find(|explanation| explanation.seat == seat(0))
        .and_then(|explanation| explanation.revealed.as_ref())
        .expect("closest loser reveal");
    assert_eq!(closest_loser.result_label, "Showdown loss");
    assert_eq!(closest_loser.hand_name, "Pair of Eights");
    assert_eq!(
        closest_loser.comparison_note,
        "Pair of Eights loses to Pair of Queens."
    );
    assert_eq!(presentation_v2.result_banner.headline, headline);
    assert_eq!(
        presentation_v2.result_banner.subheadline,
        decisive_comparison
    );
    assert_eq!(presentation_v2.decisive_reason.short_text, comparison_basis);
    assert_eq!(presentation_v2.decisive_reason.contrast_seat, Some(seat(0)));
    assert_eq!(
        presentation_v2
            .decisive_reason
            .contrast_seat_label
            .as_deref(),
        Some("Seat 1")
    );
    assert_eq!(presentation_v2.standings[0].seat, seat(1));
    assert_eq!(presentation_v2.standings[0].rank, 1);
    assert_eq!(presentation_v2.standings[0].hand_name, "Pair of Queens");
    assert_eq!(presentation_v2.standings[1].seat, seat(0));
    assert!(presentation_v2.standings[0]
        .hole_cards
        .iter()
        .any(|card| card.used_in_best_five));
    assert!(presentation_v2.board_cards.iter().any(|card| {
        card.slot == "flop_3" && card.used_by_selected.contains(&"Seat 2".to_owned())
    }));
}

#[test]
fn showdown_explanation_marks_split_and_folded_paths() {
    let mut state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
            ],
            [
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Diamonds),
            ],
            [
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Three, Suit::Hearts),
            ],
        ],
        [
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
        ],
    );
    state.ledger.seats[2].status = SeatStatus::Folded;

    let outcome = river_ledger::resolve_showdown(&state);
    assert_showdown_surface_agreement(&outcome);
    let TerminalOutcome::Showdown {
        winners,
        headline,
        decisive_comparison,
        comparison_basis,
        explanations,
        presentation_v2,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };

    assert_eq!(winners, vec![seat(0), seat(1)]);
    assert_eq!(
        headline,
        "Seat 1 and Seat 2 split the ledger with Ace-high straight flush."
    );
    assert!(!headline.contains("seat_"));
    assert_eq!(
        decisive_comparison,
        "Seat 1 and Seat 2 all hold Ace-high straight flush, so the ledger is split."
    );
    assert!(!decisive_comparison.contains("seat_"));
    assert_eq!(
        comparison_basis,
        "The best revealed hands have equal category and tie-break ranks."
    );

    for index in [0, 1] {
        let reveal = explanations
            .iter()
            .find(|explanation| explanation.seat == seat(index))
            .and_then(|explanation| explanation.revealed.as_ref())
            .expect("split reveal");
        assert_eq!(reveal.result_label, "Split win");
        assert_eq!(
            reveal.comparison_note,
            "Ties for the best hand and shares the ledger."
        );
    }
    assert!(explanations
        .iter()
        .find(|explanation| explanation.seat == seat(2))
        .expect("folded explanation")
        .revealed
        .is_none());
    assert_eq!(presentation_v2.standings.len(), 2);
    assert_eq!(presentation_v2.folded_rows.len(), 1);
    assert_eq!(presentation_v2.folded_rows[0].seat, seat(2));
    assert!(presentation_v2
        .standings
        .iter()
        .all(|standing| standing.default_expanded));
    assert!(presentation_v2
        .decisive_reason
        .rule_refs
        .contains(&"RL-SCORE-SPLIT".to_owned()));
}

#[test]
fn single_pot_remainder_uses_stable_button_order() {
    let seat = |index| RiverLedgerSeat::from_index(index).unwrap();
    let allocation =
        river_ledger::pot::allocate_single_pot(11, &[seat(0), seat(2), seat(3)], seat(2), 4);

    assert_eq!(allocation.winners, vec![seat(0), seat(2), seat(3)]);
    assert_eq!(allocation.remainder, 2);
    assert_eq!(allocation.remainder_order, vec![seat(2), seat(3), seat(0)]);
    assert_eq!(
        allocation.shares,
        vec![
            PotShare {
                seat: seat(0),
                amount: 3,
            },
            PotShare {
                seat: seat(2),
                amount: 4,
            },
            PotShare {
                seat: seat(3),
                amount: 4,
            },
        ]
    );
    assert_eq!(
        allocation
            .shares
            .iter()
            .map(|share| share.amount)
            .sum::<u16>(),
        allocation.pot_total
    );
}

#[test]
fn seed_31_split_keeps_winners_canonical_and_remainder_button_ordered() {
    let mut state = custom_showdown_state(
        vec![
            [
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Six, Suit::Diamonds),
            ],
            [
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::King, Suit::Diamonds),
            ],
            [
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Hearts),
            ],
            [
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Seven, Suit::Clubs),
            ],
        ],
        [
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Spades),
        ],
    );
    state.button = seat(2);
    state.ledger.pot_total = 11;

    let outcome = river_ledger::resolve_showdown(&state);
    assert_showdown_surface_agreement(&outcome);
    let TerminalOutcome::Showdown {
        winners,
        allocations,
        presentation_v2,
        ..
    } = outcome
    else {
        panic!("showdown expected");
    };
    let expected_winners = vec![seat(1), seat(2), seat(3)];
    let allocation = river_ledger::pot::allocate_single_pot(11, &expected_winners, state.button, 4);

    assert_eq!(winners, expected_winners);
    assert_eq!(allocation.remainder_order, vec![seat(2), seat(3), seat(1)]);
    assert_eq!(
        allocations,
        vec![
            PotShare {
                seat: seat(1),
                amount: 3,
            },
            PotShare {
                seat: seat(2),
                amount: 4,
            },
            PotShare {
                seat: seat(3),
                amount: 4,
            },
        ]
    );
    assert_eq!(
        presentation_v2
            .standings
            .iter()
            .filter(|standing| standing.result_label == "Split win")
            .map(|standing| standing.seat)
            .collect::<Vec<_>>(),
        vec![seat(1), seat(2), seat(3)]
    );
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
        "amount_owed",
        "required_to_call",
        "adds_to_pot",
        "stack_before",
        "stack_after",
        "is_all_in",
        "is_full_raise",
        "raise_right_open",
        "pot_after",
        "raises_remaining",
        "cap_remaining",
        "accessibility_copy",
        "presentation_segment",
        "presentation_label",
        "presentation_helper_text",
        "presentation_accessibility_label",
        "presentation_row_0_label",
        "presentation_row_0_value",
        "presentation_row_0_tone",
        "presentation_row_1_label",
        "presentation_row_1_value",
        "presentation_row_1_tone",
        "presentation_row_2_label",
        "presentation_row_2_value",
        "presentation_row_2_tone",
    ];

    for choice in legal_action_tree(&state, &actor("seat_3")).root.choices {
        assert!(choice
            .metadata
            .iter()
            .all(|entry| allowed.contains(&entry.key.as_str())));
        assert!(choice
            .metadata
            .iter()
            .any(|entry| entry.key == "presentation_label"));
        let serialized = format!("{choice:?}");
        for forbidden in ["hidden", "private", "deck", "hole", "rank", "suit"] {
            assert!(!serialized.contains(forbidden), "{serialized}");
        }
    }
}

fn presentation_rows(choice: &engine_core::ActionChoice) -> Vec<(&str, &str)> {
    (0..)
        .map_while(|index| {
            let label = metadata_value(choice, &format!("presentation_row_{index}_label"))?;
            let value = metadata_value(choice, &format!("presentation_row_{index}_value"))?;
            Some((label, value))
        })
        .collect()
}

fn metadata_value<'a>(choice: &'a engine_core::ActionChoice, key: &str) -> Option<&'a str> {
    choice
        .metadata
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.as_str())
}
