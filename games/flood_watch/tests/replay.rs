use engine_core::{SeatId, Seed, StableSerialize};
use flood_watch::{setup_match, ScenarioVariant, SetupOptions};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
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
