use engine_core::{SeatId, Seed, StableSerialize};
use token_bazaar::{
    apply_action, command_for_state, effect_hash, effect_stable_string, setup_match,
    validate_command, ContractId, TokenBazaarEffect, TokenBazaarSnapshot,
};

#[test]
fn state_snapshot_round_trips_with_stable_bytes() {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let state = setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds");
    let snapshot = TokenBazaarSnapshot::from_state(&state);

    assert_eq!(snapshot.clone().into_state(), state);
    assert_eq!(
        snapshot.stable_bytes(),
        snapshot.stable_summary().into_bytes()
    );
}

#[test]
fn effect_serialization_is_stable() {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let mut state = setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds");
    let command = command_for_state(&state, vec!["fulfill/slot_0".to_owned()]);
    let action = validate_command(&state, &command).expect("command validates");
    let effects = apply_action(&mut state, action);

    assert!(effects.iter().any(|effect| matches!(
        effect.payload,
        TokenBazaarEffect::ContractFulfilled {
            contract: ContractId::BalancedWares,
            ..
        }
    )));
    let strings = effects.iter().map(effect_stable_string).collect::<Vec<_>>();
    assert_eq!(
        strings,
        effects.iter().map(effect_stable_string).collect::<Vec<_>>()
    );
    assert_eq!(effect_hash(&effects), effect_hash(&effects));
}

#[test]
fn standard_fixture_metadata_is_present() {
    let fixture = include_str!("../data/fixtures/token_bazaar_standard.fixture.json");

    assert!(fixture.contains("\"fixture_id\": \"token_bazaar_standard_gate9\""));
    assert!(fixture.contains("\"game_id\": \"token_bazaar\""));
    assert!(fixture.contains("\"variant\": \"token_bazaar_standard\""));
    assert!(fixture.contains("\"fixture_kinds\""));
}
