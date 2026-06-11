use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize,
};
use flood_watch::{apply_command, setup_match, ScenarioVariant, SetupOptions, ACTION_END_TURN};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn end_turn_command(state: &flood_watch::FloodWatchState) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId("seat_0".to_owned()),
        },
        action_path: ActionPath {
            segments: vec![ACTION_END_TURN.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn setup_state_hash_is_deterministic_for_same_seed_and_scenario() {
    let options = SetupOptions::default();

    let first = setup_match(Seed(55), &seats(), &options).unwrap();
    let second = setup_match(Seed(55), &seats(), &options).unwrap();

    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn setup_state_hash_changes_with_seed_or_scenario() {
    let standard = setup_match(Seed(55), &seats(), &SetupOptions::default()).unwrap();
    let other_seed = setup_match(Seed(56), &seats(), &SetupOptions::default()).unwrap();
    let deluge = setup_match(
        Seed(55),
        &seats(),
        &SetupOptions {
            variant: ScenarioVariant::deluge(),
        },
    )
    .unwrap();

    assert_ne!(
        standard.event_deck_internal(),
        other_seed.event_deck_internal()
    );
    assert_ne!(standard.stable_hash(), other_seed.stable_hash());
    assert_ne!(standard.stable_hash(), deluge.stable_hash());
}

#[test]
fn environment_effects_and_hashes_replay_deterministically() {
    let options = SetupOptions::default();
    let mut first = setup_match(Seed(91), &seats(), &options).unwrap();
    let mut second = setup_match(Seed(91), &seats(), &options).unwrap();
    let first_command = end_turn_command(&first);
    let second_command = end_turn_command(&second);

    let first_applied = apply_command(&mut first, &first_command).unwrap();
    let second_applied = apply_command(&mut second, &second_command).unwrap();

    assert_eq!(first_applied.effects, second_applied.effects);
    assert_eq!(first.drawn, second.drawn);
    assert_eq!(first.event_deck_internal(), second.event_deck_internal());
    assert_eq!(first.stable_hash(), second.stable_hash());
}
