use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{actor_seat, parse_drop_segment},
    effects::{display_from_anchor, display_to_anchor, public_effect, ColumnFourEffect},
    ids::{board_dimensions, CellId, ColumnFourSeat, ColumnId, RowId},
    state::{CellOccupancy, ColumnFourSnapshot, ColumnFourState, TerminalOutcome, WinningLine},
};

const BOARD_COLUMNS: usize = 7;
const BOARD_ROWS: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: ColumnFourSeat,
    pub column: ColumnId,
}

pub fn validate_command(
    state: &ColumnFourState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if state.terminal_outcome.is_some() {
        return Err(diagnostic(
            "terminal_match",
            "the match is already finished",
        ));
    }

    if command.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "stale_action",
            "the action was submitted for an older decision point",
        ));
    }

    let Some(actor) = actor_seat(state, &command.actor) else {
        return Err(diagnostic("not_active_seat", "the actor is not seated"));
    };

    if actor != state.active_seat {
        return Err(diagnostic(
            "not_active_seat",
            "only the active seat may act now",
        ));
    }

    if command.action_path.segments.len() != 1 {
        return Err(diagnostic(
            "invalid_action_path",
            "the action path is not available",
        ));
    }

    let column = parse_drop_segment(&command.action_path.segments[0])
        .ok_or_else(|| diagnostic("unknown_column", "the requested column does not exist"))?;

    if landing_cell(state, column).is_none() {
        return Err(diagnostic("full_column", "the requested column is full"));
    }

    Ok(ValidatedAction { actor, column })
}

pub fn apply_action(
    state: &mut ColumnFourState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<ColumnFourEffect>> {
    let landing = landing_cell(state, action.column)
        .expect("validated column action must have a landing cell");

    state.set_occupancy(landing, CellOccupancy::Occupied(action.actor));
    state.ply_count += 1;
    state.freshness_token = state.freshness_token.next();
    let mut effects = vec![
        public_effect(ColumnFourEffect::DropAccepted {
            seat: action.actor,
            column: action.column,
            ply: state.ply_count,
        }),
        public_effect(ColumnFourEffect::PieceLanded {
            seat: action.actor,
            column: action.column,
            row: landing.row,
            cell: landing,
            display_from_anchor: display_from_anchor(action.column),
            display_to_anchor: display_to_anchor(landing),
        }),
    ];

    if let Some(line) = winning_line(state, action.actor) {
        let outcome = TerminalOutcome::Win {
            seat: action.actor,
            line,
        };
        state.terminal_outcome = Some(outcome);
        effects.push(public_effect(ColumnFourEffect::WinDetected {
            winning_seat: action.actor,
            line,
        }));
        effects.push(public_effect(ColumnFourEffect::GameEnded {
            outcome,
            final_ply: state.ply_count,
            terminal_hash_ref: ColumnFourSnapshot::from_state(state).stable_summary(),
        }));
    } else if state.cells.iter().all(|cell| !cell.is_empty()) {
        state.terminal_outcome = Some(TerminalOutcome::Draw);
        effects.push(public_effect(ColumnFourEffect::DrawDetected {
            final_ply: state.ply_count,
            full_board: true,
        }));
        effects.push(public_effect(ColumnFourEffect::GameEnded {
            outcome: TerminalOutcome::Draw,
            final_ply: state.ply_count,
            terminal_hash_ref: ColumnFourSnapshot::from_state(state).stable_summary(),
        }));
    } else {
        let previous_seat = action.actor;
        state.active_seat = action.actor.other();
        effects.push(public_effect(ColumnFourEffect::ActivePlayerChanged {
            previous_seat,
            active_seat: state.active_seat,
            ply: state.ply_count,
        }));
    }

    effects
}

pub fn legal_columns(state: &ColumnFourState) -> Vec<ColumnId> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    ColumnId::ALL
        .into_iter()
        .filter(|column| landing_cell(state, *column).is_some())
        .collect()
}

pub fn landing_cell(state: &ColumnFourState, column: ColumnId) -> Option<CellId> {
    RowId::ALL
        .into_iter()
        .map(|row| CellId::new(row, column))
        .find(|cell| state.occupancy(*cell).is_empty())
}

pub fn winning_line(state: &ColumnFourState, seat: ColumnFourSeat) -> Option<WinningLine> {
    for direction in Direction::ALL {
        let mut candidates = winning_lines_for_direction(state, seat, direction);
        candidates.sort_by_key(line_key);
        if let Some(line) = candidates.into_iter().next() {
            return Some(line);
        }
    }

    None
}

