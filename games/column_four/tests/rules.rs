use column_four::{
    apply_action, legal_action_tree, setup_match, validate_command, CellId, CellOccupancy,
    ColumnFourSeat, ColumnId, RowId, SetupOptions, TerminalOutcome, WinningLine,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(seat: ColumnFourSeat) -> Actor {
    Actor {
        seat_id: seats()[seat.index()].clone(),
    }
}

fn command(
    state: &column_four::ColumnFourState,
    seat: ColumnFourSeat,
    segment: &str,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn place(state: &mut column_four::ColumnFourState, seat: ColumnFourSeat, segment: &str) {
    let action = validate_command(state, &command(state, seat, segment)).expect("move validates");
    apply_action(state, action);
}

fn legal_segments(state: &column_four::ColumnFourState, seat: ColumnFourSeat) -> Vec<String> {
    legal_action_tree(state, &actor(seat))
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect()
}

fn cell(row: RowId, column: ColumnId) -> CellId {
    CellId::new(row, column)
}

#[test]
fn legal_columns_gravity_turns_and_full_column_rejection_work_together() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    assert_eq!(
        legal_segments(&state, ColumnFourSeat::Seat0),
        vec!["drop/c1", "drop/c2", "drop/c3", "drop/c4", "drop/c5", "drop/c6", "drop/c7"]
    );
    assert!(legal_segments(&state, ColumnFourSeat::Seat1).is_empty());

    place(&mut state, ColumnFourSeat::Seat0, "drop/c4");
    assert_eq!(
        state.occupancy(cell(RowId::R1, ColumnId::C4)),
        CellOccupancy::Occupied(ColumnFourSeat::Seat0)
    );
    assert_eq!(state.active_seat, ColumnFourSeat::Seat1);

    for _ in 0..5 {
        let active = state.active_seat;
        place(&mut state, active, "drop/c4");
    }
    assert!(!legal_segments(&state, state.active_seat).contains(&"drop/c4".to_owned()));
    assert_eq!(
        validate_command(&state, &command(&state, state.active_seat, "drop/c4"))
            .expect_err("full column rejected")
            .code,
        "full_column"
    );
}

#[test]
fn validation_rejects_stale_wrong_actor_invalid_unknown_and_terminal_without_mutation() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();

    let mut stale = command(&state, ColumnFourSeat::Seat0, "drop/c1");
    stale.freshness_token = FreshnessToken(99);
    assert_eq!(
        validate_command(&state, &stale)
            .expect_err("stale rejected")
            .code,
        "stale_action"
    );
    assert_eq!(
        validate_command(&state, &command(&state, ColumnFourSeat::Seat1, "drop/c1"))
            .expect_err("wrong actor rejected")
            .code,
        "not_active_seat"
    );
    let mut invalid_path = command(&state, ColumnFourSeat::Seat0, "drop/c1");
    invalid_path.action_path.segments.push("extra".to_owned());
    assert_eq!(
        validate_command(&state, &invalid_path)
            .expect_err("invalid path rejected")
            .code,
        "invalid_action_path"
    );
    assert_eq!(
        validate_command(&state, &command(&state, ColumnFourSeat::Seat0, "drop/c8"))
            .expect_err("unknown column rejected")
            .code,
        "unknown_column"
    );

    state.terminal_outcome = Some(TerminalOutcome::Draw);
    let before = state.clone();
    assert_eq!(
        validate_command(&state, &command(&state, ColumnFourSeat::Seat0, "drop/c1"))
            .expect_err("terminal rejected")
            .code,
        "terminal_match"
    );
    assert_eq!(state, before);
}

#[test]
fn horizontal_vertical_and_diagonal_wins_are_reported_from_real_moves() {
    let cases = [
        (
            vec![
                "drop/c1", "drop/c1", "drop/c2", "drop/c2", "drop/c3", "drop/c3", "drop/c4",
            ],
            WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R1, ColumnId::C2),
                    cell(RowId::R1, ColumnId::C3),
                    cell(RowId::R1, ColumnId::C4),
                ],
            },
        ),
        (
            vec![
                "drop/c1", "drop/c2", "drop/c1", "drop/c2", "drop/c1", "drop/c2", "drop/c1",
            ],
            WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R2, ColumnId::C1),
                    cell(RowId::R3, ColumnId::C1),
                    cell(RowId::R4, ColumnId::C1),
                ],
            },
        ),
        (
            vec![
                "drop/c1", "drop/c2", "drop/c2", "drop/c3", "drop/c3", "drop/c4", "drop/c3",
                "drop/c4", "drop/c4", "drop/c7", "drop/c4",
            ],
            WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R2, ColumnId::C2),
                    cell(RowId::R3, ColumnId::C3),
                    cell(RowId::R4, ColumnId::C4),
                ],
            },
        ),
        (
            vec![
                "drop/c4", "drop/c3", "drop/c3", "drop/c2", "drop/c2", "drop/c1", "drop/c2",
                "drop/c1", "drop/c1", "drop/c7", "drop/c1",
            ],
            WinningLine {
                cells: [
                    cell(RowId::R4, ColumnId::C1),
                    cell(RowId::R3, ColumnId::C2),
                    cell(RowId::R2, ColumnId::C3),
                    cell(RowId::R1, ColumnId::C4),
                ],
            },
        ),
    ];

    for (moves, line) in cases {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
        for segment in moves {
            let active = state.active_seat;
            place(&mut state, active, segment);
        }
        assert_eq!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win {
                seat: ColumnFourSeat::Seat0,
                line
            })
        );
        assert!(legal_segments(&state, ColumnFourSeat::Seat0).is_empty());
    }
}

#[test]
fn full_board_without_line_draw_and_win_precedence_are_preserved() {
    let mut draw_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let rows = [
        [0, 0, 1, 1, 0, 0, 1],
        [1, 1, 0, 0, 1, 1, 0],
        [0, 0, 1, 1, 0, 0, 1],
        [1, 1, 0, 0, 1, 1, 0],
        [0, 0, 1, 1, 0, 0, 1],
        [1, 1, 0, 0, 1, 1, 0],
    ];
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, seat_index) in row.iter().enumerate() {
            draw_state.set_occupancy(
                column_four::ColumnFourState::cell(row_index, column_index).unwrap(),
                CellOccupancy::Occupied(if *seat_index == 0 {
                    ColumnFourSeat::Seat0
                } else {
                    ColumnFourSeat::Seat1
                }),
            );
        }
    }
    draw_state.set_occupancy(cell(RowId::R6, ColumnId::C7), CellOccupancy::Empty);
    draw_state.ply_count = 41;
    apply_action(
        &mut draw_state,
        column_four::ValidatedAction {
            actor: ColumnFourSeat::Seat1,
            column: ColumnId::C7,
        },
    );
    assert_eq!(draw_state.terminal_outcome, Some(TerminalOutcome::Draw));

    let mut win_state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    for cell in CellId::ALL {
        win_state.set_occupancy(cell, CellOccupancy::Occupied(ColumnFourSeat::Seat1));
    }
    for row in RowId::ALL {
        win_state.set_occupancy(
            cell(row, ColumnId::C1),
            CellOccupancy::Occupied(ColumnFourSeat::Seat0),
        );
    }
    win_state.set_occupancy(cell(RowId::R6, ColumnId::C1), CellOccupancy::Empty);
    win_state.ply_count = 41;
    apply_action(
        &mut win_state,
        column_four::ValidatedAction {
            actor: ColumnFourSeat::Seat0,
            column: ColumnId::C1,
        },
    );
    assert!(matches!(
        win_state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: ColumnFourSeat::Seat0,
            ..
        })
    ));
}
