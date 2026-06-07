use directional_flip::{
    apply_action, legal_action_tree, replay_commands, setup_match, validate_command,
    DirectionalFlipReplayJson, DirectionalFlipSnapshot, Manifest, SetupOptions, VariantCatalog,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize,
};

#[test]
fn df_replay_001_replay_export_import_step_reset_hashes_are_deterministic() {
    let commands = first_legal_segments(12, 4);
    let left = replay_commands(12, &commands);
    let right = replay_commands(12, &commands);

    assert_eq!(left, right);
    assert_eq!(left.projections.len(), commands.len());
    assert_eq!(
        left.projections.last().unwrap().public_view_hash,
        left.view_hash
    );
    assert_ne!(left.replay_hash.0, 0);
}

fn first_legal_segments(seed: u64, count: usize) -> Vec<String> {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state = setup_match(Seed(seed), &seats, &SetupOptions::default()).unwrap();
    let mut commands = Vec::new();
    for _ in 0..count {
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
        );
        let segment = tree.root.choices.first().unwrap().segment.clone();
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
            action_path: ActionPath {
                segments: vec![segment.clone()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let action = validate_command(&state, &command).unwrap();
        apply_action(&mut state, action);
        commands.push(segment);
    }
    commands
}

#[test]
fn df_ser_001_replay_json_rejects_unknown_fields_and_round_trips() {
    let replay = DirectionalFlipReplayJson {
        schema_version: 1,
        game_id: "directional_flip".to_owned(),
        rules_version: "directional_flip-rules-v1".to_owned(),
        variant: "directional_flip_standard".to_owned(),
        seed: 7,
        initial_snapshot: "snapshot".to_owned(),
        command_segments: vec!["place/r3c4".to_owned()],
    };

    let json = replay.to_json();
    assert_eq!(DirectionalFlipReplayJson::from_json(&json).unwrap(), replay);
    assert!(DirectionalFlipReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
}

#[test]
fn df_ser_001_static_data_rejects_unknown_and_behavior_looking_fields() {
    assert!(Manifest::parse("game_id = \"directional_flip\"\nextra = \"nope\"\n").is_err());
    assert!(VariantCatalog::parse(
        "variant_id = \"directional_flip_standard\"\nif = \"place/r1c1\"\n"
    )
    .is_err());
}

#[test]
fn df_replay_001_snapshot_stable_serialization_is_repeatable() {
    let hashes = replay_commands(3, &["place/r3c4".to_owned()]);
    assert_eq!(hashes.state_hash, hashes.state_hash);

    let snapshot_text = DirectionalFlipSnapshot {
        schema_version: 1,
        rules_version: 1,
        rules_version_label: "directional_flip-rules-v1".to_owned(),
        variant: directional_flip::Variant::directional_flip_standard(),
        cells: directional_flip::DirectionalFlipState::empty_cells(),
        active_seat: directional_flip::DirectionalFlipSeat::Seat0,
        seats: [
            engine_core::SeatId("seat-0".to_owned()),
            engine_core::SeatId("seat-1".to_owned()),
        ],
        ply_count: 0,
        consecutive_forced_passes: 0,
        terminal_outcome: None,
        freshness_token: engine_core::FreshnessToken(0),
    }
    .stable_summary();
    assert_eq!(
        DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: "directional_flip".to_owned(),
            rules_version: "directional_flip-rules-v1".to_owned(),
            variant: "directional_flip_standard".to_owned(),
            seed: 3,
            initial_snapshot: snapshot_text,
            command_segments: vec!["place/r3c4".to_owned()],
        }
        .stable_hash(),
        DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: "directional_flip".to_owned(),
            rules_version: "directional_flip-rules-v1".to_owned(),
            variant: "directional_flip_standard".to_owned(),
            seed: 3,
            initial_snapshot: DirectionalFlipSnapshot {
                schema_version: 1,
                rules_version: 1,
                rules_version_label: "directional_flip-rules-v1".to_owned(),
                variant: directional_flip::Variant::directional_flip_standard(),
                cells: directional_flip::DirectionalFlipState::empty_cells(),
                active_seat: directional_flip::DirectionalFlipSeat::Seat0,
                seats: [
                    engine_core::SeatId("seat-0".to_owned()),
                    engine_core::SeatId("seat-1".to_owned()),
                ],
                ply_count: 0,
                consecutive_forced_passes: 0,
                terminal_outcome: None,
                freshness_token: engine_core::FreshnessToken(0),
            }
            .stable_summary(),
            command_segments: vec!["place/r3c4".to_owned()],
        }
        .stable_hash()
    );
}
