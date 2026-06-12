use engine_core::{SeatId, Seed, StableSerialize};
use event_frontier::{setup_match, EventFrontierSnapshot, SetupOptions};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn setup_snapshot_round_trips_and_serializes_stably() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::land_rush()).expect("setup");
    let snapshot = EventFrontierSnapshot::from_state(&state);

    assert_eq!(snapshot.clone().into_state(), state);
    assert_eq!(
        snapshot.stable_bytes(),
        snapshot.stable_summary().into_bytes()
    );
    assert_eq!(snapshot.stable_hash(), state.stable_hash());
    assert!(snapshot
        .stable_summary()
        .contains("sites=site_charterhouse:agents1:settlers0:depot1:caches0"));
}
