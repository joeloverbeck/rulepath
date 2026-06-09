use engine_core::{SeatId, Seed, Viewer};
use three_marks::{
    apply_action, project_view, setup_match, validate_command, CellId, CellOccupancy,
    OutcomeRationaleView, SetupOptions, TerminalOutcome, TerminalView, ThreeMarksSeat,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn viewer() -> Viewer {
    Viewer { seat_id: None }
}

fn command(
    state: &three_marks::ThreeMarksState,
    seat_index: usize,
    segment: &str,
) -> engine_core::CommandEnvelope {
    engine_core::CommandEnvelope {
        actor: engine_core::Actor {
            seat_id: seats()[seat_index].clone(),
        },
        action_path: engine_core::ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: engine_core::RulesVersion(1),
    }
}

fn place(state: &mut three_marks::ThreeMarksState, seat_index: usize, segment: &str) {
    let action = validate_command(state, &command(state, seat_index, segment)).unwrap();
    apply_action(state, action);
}

#[test]
fn public_view_contains_board_occupancy_legal_targets_and_metadata() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    place(&mut state, 0, "place/r2c2");

    let view = project_view(&state, &viewer());
    assert_eq!(view.game_id, "three_marks");
    assert_eq!(view.variant_id, "three_marks_standard");
    assert_eq!(view.rules_version_label, "three_marks-rules-v1");
    assert_eq!(view.board_rows, 3);
    assert_eq!(view.board_columns, 3);
    assert_eq!(view.cells.len(), 9);

    let center = view
        .cells
        .iter()
        .find(|cell| cell.cell == CellId::R2C2)
        .unwrap();
    assert_eq!(center.occupancy, "occupied");
    assert_eq!(center.owner, Some(ThreeMarksSeat::Seat0));
    assert_eq!(center.mark_token_key.as_deref(), Some("first_mark_loop"));
    assert_eq!(center.mark_shape_label.as_deref(), Some("loop mark"));

    assert_eq!(view.active_seat, ThreeMarksSeat::Seat1);
    assert_eq!(view.ply_count, 1);
    assert_eq!(view.legal_targets.len(), 8);
    assert!(view
        .legal_targets
        .iter()
        .all(|target| target.freshness_token == state.freshness_token));
    assert!(!view
        .legal_targets
        .iter()
        .any(|target| target.cell == CellId::R2C2));
}

#[test]
fn terminal_win_and_draw_are_projected_without_ui_inference() {
    let mut win_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    for (seat_index, segment) in [
        (0, "place/r1c1"),
        (1, "place/r2c1"),
        (0, "place/r1c2"),
        (1, "place/r2c2"),
        (0, "place/r1c3"),
    ] {
        place(&mut win_state, seat_index, segment);
    }
    let win_view = project_view(&win_state, &viewer());
    assert_eq!(
        win_view.terminal,
        TerminalView::Win {
            winning_seat: ThreeMarksSeat::Seat0,
            line: [CellId::R1C1, CellId::R1C2, CellId::R1C3],
            rationale: OutcomeRationaleView {
                result_kind: "win".to_owned(),
                decisive_cause: "line_completed".to_owned(),
                template_key: "three_marks.line_completed".to_owned(),
                decisive_rule_ids: vec!["TM-SCORE-001".to_owned(), "TM-END-001".to_owned()],
                line_cells: vec![CellId::R1C1, CellId::R1C2, CellId::R1C3],
                line_orientation: Some("row".to_owned()),
                board_full: false,
            }
        }
    );
    let win_json = win_view.to_json();
    assert!(win_json.contains("\"outcome_template_key\":\"three_marks.line_completed\""));
    assert!(win_json.contains("\"outcome_line_cells\":[\"r1c1\",\"r1c2\",\"r1c3\"]"));
    assert!(win_view.legal_targets.is_empty());

    let mut draw_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    for (seat_index, segment) in [
        (0, "place/r1c1"),
        (1, "place/r1c2"),
        (0, "place/r1c3"),
        (1, "place/r2c2"),
        (0, "place/r2c1"),
        (1, "place/r2c3"),
        (0, "place/r3c2"),
        (1, "place/r3c1"),
        (0, "place/r3c3"),
    ] {
        place(&mut draw_state, seat_index, segment);
    }
    assert_eq!(draw_state.terminal_outcome, Some(TerminalOutcome::Draw));
    let draw_view = project_view(&draw_state, &viewer());
    assert_eq!(
        draw_view.terminal,
        TerminalView::Draw {
            rationale: OutcomeRationaleView {
                result_kind: "draw".to_owned(),
                decisive_cause: "full_board_no_line".to_owned(),
                template_key: "three_marks.full_board_draw".to_owned(),
                decisive_rule_ids: vec!["TM-SCORE-001".to_owned(), "TM-END-002".to_owned()],
                line_cells: Vec::new(),
                line_orientation: None,
                board_full: true,
            }
        }
    );
    assert_eq!(draw_view.status_label, "draw");
    assert!(draw_view.legal_targets.is_empty());
}

#[test]
fn public_view_is_viewer_safe_and_has_explicit_empty_private_surface() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let view = project_view(&state, &viewer());
    let json = view.to_json();

    assert_eq!(
        view.private_view.status,
        "not_applicable_perfect_information"
    );
    assert!(view.private_view.hidden_fields.is_empty());
    assert!(!json.contains("debug"));
    assert!(!json.contains("internal"));
    assert!(json.contains("not_applicable_perfect_information"));
}

#[test]
fn public_view_serialization_round_trips_and_rejects_unknown_fields() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    let view = project_view(&state, &viewer());
    let json = view.to_json();

    let parsed = three_marks::PublicView::from_json(&json).expect("view round-trips");
    assert_eq!(parsed, view);
    assert_eq!(parsed.to_json(), json);
    assert!(three_marks::PublicView::from_json("{\"debug\":\"nope\"}").is_err());
}
