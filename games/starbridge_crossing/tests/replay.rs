use starbridge_crossing::{
    encode_step_path, legal_step_moves, replay_commands, setup_match, SetupOptions,
};

#[test]
fn replay_reproduces_hashes_for_same_seed_and_commands() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = setup_match(engine_core::Seed(11), &seats, &SetupOptions::default()).unwrap();
    let step = legal_step_moves(&state)[0];
    let commands = vec![encode_step_path(step.peg, step.to)];

    let left = replay_commands(11, 2, &commands).unwrap();
    let right = replay_commands(11, 2, &commands).unwrap();

    assert_eq!(left, right);
    assert_ne!(left.replay_hash.0, 0);
}

#[test]
fn replay_hash_changes_when_command_stream_changes() {
    let seats = vec![
        engine_core::SeatId::from_zero_based_index(0),
        engine_core::SeatId::from_zero_based_index(1),
    ];
    let state = setup_match(engine_core::Seed(11), &seats, &SetupOptions::default()).unwrap();
    let moves = legal_step_moves(&state);

    let left = replay_commands(11, 2, &[encode_step_path(moves[0].peg, moves[0].to)]).unwrap();
    let right = replay_commands(11, 2, &[encode_step_path(moves[1].peg, moves[1].to)]).unwrap();

    assert_ne!(left.replay_hash, right.replay_hash);
}
