use engine_core::{
    ActionChoice, ActionMetadata, ActionPath, ActionTree, Actor, CommandEnvelope, FreshnessToken,
    HashValue, RulesVersion, SeatId, Seed, StableBytesRecordWriter, StableBytesWriter,
    StableSerialize,
};
use race_to_n::{
    legal_action_tree, project_view,
    replay_support::{
        action_tree_hash, action_tree_legacy_bytes, action_tree_v1_bytes, action_tree_v1_hash,
    },
    setup_match, CounterValue, PublicView, RaceReplayJson, RaceSeat, RaceSnapshot, SetupOptions,
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

#[test]
fn characterization_flat_action_tree_legacy_bytes_and_hash_are_pinned() {
    let state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let actor = Actor {
        seat_id: SeatId("seat-0".to_owned()),
    };
    let tree = legal_action_tree(&state, &actor);
    let legacy_bytes = action_tree_legacy_bytes(&tree);

    assert_eq!(legacy_bytes, "add-1|add-2|add-3");
    assert_eq!(action_tree_hash(&tree), HashValue(8451402319224114161));
}

#[test]
fn race_flat_action_tree_legacy_and_v1_surfaces_are_pinned_in_parallel() {
    let state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    let actor = Actor {
        seat_id: SeatId("seat-0".to_owned()),
    };
    let tree = legal_action_tree(&state, &actor);
    let v1_bytes = action_tree_v1_bytes(&tree);

    assert_eq!(action_tree_legacy_bytes(&tree), "add-1|add-2|add-3");
    assert_eq!(action_tree_hash(&tree), HashValue(8451402319224114161));
    assert_eq!(
        v1_bytes,
        expected_race_flat_action_tree_v1_bytes(state.freshness_token.0)
    );
    assert_eq!(
        action_tree_v1_hash(&tree),
        HashValue(143_078_033_215_105_790)
    );
    assert_ne!(action_tree_hash(&tree), action_tree_v1_hash(&tree));
}

#[test]
fn characterization_flat_action_tree_delimiter_strings_are_ambiguous() {
    let split_delimiter = ActionTree::flat(
        FreshnessToken(0),
        vec![
            ActionChoice::leaf("add-1|add-2", "A", "A"),
            ActionChoice::leaf("add-3", "B", "B"),
        ],
    );
    let joined_delimiter = ActionTree::flat(
        FreshnessToken(0),
        vec![
            ActionChoice::leaf("add-1", "A", "A"),
            ActionChoice::leaf("add-2|add-3", "B", "B"),
        ],
    );

    assert_ne!(split_delimiter, joined_delimiter);
    assert_eq!(
        action_tree_hash(&split_delimiter),
        action_tree_hash(&joined_delimiter)
    );
}

#[test]
fn characterization_flat_action_tree_empty_choice_and_absent_boundary_collide() {
    let explicit_empty_segment = ActionTree::flat(
        FreshnessToken(0),
        vec![
            ActionChoice::leaf("add-1", "A", "A"),
            ActionChoice::leaf("", "empty", "empty"),
        ],
    );
    let absent_second_choice = ActionTree::flat(
        FreshnessToken(0),
        vec![ActionChoice::leaf("add-1|", "A", "A")],
    );

    assert_ne!(explicit_empty_segment, absent_second_choice);
    assert_eq!(
        action_tree_hash(&explicit_empty_segment),
        action_tree_hash(&absent_second_choice)
    );
}

#[test]
fn characterization_flat_action_tree_metadata_and_tag_order_are_ignored() {
    let mut metadata_first = ActionChoice::leaf("add-1", "A", "A");
    metadata_first.metadata = vec![
        ActionMetadata {
            key: "first".to_owned(),
            value: "1".to_owned(),
        },
        ActionMetadata {
            key: "second".to_owned(),
            value: "2".to_owned(),
        },
    ];
    metadata_first.tags = vec!["left".to_owned(), "right".to_owned()];

    let mut metadata_swapped = ActionChoice::leaf("add-1", "A", "A");
    metadata_swapped.metadata = vec![
        ActionMetadata {
            key: "second".to_owned(),
            value: "2".to_owned(),
        },
        ActionMetadata {
            key: "first".to_owned(),
            value: "1".to_owned(),
        },
    ];
    metadata_swapped.tags = vec!["right".to_owned(), "left".to_owned()];

    let first_tree = ActionTree::flat(FreshnessToken(0), vec![metadata_first]);
    let swapped_tree = ActionTree::flat(FreshnessToken(0), vec![metadata_swapped]);

    assert_ne!(first_tree, swapped_tree);
    assert_eq!(
        action_tree_hash(&first_tree),
        action_tree_hash(&swapped_tree)
    );
}

fn expected_race_flat_action_tree_v1_bytes(freshness_token: u64) -> Vec<u8> {
    let mut writer = StableBytesWriter::new(b"action_tree", 1).expect("writer");
    writer
        .write_u64_field(1, freshness_token)
        .expect("freshness");
    writer
        .write_record_field(2, |record| {
            record.write_sequence_field(
                1,
                [1_u8, 2, 3].map(expected_race_flat_action_choice_v1_record),
            )?;
            Ok(())
        })
        .expect("root");
    writer.into_bytes()
}

fn expected_race_flat_action_choice_v1_record(amount: u8) -> Vec<u8> {
    let mut record = StableBytesRecordWriter::new();
    record
        .write_string_field(1, &format!("add-{amount}"))
        .expect("segment");
    record
        .write_string_field(2, &format!("Add {amount}"))
        .expect("label");
    record
        .write_string_field(3, &format!("Add {amount} to the counter"))
        .expect("accessibility label");
    record
        .write_sequence_field(4, [expected_amount_metadata_v1_record(amount)])
        .expect("metadata");
    record
        .write_sequence_field(5, [b"flat".as_slice(), b"counter".as_slice()])
        .expect("tags");
    record.write_enum_field(6, 1).expect("preview");
    record.write_none_field(7).expect("next");
    record.into_bytes()
}

fn expected_amount_metadata_v1_record(amount: u8) -> Vec<u8> {
    let mut record = StableBytesRecordWriter::new();
    record.write_string_field(1, "amount").expect("key");
    record
        .write_string_field(2, &amount.to_string())
        .expect("value");
    record.into_bytes()
}
