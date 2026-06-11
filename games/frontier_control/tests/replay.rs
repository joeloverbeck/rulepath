use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, StableSerialize, Viewer,
};
use frontier_control::{
    apply_command, export_public_replay, import_public_export, public_replay_step, setup_match,
    SetupOptions, ACTION_END_TURN,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn end_turn_command(state: &frontier_control::FrontierControlState, seat: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(seat.to_owned()),
        },
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_and_replay_hashes_are_deterministic() {
    let first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn command_stream_reproduces_effects_state_and_public_export() {
    let mut first = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let mut second = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let first_command = end_turn_command(&first, "seat_1");
    let second_command = end_turn_command(&second, "seat_1");
    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.stable_hash(), second.stable_hash());

    let step = public_replay_step(
        0,
        &first,
        &first_command,
        &first_applied.effects,
        &Viewer { seat_id: None },
    );
    let export = export_public_replay(first.variant.id.clone(), vec![step]);
    assert_eq!(import_public_export(&export).raw_json, export.to_json());
}
