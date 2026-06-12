use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId};
use frontier_control::{
    apply_command, legal_action_metadata, legal_action_tree, setup_match, FactionId,
    FrontierControlEffect, Phase, SetupOptions, SiteId, ACTION_END_TURN, ACTION_MARCH,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(
    state: &frontier_control::FrontierControlState,
    seat: &str,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_and_action_tree_are_faction_separated() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    assert_eq!(state.active_faction, FactionId::Prospectors);
    assert_eq!(state.round_number, 1);

    let tree = legal_action_tree(&state, &actor("seat_1"));
    let segments = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>();
    assert!(segments.iter().any(|segment| segment.starts_with("march/")));
    assert!(segments.contains(&ACTION_END_TURN));
    assert!(!segments
        .iter()
        .any(|segment| segment.starts_with("patrol/")));

    assert!(legal_action_tree(&state, &actor("seat_0"))
        .root
        .choices
        .is_empty());
    assert!(legal_action_metadata(&state, &actor("seat_0"))
        .iter()
        .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
}

#[test]
fn clash_scoring_and_terminal_are_rust_owned() {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_base_camp", "site_ford"],
    );
    apply_command(&mut state, &first).unwrap();
    let second = command(
        &state,
        "seat_1",
        vec![ACTION_MARCH, "site_ford", "site_gatehouse"],
    );
    let applied = apply_command(&mut state, &second).unwrap();

    assert!(applied.turn_ended);
    assert_eq!(state.site(SiteId::Gatehouse).unwrap().guards, 1);
    assert!(applied.effects.iter().any(|effect| {
        matches!(
            effect.payload,
            FrontierControlEffect::ClashResolved {
                entering_faction: FactionId::Prospectors,
                ..
            }
        )
    }));

    state.round_number = state.variant.round_count;
    state.active_faction = FactionId::Garrison;
    state.phase = Phase::Action {
        budget_remaining: state.variant.action_budget,
    };
    let end = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &end).unwrap();
    assert!(applied.turn_ended);
    assert_eq!(state.phase, Phase::Terminal);
    assert!(state.terminal_outcome.is_some());
}

#[test]
fn diagnostics_are_viewer_safe() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let err = frontier_control::validate_command(
        &state,
        &command(
            &state,
            "seat_1",
            vec![ACTION_MARCH, "site_base_camp", "site_gatehouse"],
        ),
    )
    .unwrap_err();

    assert_eq!(err.code, "non_adjacent_sites");
    assert!(!err.message.contains("hidden"));
}
