use engine_core::{Actor, FreshnessToken, StableSerialize, Viewer};

use crate::{
    actions::{legal_action_tree, PASS_REASON_NO_MOVES},
    ids::{CellId, DirectionalFlipSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    rules::{disc_counts, Score},
    state::{CellOccupancy, DirectionalFlipState, TerminalOutcome},
    ui::{cell_layout, disc_token},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub display_name: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub board_rows: u8,
    pub board_columns: u8,
    pub cells: Vec<CellView>,
    pub active_seat: Option<DirectionalFlipSeat>,
    pub ply_count: u8,
    pub status_label: String,
    pub freshness_token: FreshnessToken,
    pub score: ScoreView,
    pub legal_targets: Vec<LegalTargetView>,
    pub terminal: TerminalView,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
    pub last_action_summary: Option<String>,
    pub bot_rationale: Option<String>,
    pub replay_step_index: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellView {
    pub cell: CellId,
    pub cell_id: String,
    pub row: u8,
    pub column: u8,
    pub occupancy: String,
    pub owner: Option<DirectionalFlipSeat>,
    pub disc_token_key: Option<String>,
    pub disc_shape_label: Option<String>,
    pub disc_pattern_label: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScoreView {
    pub seat_0: u8,
    pub seat_1: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalTargetView {
    pub action_kind: String,
    pub action_segment: String,
    pub label: String,
    pub accessibility_label: String,
    pub freshness_token: FreshnessToken,
    pub cell: Option<CellId>,
    pub preview: Option<PlacementPreviewView>,
    pub reason_code: Option<String>,
    pub explanation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementPreviewView {
    pub preview_id: String,
    pub target_cell: CellId,
    pub target_cell_id: String,
    pub row: u8,
    pub column: u8,
    pub ordered_flip_cells: Vec<CellId>,
    pub ordered_flip_cell_ids: Vec<String>,
    pub direction_groups: Vec<DirectionGroupView>,
    pub explanation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionGroupView {
    pub direction: String,
    pub cells: Vec<CellId>,
    pub cell_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win {
        winning_seat: DirectionalFlipSeat,
        final_score: ScoreView,
    },
    Draw {
        final_score: ScoreView,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateView {
    pub status: String,
    pub hidden_fields: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub board_label: String,
    pub row_count: u8,
    pub column_count: u8,
    pub first_disc_token_key: String,
    pub first_disc_shape_label: String,
    pub first_disc_pattern_label: String,
    pub second_disc_token_key: String,
    pub second_disc_shape_label: String,
    pub second_disc_pattern_label: String,
    pub legal_target_shape_label: String,
    pub forced_pass_label: String,
}

pub fn project_view(state: &DirectionalFlipState, _viewer: &Viewer) -> PublicView {
    let score = score_view(disc_counts(state));
    let terminal = terminal_view(state.terminal_outcome, score.clone());
    let legal_targets = legal_targets(state);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Directional Flip".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        board_rows: 8,
        board_columns: 8,
        cells: CellId::ALL
            .into_iter()
            .map(|cell| cell_view(state, cell))
            .collect(),
        active_seat: state
            .terminal_outcome
            .is_none()
            .then_some(state.active_seat),
        ply_count: state.ply_count,
        status_label: status_label(&terminal, state.active_seat),
        freshness_token: state.freshness_token,
        score,
        legal_targets,
        terminal,
        private_view: PrivateView {
            status: "not_applicable_perfect_information".to_owned(),
            hidden_fields: Vec::new(),
        },
        ui: ui_metadata(),
        last_action_summary: None,
        bot_rationale: None,
        replay_step_index: None,
    }
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};rows={};columns={};cells={};active={};ply={};status={};freshness={};score={}-{};legal={};terminal={};private={};hidden={};ui={};last={};bot={};replay={}",
            self.schema_version,
            self.rules_version,
            self.game_id,
            self.variant_id,
            self.rules_version_label,
            self.board_rows,
            self.board_columns,
            self.cells
                .iter()
                .map(encode_cell)
                .collect::<Vec<_>>()
                .join(","),
            self.active_seat.map_or("none", DirectionalFlipSeat::as_str),
            self.ply_count,
            self.status_label,
            self.freshness_token.0,
            self.score.seat_0,
            self.score.seat_1,
            self.legal_targets
                .iter()
                .map(encode_legal_target)
                .collect::<Vec<_>>()
                .join(","),
            encode_terminal(&self.terminal),
            self.private_view.status,
            self.private_view.hidden_fields.join(","),
            encode_ui(&self.ui),
            self.last_action_summary.as_deref().unwrap_or("none"),
            self.bot_rationale.as_deref().unwrap_or("none"),
            self.replay_step_index
                .map_or_else(|| "none".to_owned(), |step| step.to_string())
        )
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn legal_targets(state: &DirectionalFlipState) -> Vec<LegalTargetView> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    let actor = Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    };
    legal_action_tree(state, &actor)
        .root
        .choices
        .into_iter()
        .map(|choice| {
            let action_kind = metadata_value(&choice.metadata, "action_kind")
                .unwrap_or("unknown")
                .to_owned();
            if action_kind == "forced_pass" {
                return LegalTargetView {
                    action_kind,
                    action_segment: choice.segment,
                    label: choice.label,
                    accessibility_label: choice.accessibility_label,
                    freshness_token: state.freshness_token,
                    cell: None,
                    preview: None,
                    reason_code: Some(
                        metadata_value(&choice.metadata, "reason_code")
                            .unwrap_or(PASS_REASON_NO_MOVES)
                            .to_owned(),
                    ),
                    explanation: metadata_value(&choice.metadata, "explanation")
                        .unwrap_or("No legal placements are available, so this seat must pass.")
                        .to_owned(),
                };
            }

            let cell = metadata_value(&choice.metadata, "cell_id").and_then(CellId::parse);
            LegalTargetView {
                action_kind,
                action_segment: choice.segment,
                label: choice.label,
                accessibility_label: choice.accessibility_label,
                freshness_token: state.freshness_token,
                cell,
                preview: cell.map(|target| placement_preview_view(target, &choice.metadata)),
                reason_code: None,
                explanation: metadata_value(&choice.metadata, "explanation")
                    .unwrap_or("")
                    .to_owned(),
            }
        })
        .collect()
}

fn placement_preview_view(
    target_cell: CellId,
    metadata: &[engine_core::ActionMetadata],
) -> PlacementPreviewView {
    let ordered_flip_cell_ids = metadata_value(metadata, "ordered_flip_cells")
        .map(split_cell_ids)
        .unwrap_or_default();
    let ordered_flip_cells = ordered_flip_cell_ids
        .iter()
        .filter_map(|cell_id| CellId::parse(cell_id))
        .collect::<Vec<_>>();
    let direction_groups = metadata_value(metadata, "direction_groups")
        .map(parse_direction_groups)
        .unwrap_or_default();

    PlacementPreviewView {
        preview_id: metadata_value(metadata, "preview_id")
            .unwrap_or("")
            .to_owned(),
        target_cell,
        target_cell_id: target_cell.as_string(),
        row: metadata_value(metadata, "row")
            .and_then(|value| value.parse().ok())
            .unwrap_or((target_cell.row.index() + 1) as u8),
        column: metadata_value(metadata, "column")
            .and_then(|value| value.parse().ok())
            .unwrap_or((target_cell.column.index() + 1) as u8),
        ordered_flip_cells,
        ordered_flip_cell_ids,
        direction_groups,
        explanation: metadata_value(metadata, "explanation")
            .unwrap_or("")
            .to_owned(),
    }
}

fn cell_view(state: &DirectionalFlipState, cell: CellId) -> CellView {
    let layout = cell_layout(cell);
    match state.occupancy(cell) {
        CellOccupancy::Empty => CellView {
            cell,
            cell_id: cell.as_string(),
            row: layout.row,
            column: layout.column,
            occupancy: "empty".to_owned(),
            owner: None,
            disc_token_key: None,
            disc_shape_label: None,
            disc_pattern_label: None,
        },
        CellOccupancy::Occupied(owner) => {
            let token = disc_token(owner);
            CellView {
                cell,
                cell_id: cell.as_string(),
                row: layout.row,
                column: layout.column,
                occupancy: "occupied".to_owned(),
                owner: Some(owner),
                disc_token_key: Some(token.token_key.to_owned()),
                disc_shape_label: Some(token.shape_label.to_owned()),
                disc_pattern_label: Some(token.pattern_label.to_owned()),
            }
        }
    }
}

fn score_view(score: Score) -> ScoreView {
    ScoreView {
        seat_0: score.seat_0,
        seat_1: score.seat_1,
    }
}

fn terminal_view(outcome: Option<TerminalOutcome>, score: ScoreView) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Draw) => TerminalView::Draw { final_score: score },
        Some(TerminalOutcome::Win { seat }) => TerminalView::Win {
            winning_seat: seat,
            final_score: score,
        },
    }
}

fn ui_metadata() -> UiMetadata {
    let first = disc_token(DirectionalFlipSeat::Seat0);
    let second = disc_token(DirectionalFlipSeat::Seat1);
    UiMetadata {
        board_label: "Directional Flip board".to_owned(),
        row_count: 8,
        column_count: 8,
        first_disc_token_key: first.token_key.to_owned(),
        first_disc_shape_label: first.shape_label.to_owned(),
        first_disc_pattern_label: first.pattern_label.to_owned(),
        second_disc_token_key: second.token_key.to_owned(),
        second_disc_shape_label: second.shape_label.to_owned(),
        second_disc_pattern_label: second.pattern_label.to_owned(),
        legal_target_shape_label: "outlined legal target".to_owned(),
        forced_pass_label: "Forced pass".to_owned(),
    }
}

fn status_label(terminal: &TerminalView, active_seat: DirectionalFlipSeat) -> String {
    match terminal {
        TerminalView::NonTerminal => format!("{} to place", active_seat.as_str()),
        TerminalView::Win { winning_seat, .. } => format!("{} wins", winning_seat.as_str()),
        TerminalView::Draw { .. } => "draw".to_owned(),
    }
}

fn metadata_value<'a>(metadata: &'a [engine_core::ActionMetadata], key: &str) -> Option<&'a str> {
    metadata
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.as_str())
}

