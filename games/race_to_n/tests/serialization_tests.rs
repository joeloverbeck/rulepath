use engine_core::{
    ActionPath, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, StableSerialize,
};
use race_to_n::{
    project_view, setup_match, CounterValue, PublicView, RaceReplayJson, RaceSeat, RaceSnapshot,
    SetupOptions,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn snapshot_json_round_trips_with_version_fields_and_rejects_unknown_fields() {
    let state = setup_match(Seed(5), &seats(), &SetupOptions::default()).unwrap();
    let snapshot = RaceSnapshot::from_state(&state);
    let json = snapshot.to_json();

    assert!(json.contains("\"schema_version\":1"));
    assert!(json.contains("\"rules_version\":1"));

    let parsed = RaceSnapshot::from_json(&json).expect("snapshot round trip parses");
    assert_eq!(parsed, snapshot);
    assert_eq!(parsed.into_state(), state);

    let with_unknown = json.replace("\"freshness_token\":0", "\"freshness_token\":0,\"extra\":1");
    let err = RaceSnapshot::from_json(&with_unknown).expect_err("unknown field rejected");
    assert!(err.contains("unknown field"));
}

#[test]
fn public_view_json_round_trips_and_is_distinct_from_internal_state() {
    fn accepts_public_view(_: PublicView) {}

    let mut state = setup_match(Seed(5), &seats(), &SetupOptions::default()).unwrap();
    state.counter = CounterValue(20);
    let view = project_view(&state);
    accepts_public_view(view.clone());

    assert_eq!(view.legal_additions, vec![1]);
    assert_eq!(view.active_seat, RaceSeat::Seat0);

    let json = view.to_json();
    assert!(json.contains("\"schema_version\":1"));
    assert!(json.contains("\"rules_version\":1"));
    assert!(!json.contains("\"seats\""));
    assert!(!json.contains("seat-0"));

    let parsed = PublicView::from_json(&json).expect("public view round trip parses");
    assert_eq!(parsed, view);

    let with_unknown = json.replace(
        "\"legal_additions\":[1]",
        "\"legal_additions\":[1],\"state\":{}",
    );
    let err = PublicView::from_json(&with_unknown).expect_err("unknown field rejected");
    assert!(err.contains("unknown field"));
}

#[test]
fn replay_json_round_trips_with_snapshot_and_command_stream() {
    let state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let replay = RaceReplayJson {
        schema_version: 1,
        rules_version: 1,
        seed: 9,
        initial_snapshot: RaceSnapshot::from_state(&state),
        command_segments: vec!["add-1".to_owned(), "add-2".to_owned()],
    };
    let json = replay.to_json();

    assert!(json.contains("\"initial_snapshot\""));
    assert!(json.contains("\"command_segments\""));

    let parsed = RaceReplayJson::from_json(&json).expect("replay round trip parses");
    assert_eq!(parsed, replay);
}

#[test]
fn stable_serialization_order_yields_stable_hashes() {
    let state = setup_match(Seed(3), &seats(), &SetupOptions::default()).unwrap();
    let snapshot = RaceSnapshot::from_state(&state);
    let view = project_view(&state);

    assert_eq!(snapshot.to_json().as_bytes(), snapshot.stable_bytes());
    assert_eq!(view.to_json().as_bytes(), view.stable_bytes());
    assert_eq!(
        snapshot.stable_hash(),
        RaceSnapshot::from_json(&snapshot.to_json())
            .unwrap()
            .stable_hash()
    );
    assert_eq!(
        view.stable_hash(),
        PublicView::from_json(&view.to_json())
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
            segments: vec!["add-1".to_owned()],
        },
        freshness_token: FreshnessToken(0),
        rules_version: RulesVersion(1),
    };

    assert_eq!(command.action_path.segments, vec!["add-1"]);
    assert_eq!(command.rules_version, RulesVersion(1));
}
