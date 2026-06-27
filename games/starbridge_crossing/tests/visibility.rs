use engine_core::{SeatId, Viewer};
use starbridge_crossing::{project_view, setup_match, SetupOptions, StarbridgePublicView};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

fn viewer(seat: Option<&SeatId>) -> Viewer {
    Viewer {
        seat_id: seat.cloned(),
    }
}

fn public_view(count: usize) -> (Vec<SeatId>, StarbridgePublicView) {
    let seats = seats(count);
    let state = setup_match(engine_core::Seed(7), &seats, &SetupOptions::default()).unwrap();
    let view = project_view(&state, &viewer(None));
    (seats, view)
}

#[test]
fn public_observer_view_contains_all_board_facts() {
    let (_seats, view) = public_view(2);

    assert_eq!(view.game_id, "starbridge_crossing");
    assert_eq!(view.spaces.len(), 121);
    assert_eq!(view.seats.len(), 2);
    assert_eq!(view.active_seat.as_deref(), Some("seat_0"));
    assert_eq!(view.finish_ranks, Vec::new());
    assert_eq!(view.terminal, None);
    assert_eq!(
        view.spaces
            .iter()
            .filter(|space| space.occupant.is_some())
            .count(),
        20
    );
    assert!(view
        .spaces
        .iter()
        .all(|space| !space.ui.zone_label.is_empty()));
}

#[test]
fn every_supported_seat_view_matches_public_observer_board_facts() {
    for count in [2, 3, 4, 6] {
        let seats = seats(count);
        let state = setup_match(engine_core::Seed(7), &seats, &SetupOptions::default()).unwrap();
        let observer = project_view(&state, &viewer(None));

        for seat in &seats {
            let seat_view = project_view(&state, &viewer(Some(seat)));

            assert_eq!(seat_view, observer);
            assert_eq!(seat_view.stable_summary(), observer.stable_summary());
        }
    }
}

#[test]
fn no_private_visibility_class_exists_for_starbridge() {
    let (_seats, view) = public_view(6);
    let summary = view.stable_summary();

    assert_eq!(view.audit.redaction_class, "none");
    assert!(view.audit.private_fields.is_empty());
    assert!(!summary.contains("private"));
    assert!(!summary.contains("hidden"));
    assert!(!summary.contains("redacted"));
}
