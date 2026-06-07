use ai_core::RandomLegalBot;
use engine_core::{
    ActionPath, Actor, CommandEnvelope, Diagnostic, EffectEnvelope, HashValue, RulesVersion, Seed,
};

use crate::{
    effects::{bot_chose_action_effect, DirectionalFlipEffect},
    legal_action_tree,
    rules::{apply_action, disc_counts, legal_placements, parse_place_segment, validate_command},
    CellId, CellOccupancy, DirectionalFlipSeat, DirectionalFlipState, TerminalOutcome,
};

pub const RANDOM_POLICY_ID: &str = "directional_flip-random-legal-v1";
pub const LEVEL2_POLICY_ID: &str = "directional_flip_level2_lite_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub action_path: ActionPath,
    pub rationale: String,
    pub effects: Vec<EffectEnvelope<DirectionalFlipEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectionalFlipRandomBot {
    pub seed: Seed,
}

impl DirectionalFlipRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &DirectionalFlipState,
        bot_seat: DirectionalFlipSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &DirectionalFlipState,
        bot_seat: DirectionalFlipSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random legal Directional Flip action.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectionalFlipLevel2Bot {
    pub seed: Seed,
}

impl DirectionalFlipLevel2Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        &self,
        state: &DirectionalFlipState,
        bot_seat: DirectionalFlipSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        let mut candidates = tree
            .root
            .choices
            .iter()
            .map(|choice| Candidate::new(state, bot_seat, choice.path(), self.seed))
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            return Err(Diagnostic {
                code: "no_legal_actions".to_owned(),
                message: "no legal action is available".to_owned(),
            });
        }

        candidates.sort_by_key(|candidate| std::cmp::Reverse(candidate.priority_key()));
        let chosen = candidates.remove(0);
        let rationale = chosen.rationale();
        Ok(decision(2, LEVEL2_POLICY_ID, chosen.action_path, rationale))
    }

    pub fn select_action(
        &self,
        state: &DirectionalFlipState,
        bot_seat: DirectionalFlipSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Candidate {
    action_path: ActionPath,
    cell: Option<CellId>,
    forced_pass: bool,
    terminal_favorable: u8,
    corner: bool,
    avoids_open_corner_danger: bool,
    opponent_mobility_reduction: i16,
    own_mobility_preservation: i16,
    stable_extension: u8,
    frontier_score: i16,
    opening_flip_caution: i16,
    late_count_delta: i16,
    action_order_score: u8,
    seeded_key: u64,
}

impl Candidate {
    fn new(
        state: &DirectionalFlipState,
        bot_seat: DirectionalFlipSeat,
        action_path: ActionPath,
        seed: Seed,
    ) -> Self {
        let segment = action_path.segments.first().cloned().unwrap_or_default();
        let cell = cell_from_path(&action_path);
        let forced_pass = segment == crate::rules::FORCED_PASS_SEGMENT;
        let action_id = action_path.segments.join("/");
        let successor = successor_after(state, bot_seat, &action_path);
        let opponent = bot_seat.other();
        let baseline_opponent_mobility = legal_placements(state, opponent).len() as i16;
        let baseline_own_mobility = legal_placements(state, bot_seat).len() as i16;
        let empty_cells = empty_cell_count(state);
        let flip_count = successor
            .as_ref()
            .and_then(|next| cell.map(|target| flipped_count(state, next, target)))
            .unwrap_or(0) as i16;

        Self {
            action_path,
            cell,
            forced_pass,
            terminal_favorable: successor
                .as_ref()
                .map(|next| terminal_score(next.terminal_outcome, bot_seat))
                .unwrap_or(0),
            corner: cell.is_some_and(is_corner),
            avoids_open_corner_danger: cell
                .map(|target| !is_open_corner_danger(state, target))
                .unwrap_or(true),
            opponent_mobility_reduction: successor
                .as_ref()
                .map(|next| {
                    baseline_opponent_mobility - legal_placements(next, opponent).len() as i16
                })
                .unwrap_or(0),
            own_mobility_preservation: successor
                .as_ref()
                .map(|next| legal_placements(next, bot_seat).len() as i16 - baseline_own_mobility)
                .unwrap_or(0),
            stable_extension: successor
                .as_ref()
                .and_then(|next| cell.map(|target| stable_extension_score(next, bot_seat, target)))
                .unwrap_or(0),
            frontier_score: successor
                .as_ref()
                .map(|next| 64 - frontier_count(next, bot_seat) as i16)
                .unwrap_or(0),
            opening_flip_caution: if empty_cells >= 44 {
                64 - flip_count
            } else {
                0
            },
            late_count_delta: successor
                .as_ref()
                .map(|next| phase_adjusted_count_delta(next, bot_seat))
                .unwrap_or(0),
            action_order_score: cell
                .map(|target| 64_u8.saturating_sub(target.index() as u8))
                .unwrap_or(0),
            seeded_key: seeded_key(seed, &action_id),
        }
    }

    fn priority_key(&self) -> (u8, u8, u8, u8, i16, i16, u8, i16, i16, i16, u8, u64) {
        (
            self.forced_pass as u8,
            self.terminal_favorable,
            self.corner as u8,
            self.avoids_open_corner_danger as u8,
            self.opponent_mobility_reduction,
            self.own_mobility_preservation,
            self.stable_extension,
            self.frontier_score,
            self.opening_flip_caution,
            self.late_count_delta,
            self.action_order_score,
            self.seeded_key,
        )
    }

    fn rationale(&self) -> String {
        if self.forced_pass {
            return "No placement is legal, so I must pass.".to_owned();
        }
        if self.terminal_favorable == 2 {
            return format!(
                "Chose {} because it ends the game with a favorable final count.",
                self.label()
            );
        }
        if self.terminal_favorable == 1 {
            return format!(
                "Chose {} because it secures a draw from the visible final count.",
                self.label()
            );
        }
        if self.corner {
            return format!("Chose {} because it takes a corner anchor.", self.label());
        }
        if !self.avoids_open_corner_danger {
            return format!(
                "Chose {} only after higher priorities outweighed open-corner risk.",
                self.label()
            );
        }
        if self.opponent_mobility_reduction > 0 {
            return format!(
                "Chose {} because it reduces the opponent's visible choices.",
                self.label()
            );
        }
        if self.own_mobility_preservation > 0 {
            return format!("Chose {} because it keeps options open.", self.label());
        }
        if self.stable_extension > 0 {
            return format!(
                "Chose {} because it extends from a stable edge or corner.",
                self.label()
            );
        }
        if self.frontier_score > 0 {
            return format!(
                "Chose {} because it exposes fewer discs next to empty cells.",
                self.label()
            );
        }
        if self.late_count_delta > 0 {
            return format!(
                "Chose {} because late in the game it improves the visible count.",
                self.label()
            );
        }
        format!(
            "Chose {} after equivalent legal options were resolved deterministically.",
            self.label()
        )
    }

    fn label(&self) -> String {
        self.cell
            .map(|cell| cell.as_string())
            .unwrap_or_else(|| "forced pass".to_owned())
    }
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    let action_id = action_path.segments.join("/");
    BotDecision {
        action_path,
        rationale: rationale.clone(),
        effects: vec![bot_chose_action_effect(
            level, policy_id, action_id, rationale,
        )],
    }
}

fn actor_for_seat(state: &DirectionalFlipState, seat: DirectionalFlipSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn command_for_path(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn successor_after(
    state: &DirectionalFlipState,
    seat: DirectionalFlipSeat,
    action_path: &ActionPath,
) -> Option<DirectionalFlipState> {
    let command = command_for_path(state, seat, action_path.clone());
    let action = validate_command(state, &command).ok()?;
    let mut candidate = state.clone();
    apply_action(&mut candidate, action);
    Some(candidate)
}

fn cell_from_path(path: &ActionPath) -> Option<CellId> {
    if path.segments.len() != 1 {
        return None;
    }
    parse_place_segment(&path.segments[0])
}

fn terminal_score(outcome: Option<TerminalOutcome>, bot_seat: DirectionalFlipSeat) -> u8 {
    match outcome {
        Some(TerminalOutcome::Win { seat }) if seat == bot_seat => 2,
        Some(TerminalOutcome::Draw) => 1,
        _ => 0,
    }
}

fn is_corner(cell: CellId) -> bool {
    let row = cell.row.index();
    let column = cell.column.index();
    matches!((row, column), (0, 0) | (0, 7) | (7, 0) | (7, 7))
}

fn is_open_corner_danger(state: &DirectionalFlipState, cell: CellId) -> bool {
    danger_corner(cell)
        .is_some_and(|corner| matches!(state.occupancy(corner), CellOccupancy::Empty))
}

fn danger_corner(cell: CellId) -> Option<CellId> {
    let row = cell.row.index();
    let column = cell.column.index();
    let corner = match (row, column) {
        (0, 1) | (1, 0) | (1, 1) => (0, 0),
        (0, 6) | (1, 6) | (1, 7) => (0, 7),
        (6, 0) | (6, 1) | (7, 1) => (7, 0),
        (6, 6) | (6, 7) | (7, 6) => (7, 7),
        _ => return None,
    };
    DirectionalFlipState::cell(corner.0, corner.1)
}

fn stable_extension_score(
    state: &DirectionalFlipState,
    bot_seat: DirectionalFlipSeat,
    cell: CellId,
) -> u8 {
    if !is_edge(cell) {
        return 0;
    }
    let same_edge_owned_corner = corner_cells()
        .into_iter()
        .filter(|corner| shares_edge(cell, *corner))
        .any(|corner| state.occupancy(corner) == CellOccupancy::Occupied(bot_seat));
    same_edge_owned_corner as u8
}

fn is_edge(cell: CellId) -> bool {
    let row = cell.row.index();
    let column = cell.column.index();
    row == 0 || row == 7 || column == 0 || column == 7
}

fn shares_edge(cell: CellId, corner: CellId) -> bool {
    (cell.row.index() == corner.row.index()) || (cell.column.index() == corner.column.index())
}

fn corner_cells() -> [CellId; 4] {
    [
        DirectionalFlipState::cell(0, 0).unwrap(),
        DirectionalFlipState::cell(0, 7).unwrap(),
        DirectionalFlipState::cell(7, 0).unwrap(),
        DirectionalFlipState::cell(7, 7).unwrap(),
    ]
}

fn frontier_count(state: &DirectionalFlipState, seat: DirectionalFlipSeat) -> u8 {
    CellId::ALL
        .into_iter()
        .filter(|cell| state.occupancy(*cell) == CellOccupancy::Occupied(seat))
        .filter(|cell| {
            adjacent_cells(*cell)
                .into_iter()
                .any(|adjacent| state.occupancy(adjacent).is_empty())
        })
        .count() as u8
}

fn adjacent_cells(cell: CellId) -> Vec<CellId> {
    let mut cells = Vec::new();
    let row = cell.row.index() as isize;
    let column = cell.column.index() as isize;
    for row_delta in -1..=1 {
        for column_delta in -1..=1 {
            if row_delta == 0 && column_delta == 0 {
                continue;
            }
            let next_row = row + row_delta;
            let next_column = column + column_delta;
            if next_row >= 0 && next_column >= 0 {
                if let Some(next) =
                    DirectionalFlipState::cell(next_row as usize, next_column as usize)
                {
                    cells.push(next);
                }
            }
        }
    }
    cells
}

fn phase_adjusted_count_delta(state: &DirectionalFlipState, bot_seat: DirectionalFlipSeat) -> i16 {
    let empty_cells = empty_cell_count(state);
    if empty_cells > 16 {
        return 0;
    }
    let score = disc_counts(state);
    let delta = match bot_seat {
        DirectionalFlipSeat::Seat0 => score.seat_0 as i16 - score.seat_1 as i16,
        DirectionalFlipSeat::Seat1 => score.seat_1 as i16 - score.seat_0 as i16,
    };
    if empty_cells <= 8 {
        delta.saturating_mul(2)
    } else {
        delta
    }
}

fn empty_cell_count(state: &DirectionalFlipState) -> u8 {
    state
        .cells
        .iter()
        .filter(|cell| matches!(cell, CellOccupancy::Empty))
        .count() as u8
}

fn flipped_count(
    before: &DirectionalFlipState,
    after: &DirectionalFlipState,
    placed_cell: CellId,
) -> u8 {
    CellId::ALL
        .into_iter()
        .filter(|cell| *cell != placed_cell)
        .filter(|cell| before.occupancy(*cell) != after.occupancy(*cell))
        .count() as u8
}

fn seeded_key(seed: Seed, action_id: &str) -> u64 {
    HashValue::from_stable_bytes(format!("{}:{action_id}", seed.0).as_bytes()).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{setup_match, ColumnId, DirectionalFlipEffect, RowId, SetupOptions};
    use engine_core::{FreshnessToken, SeatId};

    fn seats() -> Vec<SeatId> {
        vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
    }

    fn state() -> DirectionalFlipState {
        setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()
    }

    fn command(state: &DirectionalFlipState, segment: String) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
            action_path: ActionPath {
                segments: vec![segment],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn apply_segment(state: &mut DirectionalFlipState, segment: &str) {
        let action = validate_command(state, &command(state, segment.to_owned())).unwrap();
        apply_action(state, action);
    }

    fn assert_choice_validates(state: &DirectionalFlipState, segment: &str) {
        validate_command(state, &command(state, segment.to_owned()))
            .expect("bot action validates normally");
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
        state.ply_count = 0;
        state.consecutive_forced_passes = 0;
        state.terminal_outcome = None;
        state
    }

    #[test]
    fn level0_choices_validate_for_many_seeds_and_states() {
        for seed in 0..64 {
            let mut state = state();
            for segment in ["place/r3c4", "place/r3c5", "place/r3c6"]
                .into_iter()
                .take((seed % 4) as usize)
            {
                if state.terminal_outcome.is_none() {
                    apply_segment(&mut state, segment);
                }
            }
            if state.terminal_outcome.is_some() {
                continue;
            }
            let bot_seat = state.active_seat;
            let bot = DirectionalFlipRandomBot::new(Seed(seed));
            let action_path = bot
                .select_action(&state, bot_seat)
                .expect("legal action selected");
            let legal_paths = legal_action_tree(&state, &actor_for_seat(&state, bot_seat))
                .root
                .choices
                .iter()
                .map(|choice| choice.path())
                .collect::<Vec<_>>();
            assert!(legal_paths.contains(&action_path));
            assert_choice_validates(&state, &action_path.segments[0]);
        }
    }

    #[test]
    fn level0_fixed_seed_is_deterministic_and_terminal_reports_no_action() {
        let mut state = state();
        apply_segment(&mut state, "place/r3c4");
        let bot = DirectionalFlipRandomBot::new(Seed(123));

        let left = bot
            .select_action(&state, DirectionalFlipSeat::Seat1)
            .expect("action selected");
        let right = bot
            .select_action(&state, DirectionalFlipSeat::Seat1)
            .expect("action selected");
        assert_eq!(left, right);

        state.terminal_outcome = Some(TerminalOutcome::Draw);
        let diagnostic = bot
            .select_action(&state, DirectionalFlipSeat::Seat1)
            .expect_err("terminal tree has no actions");
        assert_eq!(diagnostic.code, "no_legal_actions");
    }

    #[test]
    fn level2_takes_forced_pass() {
        let state = no_move_state();

        let decision = DirectionalFlipLevel2Bot::new(Seed(1))
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["pass/forced"]);
        assert!(decision.rationale.contains("must pass"));
        assert_choice_validates(&state, "pass/forced");
    }

    #[test]
    fn level2_prefers_favorable_terminal_outcome_before_corner_rationale() {
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

        let decision = DirectionalFlipLevel2Bot::new(Seed(1))
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["place/r1c1"]);
        assert!(decision.rationale.contains("ends the game"));
        assert_choice_validates(&state, "place/r1c1");
    }

    #[test]
    fn level2_prefers_legal_corner() {
        let mut state = empty_with_active(DirectionalFlipSeat::Seat0);
        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C2),
            DirectionalFlipSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R1, ColumnId::C3),
            DirectionalFlipSeat::Seat0,
        );
        occupy(
            &mut state,
            cell(RowId::R4, ColumnId::C4),
            DirectionalFlipSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R4, ColumnId::C5),
            DirectionalFlipSeat::Seat0,
        );

        let decision = DirectionalFlipLevel2Bot::new(Seed(1))
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["place/r1c1"]);
        assert!(decision.rationale.contains("corner anchor"));
        assert_choice_validates(&state, "place/r1c1");
    }

    #[test]
    fn level2_avoids_open_corner_adjacent_square_when_safe_alternative_exists() {
        let mut state = state();
        occupy(
            &mut state,
            cell(RowId::R2, ColumnId::C3),
            DirectionalFlipSeat::Seat1,
        );
        occupy(
            &mut state,
            cell(RowId::R2, ColumnId::C4),
            DirectionalFlipSeat::Seat0,
        );
        state.active_seat = DirectionalFlipSeat::Seat0;

        let decision = DirectionalFlipLevel2Bot::new(Seed(1))
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();

        assert_ne!(decision.action_path.segments, vec!["place/r2c2"]);
        assert_choice_validates(&state, &decision.action_path.segments[0]);
    }

    #[test]
    fn level2_is_deterministic_explains_safely_and_emits_bot_choice_effect() {
        let state = state();
        let bot = DirectionalFlipLevel2Bot::new(Seed(99));
        let left = bot
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();
        let right = bot
            .select_decision(&state, DirectionalFlipSeat::Seat0)
            .unwrap();

        assert_eq!(left, right);
        assert!(!left.rationale.is_empty());
        assert!(!left.rationale.contains("candidate"));
        assert!(!left.rationale.contains("score"));
        assert!(!left.rationale.contains("debug"));
        assert!(!left.rationale.contains("hash"));
        assert!(!left.rationale.contains('['));
        assert!(matches!(
            left.effects[0].payload,
            DirectionalFlipEffect::BotChoseAction { level: 2, .. }
        ));
        assert_choice_validates(&state, &left.action_path.segments[0]);
    }

    #[test]
    fn terminal_level2_reports_no_action() {
        let mut state = state();
        state.terminal_outcome = Some(TerminalOutcome::Draw);

        let diagnostic = DirectionalFlipLevel2Bot::new(Seed(1))
            .select_action(&state, DirectionalFlipSeat::Seat0)
            .expect_err("terminal has no actions");

        assert_eq!(diagnostic.code, "no_legal_actions");
    }

    #[test]
    fn invalid_bot_action_path_is_rejected_by_parser() {
        assert_eq!(
            cell_from_path(&ActionPath {
                segments: vec!["place/r1c1".to_owned(), "extra".to_owned()]
            }),
            None
        );
        assert_eq!(
            cell_from_path(&ActionPath {
                segments: vec!["drop/r1c1".to_owned()]
            }),
            None
        );
    }

    #[test]
    fn stale_human_command_still_rejects_after_bot_module_is_loaded() {
        let state = state();
        let mut stale = command(&state, "place/r3c4".to_owned());
        stale.freshness_token = FreshnessToken(99);

        assert_eq!(
            validate_command(&state, &stale)
                .expect_err("stale rejected")
                .code,
            "stale_action"
        );
    }

    fn no_move_state() -> DirectionalFlipState {
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
}
