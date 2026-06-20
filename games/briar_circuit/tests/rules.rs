use briar_circuit::{
    apply_pass_action,
    setup::{deal_order_after, next_dealer},
    setup_match, validate_pass_command, BriarCircuitSeat, BriarCircuitState, PassAction,
    PassDirection, Phase, SetupOptions,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

fn fresh_state() -> BriarCircuitState {
    setup_match(Seed(1606), &seats(4), &SetupOptions::default()).expect("setup succeeds")
}

fn pass_command(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    segments: &[&str],
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.as_str().to_owned()),
        },
        action_path: ActionPath {
            segments: segments
                .iter()
                .map(|segment| (*segment).to_owned())
                .collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_accepts_exactly_four_seats() {
    let state = setup_match(Seed(16), &seats(4), &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(state.seats[0], SeatId("seat_0".to_owned()));
    assert_eq!(state.dealer, BriarCircuitSeat::Seat0);
    assert_eq!(state.hand_index, 0);
    assert_eq!(state.cumulative_scores, [0, 0, 0, 0]);
    assert!(matches!(
        state.phase,
        Phase::Passing(ref pass) if pass.direction == PassDirection::Left
    ));
}

#[test]
fn deal_starts_left_of_dealer_and_dealer_rotates_clockwise() {
    assert_eq!(
        deal_order_after(BriarCircuitSeat::Seat0),
        [
            BriarCircuitSeat::Seat1,
            BriarCircuitSeat::Seat2,
            BriarCircuitSeat::Seat3,
            BriarCircuitSeat::Seat0,
        ]
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat1
    );
    assert_eq!(
        next_dealer(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
}

#[test]
fn setup_rejects_every_non_four_count_with_stable_diagnostic() {
    for count in [0, 1, 2, 3, 5, 6, 7] {
        let err = setup_match(Seed(16), &seats(count), &SetupOptions::default())
            .expect_err("unsupported seat count rejects");

        assert_eq!(err.code, "BC_UNSUPPORTED_SEAT_COUNT");
        assert!(
            err.message.contains("requires exactly four seats"),
            "unexpected diagnostic for count {count}: {}",
            err.message
        );
    }
}

#[test]
fn pass_direction_cycle_and_targets_are_stable() {
    assert_eq!(PassDirection::for_hand_index(0), PassDirection::Left);
    assert_eq!(PassDirection::for_hand_index(1), PassDirection::Right);
    assert_eq!(PassDirection::for_hand_index(2), PassDirection::Across);
    assert_eq!(PassDirection::for_hand_index(3), PassDirection::Hold);
    assert_eq!(PassDirection::for_hand_index(4), PassDirection::Left);
    assert_eq!(
        PassDirection::Left.target_for(BriarCircuitSeat::Seat3),
        BriarCircuitSeat::Seat0
    );
    assert_eq!(
        PassDirection::Right.target_for(BriarCircuitSeat::Seat0),
        BriarCircuitSeat::Seat3
    );
    assert_eq!(
        PassDirection::Across.target_for(BriarCircuitSeat::Seat1),
        BriarCircuitSeat::Seat3
    );
}

#[test]
fn pass_command_validates_actor_path_freshness_and_rules_version() {
    let state = fresh_state();
    let seat = BriarCircuitSeat::Seat0;
    let card = state.hand_for_internal(seat)[0];
    let card_segment = card.as_str();
    let envelope = pass_command(&state, seat, &["pass", "select", &card_segment]);

    let (validated_seat, action) =
        validate_pass_command(&state, &envelope).expect("command validates");

    assert_eq!(validated_seat, seat);
    assert_eq!(action, PassAction::Select(card));

    let mut stale = envelope.clone();
    stale.freshness_token = FreshnessToken(state.freshness_token.0 + 1);
    assert_eq!(
        validate_pass_command(&state, &stale)
            .expect_err("stale command rejects")
            .code,
        "BC_STALE_COMMAND"
    );

    let mut wrong_rules = envelope;
    wrong_rules.rules_version = RulesVersion(2);
    assert_eq!(
        validate_pass_command(&state, &wrong_rules)
            .expect_err("wrong rules reject")
            .code,
        "BC_WRONG_RULES_VERSION"
    );
}

#[test]
fn pass_select_unselect_and_confirm_require_three_distinct_owned_cards() {
    let mut state = fresh_state();
    let seat = BriarCircuitSeat::Seat0;
    let first = state.hand_for_internal(seat)[0];
    let second = state.hand_for_internal(seat)[1];
    let third = state.hand_for_internal(seat)[2];
    let unowned = state.hand_for_internal(BriarCircuitSeat::Seat1)[0];

    apply_pass_action(&mut state, seat, PassAction::Select(first)).expect("first select");

    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Select(first))
            .expect_err("duplicate select rejects")
            .code,
        "BC_PASS_DUPLICATE_CARD"
    );
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Select(unowned))
            .expect_err("unowned select rejects")
            .code,
        "BC_CARD_NOT_OWNED"
    );
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Confirm)
            .expect_err("short confirm rejects")
            .code,
        "BC_PASS_REQUIRES_THREE"
    );

    apply_pass_action(&mut state, seat, PassAction::Unselect(first)).expect("unselect");
    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Unselect(first))
            .expect_err("missing unselect rejects")
            .code,
        "BC_CARD_NOT_SELECTED"
    );

    for card in [first, second, third] {
        apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select card");
    }
    apply_pass_action(&mut state, seat, PassAction::Confirm).expect("confirm succeeds");

    assert_eq!(
        apply_pass_action(&mut state, seat, PassAction::Unselect(first))
            .expect_err("committed seat cannot mutate")
            .code,
        "BC_PASS_ALREADY_COMMITTED"
    );
}

