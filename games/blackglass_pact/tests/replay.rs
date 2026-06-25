use blackglass_pact::{
    canonical_seat_ids, export_for_viewer, export_stable_bytes, import_for_viewer, setup_match,
    BlackglassSeat, BlackglassViewer, Card, Rank, SetupOptions, Suit, ADR_0009_MIGRATION_NOTE,
    PUBLIC_EXPORT_V1, SEAT_PRIVATE_EXPORT_V1,
};
use engine_core::Seed;

#[test]
fn public_and_seat_private_exports_round_trip_for_same_viewer() {
    let state = private_hand_state();

    let public_export = export_for_viewer(&state, BlackglassViewer::Observer);
    assert_eq!(public_export.format, PUBLIC_EXPORT_V1);
    assert_eq!(public_export.migration_note, ADR_0009_MIGRATION_NOTE);
    assert_eq!(
        import_for_viewer(&public_export, BlackglassViewer::Observer).expect("public import"),
        public_export.view
    );

    for seat in BlackglassSeat::ALL {
        let viewer = BlackglassViewer::Seat(seat);
        let export = export_for_viewer(&state, viewer);
        assert_eq!(export.format, SEAT_PRIVATE_EXPORT_V1);
        assert_eq!(
            import_for_viewer(&export, viewer).expect("seat import"),
            export.view
        );
    }
}

#[test]
fn seat_private_export_cannot_import_as_other_viewer() {
    let state = private_hand_state();
    let north_export = export_for_viewer(&state, BlackglassViewer::Seat(BlackglassSeat::North));

    let diagnostic =
        import_for_viewer(&north_export, BlackglassViewer::Seat(BlackglassSeat::South))
            .expect_err("cross-seat import rejected");
    assert_eq!(diagnostic.code, "BP_VIEWER_SCOPE_MISMATCH");

    let diagnostic = import_for_viewer(&north_export, BlackglassViewer::Observer)
        .expect_err("seat import as observer rejected");
    assert_eq!(diagnostic.code, "BP_VIEWER_SCOPE_MISMATCH");
}

#[test]
fn export_stable_bytes_are_viewer_scoped_and_repeatable() {
    let state = private_hand_state();
    let north = export_for_viewer(&state, BlackglassViewer::Seat(BlackglassSeat::North));
    let south = export_for_viewer(&state, BlackglassViewer::Seat(BlackglassSeat::South));

    assert_eq!(export_stable_bytes(&north), export_stable_bytes(&north));
    assert_ne!(export_stable_bytes(&north), export_stable_bytes(&south));
}

fn private_hand_state() -> blackglass_pact::BlackglassPactState {
    let mut state = setup_match(Seed(1821), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.private_hands = vec![
        (
            BlackglassSeat::North,
            vec![
                Card::new(Rank::Two, Suit::Clubs).id(),
                Card::new(Rank::Three, Suit::Clubs).id(),
            ],
        ),
        (
            BlackglassSeat::East,
            vec![
                Card::new(Rank::Four, Suit::Diamonds).id(),
                Card::new(Rank::Five, Suit::Diamonds).id(),
            ],
        ),
        (
            BlackglassSeat::South,
            vec![
                Card::new(Rank::Six, Suit::Hearts).id(),
                Card::new(Rank::Seven, Suit::Hearts).id(),
            ],
        ),
        (
            BlackglassSeat::West,
            vec![
                Card::new(Rank::Eight, Suit::Spades).id(),
                Card::new(Rank::Nine, Suit::Spades).id(),
            ],
        ),
    ];
    state
}
