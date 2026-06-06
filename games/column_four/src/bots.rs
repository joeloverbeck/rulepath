use ai_core::RandomLegalBot;
use engine_core::{ActionPath, Actor, Diagnostic, EffectEnvelope, HashValue, Seed};

use crate::{
    actions::{legal_action_tree, parse_drop_segment},
    effects::{bot_chose_action_effect, ColumnFourEffect},
    rules::{apply_action, landing_cell, legal_columns},
    state::{ColumnFourState, TerminalOutcome},
    ColumnFourSeat, ColumnId, ValidatedAction,
};

pub const RANDOM_POLICY_ID: &str = "column_four-random-legal-v1";
pub const LEVEL2_POLICY_ID: &str = "column_four_tactical_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub action_path: ActionPath,
    pub rationale: String,
    pub effects: Vec<EffectEnvelope<ColumnFourEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColumnFourRandomBot {
    pub seed: Seed,
}

impl ColumnFourRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &ColumnFourState,
        bot_seat: ColumnFourSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let actor = Actor {
            seat_id: state.seats[bot_seat.index()].clone(),
        };
        let tree = legal_action_tree(state, &actor);
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &ColumnFourState,
        bot_seat: ColumnFourSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        let column = column_from_path(&action_path).ok_or_else(|| Diagnostic {
            code: "invalid_bot_action".to_owned(),
            message: "bot selected an invalid action path".to_owned(),
        })?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            column,
            "Selected a seeded random legal column.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColumnFourLevel2Bot {
    pub seed: Seed,
}

