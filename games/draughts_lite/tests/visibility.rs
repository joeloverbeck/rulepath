use draughts_lite::{
    apply_action, project_view, setup_match, validate_command, CellOccupancy, DraughtsLiteSeat,
    PieceId, SetupOptions, TerminalOutcome, TerminalView,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &draughts_lite::DraughtsLiteState, segments: &[&str]) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: segments
                .iter()
                .map(|segment| (*segment).to_owned())
                .collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn public_view_is_complete_viewer_safe_and_stably_serialized() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let action = validate_command(&state, &command(&state, &["from/r3c2", "to/r4c1"])).unwrap();
    apply_action(&mut state, action);

    let view = project_view(&state, &Viewer { seat_id: None });
    let summary = view.stable_summary();

    assert_eq!(view.game_id, "draughts_lite");
    assert_eq!(view.cells.len(), 64);
    assert_eq!(view.cells.iter().filter(|cell| cell.playable).count(), 32);
    assert_eq!(
        view.private_view.status,
        "not_applicable_perfect_information"
    );
    assert!(view.private_view.hidden_fields.is_empty());
    assert_eq!(summary.as_bytes(), view.stable_bytes());
    assert!(!summary.contains("debug"));
    assert!(!summary.contains("candidate"));
    assert!(!summary.contains("internal"));
    assert!(!summary.contains("rng"));
    assert!(!summary.contains("seed"));
}

#[test]
fn occupied_and_non_playable_cells_project_public_metadata_only() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let view = project_view(&state, &Viewer { seat_id: None });

    let occupied = view
        .cells
        .iter()
        .find(|cell| cell.cell_id == "r1c2")
        .unwrap();
    assert_eq!(occupied.occupancy, "occupied");
    assert_eq!(occupied.owner, Some(DraughtsLiteSeat::Seat0));
    assert_eq!(
        occupied.piece_id.as_deref(),
        Some(
            PieceId::new(DraughtsLiteSeat::Seat0, 1)
                .unwrap()
                .stable_id()
                .as_str()
        )
    );
    assert_eq!(occupied.piece_shape_label.as_deref(), Some("ring man"));
    assert_eq!(
        occupied.piece_accessibility_label.as_deref(),
        Some("seat_0 man at r1c2")
    );

    let non_playable = view
        .cells
        .iter()
        .find(|cell| cell.cell_id == "r1c1")
        .unwrap();
    assert!(!non_playable.playable);
    assert_eq!(non_playable.occupancy, "empty");
    assert_eq!(non_playable.owner, None);
    assert_eq!(non_playable.presentation_token, "non_playable_light_square");
}

#[test]
fn terminal_view_has_no_active_seat_and_preserves_final_board() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.terminal_outcome = Some(TerminalOutcome::Win {
        seat: DraughtsLiteSeat::Seat1,
    });
    let view = project_view(&state, &Viewer { seat_id: None });

    assert_eq!(view.active_seat, None);
    assert_eq!(
        view.terminal,
        TerminalView::Win {
            winning_seat: DraughtsLiteSeat::Seat1
        }
    );
    assert_eq!(view.status_label, "seat_1 wins");
    assert_eq!(
        view.cells
            .iter()
            .filter(|cell| cell.occupancy == "occupied")
            .count(),
        24
    );
}

#[test]
fn public_view_matches_state_occupancy_for_every_cell() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let view = project_view(&state, &Viewer { seat_id: None });

    for cell in &view.cells {
        match state.occupancy(cell.cell).unwrap() {
            CellOccupancy::Empty => {
                assert_eq!(cell.occupancy, "empty");
                assert_eq!(cell.piece_id, None);
            }
            CellOccupancy::Occupied(piece_id) => {
                assert_eq!(cell.occupancy, "occupied");
                assert_eq!(
                    cell.piece_id.as_deref(),
                    Some(piece_id.stable_id().as_str())
                );
            }
        }
    }
}
