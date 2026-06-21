use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use vow_tide::{
    actions::{legal_action_tree, legal_bids, validate_bid_command},
    ids::{
        canonical_seat_ids, hand_schedule_for_seats, max_hand_size_for_seats, VowTideSeat,
        STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    },
    rules::apply_bid,
    setup::{setup_match, SetupOptions},
    state::Phase,
};

#[test]
fn setup_accepts_three_to_seven_ordered_seats() {
    let options = SetupOptions::default();

    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let seats = canonical_seat_ids(seat_count);
        let state = setup_match(Seed(11), &seats, &options).expect("setup succeeds");

        assert_eq!(state.seats, seats);
        assert_eq!(state.dealer, VowTideSeat::Seat0);
        assert_eq!(state.hand_index, 0);
        assert_eq!(state.cumulative_scores.len(), seat_count);
        assert_eq!(state.private_hands.len(), seat_count);
        assert_eq!(
            state.current_hand_size(),
            max_hand_size_for_seats(seat_count)
        );
        assert_eq!(
            state.seat_labels,
            (1..=seat_count)
                .map(|index| format!("Tide {index}"))
                .collect::<Vec<_>>()
        );
        assert!(matches!(state.phase, Phase::Bidding(_)));
        assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_stable_diagnostic() {
    let options = SetupOptions::default();

    for seat_count in [0, 1, 2, 8] {
        let seats = (0..seat_count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>();
        let diagnostic =
            setup_match(Seed(0), &seats, &options).expect_err("unsupported count is rejected");

        assert_eq!(diagnostic.code, "VT_INVALID_SEAT_COUNT");
        assert!(diagnostic.message.contains(&seat_count.to_string()));
    }
}

#[test]
fn schedule_is_max_down_to_one_up_to_max_for_every_supported_count() {
    let cases = [
        (3, 10, 19),
        (4, 10, 19),
        (5, 10, 19),
        (6, 8, 15),
        (7, 7, 13),
    ];

    for (seat_count, max_hand_size, total_hands) in cases {
        let schedule = hand_schedule_for_seats(seat_count).expect("supported count has schedule");

        assert_eq!(max_hand_size_for_seats(seat_count), Some(max_hand_size));
        assert_eq!(schedule.len(), total_hands);
        assert_eq!(schedule.first(), Some(&max_hand_size));
        assert_eq!(schedule.last(), Some(&max_hand_size));
        assert_eq!(
            schedule.iter().filter(|hand_size| **hand_size == 1).count(),
            1
        );

        for window in schedule[..max_hand_size as usize].windows(2) {
            assert_eq!(window[0], window[1] + 1);
        }
        for window in schedule[max_hand_size as usize - 1..].windows(2) {
            assert_eq!(window[0] + 1, window[1]);
        }
    }
}

#[test]
fn bidding_starts_left_of_dealer_and_advances_clockwise_to_dealer() {
    let mut state = setup_state(4);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));

    apply_bid_value(&mut state, VowTideSeat::Seat1, 2);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat2));
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat3));
    apply_bid_value(&mut state, VowTideSeat::Seat3, 4);
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));

    apply_bid_value(&mut state, VowTideSeat::Seat0, 0);
    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));
}

#[test]
fn legal_tree_lists_ascending_bids_and_omits_dealer_hook_value() {
    let mut state = setup_state(4);

    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat1),
        (0..=10).collect::<Vec<_>>()
    );
    let tree = legal_action_tree(&state, &actor(VowTideSeat::Seat1));
    assert_eq!(
        bid_leaf_segments(&tree),
        (0..=10).map(|value| value.to_string()).collect::<Vec<_>>()
    );

    apply_bid_value(&mut state, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 3);

    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));
    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat0),
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    let tree = legal_action_tree(&state, &actor(VowTideSeat::Seat0));
    assert!(!bid_leaf_segments(&tree).contains(&"1".to_owned()));
}

#[test]
fn dealer_hook_removes_nothing_when_prefix_total_exceeds_hand_size() {
    let mut state = setup_state(4);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 10);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 10);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 10);

    assert_eq!(state.active_seat(), Some(VowTideSeat::Seat0));
    assert_eq!(
        legal_bids(&state, VowTideSeat::Seat0),
        (0..=10).collect::<Vec<_>>()
    );
}

#[test]
fn invalid_bids_return_stable_diagnostics() {
    let mut state = setup_state(4);

    let wrong_seat =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat2, 1));
    assert_eq!(
        wrong_seat.expect_err("wrong seat rejected").code,
        "VT_WRONG_SEAT"
    );

    let out_of_range =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 11));
    assert_eq!(
        out_of_range.expect_err("out of range rejected").code,
        "VT_BID_OUT_OF_RANGE"
    );

    let stale = validate_bid_command(
        &state,
        &CommandEnvelope {
            freshness_token: FreshnessToken(99),
            ..command_for_state(&state, VowTideSeat::Seat1, 1)
        },
    );
    assert_eq!(stale.expect_err("stale rejected").code, "VT_STALE_COMMAND");

    *state
        .bidding_state_mut()
        .expect("bidding")
        .bid_for_mut(VowTideSeat::Seat1)
        .expect("bid row") = Some(1);
    let duplicate = validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 2));
    assert_eq!(
        duplicate.expect_err("duplicate rejected").code,
        "VT_BID_ALREADY_SET"
    );
}

#[test]
fn hook_forbidden_bid_is_rejected_by_validator() {
    let mut state = setup_state(4);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 3);
    apply_bid_value(&mut state, VowTideSeat::Seat3, 3);

    let diagnostic =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat0, 1))
            .expect_err("hook bid rejected");
    assert_eq!(diagnostic.code, "VT_BID_HOOK_FORBIDDEN");
}

#[test]
fn bid_during_playing_phase_is_wrong_phase() {
    let mut state = setup_state(3);
    apply_bid_value(&mut state, VowTideSeat::Seat1, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat2, 0);
    apply_bid_value(&mut state, VowTideSeat::Seat0, 1);

    let diagnostic =
        validate_bid_command(&state, &command_for_state(&state, VowTideSeat::Seat1, 0))
            .expect_err("bid after bidding rejected");
    assert_eq!(diagnostic.code, "VT_WRONG_PHASE");
}

fn setup_state(seat_count: usize) -> vow_tide::state::VowTideState {
    setup_match(
        Seed(19),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn apply_bid_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, value: u8) {
    let bid =
        validate_bid_command(state, &command_for_state(state, seat, value)).expect("bid validates");
    apply_bid(state, bid).expect("bid applies");
}

fn command_for_state(
    state: &vow_tide::state::VowTideState,
    seat: VowTideSeat,
    value: u8,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec!["bid".to_owned(), value.to_string()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn actor(seat: VowTideSeat) -> Actor {
    Actor {
        seat_id: SeatId(seat.as_str().to_owned()),
    }
}

fn bid_leaf_segments(tree: &engine_core::ActionTree) -> Vec<String> {
    tree.root.choices[0]
        .next
        .as_ref()
        .expect("bid branch")
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect()
}