#[test]
fn fourth_confirm_exchanges_cards_atomically_and_enters_playing_phase() {
    let mut state = fresh_state();
    let initial_hands: Vec<_> = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| (seat, state.hand_for_internal(seat).to_vec()))
        .collect();
    let selected: Vec<_> = initial_hands
        .iter()
        .map(|(seat, hand)| (*seat, hand[..3].to_vec()))
        .collect();

    for (seat, cards) in &selected {
        for card in cards {
            apply_pass_action(&mut state, *seat, PassAction::Select(*card))
                .expect("select pass card");
        }
        let result = apply_pass_action(&mut state, *seat, PassAction::Confirm)
            .expect("confirm pass selection");
        assert_eq!(
            result.exchange_completed,
            *seat == BriarCircuitSeat::Seat3,
            "only fourth confirm completes exchange"
        );
    }

    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    for seat in BriarCircuitSeat::ALL {
        assert_eq!(state.hand_for_internal(seat).len(), 13);
    }

    let direction = PassDirection::Left;
    for (source, cards) in &selected {
        let target = direction.target_for(*source);
        for card in cards {
            assert!(!state.hand_for_internal(*source).contains(card));
            assert!(state.hand_for_internal(target).contains(card));
        }
    }
}

#[test]
fn hold_hand_skips_selection_and_exchange() {
    use briar_circuit::{setup::deal_hand, Variant};
    use engine_core::SeededRng;

    let mut rng = SeededRng::from_seed(Seed(1616));
    let deal = deal_hand(&mut rng, BriarCircuitSeat::Seat3, 3).expect("deal succeeds");
    let state = BriarCircuitState::new_after_deal(
        Variant::briar_circuit_standard(),
        [
            SeatId("seat_0".to_owned()),
            SeatId("seat_1".to_owned()),
            SeatId("seat_2".to_owned()),
            SeatId("seat_3".to_owned()),
        ],
        BriarCircuitSeat::Seat3,
        3,
        deal.hands,
        deal.pass_direction,
    );

    assert_eq!(deal.pass_direction, PassDirection::Hold);
    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    assert_eq!(
        apply_pass_action(
            &mut state.clone(),
            BriarCircuitSeat::Seat0,
            PassAction::Confirm
        )
        .expect_err("hold hand has no pass phase")
        .code,
        "BC_WRONG_PHASE"
    );
}
