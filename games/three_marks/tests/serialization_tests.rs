use engine_core::{
    ActionPath, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use three_marks::{
    project_view, replay_support::ThreeMarksReplayJson, setup_match, PublicView, SetupOptions,
    ThreeMarksSnapshot,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn public_view_json_round_trips_and_rejects_unknown_fields() {
    let state = setup_match(Seed(5), &seats(), &SetupOptions::default()).unwrap();
    let view = project_view(&state, &Viewer { seat_id: None });
    let json = view.to_json();

    assert!(json.contains("\"schema_version\":1"));
    assert!(json.contains("\"rules_version\":1"));
    assert!(!json.contains("debug"));

    let parsed = PublicView::from_json(&json).expect("public view round trip parses");
    assert_eq!(parsed, view);

    let with_unknown = json.replace(
        "\"replay_step_index\":null",
        "\"replay_step_index\":null,\"state\":{}",
    );
    let err = PublicView::from_json(&with_unknown).expect_err("unknown field rejected");
    assert!(err.contains("unknown field"));
}

#[test]
fn replay_json_round_trips_with_snapshot_summary_and_command_stream() {
    let state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let replay = ThreeMarksReplayJson {
        schema_version: 1,
        rules_version: 1,
        seed: 9,
        initial_snapshot: ThreeMarksSnapshot::from_state(&state).stable_summary(),
        command_segments: vec!["place/r1c1".to_owned(), "place/r2c2".to_owned()],
    };
    let json = replay.to_json();

    assert!(json.contains("\"initial_snapshot\""));
    assert!(json.contains("\"command_segments\""));

    let parsed = ThreeMarksReplayJson::from_json(&json).expect("replay round trip parses");
    assert_eq!(parsed, replay);
    assert_eq!(parsed.to_json(), json);
}

#[test]
fn stable_serialization_order_yields_stable_hashes() {
    let state = setup_match(Seed(3), &seats(), &SetupOptions::default()).unwrap();
    let snapshot = ThreeMarksSnapshot::from_state(&state);
    let view = project_view(&state, &Viewer { seat_id: None });
    let replay = ThreeMarksReplayJson {
        schema_version: 1,
        rules_version: 1,
        seed: 3,
        initial_snapshot: snapshot.stable_summary(),
        command_segments: vec!["place/r1c1".to_owned()],
    };

    assert_eq!(
        snapshot.stable_summary().as_bytes(),
        snapshot.stable_bytes()
    );
    assert_eq!(view.to_json().as_bytes(), view.stable_bytes());
    assert_eq!(replay.to_json().as_bytes(), replay.stable_bytes());
    assert_eq!(
        view.stable_hash(),
        PublicView::from_json(&view.to_json())
            .unwrap()
            .stable_hash()
    );
    assert_eq!(
        replay.stable_hash(),
        ThreeMarksReplayJson::from_json(&replay.to_json())
            .unwrap()
            .stable_hash()
    );
}

#[test]
fn command_envelope_shape_remains_replay_ready() {
    let command = CommandEnvelope {
        actor: engine_core::Actor {
            seat_id: SeatId("seat-0".to_owned()),
        },
        action_path: ActionPath {
            segments: vec!["place/r1c1".to_owned()],
        },
        freshness_token: FreshnessToken(0),
        rules_version: RulesVersion(1),
    };

    assert_eq!(command.action_path.segments, vec!["place/r1c1"]);
    assert_eq!(command.rules_version, RulesVersion(1));
}
