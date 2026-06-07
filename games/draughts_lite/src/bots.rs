use ai_core::RandomLegalBot;
use engine_core::{
    ActionNode, ActionPath, ActionTree, Actor, CommandEnvelope, Diagnostic, EffectEnvelope,
    HashValue, RulesVersion, Seed,
};

use crate::{
    effects::{bot_chose_action_effect, DraughtsLiteEffect},
    legal_action_tree,
    rules::{apply_action, legal_moves_for, validate_command, LegalMove, MoveKind},
    state::{DraughtsLiteState, PieceKind, TerminalOutcome},
    DraughtsLiteSeat,
};

pub const RANDOM_POLICY_ID: &str = "draughts_lite-random-legal-v1";
pub const LEVEL1_POLICY_ID: &str = "draughts_lite_level1_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotDecision {
    pub action_path: ActionPath,
    pub rationale: String,
    pub effects: Vec<EffectEnvelope<DraughtsLiteEffect>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DraughtsLiteRandomBot {
    pub seed: Seed,
}

impl DraughtsLiteRandomBot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_action(
        &self,
        state: &DraughtsLiteState,
        bot_seat: DraughtsLiteSeat,
    ) -> Result<ActionPath, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        RandomLegalBot::new(self.seed).select_action(&tree)
    }

    pub fn select_decision(
        &self,
        state: &DraughtsLiteState,
        bot_seat: DraughtsLiteSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let action_path = self.select_action(state, bot_seat)?;
        Ok(decision(
            0,
            RANDOM_POLICY_ID,
            action_path,
            "Selected a seeded random complete legal Draughts Lite path.".to_owned(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DraughtsLiteLevel1Bot {
    pub seed: Seed,
}

impl DraughtsLiteLevel1Bot {
    pub fn new(seed: Seed) -> Self {
        Self { seed }
    }

    pub fn select_decision(
        &self,
        state: &DraughtsLiteState,
        bot_seat: DraughtsLiteSeat,
    ) -> Result<BotDecision, Diagnostic> {
        let tree = legal_action_tree(state, &actor_for_seat(state, bot_seat));
        let mut candidates = complete_legal_paths(&tree)
            .into_iter()
            .filter_map(|path| Candidate::new(state, bot_seat, path, self.seed))
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
        Ok(decision(1, LEVEL1_POLICY_ID, chosen.action_path, rationale))
    }

    pub fn select_action(
        &self,
        state: &DraughtsLiteState,
        bot_seat: DraughtsLiteSeat,
    ) -> Result<ActionPath, Diagnostic> {
        self.select_decision(state, bot_seat)
            .map(|decision| decision.action_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Candidate {
    action_path: ActionPath,
    legal_move: LegalMove,
    terminal_win: bool,
    capture_to_promotion: bool,
    promotes: bool,
    captures: bool,
    capture_count: u8,
    avoids_hanging_king: bool,
    material_delta: i16,
    path_order_score: u8,
    seeded_key: u64,
}

impl Candidate {
    fn new(
        state: &DraughtsLiteState,
        bot_seat: DraughtsLiteSeat,
        action_path: ActionPath,
        seed: Seed,
    ) -> Option<Self> {
        let command = command_for_path(state, bot_seat, action_path.clone());
        let action = validate_command(state, &command).ok()?;
        let legal_move = action.legal_move.clone();
        let successor = successor_after(state, bot_seat, action_path.clone())?;
        let captures = legal_move.kind == MoveKind::Capture;
        let capture_count = legal_move.captured_piece_ids().len() as u8;
        let promotes = legal_move.promotes();
        let action_id = action_path.segments.join(">");

        Some(Self {
            action_path,
            legal_move,
            terminal_win: matches!(
                successor.terminal_outcome,
                Some(TerminalOutcome::Win { seat }) if seat == bot_seat
            ),
            capture_to_promotion: captures && promotes,
            promotes,
            captures,
            capture_count,
            avoids_hanging_king: !king_hangs_after(&successor, bot_seat),
            material_delta: material_delta(&successor, bot_seat),
            path_order_score: path_order_score(&action_id),
            seeded_key: seeded_key(seed, &action_id),
        })
    }

    fn priority_key(&self) -> (u8, u8, u8, u8, u8, u8, i16, u8, u64) {
        (
            self.terminal_win as u8,
            self.capture_to_promotion as u8,
            self.promotes as u8,
            self.captures as u8,
            self.capture_count,
            self.avoids_hanging_king as u8,
            self.material_delta,
            self.path_order_score,
            self.seeded_key,
        )
    }

    fn rationale(&self) -> String {
        if self.terminal_win {
            return format!("Chose {} because it wins immediately.", self.label());
        }
        if self.capture_to_promotion {
            return format!(
                "Chose {} because it captures and promotes the piece.",
                self.label()
            );
        }
        if self.promotes {
            return format!("Chose {} because it creates a king.", self.label());
        }
        if self.captures && self.capture_count > 1 {
            return format!(
                "Chose {} because it captures more pieces; longest capture is a preference here, not a rule.",
                self.label()
            );
        }
        if self.captures {
            return format!(
                "Chose {} because it wins material with a legal capture.",
                self.label()
            );
        }
        if self.avoids_hanging_king
            && self.legal_move.steps[0].piece_kind_before == PieceKind::Crown
        {
            return format!(
                "Chose {} because it keeps the king out of an immediate capture.",
                self.label()
            );
        }
        if self.material_delta > 0 {
            return format!("Chose {} because it leaves better material.", self.label());
        }
        format!(
            "Chose {} after equivalent legal options were resolved deterministically.",
            self.label()
        )
    }

    fn label(&self) -> String {
        self.action_path.segments.join(", ")
    }
}

fn decision(level: u8, policy_id: &str, action_path: ActionPath, rationale: String) -> BotDecision {
    BotDecision {
        action_path: action_path.clone(),
        rationale: rationale.clone(),
        effects: vec![bot_chose_action_effect(
            level,
            policy_id,
            action_path.segments,
            rationale,
        )],
    }
}

fn actor_for_seat(state: &DraughtsLiteState, seat: DraughtsLiteSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn command_for_path(
    state: &DraughtsLiteState,
    seat: DraughtsLiteSeat,
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
    state: &DraughtsLiteState,
    seat: DraughtsLiteSeat,
    action_path: ActionPath,
) -> Option<DraughtsLiteState> {
    let command = command_for_path(state, seat, action_path);
    let action = validate_command(state, &command).ok()?;
    let mut candidate = state.clone();
    apply_action(&mut candidate, action);
    Some(candidate)
}

fn complete_legal_paths(tree: &ActionTree) -> Vec<ActionPath> {
    let mut paths = Vec::new();
    collect_paths(&tree.root, Vec::new(), &mut paths);
    paths
}

fn collect_paths(node: &ActionNode, prefix: Vec<String>, paths: &mut Vec<ActionPath>) {
    for choice in &node.choices {
        let mut next_prefix = prefix.clone();
        next_prefix.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            collect_paths(next, next_prefix, paths);
        } else {
            paths.push(ActionPath {
                segments: next_prefix,
            });
        }
    }
}

fn king_hangs_after(state: &DraughtsLiteState, bot_seat: DraughtsLiteSeat) -> bool {
    let own_kings = state
        .pieces_for_seat(bot_seat)
        .filter(|piece| piece.kind == PieceKind::Crown)
        .map(|piece| piece.id)
        .collect::<Vec<_>>();
    if own_kings.is_empty() || state.terminal_outcome.is_some() {
        return false;
    }

    legal_moves_for(state, bot_seat.other())
        .into_iter()
        .filter(|legal_move| legal_move.kind == MoveKind::Capture)
        .any(|legal_move| {
            legal_move
                .captured_piece_ids()
                .into_iter()
                .any(|captured| own_kings.contains(&captured))
        })
}

fn material_delta(state: &DraughtsLiteState, bot_seat: DraughtsLiteSeat) -> i16 {
    let own = state.pieces_for_seat(bot_seat).count() as i16;
    let opponent = state.pieces_for_seat(bot_seat.other()).count() as i16;
    own - opponent
}

fn path_order_score(action_id: &str) -> u8 {
    let first = action_id.as_bytes().first().copied().unwrap_or_default();
    u8::MAX.saturating_sub(first)
}

fn seeded_key(seed: Seed, action_id: &str) -> u64 {
    HashValue::from_stable_bytes(format!("{}:{action_id}", seed.0).as_bytes()).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{FreshnessToken, SeatId};
    use game_stdlib::board_space::Coord;

    use crate::{
        ids::board_dimensions,
        state::{sorted_pieces, CellOccupancy, Piece},
        PieceId, Variant,
    };

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
        PieceId::new(owner, ordinal).unwrap()
    }

    fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
        Piece {
            id: piece_id(owner, ordinal),
            owner,
            kind: PieceKind::Man,
            cell: coord(row, col),
        }
    }

    fn crown(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
        Piece {
            id: piece_id(owner, ordinal),
            owner,
            kind: PieceKind::Crown,
            cell: coord(row, col),
        }
    }

    fn empty_state(active_seat: DraughtsLiteSeat, pieces: Vec<Piece>) -> DraughtsLiteState {
        let board = board_dimensions();
        let mut cells = DraughtsLiteState::empty_cells();
        for piece in &pieces {
            cells[piece.cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(piece.id);
        }

        DraughtsLiteState {
            variant: Variant::draughts_lite_standard(),
            board,
            cells,
            pieces: sorted_pieces(pieces),
            active_seat,
            seats: [SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())],
            ply_count: 0,
            command_count: 0,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }

    fn command(state: &DraughtsLiteState, path: ActionPath) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor_for_seat(state, state.active_seat),
            action_path: path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn assert_choice_validates(state: &DraughtsLiteState, action_path: &ActionPath) {
        validate_command(state, &command(state, action_path.clone()))
            .expect("bot action validates normally");
    }

    fn multi_jump_state() -> DraughtsLiteState {
        empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
                man(DraughtsLiteSeat::Seat1, 3, 8, 7),
            ],
        )
    }

    #[test]
    fn level0_selects_complete_legal_leaf_paths_including_multi_segment() {
        let state = multi_jump_state();
        let bot = DraughtsLiteRandomBot::new(Seed(7));
        let action_path = bot
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect("legal action selected");

        assert_eq!(
            action_path.segments,
            ["from/r3c2", "jump/r5c4", "jump/r7c6"]
        );
        assert_choice_validates(&state, &action_path);
    }

    #[test]
    fn level0_fixed_seed_is_deterministic_and_terminal_reports_no_action() {
        let mut state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
        );
        let bot = DraughtsLiteRandomBot::new(Seed(123));

        let left = bot
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect("action selected");
        let right = bot
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect("action selected");
        assert_eq!(left, right);

        state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0,
        });
        let diagnostic = bot
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect_err("terminal tree has no actions");
        assert_eq!(diagnostic.code, "no_legal_actions");
    }

    #[test]
    fn level1_prefers_terminal_capture_and_emits_public_effect() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            ],
        );

        let decision = DraughtsLiteLevel1Bot::new(Seed(1))
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments, ["from/r3c2", "jump/r5c4"]);
        assert!(decision.rationale.contains("wins immediately"));
        assert!(matches!(
            decision.effects[0].payload,
            DraughtsLiteEffect::BotChoseAction { level: 1, .. }
        ));
        assert_choice_validates(&state, &decision.action_path);
    }

    #[test]
    fn level1_prefers_promotion_over_comparable_quiet_move() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 7, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 6),
                man(DraughtsLiteSeat::Seat1, 1, 6, 7),
            ],
        );

        let decision = DraughtsLiteLevel1Bot::new(Seed(1))
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_eq!(decision.action_path.segments[0], "from/r7c2");
        assert!(decision
            .action_path
            .segments
            .iter()
            .any(|segment| segment == "to/r8c1" || segment == "to/r8c3"));
        assert!(decision.rationale.contains("creates a king"));
        assert_choice_validates(&state, &decision.action_path);
    }

    #[test]
    fn level1_prefers_longer_capture_without_claiming_it_is_required() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 6),
                man(DraughtsLiteSeat::Seat1, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 2, 6, 5),
                man(DraughtsLiteSeat::Seat1, 3, 4, 7),
                man(DraughtsLiteSeat::Seat1, 4, 8, 7),
            ],
        );

        let decision = DraughtsLiteLevel1Bot::new(Seed(1))
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_eq!(
            decision.action_path.segments,
            ["from/r3c2", "jump/r5c4", "jump/r7c6"]
        );
        assert!(decision.rationale.contains("captures more pieces"));
        assert!(decision.rationale.contains("not a rule"));
        assert_choice_validates(&state, &decision.action_path);
    }

    #[test]
    fn level1_never_emits_partial_continuation_path() {
        let state = multi_jump_state();

        let decision = DraughtsLiteLevel1Bot::new(Seed(9))
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_eq!(
            decision.action_path.segments,
            ["from/r3c2", "jump/r5c4", "jump/r7c6"]
        );
        assert_choice_validates(&state, &decision.action_path);
    }

    #[test]
    fn level1_avoids_obvious_king_hang_when_priorities_tie() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                crown(DraughtsLiteSeat::Seat0, 1, 4, 3),
                man(DraughtsLiteSeat::Seat1, 1, 6, 5),
            ],
        );

        let decision = DraughtsLiteLevel1Bot::new(Seed(1))
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_ne!(decision.action_path.segments, ["from/r4c3", "to/r5c4"]);
        assert_choice_validates(&state, &decision.action_path);
    }

    #[test]
    fn level1_is_deterministic_and_explains_without_internals_or_search_claims() {
        let state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![
                man(DraughtsLiteSeat::Seat0, 1, 3, 2),
                man(DraughtsLiteSeat::Seat0, 2, 3, 4),
                man(DraughtsLiteSeat::Seat1, 1, 6, 7),
            ],
        );
        let bot = DraughtsLiteLevel1Bot::new(Seed(99));
        let left = bot
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();
        let right = bot
            .select_decision(&state, DraughtsLiteSeat::Seat0)
            .unwrap();

        assert_eq!(left, right);
        assert!(!left.rationale.is_empty());
        assert!(!left.rationale.contains("candidate"));
        assert!(!left.rationale.contains("score"));
        assert!(!left.rationale.contains("debug"));
        assert!(!left.rationale.contains("hash"));
        assert!(!left.rationale.contains("search"));
        assert!(!left.rationale.contains('['));
        assert_choice_validates(&state, &left.action_path);
    }

    #[test]
    fn terminal_level1_reports_no_action() {
        let mut state = empty_state(
            DraughtsLiteSeat::Seat0,
            vec![man(DraughtsLiteSeat::Seat0, 1, 3, 2)],
        );
        state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat0,
        });

        let diagnostic = DraughtsLiteLevel1Bot::new(Seed(1))
            .select_action(&state, DraughtsLiteSeat::Seat0)
            .expect_err("terminal has no actions");

        assert_eq!(diagnostic.code, "no_legal_actions");
    }
}
