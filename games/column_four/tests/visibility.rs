use column_four::{
    apply_action, project_view, setup_match, CellId, CellOccupancy, ColumnFourSeat, ColumnId,
    RowId, SetupOptions, TerminalOutcome,
};
use engine_core::{SeatId, Seed, StableSerialize, Viewer};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

#[test]
fn public_view_is_viewer_safe_and_stably_serialized() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    apply_action(
        &mut state,
        column_four::ValidatedAction {
            actor: ColumnFourSeat::Seat0,
            column: ColumnId::C4,
        },
    );

    let view = project_view(&state, &Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert_eq!(
        view.private_view.status,
        "not_applicable_perfect_information"
    );
    assert!(view.private_view.hidden_fields.is_empty());
    assert!(!summary.contains("debug"));
    assert!(!summary.contains("internal"));
    assert!(!summary.contains("candidate"));
    assert_eq!(summary.as_bytes(), view.stable_bytes());
    assert_eq!(
        view.cells.first().unwrap().cell,
        CellId::new(RowId::R1, ColumnId::C1)
    );
    assert_eq!(
        view.cells.last().unwrap().cell,
        CellId::new(RowId::R6, ColumnId::C7)
    );
}

#[test]
fn terminal_view_has_no_active_seat_or_legal_targets() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.terminal_outcome = Some(TerminalOutcome::Draw);
    let view = project_view(&state, &Viewer { seat_id: None });

    assert_eq!(view.active_seat, None);
    assert!(view.legal_targets.is_empty());
    assert_eq!(view.terminal, column_four::TerminalView::Draw);
}

#[test]
fn occupied_cells_have_public_piece_tokens_only() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.set_occupancy(
        CellId::new(RowId::R1, ColumnId::C1),
        CellOccupancy::Occupied(ColumnFourSeat::Seat0),
    );
    let view = project_view(&state, &Viewer { seat_id: None });
    let occupied = view
        .cells
        .iter()
        .find(|cell| cell.cell == CellId::new(RowId::R1, ColumnId::C1))
        .unwrap();

    assert_eq!(occupied.owner, Some(ColumnFourSeat::Seat0));
    assert_eq!(
        occupied.piece_token_key.as_deref(),
        Some("first_piece_ring")
    );
    assert_eq!(occupied.piece_shape_label.as_deref(), Some("ring piece"));
}
