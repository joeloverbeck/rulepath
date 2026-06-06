use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, Game, RulesVersion, SeatId, Seed,
};
use three_marks::{
    apply_action, legal_action_tree, setup_match, validate_command, validate_command_with_effects,
    CellId, CellOccupancy, RejectionReason, SetupOptions, TerminalOutcome, ThreeMarks,
    ThreeMarksEffect, ThreeMarksSeat, ValidatedAction,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor(index: usize) -> Actor {
    Actor {
        seat_id: seats()[index].clone(),
    }
}

fn command(index: usize, segment: &str, freshness_token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(index),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn place(state: &mut three_marks::ThreeMarksState, seat_index: usize, segment: &str) {
    let action = validate_command(state, &command(seat_index, segment, state.freshness_token))
        .expect("placement validates");
    apply_action(state, action);
}

fn legal_segments(state: &three_marks::ThreeMarksState, seat_index: usize) -> Vec<String> {
    legal_action_tree(state, &actor(seat_index))
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.clone())
        .collect()
}

#[test]
fn legal_actions_are_empty_cells_for_active_actor() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();

    assert_eq!(
        legal_segments(&state, 0),
        vec![
            "place/r1c1",
            "place/r1c2",
            "place/r1c3",
            "place/r2c1",
            "place/r2c2",
            "place/r2c3",
            "place/r3c1",
            "place/r3c2",
            "place/r3c3"
        ]
    );
    assert!(legal_segments(&state, 1).is_empty());

    place(&mut state, 0, "place/r2c2");
    assert!(!legal_segments(&state, 1).contains(&"place/r2c2".to_owned()));
    assert_eq!(legal_segments(&state, 1).len(), 8);
}

#[test]
fn validation_rejects_occupied_invalid_stale_wrong_actor_and_terminal_without_mutation() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();

    let stale = validate_command(&state, &command(0, "place/r1c1", FreshnessToken(99)))
        .expect_err("stale token rejected");
    assert_eq!(stale.code, "stale_action");

    let wrong_actor = validate_command(&state, &command(1, "place/r1c1", state.freshness_token))
        .expect_err("wrong actor rejected");
    assert_eq!(wrong_actor.code, "wrong_actor");

    let invalid = validate_command(&state, &command(0, "place/r9c9", state.freshness_token))
        .expect_err("invalid cell rejected");
    assert_eq!(invalid.code, "invalid_cell");

    place(&mut state, 0, "place/r1c1");
    let before_occupied = state.clone();
    let occupied =
        validate_command_with_effects(&state, &command(1, "place/r1c1", state.freshness_token))
            .expect_err("occupied cell rejected");
    assert_eq!(occupied.diagnostic.code, "occupied_cell");
    assert!(matches!(
        occupied.effects[0].payload,
        ThreeMarksEffect::PlacementRejected {
            reason: RejectionReason::Occupied,
            ..
        }
    ));
    assert_eq!(state, before_occupied);

    state.terminal_outcome = Some(TerminalOutcome::Draw);
    let before_terminal = state.clone();
    let terminal = validate_command(&state, &command(1, "place/r1c2", state.freshness_token))
        .expect_err("terminal match rejects further commands");
    assert_eq!(terminal.code, "match_finished");
    assert_eq!(state, before_terminal);
}

#[test]
fn valid_action_places_mark_advances_turn_ply_and_token() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let action = validate_command(&state, &command(0, "place/r2c2", state.freshness_token))
        .expect("placement validates");
    let effects = apply_action(&mut state, action);

    assert_eq!(effects.len(), 2);
    assert!(matches!(
        effects[0].payload,
        ThreeMarksEffect::MarkPlaced {
            seat: ThreeMarksSeat::Seat0,
            cell: CellId::R2C2,
            ply: 1,
            ..
        }
    ));
    assert!(matches!(
        effects[1].payload,
        ThreeMarksEffect::ActivePlayerChanged {
            previous_seat: ThreeMarksSeat::Seat0,
            active_seat: ThreeMarksSeat::Seat1,
            ply: 1
        }
    ));
    assert_eq!(
        state.occupancy(CellId::R2C2),
        CellOccupancy::Occupied(ThreeMarksSeat::Seat0)
    );
    assert_eq!(state.active_seat, ThreeMarksSeat::Seat1);
    assert_eq!(state.ply_count, 1);
    assert_eq!(state.freshness_token, FreshnessToken(1));
}

