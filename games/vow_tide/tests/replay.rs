use engine_core::{Seed, StableSerialize};
use vow_tide::{
    cards::{Card, Rank, Suit},
    ids::{canonical_seat_ids, VowTideSeat},
    replay_support::{
        export_for_viewer, import_viewer_export, observer, seat_viewer, snapshot, stable_hash,
    },
    setup::{setup_match, SetupOptions},
};

#[test]
fn identical_setup_reproduces_snapshot_hashes() {
    let seats = canonical_seat_ids(4);
    let first = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");
    let second = setup_match(Seed(77), &seats, &SetupOptions::default()).expect("setup succeeds");

    assert_eq!(snapshot(&first, &[]), snapshot(&second, &[]));
}

#[test]
fn viewer_exports_round_trip_and_remain_viewer_scoped() {
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    install_canaries(&mut state);
    let observer_export = export_for_viewer(&state, &[], &observer());
    let seat_0_export = export_for_viewer(&state, &[], &seat_viewer("seat_0"));

    assert_eq!(
        import_viewer_export(&observer_export).expect("observer import"),
        observer_export
    );
    assert_eq!(
        import_viewer_export(&seat_0_export).expect("seat import"),
        seat_0_export
    );
    assert_ne!(observer_export.stable_bytes(), seat_0_export.stable_bytes());

    let observer_text = observer_export.stable_summary();
    assert!(!observer_text.contains("two_clubs"));
    assert!(!observer_text.contains("king_clubs"));

    let seat_0_text = seat_0_export.stable_summary();
    assert!(seat_0_text.contains("two_clubs"));
    assert!(!seat_0_text.contains("three_diamonds"));
    assert!(!seat_0_text.contains("king_clubs"));
}

#[test]
fn characterization_viewer_export_artifacts_are_pinned() {
    let public_trace = include_str!("golden_traces/public-replay-export-import.trace.json");
    let seat_private_trace =
        include_str!("golden_traces/seat-private-replay-export-import-all-viewers.trace.json");
    let state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(4),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let observer_export = export_for_viewer(&state, &[], &observer());
    let seat_0_export = export_for_viewer(&state, &[], &seat_viewer("seat_0"));

    assert_eq!(stable_hash(public_trace.as_bytes()), 9606057229737834804);
    assert_eq!(stable_hash(seat_private_trace.as_bytes()), 16909558442784598481);
    assert_eq!(observer_export.stable_hash().0, 14136592432406028852);
    assert_eq!(seat_0_export.stable_hash().0, 12688236753872554050);
    assert_eq!(observer_export.viewer, "observer");
    assert_eq!(seat_0_export.viewer, "seat_0");
}

#[test]
fn stable_hash_is_byte_order_sensitive_and_repeatable() {
    assert_eq!(stable_hash(b"vow_tide"), stable_hash(b"vow_tide"));
    assert_ne!(stable_hash(b"vow_tide"), stable_hash(b"tide_vow"));
}

fn install_canaries(state: &mut vow_tide::state::VowTideState) {
    let canaries = [
        Card::new(Rank::Two, Suit::Clubs).id(),
        Card::new(Rank::Three, Suit::Diamonds).id(),
        Card::new(Rank::Four, Suit::Hearts).id(),
        Card::new(Rank::Five, Suit::Spades).id(),
    ];
    for (seat, card) in VowTideSeat::ALL
        .into_iter()
        .take(state.seat_count())
        .zip(canaries)
    {
        *state.hand_for_internal_mut(seat).expect("hand exists") = vec![card];
    }
    state.trump_indicator = Card::new(Rank::Ace, Suit::Spades).id();
    state.hidden_stock = vec![Card::new(Rank::King, Suit::Clubs).id()];
}
