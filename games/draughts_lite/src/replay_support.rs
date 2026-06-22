use engine_core::{
    ActionNode, ActionPath, ActionPreview, ActionTree, ActionTreeEncodingVersion, Actor,
    CommandEnvelope, Diagnostic, EffectEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize, Viewer,
};

use crate::{
    apply_action,
    effects::{DraughtsLiteEffect, TerminalWinReason},
    legal_action_tree, project_view, setup_match, validate_command, CellOccupancy,
    DraughtsLiteSnapshot, DraughtsLiteState, MoveKind, SetupOptions, TerminalOutcome,
};

pub type ReplayCommandPath = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayHashes {
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub view_hash: HashValue,
    pub replay_hash: HashValue,
    pub diagnostic_hash: Option<HashValue>,
    pub outcome: Option<TerminalOutcome>,
    pub terminal: bool,
    pub projections: Vec<ReplayStepProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayStepProjection {
    pub step_index: usize,
    pub board: Vec<String>,
    pub effects: Vec<String>,
    pub terminal: Option<TerminalOutcome>,
    pub public_view_hash: HashValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraughtsLiteReplayJson {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed: u64,
    pub initial_snapshot: String,
    pub command_paths: Vec<ReplayCommandPath>,
}

impl DraughtsLiteReplayJson {
    pub fn to_json(&self) -> String {
        let commands = self
            .command_paths
            .iter()
            .map(|path| {
                let segments = path
                    .iter()
                    .map(|segment| format!("\"{}\"", escape_json(segment)))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{segments}]")
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed\":{},\"initial_snapshot\":\"{}\",\"command_paths\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed,
            escape_json(&self.initial_snapshot),
            commands
        )
    }
}

impl StableSerialize for DraughtsLiteReplayJson {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &DraughtsLiteState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(
    state: &DraughtsLiteState,
    action_path: ReplayCommandPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn replay_commands(seed: u64, commands: &[ReplayCommandPath]) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = DraughtsLiteSnapshot::from_state(&state).stable_summary();
    replay_from_state(seed, initial_snapshot, commands, &mut state)
}

pub fn replay_from_state(
    seed: u64,
    initial_snapshot: String,
    commands: &[ReplayCommandPath],
    state: &mut DraughtsLiteState,
) -> ReplayHashes {
    let mut effects = Vec::new();
    let mut projections = Vec::new();

    for (index, action_path) in commands.iter().enumerate() {
        let command = command_for_state(state, action_path.clone());
        let action = validate_command(state, &command).expect("trace command validates");
        let step_effects = apply_action(state, action);
        projections.push(project_step(index, state, &step_effects));
        effects.extend(step_effects);
    }

    hashes_for_state(
        seed,
        &initial_snapshot,
        commands,
        state,
        &effects,
        None,
        projections,
    )
}

pub fn replay_invalid(
    seed: u64,
    invalid_path: ReplayCommandPath,
    stale_path: ReplayCommandPath,
) -> ReplayHashes {
    let state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let invalid_diagnostic = validate_command(&state, &command_for_state(&state, invalid_path))
        .expect_err("invalid command rejected");
    let stale_command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: ActionPath {
            segments: stale_path,
        },
        freshness_token: state.freshness_token.next(),
        rules_version: RulesVersion(1),
    };
    let stale_diagnostic =
        validate_command(&state, &stale_command).expect_err("stale command rejected");
    let diagnostic_hash = diagnostic_hash(&[invalid_diagnostic, stale_diagnostic]);
    let initial_snapshot = DraughtsLiteSnapshot::from_state(&state).stable_summary();

    hashes_for_state(
        seed,
        &initial_snapshot,
        &[],
        &state,
        &[],
        Some(diagnostic_hash),
        Vec::new(),
    )
}

pub fn hashes_for_state(
    seed: u64,
    initial_snapshot: &str,
    commands: &[ReplayCommandPath],
    state: &DraughtsLiteState,
    effects: &[EffectEnvelope<DraughtsLiteEffect>],
    diagnostic_hash: Option<HashValue>,
    projections: Vec<ReplayStepProjection>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    let replay = DraughtsLiteReplayJson {
        schema_version: 1,
        game_id: crate::ids::GAME_ID.to_owned(),
        rules_version: crate::ids::RULES_VERSION_LABEL.to_owned(),
        variant: crate::ids::VARIANT_ID.to_owned(),
        seed,
        initial_snapshot: initial_snapshot.to_owned(),
        command_paths: commands.to_vec(),
    };
    ReplayHashes {
        state_hash: DraughtsLiteSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash: replay.stable_hash(),
        diagnostic_hash,
        outcome: state.terminal_outcome,
        terminal: state.terminal_outcome.is_some(),
        projections,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<DraughtsLiteEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<DraughtsLiteEffect>) -> String {
    match &effect.payload {
        DraughtsLiteEffect::MoveCommitted {
            action_path,
            seat,
            piece_id,
            start_cell,
            final_cell,
            move_kind,
            path_length,
        } => format!(
            "MoveCommitted:{}:{}:{}:{}:{}:{}:{path_length}",
            action_path.join(">"),
            seat.as_str(),
            piece_id.stable_id(),
            start_cell.id(),
            final_cell.id(),
            move_kind_summary(*move_kind)
        ),
        DraughtsLiteEffect::QuietStep {
            piece_id,
            origin,
            landing,
            piece_kind_before,
            piece_kind_after,
        } => format!(
            "QuietStep:{}:{}:{}:{}>{}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            piece_kind_before.as_str(),
            piece_kind_after.as_str()
        ),
        DraughtsLiteEffect::CaptureStep {
            piece_id,
            origin,
            landing,
            captured_cell,
            captured_piece_id,
            captured_owner,
        } => format!(
            "CaptureStep:{}:{}:{}:{}:{}:{}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            captured_cell.id(),
            captured_piece_id.stable_id(),
            captured_owner.as_str()
        ),
        DraughtsLiteEffect::Promotion {
            piece_id,
            seat,
            cell,
            from,
            to,
            during_capture,
        } => format!(
            "Promotion:{}:{}:{}:{}>{}:{during_capture}",
            piece_id.stable_id(),
            seat.as_str(),
            cell.id(),
            from.as_str(),
            to.as_str()
        ),
        DraughtsLiteEffect::ForcedCaptureAvailable {
            active_seat,
            capture_origin_count,
            explanation,
        } => format!(
            "ForcedCaptureAvailable:{}:{capture_origin_count}:{explanation}",
            active_seat.as_str()
        ),
        DraughtsLiteEffect::ForcedContinuationRequired {
            piece_id,
            current_landing,
            continuation_destination_count,
            explanation,
        } => format!(
            "ForcedContinuationRequired:{}:{}:{continuation_destination_count}:{explanation}",
            piece_id.stable_id(),
            current_landing.id()
        ),
        DraughtsLiteEffect::InvalidCommand {
            code,
            public_message,
            rejected_action_path,
        } => format!(
            "InvalidCommand:{code}:{public_message}:{}",
            rejected_action_path.join(">")
        ),
        DraughtsLiteEffect::TerminalWin {
            winner,
            loser,
            reason,
        } => format!(
            "TerminalWin:{}:{}:{}",
            winner.as_str(),
            loser.as_str(),
            terminal_reason_summary(*reason)
        ),
        DraughtsLiteEffect::BotChoseAction {
            level,
            policy_id,
            action_path,
            rationale,
        } => format!(
            "BotChoseAction:{level}:{policy_id}:{}:{rationale}",
            action_path.join(">")
        ),
    }
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    HashValue::from_stable_bytes(action_tree_legacy_bytes(tree).as_bytes())
}

pub fn action_tree_legacy_bytes(tree: &ActionTree) -> String {
    encode_action_node(&tree.root)
}

pub fn action_tree_v1_bytes(tree: &ActionTree) -> Vec<u8> {
    tree.stable_bytes(ActionTreeEncodingVersion::V1)
}

pub fn action_tree_v1_hash(tree: &ActionTree) -> HashValue {
    tree.stable_hash(ActionTreeEncodingVersion::V1)
}

pub fn diagnostic_hash(diagnostics: &[Diagnostic]) -> HashValue {
    let bytes = diagnostics
        .iter()
        .map(|diagnostic| format!("{}:{}", diagnostic.code, diagnostic.message))
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn project_step(
    step_index: usize,
    state: &DraughtsLiteState,
    effects: &[EffectEnvelope<DraughtsLiteEffect>],
) -> ReplayStepProjection {
    ReplayStepProjection {
        step_index,
        board: state
            .board
            .row_major()
            .map(|cell| {
                let occupant = match state.occupancy(cell).expect("projection cell is in board") {
                    CellOccupancy::Empty => "empty".to_owned(),
                    CellOccupancy::Occupied(piece_id) => piece_id.stable_id(),
                };
                format!("{}:{occupant}", cell.id())
            })
            .collect(),
        effects: effects.iter().map(effect_stable_string).collect(),
        terminal: state.terminal_outcome,
        public_view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
    }
}

fn encode_action_node(node: &ActionNode) -> String {
    node.choices
        .iter()
        .map(|choice| {
            let metadata = choice
                .metadata
                .iter()
                .map(|entry| format!("{}={}", entry.key, entry.value))
                .collect::<Vec<_>>()
                .join(",");
            let child = choice
                .next
                .as_ref()
                .map_or_else(|| "leaf".to_owned(), |node| encode_action_node(node));
            format!(
                "{}|{}|{}|{}|{}|{}|{}",
                choice.segment,
                choice.label,
                choice.accessibility_label,
                preview_summary(choice.preview),
                metadata,
                choice.tags.join(","),
                child
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn move_kind_summary(kind: MoveKind) -> &'static str {
    match kind {
        MoveKind::Quiet => "quiet",
        MoveKind::Capture => "capture",
    }
}

fn terminal_reason_summary(reason: TerminalWinReason) -> &'static str {
    match reason {
        TerminalWinReason::OpponentNoPieces => "opponent_no_pieces",
        TerminalWinReason::OpponentNoLegalMove => "opponent_no_legal_move",
    }
}

fn preview_summary(preview: ActionPreview) -> &'static str {
    match preview {
        ActionPreview::Unavailable => "unavailable",
        ActionPreview::Available => "available",
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::FreshnessToken;
    use game_stdlib::board_space::Coord;

    use crate::{
        ids::board_dimensions,
        state::{sorted_pieces, Piece},
        DraughtsLiteSeat, PieceId, PieceKind, Variant,
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
            terminal_reason: None,
            freshness_token: FreshnessToken(0),
        }
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
    fn replay_reproduces_one_command_multi_segment_jump() {
        let mut left_state = multi_jump_state();
        let mut right_state = multi_jump_state();
        let initial_snapshot = DraughtsLiteSnapshot::from_state(&left_state).stable_summary();
        let commands = vec![vec![
            "from/r3c2".to_owned(),
            "jump/r5c4".to_owned(),
            "jump/r7c6".to_owned(),
        ]];

        let left = replay_from_state(7, initial_snapshot.clone(), &commands, &mut left_state);
        let right = replay_from_state(7, initial_snapshot, &commands, &mut right_state);

        assert_eq!(left, right);
        assert_eq!(left.projections.len(), 1);
        assert!(!left.terminal);
        assert!(left.projections[0]
            .board
            .iter()
            .any(|cell| cell == "r7c6:seat_0-p01"));
        assert!(left.projections[0]
            .effects
            .iter()
            .any(|effect| effect
                == "MoveCommitted:from/r3c2>jump/r5c4>jump/r7c6:seat_0:seat_0-p01:r3c2:r7c6:capture:2"));
        assert!(left.projections[0]
            .effects
            .iter()
            .any(|effect| effect.starts_with("ForcedContinuationRequired:seat_0-p01:r5c4")));
        assert_eq!(left.projections[0].public_view_hash, left.view_hash);
    }

    #[test]
    fn replay_hashes_are_stable_for_identical_command_paths() {
        let mut left_state = multi_jump_state();
        let mut right_state = multi_jump_state();
        let initial_snapshot = DraughtsLiteSnapshot::from_state(&left_state).stable_summary();
        let commands = vec![vec![
            "from/r3c2".to_owned(),
            "jump/r5c4".to_owned(),
            "jump/r7c6".to_owned(),
        ]];

        let left = replay_from_state(12, initial_snapshot.clone(), &commands, &mut left_state);
        let right = replay_from_state(12, initial_snapshot, &commands, &mut right_state);

        assert_eq!(left.state_hash, right.state_hash);
        assert_eq!(left.effect_hash, right.effect_hash);
        assert_eq!(left.action_tree_hash, right.action_tree_hash);
        assert_eq!(left.view_hash, right.view_hash);
        assert_eq!(left.replay_hash, right.replay_hash);
    }

    #[test]
    fn replay_json_preserves_command_path_segment_order() {
        let replay = DraughtsLiteReplayJson {
            schema_version: 1,
            game_id: "draughts_lite".to_owned(),
            rules_version: "draughts_lite-rules-v1".to_owned(),
            variant: "draughts_lite_standard".to_owned(),
            seed: 7,
            initial_snapshot: "snapshot".to_owned(),
            command_paths: vec![vec![
                "from/r3c2".to_owned(),
                "jump/r5c4".to_owned(),
                "jump/r7c6".to_owned(),
            ]],
        };

        let json = replay.to_json();

        assert!(json.contains("\"command_paths\":[[\"from/r3c2\",\"jump/r5c4\",\"jump/r7c6\"]]"));
        assert_eq!(replay.stable_hash(), replay.clone().stable_hash());
    }

    #[test]
    fn invalid_and_stale_diagnostics_hash_stably_without_mutation() {
        let left = replay_invalid(
            3,
            vec!["from/r1c1".to_owned()],
            vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
        );
        let right = replay_invalid(
            3,
            vec!["from/r1c1".to_owned()],
            vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
        );

        assert_eq!(left, right);
        assert!(left.diagnostic_hash.is_some());
        assert_eq!(left.projections.len(), 0);
        assert_eq!(left.state_hash, right.state_hash);
        assert_eq!(left.effect_hash, effect_hash(&[]));
    }
}