fn split_cell_ids(value: &str) -> Vec<String> {
    if value.is_empty() {
        Vec::new()
    } else {
        value.split(',').map(str::to_owned).collect()
    }
}

fn parse_direction_groups(value: &str) -> Vec<DirectionGroupView> {
    if value.is_empty() {
        return Vec::new();
    }

    value
        .split('|')
        .filter_map(|group| {
            let (direction, cells) = group.split_once(':')?;
            let cell_ids = split_cell_ids(cells);
            let parsed_cells = cell_ids
                .iter()
                .filter_map(|cell_id| CellId::parse(cell_id))
                .collect::<Vec<_>>();
            Some(DirectionGroupView {
                direction: direction.to_owned(),
                cells: parsed_cells,
                cell_ids,
            })
        })
        .collect()
}

fn encode_cell(cell: &CellView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        cell.cell_id,
        cell.row,
        cell.column,
        cell.occupancy,
        cell.owner.map_or("none", DirectionalFlipSeat::as_str),
        cell.disc_token_key.as_deref().unwrap_or("none"),
        cell.disc_shape_label.as_deref().unwrap_or("none"),
        cell.disc_pattern_label.as_deref().unwrap_or("none")
    )
}

fn encode_legal_target(target: &LegalTargetView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}",
        target.action_kind,
        target.action_segment,
        target.label,
        target.accessibility_label,
        target.freshness_token.0,
        target
            .cell
            .map_or_else(|| "none".to_owned(), CellId::as_string),
        target
            .preview
            .as_ref()
            .map_or_else(|| "none".to_owned(), encode_preview),
        target.reason_code.as_deref().unwrap_or("none"),
        target.explanation
    )
}

