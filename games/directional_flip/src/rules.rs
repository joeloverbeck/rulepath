use engine_core::{Actor, CommandEnvelope, Diagnostic};

use crate::{
    ids::{CellId, DirectionalFlipSeat},
    state::{CellOccupancy, DirectionalFlipState, TerminalOutcome},
};

pub const PLACE_SEGMENT_PREFIX: &str = "place/";
pub const FORCED_PASS_SEGMENT: &str = "pass/forced";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl Direction {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::Northeast,
        Self::East,
        Self::Southeast,
        Self::South,
        Self::Southwest,
        Self::West,
        Self::Northwest,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::North => "north",
            Self::Northeast => "northeast",
            Self::East => "east",
            Self::Southeast => "southeast",
            Self::South => "south",
            Self::Southwest => "southwest",
            Self::West => "west",
            Self::Northwest => "northwest",
        }
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::Northeast => (-1, 1),
            Self::East => (0, 1),
            Self::Southeast => (1, 1),
            Self::South => (1, 0),
            Self::Southwest => (1, -1),
            Self::West => (0, -1),
            Self::Northwest => (-1, -1),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlipRun {
    pub direction: Direction,
    pub cells: Vec<CellId>,
}

impl FlipRun {
    pub fn ordered_cells(&self) -> impl Iterator<Item = CellId> + '_ {
        self.cells.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Placement {
    pub actor: DirectionalFlipSeat,
    pub cell: CellId,
    pub flip_runs: Vec<FlipRun>,
}

impl Placement {
    pub fn ordered_flips(&self) -> Vec<CellId> {
        self.flip_runs
            .iter()
            .flat_map(FlipRun::ordered_cells)
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForcedPass {
    pub actor: DirectionalFlipSeat,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidatedAction {
    Place(Placement),
    ForcedPass(ForcedPass),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Score {
    pub seat_0: u8,
    pub seat_1: u8,
}

impl Score {
    pub fn winner(self) -> Option<DirectionalFlipSeat> {
        match self.seat_0.cmp(&self.seat_1) {
            std::cmp::Ordering::Greater => Some(DirectionalFlipSeat::Seat0),
            std::cmp::Ordering::Less => Some(DirectionalFlipSeat::Seat1),
            std::cmp::Ordering::Equal => None,
        }
    }
}

pub fn validate_command(
    state: &DirectionalFlipState,
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
        return Err(diagnostic("unknown_actor", "the actor is not seated"));
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

    let segment = &command.action_path.segments[0];
    if segment == FORCED_PASS_SEGMENT {
        if has_legal_placement(state, actor) {
            return Err(diagnostic(
                "pass_not_available",
                "forced pass is available only when no placement is legal",
            ));
        }
        return Ok(ValidatedAction::ForcedPass(ForcedPass { actor }));
    }

    let cell = parse_place_segment(segment)
        .ok_or_else(|| diagnostic("invalid_cell", "the requested cell does not exist"))?;
    validate_placement(state, actor, cell).map(ValidatedAction::Place)
}

pub fn apply_action(state: &mut DirectionalFlipState, action: ValidatedAction) {
    match action {
        ValidatedAction::Place(placement) => apply_placement(state, placement),
        ValidatedAction::ForcedPass(pass) => apply_forced_pass(state, pass),
    }
}

pub fn legal_placements(state: &DirectionalFlipState, seat: DirectionalFlipSeat) -> Vec<Placement> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    CellId::ALL
        .into_iter()
        .filter_map(|cell| validate_placement(state, seat, cell).ok())
        .collect()
}

pub fn has_legal_placement(state: &DirectionalFlipState, seat: DirectionalFlipSeat) -> bool {
    if state.terminal_outcome.is_some() {
        return false;
    }

    CellId::ALL.into_iter().any(|cell| {
        state.occupancy(cell).is_empty() && !placement_flips(state, seat, cell).is_empty()
    })
}

pub fn placement_flips(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
    cell: CellId,
) -> Vec<FlipRun> {
    if !state.occupancy(cell).is_empty() {
        return Vec::new();
    }

    Direction::ALL
        .into_iter()
        .filter_map(|direction| flip_run_for_direction(state, seat, cell, direction))
        .collect()
}

pub fn disc_counts(state: &DirectionalFlipState) -> Score {
    let mut score = Score {
        seat_0: 0,
        seat_1: 0,
    };

    for occupancy in state.cells {
        match occupancy {
            CellOccupancy::Empty => {}
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0) => score.seat_0 += 1,
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1) => score.seat_1 += 1,
        }
    }

    score
}

pub fn actor_seat(state: &DirectionalFlipState, actor: &Actor) -> Option<DirectionalFlipSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(DirectionalFlipSeat::from_index)
}

pub fn parse_place_segment(segment: &str) -> Option<CellId> {
    CellId::parse(segment.strip_prefix(PLACE_SEGMENT_PREFIX)?)
}

fn validate_placement(
    state: &DirectionalFlipState,
    actor: DirectionalFlipSeat,
    cell: CellId,
) -> Result<Placement, Diagnostic> {
    if !state.occupancy(cell).is_empty() {
        return Err(diagnostic(
            "occupied_cell",
            "the requested cell is already occupied",
        ));
    }

    let flip_runs = placement_flips(state, actor, cell);
    if flip_runs.is_empty() {
        return Err(diagnostic(
            "non_flipping_placement",
            "the requested placement flips no discs",
        ));
    }

    Ok(Placement {
        actor,
        cell,
        flip_runs,
    })
}

fn apply_placement(state: &mut DirectionalFlipState, placement: Placement) {
    state.set_occupancy(placement.cell, CellOccupancy::Occupied(placement.actor));
    for cell in placement.ordered_flips() {
        state.set_occupancy(cell, CellOccupancy::Occupied(placement.actor));
    }

    state.ply_count = state.ply_count.saturating_add(1);
    state.consecutive_forced_passes = 0;
    state.freshness_token = state.freshness_token.next();

    if is_terminal_after_placement(state) {
        state.terminal_outcome = Some(outcome_from_score(disc_counts(state)));
    } else {
        state.active_seat = placement.actor.other();
    }
}

fn apply_forced_pass(state: &mut DirectionalFlipState, pass: ForcedPass) {
    state.ply_count = state.ply_count.saturating_add(1);
    state.consecutive_forced_passes = state.consecutive_forced_passes.saturating_add(1);
    state.freshness_token = state.freshness_token.next();

    if state.consecutive_forced_passes >= 2 {
        state.terminal_outcome = Some(outcome_from_score(disc_counts(state)));
    } else {
        state.active_seat = pass.actor.other();
    }
}

fn is_terminal_after_placement(state: &DirectionalFlipState) -> bool {
    state.cells.iter().all(|cell| !cell.is_empty())
        || !has_legal_placement(state, DirectionalFlipSeat::Seat0)
            && !has_legal_placement(state, DirectionalFlipSeat::Seat1)
}

fn outcome_from_score(score: Score) -> TerminalOutcome {
    match score.winner() {
        Some(seat) => TerminalOutcome::Win { seat },
        None => TerminalOutcome::Draw,
    }
}

fn flip_run_for_direction(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
    origin: CellId,
    direction: Direction,
) -> Option<FlipRun> {
    let mut cells = Vec::new();
    let mut current = step(origin, direction)?;

    loop {
        match state.occupancy(current) {
            CellOccupancy::Empty => return None,
            CellOccupancy::Occupied(owner) if owner == seat => {
                return if cells.is_empty() {
                    None
                } else {
                    Some(FlipRun { direction, cells })
                };
            }
            CellOccupancy::Occupied(_) => cells.push(current),
        }

        current = step(current, direction)?;
    }
}

fn step(cell: CellId, direction: Direction) -> Option<CellId> {
    let (row_delta, column_delta) = direction.delta();
    let row = cell.row.index() as isize + row_delta;
    let column = cell.column.index() as isize + column_delta;
    if row < 0 || column < 0 {
        return None;
    }

    DirectionalFlipState::cell(row as usize, column as usize)
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ids::{ColumnId, RowId},
        setup::setup_match,
    };
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
    };

    fn state() -> DirectionalFlipState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn command(
        state: &DirectionalFlipState,
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

    fn occupy(state: &mut DirectionalFlipState, cell: CellId, seat: DirectionalFlipSeat) {
        state.set_occupancy(cell, CellOccupancy::Occupied(seat));
    }

    fn empty_with_active(active: DirectionalFlipSeat) -> DirectionalFlipState {
        let mut state = state();
        state.cells = DirectionalFlipState::empty_cells();
        state.active_seat = active;
        state
    }

    #[test]
    fn initial_legal_placements_are_standard_opening_cells() {
        let state = state();

        let legal = legal_placements(&state, DirectionalFlipSeat::Seat0)
            .into_iter()
            .map(|placement| placement.cell.as_string())
            .collect::<Vec<_>>();

        assert_eq!(legal, vec!["r3c4", "r4c3", "r5c6", "r6c5"]);
    }

    #[test]
    fn diagnostics_cover_invalid_placement_submissions() {
        let mut state = state();
        let stale = CommandEnvelope {
            freshness_token: FreshnessToken(99),
            ..command(&state, DirectionalFlipSeat::Seat0, "place/r3c4")
        };
        assert_eq!(
            validate_command(&state, &stale)
                .expect_err("stale command")
                .code,
            "stale_action"
        );
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat1, "place/r3c4")
            )
            .expect_err("wrong actor")
            .code,
            "not_active_seat"
        );
        assert_eq!(
            validate_command(
                &state,
                &CommandEnvelope {
                    actor: Actor {
                        seat_id: state.seats[0].clone()
                    },
                    action_path: ActionPath {
                        segments: vec!["place/r3c4".to_owned(), "extra".to_owned()]
                    },
                    freshness_token: state.freshness_token,
                    rules_version: RulesVersion(1),
                },
            )
            .expect_err("invalid path")
            .code,
            "invalid_action_path"
        );
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat0, "place/r9c1")
            )
            .expect_err("invalid cell")
            .code,
            "invalid_cell"
        );
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat0, "place/r4c4")
            )
            .expect_err("occupied cell")
            .code,
            "occupied_cell"
        );
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat0, "place/r1c1")
            )
            .expect_err("non-flipping placement")
            .code,
            "non_flipping_placement"
        );

        state.terminal_outcome = Some(TerminalOutcome::Draw);
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat0, "place/r3c4")
            )
            .expect_err("terminal")
            .code,
            "terminal_match"
        );
    }

    #[test]
    fn valid_placement_flips_all_qualifying_directions_in_order() {
        let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
        let target = cell(RowId::R4, ColumnId::C4);
        let own_anchors = [
            cell(RowId::R1, ColumnId::C4),
            cell(RowId::R1, ColumnId::C7),
            cell(RowId::R4, ColumnId::C7),
            cell(RowId::R7, ColumnId::C7),
            cell(RowId::R7, ColumnId::C4),
            cell(RowId::R7, ColumnId::C1),
            cell(RowId::R4, ColumnId::C1),
            cell(RowId::R1, ColumnId::C1),
        ];
        for anchor in own_anchors {
            occupy(&mut state, anchor, DirectionalFlipSeat::Seat0);
        }
        for flip in [
            cell(RowId::R3, ColumnId::C4),
            cell(RowId::R2, ColumnId::C4),
            cell(RowId::R3, ColumnId::C5),
            cell(RowId::R2, ColumnId::C6),
            cell(RowId::R4, ColumnId::C5),
            cell(RowId::R4, ColumnId::C6),
            cell(RowId::R5, ColumnId::C5),
            cell(RowId::R6, ColumnId::C6),
            cell(RowId::R5, ColumnId::C4),
            cell(RowId::R6, ColumnId::C4),
            cell(RowId::R5, ColumnId::C3),
            cell(RowId::R6, ColumnId::C2),
            cell(RowId::R4, ColumnId::C3),
            cell(RowId::R4, ColumnId::C2),
            cell(RowId::R3, ColumnId::C3),
            cell(RowId::R2, ColumnId::C2),
        ] {
            occupy(&mut state, flip, DirectionalFlipSeat::Seat1);
        }

        let placement = validate_placement(&state, DirectionalFlipSeat::Seat0, target)
            .expect("multi-direction placement");
        let ordered = placement
            .ordered_flips()
            .into_iter()
            .map(CellId::as_string)
            .collect::<Vec<_>>();

        assert_eq!(
            ordered,
            vec![
                "r3c4", "r2c4", "r3c5", "r2c6", "r4c5", "r4c6", "r5c5", "r6c6", "r5c4", "r6c4",
                "r5c3", "r6c2", "r4c3", "r4c2", "r3c3", "r2c2",
            ]
        );

        apply_action(&mut state, ValidatedAction::Place(placement));
        assert!(ordered.into_iter().all(|cell_id| {
            let cell = CellId::parse(&cell_id).expect("cell id parses");
            state.occupancy(cell) == CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
        }));
    }

    #[test]
    fn scan_does_not_skip_own_or_flip_indirect_discs() {
        let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
        let target = cell(RowId::R4, ColumnId::C4);
        occupy(
            &mut state,
            cell(RowId::R4, ColumnId::C5),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R4, ColumnId::C6),
            DirectionalFlipSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R4, ColumnId::C7),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R5, ColumnId::C4),
            DirectionalFlipSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R7, ColumnId::C4),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R5, ColumnId::C5),
            DirectionalFlipSeat::Seat1,
        );

        assert!(placement_flips(&state, DirectionalFlipSeat::Seat0, target).is_empty());
    }

    #[test]
    fn forced_pass_is_available_only_without_legal_placement() {
        let state = state();
        assert_eq!(
            validate_command(
                &state,
                &command(&state, DirectionalFlipSeat::Seat0, FORCED_PASS_SEGMENT)
            )
            .expect_err("pass forbidden")
            .code,
            "pass_not_available"
        );

        let no_move = empty_with_active(DirectionalFlipSeat::Seat0);
        let action = validate_command(
            &no_move,
            &command(&no_move, DirectionalFlipSeat::Seat0, FORCED_PASS_SEGMENT),
        )
        .expect("forced pass validates");
        assert_eq!(
            action,
            ValidatedAction::ForcedPass(ForcedPass {
                actor: DirectionalFlipSeat::Seat0
            })
        );
    }

    #[test]
    fn double_forced_pass_terminalizes_and_scores_draw() {
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

        let first = validate_command(
            &state,
            &command(&state, DirectionalFlipSeat::Seat0, FORCED_PASS_SEGMENT),
        )
        .expect("first pass");
        apply_action(&mut state, first);
        assert_eq!(state.active_seat, DirectionalFlipSeat::Seat1);
        assert_eq!(state.consecutive_forced_passes, 1);
        assert_eq!(state.terminal_outcome, None);

        let second = validate_command(
            &state,
            &command(&state, DirectionalFlipSeat::Seat1, FORCED_PASS_SEGMENT),
        )
        .expect("second pass");
        apply_action(&mut state, second);
        assert_eq!(state.consecutive_forced_passes, 2);
        assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
    }

    #[test]
    fn terminal_scoring_reports_higher_count_winner() {
        let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C1),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C2),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C3),
            DirectionalFlipSeat::Seat1,
        );

        apply_action(
            &mut state,
            ValidatedAction::ForcedPass(ForcedPass {
                actor: DirectionalFlipSeat::Seat0,
            }),
        );
        apply_action(
            &mut state,
            ValidatedAction::ForcedPass(ForcedPass {
                actor: DirectionalFlipSeat::Seat1,
            }),
        );

        assert_eq!(
            disc_counts(&state),
            Score {
                seat_0: 2,
                seat_1: 1
            }
        );
        assert_eq!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win {
                seat: DirectionalFlipSeat::Seat0
            })
        );
    }
}