#[test]
fn row_column_and_diagonal_wins_report_ordered_line_cells() {
    let cases = [
        (
            [
                "place/r1c1",
                "place/r2c1",
                "place/r1c2",
                "place/r2c2",
                "place/r1c3",
            ],
            [CellId::R1C1, CellId::R1C2, CellId::R1C3],
        ),
        (
            [
                "place/r1c1",
                "place/r1c2",
                "place/r2c1",
                "place/r2c2",
                "place/r3c1",
            ],
            [CellId::R1C1, CellId::R2C1, CellId::R3C1],
        ),
        (
            [
                "place/r1c1",
                "place/r1c2",
                "place/r2c2",
                "place/r1c3",
                "place/r3c3",
            ],
            [CellId::R1C1, CellId::R2C2, CellId::R3C3],
        ),
        (
            [
                "place/r1c3",
                "place/r1c1",
                "place/r2c2",
                "place/r1c2",
                "place/r3c1",
            ],
            [CellId::R1C3, CellId::R2C2, CellId::R3C1],
        ),
    ];

    for (moves, expected_line) in cases {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
        for (index, segment) in moves.iter().enumerate() {
            place(&mut state, index % 2, segment);
        }

        assert_eq!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win {
                seat: ThreeMarksSeat::Seat0,
                line: three_marks::WinningLine {
                    cells: expected_line
                }
            })
        );
        assert!(legal_segments(&state, 0).is_empty());
        assert!(legal_segments(&state, 1).is_empty());
    }
}

#[test]
fn full_board_without_line_is_draw() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let mut effects = Vec::new();
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
        let action = validate_command(&state, &command(seat_index, segment, state.freshness_token))
            .expect("placement validates");
        effects = apply_action(&mut state, action);
    }

    assert_eq!(state.ply_count, 9);
    assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
    assert!(matches!(
        effects[1].payload,
        ThreeMarksEffect::DrawReached {
            final_ply: 9,
            full_board: true
        }
    ));
    assert!(matches!(
        effects[2].payload,
        ThreeMarksEffect::GameEnded {
            outcome: TerminalOutcome::Draw,
            final_ply: 9,
            ..
        }
    ));
    assert!(legal_segments(&state, 0).is_empty());
    assert!(legal_segments(&state, 1).is_empty());
}

#[test]
fn game_impl_uses_rules_surface() {
    let game = ThreeMarks;
    let mut state = game
        .setup(Seed(1), &seats(), &SetupOptions::default())
        .expect("setup succeeds");
    let tree = game.legal_action_tree(&state, &actor(0));
    let path = tree.root.choices[0].path();
    let action = game
        .validate(
            &state,
            &CommandEnvelope {
                actor: actor(0),
                action_path: path,
                freshness_token: tree.freshness_token,
                rules_version: RulesVersion(1),
            },
        )
        .expect("action validates");

    let mut rng = engine_core::SeededRng::from_seed(Seed(0));
    let effects = game.apply(&mut state, action, &mut rng);

    assert_eq!(
        state.occupancy(CellId::R1C1),
        CellOccupancy::Occupied(ThreeMarksSeat::Seat0)
    );
    assert!(matches!(
        effects[0].payload,
        ThreeMarksEffect::MarkPlaced {
            seat: ThreeMarksSeat::Seat0,
            cell: CellId::R1C1,
            ..
        }
    ));
}

#[test]
fn direct_validated_action_can_drive_terminal_state() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.cells[CellId::R1C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    state.cells[CellId::R1C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat0);
    state.cells[CellId::R2C1.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.cells[CellId::R2C2.index()] = CellOccupancy::Occupied(ThreeMarksSeat::Seat1);
    state.ply_count = 4;

    let effects = apply_action(
        &mut state,
        ValidatedAction {
            actor: ThreeMarksSeat::Seat0,
            cell: CellId::R1C3,
        },
    );

    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: ThreeMarksSeat::Seat0,
            line: three_marks::WinningLine {
                cells: [CellId::R1C1, CellId::R1C2, CellId::R1C3]
            }
        })
    );
    assert!(matches!(
        effects[1].payload,
        ThreeMarksEffect::LineCompleted {
            winning_seat: ThreeMarksSeat::Seat0,
            line: three_marks::WinningLine {
                cells: [CellId::R1C1, CellId::R1C2, CellId::R1C3]
            }
        }
    ));
    assert!(matches!(
        effects[2].payload,
        ThreeMarksEffect::GameEnded {
            outcome: TerminalOutcome::Win {
                seat: ThreeMarksSeat::Seat0,
                line: three_marks::WinningLine {
                    cells: [CellId::R1C1, CellId::R1C2, CellId::R1C3]
                }
            },
            final_ply: 5,
            ..
        }
    ));
}