fn encode_preview(preview: &PlacementPreviewView) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        preview.preview_id,
        preview.target_cell_id,
        preview.row,
        preview.column,
        preview.ordered_flip_cell_ids.join(","),
        preview
            .direction_groups
            .iter()
            .map(|group| format!("{}:{}", group.direction, group.cell_ids.join(",")))
            .collect::<Vec<_>>()
            .join("/")
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Draw { final_score } => {
            format!("draw:{}-{}", final_score.seat_0, final_score.seat_1)
        }
        TerminalView::Win {
            winning_seat,
            final_score,
        } => format!(
            "win:{}:{}-{}",
            winning_seat.as_str(),
            final_score.seat_0,
            final_score.seat_1
        ),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}x{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ui.board_label,
        ui.row_count,
        ui.column_count,
        ui.first_disc_token_key,
        ui.first_disc_shape_label,
        ui.first_disc_pattern_label,
        ui.second_disc_token_key,
        ui.second_disc_shape_label,
        ui.second_disc_pattern_label,
        ui.legal_target_shape_label,
        ui.forced_pass_label
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rules::{apply_action, ValidatedAction},
        setup::setup_match,
        DirectionalFlipState, ForcedPass,
    };
    use engine_core::{SeatId, Seed};

    fn state() -> DirectionalFlipState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn viewer() -> Viewer {
        Viewer { seat_id: None }
    }

    #[test]
    fn public_view_contains_board_counts_legal_targets_and_previews() {
        let state = state();
        let view = project_view(&state, &viewer());

        assert_eq!(view.game_id, "directional_flip");
        assert_eq!(view.display_name, "Directional Flip");
        assert_eq!(view.variant_id, "directional_flip_standard");
        assert_eq!(view.board_rows, 8);
        assert_eq!(view.board_columns, 8);
        assert_eq!(view.cells.len(), 64);
        assert_eq!(view.active_seat, Some(DirectionalFlipSeat::Seat0));
        assert_eq!(
            view.score,
            ScoreView {
                seat_0: 2,
                seat_1: 2
            }
        );
        assert_eq!(view.legal_targets.len(), 4);

        let target = view
            .legal_targets
            .iter()
            .find(|target| target.action_segment == "place/r3c4")
            .expect("opening target exists");
        assert_eq!(target.action_kind, "place");
        assert_eq!(target.cell, Some(CellId::parse("r3c4").unwrap()));
        let preview = target.preview.as_ref().expect("placement preview");
        assert_eq!(preview.preview_id, "preview:r3c4");
        assert_eq!(preview.ordered_flip_cell_ids, vec!["r4c4"]);
        assert_eq!(preview.direction_groups[0].direction, "south");
        assert_eq!(preview.explanation, target.explanation);
    }

    #[test]
    fn public_view_represents_forced_pass_without_preview() {
        let mut state = state();
        state.cells = DirectionalFlipState::empty_cells();

        let view = project_view(&state, &viewer());

        assert_eq!(view.legal_targets.len(), 1);
        let pass = &view.legal_targets[0];
        assert_eq!(pass.action_kind, "forced_pass");
        assert_eq!(pass.action_segment, "pass/forced");
        assert_eq!(pass.reason_code.as_deref(), Some(PASS_REASON_NO_MOVES));
        assert_eq!(pass.preview, None);
        assert_eq!(pass.cell, None);
    }

    #[test]
    fn public_view_is_empty_of_legal_targets_after_terminal() {
        let mut state = state();
        state.cells = DirectionalFlipState::empty_cells();
        state.set_occupancy(
            CellId::parse("r1c1").unwrap(),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0),
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

        let view = project_view(&state, &viewer());

        assert_eq!(view.active_seat, None);
        assert!(view.legal_targets.is_empty());
        assert_eq!(
            view.terminal,
            TerminalView::Win {
                winning_seat: DirectionalFlipSeat::Seat0,
                final_score: ScoreView {
                    seat_0: 1,
                    seat_1: 0
                },
            }
        );
    }

    #[test]
    fn public_view_no_leak_fields_are_empty_and_summary_has_no_internals() {
        let state = state();
        let view = project_view(&state, &viewer());
        let summary = view.stable_summary();

        assert_eq!(
            view.private_view.status,
            "not_applicable_perfect_information"
        );
        assert!(view.private_view.hidden_fields.is_empty());
        for forbidden in [
            "Seed(",
            "rng",
            "internal",
            "debug",
            "consecutive_forced_passes",
            "terminal_outcome",
            "DirectionalFlipState",
            "stale",
        ] {
            assert!(
                !summary.contains(forbidden),
                "summary leaked forbidden marker {forbidden}"
            );
        }
    }

    #[test]
    fn public_view_stable_serialization_is_deterministic() {
        let state = state();
        let left = project_view(&state, &viewer());
        let right = project_view(&state, &viewer());

        assert_eq!(left.stable_bytes(), right.stable_bytes());
    }

    #[test]
    fn ui_metadata_supports_non_color_only_encoding() {
        let view = project_view(&state(), &viewer());

        assert_eq!(view.ui.first_disc_shape_label, "ring disc");
        assert_eq!(view.ui.first_disc_pattern_label, "single-ring pattern");
        assert_eq!(view.ui.second_disc_shape_label, "cross disc");
        assert_eq!(view.ui.second_disc_pattern_label, "cross-hatch pattern");
        assert_eq!(view.ui.legal_target_shape_label, "outlined legal target");
    }
}
