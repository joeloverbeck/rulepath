use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use poker_lite::{
    legal_action_tree, setup_match, validate_command, Phase, PokerLiteSeat, SetupOptions,
};

fn standard_state() -> poker_lite::PokerLiteState {
    setup_match(
        Seed(3),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(state: &poker_lite::PokerLiteState, seat: &str, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn legal_action_generation_follows_pledge_state() {
    let mut state = standard_state();

    let opening_segments = legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(opening_segments, vec!["hold", "press"]);

    state.active_seat = Some(PokerLiteSeat::Seat1);
    state.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
    state.round.outstanding_amount = 1;
    let facing_segments = legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(facing_segments, vec!["lift", "match", "yield"]);

    state.round.lift_used = true;
    let capped_segments = legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .into_iter()
        .map(|choice| choice.segment)
        .collect::<Vec<_>>();
    assert_eq!(capped_segments, vec!["match", "yield"]);
}

#[test]
fn metadata_is_public_allow_list_only() {
    let state = standard_state();
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let allowed = [
        "action_family",
        "round_index",
        "round_unit",
        "actor_seat",
        "required_to_match",
        "adds_to_pool",
        "shared_pool_after",
        "lift_cap_remaining",
        "center_visible",
        "accessibility_copy",
    ];

    for choice in &tree.root.choices {
        assert!(choice
            .metadata
            .iter()
            .all(|entry| allowed.contains(&entry.key.as_str())));
        let serialized = format!("{choice:?}");
        for forbidden in ["card", "rank", "deck", "hidden", "strength", "private"] {
            assert!(!serialized.contains(forbidden), "{serialized}");
        }
    }
}

#[test]
fn validation_reports_fail_closed_diagnostics() {
    let state = standard_state();
    assert_eq!(
        validate_command(&state, &command(&state, "seat_1", "hold"))
            .unwrap_err()
            .code,
        "wrong_seat"
    );
    assert_eq!(
        validate_command(&state, &command(&state, "seat_0", "match"))
            .unwrap_err()
            .code,
        "action_unavailable"
    );

    let mut terminal = standard_state();
    terminal.phase = Phase::Terminal;
    assert_eq!(
        validate_command(&terminal, &command(&terminal, "seat_0", "hold"))
            .unwrap_err()
            .code,
        "terminal_state"
    );

    let malformed = CommandEnvelope {
        actor: actor("seat_0"),
        action_path: ActionPath {
            segments: vec!["hold".to_owned(), "extra".to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    assert_eq!(
        validate_command(&state, &malformed).unwrap_err().code,
        "malformed_action"
    );

    let mut capped = standard_state();
    capped.active_seat = Some(PokerLiteSeat::Seat1);
    capped.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
    capped.round.outstanding_amount = 1;
    capped.round.lift_used = true;
    let diagnostic = validate_command(&capped, &command(&capped, "seat_1", "lift")).unwrap_err();
    assert_eq!(diagnostic.code, "lift_cap_exceeded");
    assert!(!format!("{diagnostic:?}").contains("low_"));
}
