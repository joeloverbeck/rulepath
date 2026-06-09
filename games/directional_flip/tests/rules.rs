use directional_flip::{
    apply_action, legal_action_tree, setup_match, validate_command, CellId, CellOccupancy,
    ColumnId, DirectionalFlipSeat, DirectionalFlipState, RowId, SetupOptions, TerminalOutcome,
    TerminalReason,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn state() -> directional_flip::DirectionalFlipState {
    setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()
}

fn command(state: &directional_flip::DirectionalFlipState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn command_for_seat(
    state: &directional_flip::DirectionalFlipState,
    seat: DirectionalFlipSeat,
    segment: &str,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn cell(row: RowId, column: ColumnId) -> CellId {
    CellId::new(row, column)
}

fn occupy(
    state: &mut directional_flip::DirectionalFlipState,
    cell: CellId,
    seat: DirectionalFlipSeat,
) {
    state.set_occupancy(cell, CellOccupancy::Occupied(seat));
}

fn empty_with_active(active: DirectionalFlipSeat) -> directional_flip::DirectionalFlipState {
    let mut state = state();
    state.cells = DirectionalFlipState::empty_cells();
    state.active_seat = active;
    state.ply_count = 0;
    state.consecutive_forced_passes = 0;
    state.terminal_outcome = None;
    state
}

fn action_segments(state: &directional_flip::DirectionalFlipState) -> Vec<String> {
    legal_action_tree(
        state,
        &Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
    )
    .root
    .choices
    .into_iter()
    .map(|choice| choice.segment)
    .collect()
}

#[test]
fn df_setup_001_standard_setup_first_seat_and_opening_cells() {
    let state = state();

    assert_eq!(state.active_seat, DirectionalFlipSeat::Seat0);
    assert_eq!(
        state.occupancy(cell(RowId::R4, ColumnId::C5)),
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
    );
    assert_eq!(
        state.occupancy(cell(RowId::R5, ColumnId::C4)),
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
    );
    assert_eq!(
        state.occupancy(cell(RowId::R4, ColumnId::C4)),
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat1)
    );
    assert_eq!(
        state.occupancy(cell(RowId::R5, ColumnId::C5)),
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat1)
    );
}

#[test]
fn df_action_001_002_003_legal_placements_and_forced_pass_surface() {
    let state = state();
    assert_eq!(
        action_segments(&state),
        vec!["place/r3c4", "place/r4c3", "place/r5c6", "place/r6c5"]
    );

    let pass = validate_command(&state, &command(&state, "pass/forced"))
        .expect_err("pass is forbidden when a placement exists");
    assert_eq!(pass.code, "pass_not_available");

    let no_move = no_move_state();
    assert_eq!(action_segments(&no_move), vec!["pass/forced"]);
}

#[test]
fn df_legal_001_to_006_validation_diagnostics_are_fail_closed() {
    let state = state();

    assert_eq!(
        validate_command(&state, &command(&state, "place/r4c4"))
            .expect_err("occupied target rejected")
            .code,
        "occupied_cell"
    );
    assert_eq!(
        validate_command(&state, &command(&state, "place/r1c1"))
            .expect_err("non-flipping target rejected")
            .code,
        "non_flipping_placement"
    );
    assert_eq!(
        validate_command(&state, &command(&state, "place/r9c9"))
            .expect_err("bad cell rejected")
            .code,
        "invalid_cell"
    );
    let mut stale = command(&state, "place/r3c4");
    stale.freshness_token = engine_core::FreshnessToken(99);
    assert_eq!(
        validate_command(&state, &stale)
            .expect_err("stale command rejected")
            .code,
        "stale_action"
    );
    assert_eq!(
        validate_command(
            &state,
            &command_for_seat(&state, DirectionalFlipSeat::Seat1, "place/r3c4")
        )
        .expect_err("wrong actor rejected")
        .code,
        "not_active_seat"
    );
}

#[test]
fn df_flip_001_to_004_all_directions_stable_order_and_no_indirect_flips() {
    let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
    for adjacent in [
        cell(RowId::R3, ColumnId::C4),
        cell(RowId::R3, ColumnId::C5),
        cell(RowId::R4, ColumnId::C5),
        cell(RowId::R5, ColumnId::C5),
        cell(RowId::R5, ColumnId::C4),
        cell(RowId::R5, ColumnId::C3),
        cell(RowId::R4, ColumnId::C3),
        cell(RowId::R3, ColumnId::C3),
    ] {
        occupy(&mut state, adjacent, DirectionalFlipSeat::Seat1);
    }
    for anchor in [
        cell(RowId::R2, ColumnId::C4),
        cell(RowId::R2, ColumnId::C6),
        cell(RowId::R4, ColumnId::C6),
        cell(RowId::R6, ColumnId::C6),
        cell(RowId::R6, ColumnId::C4),
        cell(RowId::R6, ColumnId::C2),
        cell(RowId::R4, ColumnId::C2),
        cell(RowId::R2, ColumnId::C2),
    ] {
        occupy(&mut state, anchor, DirectionalFlipSeat::Seat0);
    }

    let action = validate_command(&state, &command(&state, "place/r4c4")).unwrap();
    let effects = apply_action(&mut state, action);

    for adjacent in [
        cell(RowId::R3, ColumnId::C4),
        cell(RowId::R3, ColumnId::C5),
        cell(RowId::R4, ColumnId::C5),
        cell(RowId::R5, ColumnId::C5),
        cell(RowId::R5, ColumnId::C4),
        cell(RowId::R5, ColumnId::C3),
        cell(RowId::R4, ColumnId::C3),
        cell(RowId::R3, ColumnId::C3),
    ] {
        assert_eq!(
            state.occupancy(adjacent),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
        );
    }
    assert!(format!("{effects:?}").contains("DiscsFlipped"));
    assert!(format!("{effects:?}").contains("North"));
    assert!(format!("{effects:?}").contains("Northwest"));
}

#[test]
fn df_pass_001_002_term_001_score_001_002_forced_pass_terminal_and_scores() {
    let mut state = no_move_state();

    let first = validate_command(&state, &command(&state, "pass/forced")).unwrap();
    apply_action(&mut state, first);
    assert_eq!(state.active_seat, DirectionalFlipSeat::Seat1);
    assert_eq!(state.terminal_outcome, None);

    let second = validate_command(&state, &command(&state, "pass/forced")).unwrap();
    apply_action(&mut state, second);
    assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
    assert_eq!(
        state.terminal_reason,
        Some(TerminalReason::DoubleForcedPass)
    );
    assert!(action_segments(&state).is_empty());
}

#[test]
fn df_score_001_higher_disc_count_wins_on_full_board_terminal() {
    let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
    for cell in CellId::ALL {
        occupy(&mut state, cell, DirectionalFlipSeat::Seat0);
    }
    state.set_occupancy(cell(RowId::R1, ColumnId::C1), CellOccupancy::Empty);
    occupy(
        &mut state,
        cell(RowId::R1, ColumnId::C2),
        DirectionalFlipSeat::Seat1,
    );

    let action = validate_command(&state, &command(&state, "place/r1c1")).unwrap();
    apply_action(&mut state, action);

    assert_eq!(
        state.terminal_outcome,
        Some(TerminalOutcome::Win {
            seat: DirectionalFlipSeat::Seat0
        })
    );
    assert_eq!(state.terminal_reason, Some(TerminalReason::BoardFull));
    assert!(action_segments(&state).is_empty());
}

fn no_move_state() -> directional_flip::DirectionalFlipState {
    let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
    occupy(
        &mut state,
        cell(RowId::R1, ColumnId::C1),
        DirectionalFlipSeat::Seat0,
    );
    occupy(
        &mut state,
        cell(RowId::R8, ColumnId::C8),
        DirectionalFlipSeat::Seat1,
    );
    state
}