fn winning_lines_for_direction(
    state: &ColumnFourState,
    seat: ColumnFourSeat,
    direction: Direction,
) -> Vec<WinningLine> {
    let mut lines = Vec::new();

    for row in 0..BOARD_ROWS {
        for column in 0..BOARD_COLUMNS {
            let cells = [
                offset_cell(row, column, direction, 0),
                offset_cell(row, column, direction, 1),
                offset_cell(row, column, direction, 2),
                offset_cell(row, column, direction, 3),
            ];
            let Some(cells) = collect_cells(cells) else {
                continue;
            };
            if cells
                .iter()
                .all(|cell| state.occupancy(*cell) == CellOccupancy::Occupied(seat))
            {
                lines.push(WinningLine { cells });
            }
        }
    }

    lines
}

fn offset_cell(row: usize, column: usize, direction: Direction, step: usize) -> Option<CellId> {
    let start =
        board_dimensions().coord(u8::try_from(row + 1).ok()?, u8::try_from(column + 1).ok()?)?;
    let coord = board_dimensions().offset(
        start,
        i16::try_from(direction.row_delta() * step as isize).ok()?,
        i16::try_from(direction.column_delta() * step as isize).ok()?,
    )?;
    CellId::from_coord(coord)
}

fn collect_cells(cells: [Option<CellId>; 4]) -> Option<[CellId; 4]> {
    Some([cells[0]?, cells[1]?, cells[2]?, cells[3]?])
}

fn line_key(line: &WinningLine) -> String {
    line.cells
        .iter()
        .map(|cell| cell.as_string())
        .collect::<Vec<_>>()
        .join("-")
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    RisingDiagonal,
    FallingDiagonal,
}

impl Direction {
    const ALL: [Self; 4] = [
        Self::Horizontal,
        Self::Vertical,
        Self::RisingDiagonal,
        Self::FallingDiagonal,
    ];

    fn row_delta(self) -> isize {
        match self {
            Self::Horizontal => 0,
            Self::Vertical => 1,
            Self::RisingDiagonal => 1,
            Self::FallingDiagonal => -1,
        }
    }

    fn column_delta(self) -> isize {
        match self {
            Self::Horizontal => 1,
            Self::Vertical => 0,
            Self::RisingDiagonal => 1,
            Self::FallingDiagonal => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::setup_match;
    use engine_core::{SeatId, Seed};

    fn state() -> ColumnFourState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn occupy(state: &mut ColumnFourState, cell: CellId, seat: ColumnFourSeat) {
        state.set_occupancy(cell, CellOccupancy::Occupied(seat));
    }

    fn cell(row: RowId, column: ColumnId) -> CellId {
        CellId::new(row, column)
    }

    #[test]
    fn gravity_lands_in_lowest_empty_row() {
        let mut state = state();
        assert_eq!(
            landing_cell(&state, ColumnId::C3),
            Some(cell(RowId::R1, ColumnId::C3))
        );

        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C3),
            ColumnFourSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R2, ColumnId::C3),
            ColumnFourSeat::Seat0,
        );

        assert_eq!(
            landing_cell(&state, ColumnId::C3),
            Some(cell(RowId::R3, ColumnId::C3))
        );

        for row in [RowId::R3, RowId::R4, RowId::R5, RowId::R6] {
            occupy(&mut state, cell(row, ColumnId::C3), ColumnFourSeat::Seat1);
        }
        assert_eq!(landing_cell(&state, ColumnId::C3), None);

        state.set_occupancy(cell(RowId::R3, ColumnId::C3), CellOccupancy::Empty);

