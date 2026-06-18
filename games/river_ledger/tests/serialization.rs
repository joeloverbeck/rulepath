use engine_core::StableSerialize;
use river_ledger::replay_support::{
    export_public_replay, replay_internal_full_trace, trace_from_commands,
};
use river_ledger::{project_view, setup_match, RiverLedgerSeat, SetupOptions};

const FOUR_PLAYER_CHECKDOWN: &[(usize, &str)] = &[
    (3, "call"),
    (0, "call"),
    (1, "call"),
    (2, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
    (1, "check"),
    (2, "check"),
    (3, "check"),
    (0, "check"),
];

#[test]
fn public_replay_export_json_order_is_stable() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let export = export_public_replay(&trace, &engine_core::Viewer { seat_id: None });
    let first = export.to_json();
    let second = export.to_json();

    assert_eq!(first, second);
    assert_eq!(export.stable_bytes(), first.as_bytes());
    assert!(first.starts_with(
        "{\"schema_version\":1,\"export_class\":\"viewer_scoped_public_replay\",\"viewer\":\"observer\",\"game_id\":\"river_ledger\""
    ));
}

#[test]
fn internal_trace_json_order_is_stable() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let json = trace.to_json();

    assert_eq!(trace.stable_bytes(), json.as_bytes());
    assert!(json.starts_with(
        "{\"schema_version\":1,\"game_id\":\"river_ledger\",\"rules_version\":\"river-ledger-rules-v1\""
    ));
    assert!(json.contains("\"commands\":[{\"actor\":\"seat_3\",\"path\":[\"call\"]},{\"actor\":\"seat_0\",\"path\":[\"call\"]}]"));
}

#[test]
fn public_display_labels_do_not_replace_canonical_seat_ids() {
    let state = setup_match(
        engine_core::Seed(21),
        &(0..4)
            .map(|index| engine_core::SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>(),
        &SetupOptions::default(),
    )
    .expect("setup");

    assert_eq!(RiverLedgerSeat::from_index(1).unwrap().as_str(), "seat_1");
    assert!(state.stable_internal_summary().contains("seat_1"));
}

#[test]
fn terminal_view_summary_includes_v2_showdown_presentation_deterministically() {
    let trace = trace_from_commands(79, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let first = view.stable_summary();
    let second = view.stable_summary();

    assert_eq!(first, second);
    assert!(first.contains("showdown:"));
    assert!(first.contains("wins with"));
}

#[test]
fn seed_10018_terminal_serialization_uses_public_winner_label() {
    let trace = trace_from_commands(10018, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert!(summary.contains("showdown:seat_0:"));
    assert!(summary.contains("Seat 1 wins with Two pair, Queens and Fives."));
    assert!(!summary.contains("Seat 0 wins"));
}

#[test]
fn seed_31_terminal_serialization_keeps_canonical_split_order() {
    let trace = trace_from_commands(31, 4, FOUR_PLAYER_CHECKDOWN);
    let result = replay_internal_full_trace(&trace);
    let view = project_view(&result.final_state, &engine_core::Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert!(summary.contains("showdown:seat_1,seat_2,seat_3:"));
    assert!(summary.contains("seat_1=3,seat_2=3,seat_3=2"));
    assert!(summary.contains("Seat 2, Seat 3, and Seat 4 split the ledger"));
}
