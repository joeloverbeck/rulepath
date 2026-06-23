use engine_core::{ActionTreeEncodingVersion, Actor, HashValue, SeatId, Seed, StableSerialize};
use token_bazaar::replay_support::{action_tree_hash, action_tree_v1_bytes, action_tree_v1_hash};
use token_bazaar::{
    apply_action, command_for_state, effect_hash, effect_stable_string, export_public_replay,
    legal_action_tree, setup_match, validate_command, ContractId, TokenBazaarEffect,
    TokenBazaarSnapshot,
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
fn action_tree_legacy_and_v1_surfaces_are_pinned_in_parallel() {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    let state = setup_match(Seed(9), &seats, &Default::default()).expect("setup succeeds");
    let tree = legal_action_tree(
        &state,
        &Actor {
            seat_id: SeatId("seat-0".to_owned()),
        },
    );
    let v1_bytes = action_tree_v1_bytes(&tree);

    assert_eq!(action_tree_hash(&tree), HashValue(18446619012895702437));
    assert_eq!(v1_bytes, tree.stable_bytes(ActionTreeEncodingVersion::V1));
    assert_eq!(
        action_tree_v1_hash(&tree),
        tree.stable_hash(ActionTreeEncodingVersion::V1)
    );
    assert_eq!(action_tree_v1_hash(&tree), HashValue(12286856647248594947));
    assert_eq!(v1_bytes.len(), 2087);
    assert!(v1_bytes.starts_with(b"RPSB"));
    assert!(v1_bytes
        .windows(b"action_tree".len())
        .any(|window| window == b"action_tree"));
    assert!(byte_offset(&v1_bytes, b"collect/amber") < byte_offset(&v1_bytes, b"collect/jade"));
    assert!(byte_offset(&v1_bytes, b"collect/jade") < byte_offset(&v1_bytes, b"collect/iron"));
    assert!(
        byte_offset(&v1_bytes, b"collect/iron") < byte_offset(&v1_bytes, b"collect/amber-jade")
    );
    assert!(
        byte_offset(&v1_bytes, b"collect/iron-amber") < byte_offset(&v1_bytes, b"fulfill/slot_0")
    );
    assert!(
        byte_offset(&v1_bytes, b"family") < byte_offset(&v1_bytes, b"gain")
            && byte_offset(&v1_bytes, b"gain") < byte_offset(&v1_bytes, b"bundle_id")
    );
    let fulfill = byte_offset(&v1_bytes, b"fulfill/slot_0");
    let fulfill_family = byte_offset_after(&v1_bytes, b"family", fulfill);
    let fulfill_cost = byte_offset_after(&v1_bytes, b"cost", fulfill_family);
    let fulfill_slot = byte_offset_after(&v1_bytes, b"slot_id", fulfill_cost);
    let fulfill_contract = byte_offset_after(&v1_bytes, b"contract_id", fulfill_slot);
    let fulfill_points = byte_offset_after(&v1_bytes, b"points", fulfill_contract);
    assert!(
        fulfill_family < fulfill_cost
            && fulfill_cost < fulfill_slot
            && fulfill_slot < fulfill_contract
            && fulfill_contract < fulfill_points
    );
    assert_ne!(action_tree_hash(&tree), action_tree_v1_hash(&tree));
    assert_eq!(
        export_public_replay(9, &[]).stable_hash(),
        HashValue(15196735406343894975)
    );
}

fn byte_offset(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("needle appears in v1 bytes")
}

fn byte_offset_after(haystack: &[u8], needle: &[u8], offset: usize) -> usize {
    offset
        + haystack[offset..]
            .windows(needle.len())
            .position(|window| window == needle)
            .expect("needle appears in v1 bytes after offset")
}

#[test]
fn standard_fixture_metadata_is_present() {
    let fixture = include_str!("../data/fixtures/token_bazaar_standard.fixture.json");

    assert!(fixture.contains("\"fixture_id\": \"token_bazaar_standard_gate9\""));
    assert!(fixture.contains("\"game_id\": \"token_bazaar\""));
    assert!(fixture.contains("\"variant\": \"token_bazaar_standard\""));
    assert!(fixture.contains("\"fixture_kinds\""));
}