impl ColumnFourLevel2Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        &self,
        state: &ColumnFourState,
        bot_seat: ColumnFourSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let actor = Actor {
            seat_id: state.seats[bot_seat.index()].clone(),
        };
        let tree = legal_action_tree(state, &actor);
        let mut candidates = tree
            .root
            .choices
            .iter()
            .filter_map(|choice| {
                let path = choice.path();
                let column = column_from_path(&path)?;
                Some(Candidate::new(state, bot_seat, column, path, self.seed))
            })
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
        Ok(decision(
            2,
            LEVEL2_POLICY_ID,
            chosen.action_path,
            chosen.column,
            rationale,
        ))
    }

    pub fn select_action(
        &self,
        state: &ColumnFourState,
        bot_seat: ColumnFourSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Candidate {
    column: ColumnId,
    action_path: ActionPath,
    wins_now: bool,
    blocks_now: bool,
    is_safe: bool,
    own_threats_after: u8,
    denied_opponent_threats: u8,
    center_score: u8,
    seeded_key: u64,
}

impl Candidate {
    fn new(
        state: &ColumnFourState,
        bot_seat: ColumnFourSeat,
        column: ColumnId,
        action_path: ActionPath,
        seed: Seed,
    ) -> Self {
        let opponent = bot_seat.other();
        let opponent_winning_columns = immediate_winning_columns(state, opponent);
        let wins_now = immediate_win_column(state, bot_seat, column);
        let blocks_now = opponent_winning_columns.contains(&column);
        let successor = successor_after(state, bot_seat, column);
        let opponent_wins_after = successor
            .as_ref()
            .map(|next| immediate_winning_columns(next, opponent))
            .unwrap_or_default();
        let own_threats_after = successor
            .as_ref()
            .map(|next| immediate_winning_columns(next, bot_seat).len() as u8)
            .unwrap_or(0);
        let denied_opponent_threats = opponent_winning_columns
            .len()
            .saturating_sub(opponent_wins_after.len()) as u8;
        let action_id = action_path.segments.join("/");

        Self {
            column,
            action_path,
            wins_now,
            blocks_now,
            is_safe: wins_now || opponent_wins_after.is_empty(),
            own_threats_after,
            denied_opponent_threats,
            center_score: center_score(column),
            seeded_key: seeded_key(seed, &action_id),
        }
    }

    fn priority_key(&self) -> (u8, u8, u8, u8, u8, u8, u64) {
        (
            self.wins_now as u8,
            self.blocks_now as u8,
            self.is_safe as u8,
            self.own_threats_after,
            self.denied_opponent_threats,
            self.center_score,
            self.seeded_key,
        )
    }

    fn rationale(&self) -> String {
        if self.wins_now {
            return format!("Chose {} because it wins now.", self.label());
        }
        if self.blocks_now {
            return format!(
                "Chose {} to block a visible immediate threat.",
                self.label()
            );
        }
        if self.is_safe && self.own_threats_after > 0 {
            return format!("Chose {} because it builds a visible threat.", self.label());
        }
        if self.is_safe && self.denied_opponent_threats > 0 {
            return format!(
                "Chose {} because it interrupts opposing pressure.",
                self.label()
            );
        }
        if self.center_score > 0 {
            return format!(
                "Chose {} because no immediate win or block exists and this policy prefers central pressure.",
                self.label()
            );
        }
        format!(
            "Chose {} after equivalent legal options were resolved deterministically.",
            self.label()
        )
    }

    fn label(&self) -> String {
        format!("Column {}", self.column.index() + 1)
    }
}

fn decision(
    level: u8,
    policy_id: &str,
    action_path: ActionPath,
    column: ColumnId,
    rationale: String,
) -> BotDecision {
    let action_id = action_path.segments.join("/");
    BotDecision {
        action_path,
        rationale: rationale.clone(),
        effects: vec![bot_chose_action_effect(
            level, policy_id, action_id, column, rationale,
        )],
    }
}

fn immediate_winning_columns(state: &ColumnFourState, seat: ColumnFourSeat) -> Vec<ColumnId> {
    legal_columns(state)
        .into_iter()
        .filter(|column| immediate_win_column(state, seat, *column))
        .collect()
}

fn immediate_win_column(state: &ColumnFourState, seat: ColumnFourSeat, column: ColumnId) -> bool {
    if landing_cell(state, column).is_none() {
        return false;
    }
    let mut candidate = state.clone();
    candidate.active_seat = seat;
    apply_action(
        &mut candidate,
        ValidatedAction {
            actor: seat,
            column,
        },
    );
    matches!(
        candidate.terminal_outcome,
        Some(TerminalOutcome::Win { seat: winner, .. }) if winner == seat
    )
}

fn successor_after(
    state: &ColumnFourState,
    seat: ColumnFourSeat,
    column: ColumnId,
) -> Option<ColumnFourState> {
    landing_cell(state, column)?;
    let mut candidate = state.clone();
    candidate.active_seat = seat;
    apply_action(
        &mut candidate,
        ValidatedAction {
            actor: seat,
            column,
        },
    );
    Some(candidate)
}

fn center_score(column: ColumnId) -> u8 {
    match column {
        ColumnId::C4 => 6,
        ColumnId::C3 | ColumnId::C5 => 5,
        ColumnId::C2 | ColumnId::C6 => 4,
        ColumnId::C1 | ColumnId::C7 => 3,
    }
}

fn seeded_key(seed: Seed, action_id: &str) -> u64 {
    HashValue::from_stable_bytes(format!("{}:{action_id}", seed.0).as_bytes()).0
}

fn column_from_path(path: &ActionPath) -> Option<ColumnId> {
    if path.segments.len() != 1 {
        return None;
    }
    parse_drop_segment(&path.segments[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rules::validate_command, setup::setup_match, CellId, CellOccupancy, ColumnFourEffect,
        RowId, SetupOptions,
    };
    use engine_core::{ActionPath, CommandEnvelope, FreshnessToken, RulesVersion, SeatId};

    fn seats() -> Vec<SeatId> {
        vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
    }

    fn state() -> ColumnFourState {
        setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()
    }

    fn command(state: &ColumnFourState, seat: ColumnFourSeat, segment: String) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: seats()[seat.index()].clone(),
            },
            action_path: ActionPath {
                segments: vec![segment],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn assert_choice_validates(state: &ColumnFourState, seat: ColumnFourSeat, segment: &str) {
        validate_command(state, &command(state, seat, segment.to_owned()))
            .expect("bot action validates normally");
    }

    fn occupy(state: &mut ColumnFourState, row: RowId, column: ColumnId, seat: ColumnFourSeat) {
        state.set_occupancy(CellId::new(row, column), CellOccupancy::Occupied(seat));
    }

    #[test]
    fn level0_choices_validate_for_many_seeds_and_states() {
        for seed in 0..64 {
            for occupied_count in 0..5 {
                let mut state = state();
                for (seat, segment) in [
                    (ColumnFourSeat::Seat0, "drop/c1"),
                    (ColumnFourSeat::Seat1, "drop/c2"),
                    (ColumnFourSeat::Seat0, "drop/c1"),
                    (ColumnFourSeat::Seat1, "drop/c2"),
                ]
                .into_iter()
                .take(occupied_count)
                {
                    let action =
                        validate_command(&state, &command(&state, seat, segment.to_owned()))
                            .unwrap();
                    apply_action(&mut state, action);
                }
                if state.terminal_outcome.is_some() {
                    continue;
                }
                let bot_seat = state.active_seat;
                let bot = ColumnFourRandomBot::new(Seed(seed));
                let action_path = bot
                    .select_action(&state, bot_seat)
                    .expect("legal action selected");
                let tree = legal_action_tree(
                    &state,
                    &Actor {
                        seat_id: state.seats[bot_seat.index()].clone(),
                    },
                );
                let legal_paths: Vec<_> = tree
                    .root
                    .choices
                    .iter()
                    .map(|choice| choice.path())
                    .collect();
                assert!(legal_paths.contains(&action_path));
                assert_choice_validates(&state, bot_seat, &action_path.segments[0]);
            }
        }
    }

    #[test]
    fn level0_fixed_seed_is_deterministic_and_terminal_reports_no_action() {
        let mut state = state();
        let action = validate_command(
            &state,
            &command(&state, ColumnFourSeat::Seat0, "drop/c4".to_owned()),
        )
        .unwrap();
        apply_action(&mut state, action);
        let bot = ColumnFourRandomBot::new(Seed(123));

        let left = bot
            .select_action(&state, ColumnFourSeat::Seat1)
            .expect("action selected");
        let right = bot
            .select_action(&state, ColumnFourSeat::Seat1)
            .expect("action selected");
        assert_eq!(left, right);

        state.terminal_outcome = Some(TerminalOutcome::Draw);
        let diagnostic = bot
            .select_action(&state, ColumnFourSeat::Seat1)
            .expect_err("terminal tree has no actions");
        assert_eq!(diagnostic.code, "no_legal_actions");
    }

    #[test]
    fn level2_takes_immediate_win_before_blocking() {
        let mut state = state();
        for row in [RowId::R1, RowId::R2, RowId::R3] {
            occupy(&mut state, row, ColumnId::C4, ColumnFourSeat::Seat0);
        }
        for row in [RowId::R1, RowId::R2, RowId::R3] {
            occupy(&mut state, row, ColumnId::C2, ColumnFourSeat::Seat1);
        }
        state.active_seat = ColumnFourSeat::Seat0;
        state.ply_count = 6;

        let decision = ColumnFourLevel2Bot::new(Seed(1))
            .select_decision(&state, ColumnFourSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["drop/c4"]);
        assert!(decision.rationale.contains("wins now"));
        assert_choice_validates(&state, ColumnFourSeat::Seat0, "drop/c4");
    }

    #[test]
    fn level2_blocks_immediate_opponent_win_when_no_own_win_exists() {
        let mut state = state();
        for row in [RowId::R1, RowId::R2, RowId::R3] {
            occupy(&mut state, row, ColumnId::C2, ColumnFourSeat::Seat1);
        }
        occupy(&mut state, RowId::R1, ColumnId::C4, ColumnFourSeat::Seat0);
        state.active_seat = ColumnFourSeat::Seat0;
        state.ply_count = 4;

        let decision = ColumnFourLevel2Bot::new(Seed(1))
            .select_decision(&state, ColumnFourSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["drop/c2"]);
        assert!(decision.rationale.contains("block"));
        assert_choice_validates(&state, ColumnFourSeat::Seat0, "drop/c2");
    }

    #[test]
    fn level2_prefers_safe_move_over_concession() {
        let mut state = state();
        occupy(&mut state, RowId::R1, ColumnId::C1, ColumnFourSeat::Seat1);
        occupy(&mut state, RowId::R1, ColumnId::C2, ColumnFourSeat::Seat1);
        occupy(&mut state, RowId::R1, ColumnId::C3, ColumnFourSeat::Seat1);
        state.set_occupancy(CellId::new(RowId::R1, ColumnId::C4), CellOccupancy::Empty);
        state.active_seat = ColumnFourSeat::Seat0;
        state.ply_count = 3;

        let decision = ColumnFourLevel2Bot::new(Seed(1))
            .select_decision(&state, ColumnFourSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["drop/c4"]);
    }

    #[test]
    fn level2_prefers_center_when_no_urgent_tactic_exists() {
        let state = state();
        let decision = ColumnFourLevel2Bot::new(Seed(1))
            .select_decision(&state, ColumnFourSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, vec!["drop/c4"]);
        assert!(decision.rationale.contains("central pressure"));
    }

    #[test]
    fn level2_is_deterministic_explains_safely_and_emits_bot_choice_effect() {
        let state = state();
        let bot = ColumnFourLevel2Bot::new(Seed(99));
        let left = bot.select_decision(&state, ColumnFourSeat::Seat0).unwrap();
        let right = bot.select_decision(&state, ColumnFourSeat::Seat0).unwrap();

        assert_eq!(left, right);
        assert!(!left.rationale.is_empty());
        assert!(!left.rationale.contains("candidate"));
        assert!(!left.rationale.contains("score"));
        assert!(!left.rationale.contains("debug"));
        assert!(!left.rationale.contains('['));
        assert!(matches!(
            left.effects[0].payload,
            ColumnFourEffect::BotChoseAction {
                level: 2,
                column: ColumnId::C4,
                ..
            }
        ));
        assert_choice_validates(&state, ColumnFourSeat::Seat0, &left.action_path.segments[0]);
    }

    #[test]
    fn terminal_level2_reports_no_action() {
        let mut state = state();
        state.terminal_outcome = Some(TerminalOutcome::Draw);

        let diagnostic = ColumnFourLevel2Bot::new(Seed(1))
            .select_action(&state, ColumnFourSeat::Seat0)
            .expect_err("terminal has no actions");

        assert_eq!(diagnostic.code, "no_legal_actions");
    }

    #[test]
    fn full_columns_are_not_selected() {
        let mut state = state();
        for row in RowId::ALL {
            occupy(&mut state, row, ColumnId::C4, ColumnFourSeat::Seat1);
        }

        let decision = ColumnFourLevel2Bot::new(Seed(1))
            .select_decision(&state, ColumnFourSeat::Seat0)
            .unwrap();

        assert_ne!(decision.action_path.segments, vec!["drop/c4"]);
    }

    #[test]
    fn invalid_bot_action_path_is_rejected_by_parser() {
        assert_eq!(
            column_from_path(&ActionPath {
                segments: vec!["drop/c1".to_owned(), "extra".to_owned()]
            }),
            None
        );
        assert_eq!(
            column_from_path(&ActionPath {
                segments: vec!["place/c1".to_owned()]
            }),
            None
        );
    }

    #[test]
    fn stale_human_command_still_rejects_after_bot_module_is_loaded() {
        let state = state();
        let mut stale = command(&state, ColumnFourSeat::Seat0, "drop/c4".to_owned());
        stale.freshness_token = FreshnessToken(99);

        assert_eq!(
            validate_command(&state, &stale)
                .expect_err("stale rejected")
                .code,
            "stale_action"
        );
    }
}
