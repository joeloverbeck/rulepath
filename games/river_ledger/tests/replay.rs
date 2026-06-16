use engine_core::{HashValue, SeatId, StableSerialize, Viewer};
use river_ledger::{
    replay_support::{
        export_public_replay, import_public_export, replay_internal_full_trace, trace_from_commands,
    },
    setup_match, SetupOptions,
};

fn hidden_ids(seed: u64, seat_count: usize) -> Vec<String> {
    let seats = (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect::<Vec<_>>();
    let state =
        setup_match(engine_core::Seed(seed), &seats, &SetupOptions::default()).expect("setup");
    state
        .private_hands_internal()
        .iter()
        .flatten()
        .chain(state.community_deck_internal().iter())
        .chain(state.deck_tail_internal().iter())
        .map(|card| card.id())
        .collect()
}

#[test]
fn internal_trace_replays_to_same_hashes_and_state() {
    let trace = trace_from_commands(
        21,
        4,
        &[(3, "call"), (0, "call"), (1, "call"), (2, "check")],
    );
    let first = replay_internal_full_trace(&trace);
    let second = replay_internal_full_trace(&trace);

    assert_eq!(first.trace_hash, second.trace_hash);
    assert_eq!(first.state_hash, second.state_hash);
    assert_eq!(first.effect_hash, second.effect_hash);
    assert_eq!(first.view_hash, second.view_hash);
    assert_eq!(first.action_tree_hashes, second.action_tree_hashes);
    assert_eq!(
        first.final_state.stable_internal_summary(),
        second.final_state.stable_internal_summary()
    );
}

#[test]
fn public_export_import_round_trips_for_observer_and_seat_viewer() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    for viewer in [
        Viewer { seat_id: None },
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    ] {
        let export = export_public_replay(&trace, &viewer);
        let imported = import_public_export(&export);

        assert_eq!(imported.viewer, export.viewer);
        assert_eq!(imported.steps, export.steps);
        assert_eq!(
            HashValue::from_stable_bytes(export.to_json().as_bytes()),
            export.stable_hash()
        );
    }
}

#[test]
fn observer_public_export_omits_hidden_facts_and_seed() {
    let trace = trace_from_commands(21, 4, &[(3, "call"), (0, "call")]);
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = export.to_json();

    for hidden in hidden_ids(21, 4) {
        assert!(!json.contains(&hidden), "public export leaked {hidden}");
    }
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}

#[test]
fn terminal_public_export_keeps_v2_showdown_surface_public_and_deterministic() {
    let trace = trace_from_commands(
        79,
        4,
        &[
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
        ],
    );
    let first = export_public_replay(&trace, &Viewer { seat_id: None });
    let second = export_public_replay(&trace, &Viewer { seat_id: None });
    let json = first.to_json();

    assert_eq!(first.steps, second.steps);
    assert!(json.contains("wins with"));
    assert!(json.contains("showdown:"));
    assert!(!json.contains("private_hands"));
    assert!(!json.contains("seed_evidence"));
    assert!(!json.contains("\"seed\""));
}