        apply_action(
            &mut state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat0,
                column: ColumnId::C3,
            },
        );

        assert_eq!(
            state.occupancy(cell(RowId::R3, ColumnId::C3)),
            CellOccupancy::Occupied(ColumnFourSeat::Seat0)
        );
    }

    #[test]
    fn horizontal_vertical_and_diagonal_wins_are_detected() {
        let mut horizontal = state();
        for column in [ColumnId::C1, ColumnId::C2, ColumnId::C3, ColumnId::C4] {
            occupy(
                &mut horizontal,
                cell(RowId::R1, column),
                ColumnFourSeat::Seat0,
            );
        }
        assert_eq!(
            winning_line(&horizontal, ColumnFourSeat::Seat0)
                .unwrap()
                .cells,
            [
                cell(RowId::R1, ColumnId::C1),
                cell(RowId::R1, ColumnId::C2),
                cell(RowId::R1, ColumnId::C3),
                cell(RowId::R1, ColumnId::C4)
            ]
        );

        let mut vertical = state();
        for row in [RowId::R1, RowId::R2, RowId::R3, RowId::R4] {
            occupy(
                &mut vertical,
                cell(row, ColumnId::C2),
                ColumnFourSeat::Seat1,
            );
        }
        assert_eq!(
            winning_line(&vertical, ColumnFourSeat::Seat1)
                .unwrap()
                .cells,
            [
                cell(RowId::R1, ColumnId::C2),
                cell(RowId::R2, ColumnId::C2),
                cell(RowId::R3, ColumnId::C2),
                cell(RowId::R4, ColumnId::C2)
            ]
        );

        let mut rising = state();
        for cell in [
            cell(RowId::R1, ColumnId::C1),
            cell(RowId::R2, ColumnId::C2),
            cell(RowId::R3, ColumnId::C3),
            cell(RowId::R4, ColumnId::C4),
        ] {
            occupy(&mut rising, cell, ColumnFourSeat::Seat0);
        }
        assert_eq!(
            winning_line(&rising, ColumnFourSeat::Seat0).unwrap().cells,
            [
                cell(RowId::R1, ColumnId::C1),
                cell(RowId::R2, ColumnId::C2),
                cell(RowId::R3, ColumnId::C3),
                cell(RowId::R4, ColumnId::C4)
            ]
        );

        let mut falling = state();
        for cell in [
            cell(RowId::R4, ColumnId::C1),
            cell(RowId::R3, ColumnId::C2),
            cell(RowId::R2, ColumnId::C3),
            cell(RowId::R1, ColumnId::C4),
        ] {
            occupy(&mut falling, cell, ColumnFourSeat::Seat1);
        }
        assert_eq!(
            winning_line(&falling, ColumnFourSeat::Seat1).unwrap().cells,
            [
                cell(RowId::R4, ColumnId::C1),
                cell(RowId::R3, ColumnId::C2),
                cell(RowId::R2, ColumnId::C3),
                cell(RowId::R1, ColumnId::C4)
            ]
        );
    }

    #[test]
    fn multiple_line_completion_uses_documented_tiebreak() {
        let mut state = state();
        for cell in [
            cell(RowId::R1, ColumnId::C1),
            cell(RowId::R1, ColumnId::C2),
            cell(RowId::R1, ColumnId::C3),
            cell(RowId::R1, ColumnId::C4),
            cell(RowId::R2, ColumnId::C1),
            cell(RowId::R3, ColumnId::C1),
            cell(RowId::R4, ColumnId::C1),
        ] {
            occupy(&mut state, cell, ColumnFourSeat::Seat0);
        }

        assert_eq!(
            winning_line(&state, ColumnFourSeat::Seat0).unwrap().cells,
            [
                cell(RowId::R1, ColumnId::C1),
                cell(RowId::R1, ColumnId::C2),
                cell(RowId::R1, ColumnId::C3),
                cell(RowId::R1, ColumnId::C4)
            ]
        );
    }

    #[test]
    fn full_board_without_line_is_draw() {
        let mut state = state();
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
                let seat = if *seat_index == 0 {
                    ColumnFourSeat::Seat0
                } else {
                    ColumnFourSeat::Seat1
                };
                occupy(
                    &mut state,
                    ColumnFourState::cell(row_index, column_index).unwrap(),
                    seat,
                );
            }
        }

        assert_eq!(winning_line(&state, ColumnFourSeat::Seat0), None);
        assert_eq!(winning_line(&state, ColumnFourSeat::Seat1), None);
        state.ply_count = 41;
        state.set_occupancy(cell(RowId::R6, ColumnId::C7), CellOccupancy::Empty);
        apply_action(
            &mut state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat1,
                column: ColumnId::C7,
            },
        );

        assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
    }

    #[test]
    fn final_placement_win_takes_precedence_over_draw() {
        let mut state = state();
        for cell in CellId::ALL {
            state.set_occupancy(cell, CellOccupancy::Occupied(ColumnFourSeat::Seat1));
        }
        for cell in [
            cell(RowId::R1, ColumnId::C1),
            cell(RowId::R2, ColumnId::C1),
            cell(RowId::R3, ColumnId::C1),
            cell(RowId::R4, ColumnId::C1),
            cell(RowId::R5, ColumnId::C1),
            cell(RowId::R6, ColumnId::C1),
        ] {
            state.set_occupancy(cell, CellOccupancy::Occupied(ColumnFourSeat::Seat0));
        }
        state.set_occupancy(cell(RowId::R6, ColumnId::C1), CellOccupancy::Empty);
        state.ply_count = 41;

        apply_action(
            &mut state,
            ValidatedAction {
                actor: ColumnFourSeat::Seat0,
                column: ColumnId::C1,
            },
        );

        assert!(matches!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win {
                seat: ColumnFourSeat::Seat0,
                ..
            })
        ));
    }

    #[test]
    fn terminal_state_has_no_legal_columns() {
        let mut state = state();
        state.terminal_outcome = Some(TerminalOutcome::Draw);

        assert!(legal_columns(&state).is_empty());
    }
}
