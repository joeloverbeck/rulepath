use engine_core::{Actor, FreshnessToken, StableSerialize, Viewer};

use crate::{
    actions::legal_action_tree,
    ids::{CellId, ColumnFourSeat, ColumnId, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    rules::{landing_cell, legal_columns},
    state::{CellOccupancy, ColumnFourState, TerminalOutcome},
    ui::{cell_layout, column_control, piece_token},
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
    pub columns: Vec<ColumnSummaryView>,
    pub active_seat: Option<ColumnFourSeat>,
    pub ply_count: u8,
    pub status_label: String,
    pub freshness_token: FreshnessToken,
    pub legal_targets: Vec<LegalColumnTargetView>,
    pub terminal: TerminalView,
    pub private_view: PrivateView,
    pub replay_step_index: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellView {
    pub cell: CellId,
    pub row: u8,
    pub column: u8,
    pub occupancy: String,
    pub owner: Option<ColumnFourSeat>,
    pub piece_token_key: Option<String>,
    pub piece_shape_label: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnSummaryView {
    pub column: ColumnId,
    pub column_id: String,
    pub label: String,
    pub is_full: bool,
    pub legal_action_segment: Option<String>,
    pub landing_preview: Option<CellId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalColumnTargetView {
    pub column: ColumnId,
    pub action_segment: String,
    pub label: String,
    pub accessibility_label: String,
    pub freshness_token: FreshnessToken,
    pub landing_preview: CellId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win {
        winning_seat: ColumnFourSeat,
        line: [CellId; 4],
        rationale: OutcomeRationaleView,
    },
    Draw {
        rationale: OutcomeRationaleView,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub decisive_rule_ids: Vec<String>,
    pub line_cells: Vec<CellId>,
    pub line_orientation: Option<String>,
    pub board_full: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateView {
    pub status: String,
    pub hidden_fields: Vec<String>,
}

pub fn project_view(state: &ColumnFourState, _viewer: &Viewer) -> PublicView {
    let terminal = match state.terminal_outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Draw) => TerminalView::Draw {
            rationale: draw_rationale(),
        },
        Some(TerminalOutcome::Win { seat, line }) => TerminalView::Win {
            winning_seat: seat,
            line: line.cells,
            rationale: line_win_rationale(line.cells),
        },
    };
    let legal_targets = legal_targets(state);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Column Four".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        board_rows: 6,
        board_columns: 7,
        cells: CellId::ALL
            .into_iter()
            .map(|cell| cell_view(state, cell))
            .collect(),
        columns: ColumnId::ALL
            .into_iter()
            .map(|column| column_summary(state, column))
            .collect(),
        active_seat: state
            .terminal_outcome
            .is_none()
            .then_some(state.active_seat),
        ply_count: state.ply_count,
        status_label: status_label(&terminal, state.active_seat),
        freshness_token: state.freshness_token,
        legal_targets,
        terminal,
        private_view: PrivateView {
            status: "not_applicable_perfect_information".to_owned(),
            hidden_fields: Vec::new(),
        },
        replay_step_index: None,
    }
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};rows={};columns={};cells={};column_summaries={};active={};ply={};status={};freshness={};legal={};terminal={};private={};hidden={};replay={}",
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
            self.columns
                .iter()
                .map(encode_column)
                .collect::<Vec<_>>()
                .join(","),
            self.active_seat.map_or("none", ColumnFourSeat::as_str),
            self.ply_count,
            self.status_label,
            self.freshness_token.0,
            self.legal_targets
                .iter()
                .map(encode_legal_target)
                .collect::<Vec<_>>()
                .join(","),
            encode_terminal(&self.terminal),
            self.private_view.status,
            self.private_view.hidden_fields.join(","),
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

fn legal_targets(state: &ColumnFourState) -> Vec<LegalColumnTargetView> {
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
        .filter_map(|choice| {
            let column = choice
                .metadata
                .iter()
                .find(|entry| entry.key == "column")
                .and_then(|entry| ColumnId::parse(&entry.value))?;
            Some(LegalColumnTargetView {
                column,
                action_segment: choice.segment,
                label: choice.label,
                accessibility_label: choice.accessibility_label,
                freshness_token: state.freshness_token,
                landing_preview: landing_cell(state, column)?,
            })
        })
        .collect()
}

fn column_summary(state: &ColumnFourState, column: ColumnId) -> ColumnSummaryView {
    let metadata = column_control(column);
    let landing_preview = landing_cell(state, column);
    let legal_action_segment = legal_columns(state)
        .contains(&column)
        .then(|| format!("drop/{}", column.as_str()));

    ColumnSummaryView {
        column,
        column_id: column.as_str().to_owned(),
        label: metadata.label,
        is_full: landing_preview.is_none(),
        legal_action_segment,
        landing_preview,
    }
}

fn cell_view(state: &ColumnFourState, cell: CellId) -> CellView {
    let layout = cell_layout(cell);
    match state.occupancy(cell) {
        CellOccupancy::Empty => CellView {
            cell,
            row: layout.row,
            column: layout.column,
            occupancy: "empty".to_owned(),
            owner: None,
            piece_token_key: None,
            piece_shape_label: None,
        },
        CellOccupancy::Occupied(owner) => {
            let token = piece_token(owner);
            CellView {
                cell,
                row: layout.row,
                column: layout.column,
                occupancy: "occupied".to_owned(),
                owner: Some(owner),
                piece_token_key: Some(token.token_key.to_owned()),
                piece_shape_label: Some(token.shape_label.to_owned()),
            }
        }
    }
}

fn status_label(terminal: &TerminalView, active_seat: ColumnFourSeat) -> String {
    match terminal {
        TerminalView::NonTerminal => format!("{} to drop", active_seat.as_str()),
        TerminalView::Win { winning_seat, .. } => format!("{} wins", winning_seat.as_str()),
        TerminalView::Draw { .. } => "draw".to_owned(),
    }
}

fn encode_cell(cell: &CellView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        cell.cell.as_string(),
        cell.row,
        cell.column,
        cell.occupancy,
        cell.owner.map_or("none", ColumnFourSeat::as_str),
        cell.piece_token_key.as_deref().unwrap_or("none"),
        cell.piece_shape_label.as_deref().unwrap_or("none")
    )
}

fn encode_column(column: &ColumnSummaryView) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        column.column_id,
        column.label,
        column.is_full,
        column.legal_action_segment.as_deref().unwrap_or("none"),
        column
            .landing_preview
            .map_or_else(|| "none".to_owned(), CellId::as_string)
    )
}

fn encode_legal_target(target: &LegalColumnTargetView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        target.column.as_str(),
        target.action_segment,
        target.label,
        target.accessibility_label,
        target.freshness_token.0,
        target.landing_preview.as_string()
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Draw { rationale } => format!("draw:{}", encode_rationale(rationale)),
        TerminalView::Win {
            winning_seat,
            line,
            rationale,
        } => format!(
            "win:{}:{}:{}",
            winning_seat.as_str(),
            line.iter()
                .map(|cell| cell.as_string())
                .collect::<Vec<_>>()
                .join("-"),
            encode_rationale(rationale)
        ),
    }
}

fn line_win_rationale(line: [CellId; 4]) -> OutcomeRationaleView {
    OutcomeRationaleView {
        result_kind: "win".to_owned(),
        decisive_cause: "line_completed".to_owned(),
        template_key: "column_four.line_completed".to_owned(),
        decisive_rule_ids: vec![
            "CF-SCORE-001".to_owned(),
            terminal_rule_for_line(line).to_owned(),
        ],
        line_cells: line.into_iter().collect(),
        line_orientation: Some(line_orientation(line).to_owned()),
        board_full: false,
    }
}

fn draw_rationale() -> OutcomeRationaleView {
    OutcomeRationaleView {
        result_kind: "draw".to_owned(),
        decisive_cause: "full_board_no_line".to_owned(),
        template_key: "column_four.full_board_draw".to_owned(),
        decisive_rule_ids: vec!["CF-SCORE-001".to_owned(), "CF-END-005".to_owned()],
        line_cells: Vec::new(),
        line_orientation: None,
        board_full: true,
    }
}

fn terminal_rule_for_line(line: [CellId; 4]) -> &'static str {
    match line_orientation(line) {
        "horizontal" => "CF-END-001",
        "vertical" => "CF-END-002",
        "rising_diagonal" => "CF-END-003",
        "falling_diagonal" => "CF-END-004",
        _ => unreachable!("line orientation is one of the documented Column Four directions"),
    }
}

fn line_orientation(line: [CellId; 4]) -> &'static str {
    let rows = line.map(|cell| cell.row.index());
    let columns = line.map(|cell| cell.column.index());
    if rows[0] == rows[1] && rows[1] == rows[2] && rows[2] == rows[3] {
        "horizontal"
    } else if columns[0] == columns[1] && columns[1] == columns[2] && columns[2] == columns[3] {
        "vertical"
    } else if rows[0] < rows[3] {
        "rising_diagonal"
    } else {
        "falling_diagonal"
    }
}

fn encode_rationale(rationale: &OutcomeRationaleView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        rationale.result_kind,
        rationale.decisive_cause,
        rationale.template_key,
        rationale.decisive_rule_ids.join("+"),
        rationale
            .line_cells
            .iter()
            .map(|cell| cell.as_string())
            .collect::<Vec<_>>()
            .join("-"),
        rationale.line_orientation.as_deref().unwrap_or("none"),
        rationale.board_full
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rules::{apply_action, validate_command, ValidatedAction},
        setup::{setup_match, SetupOptions},
        state::WinningLine,
        RowId,
    };
    use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed};

    fn seats() -> Vec<SeatId> {
        vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
    }

    fn viewer() -> Viewer {
        Viewer { seat_id: None }
    }

    fn state() -> ColumnFourState {
        setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()
    }

    fn command(state: &ColumnFourState, seat_index: usize, segment: &str) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: seats()[seat_index].clone(),
            },
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn place(state: &mut ColumnFourState, seat_index: usize, segment: &str) {
        let action = validate_command(state, &command(state, seat_index, segment)).unwrap();
        apply_action(state, action);
    }

    fn cell(row: RowId, column: ColumnId) -> CellId {
        CellId::new(row, column)
    }

    #[test]
    fn public_view_contains_required_fields_and_column_previews() {
        let mut state = state();
        place(&mut state, 0, "drop/c4");

        let view = project_view(&state, &viewer());

        assert_eq!(view.game_id, "column_four");
        assert_eq!(view.display_name, "Column Four");
        assert_eq!(view.variant_id, "column_four_standard");
        assert_eq!(view.rules_version_label, "column_four-rules-v1");
        assert_eq!(view.board_rows, 6);
        assert_eq!(view.board_columns, 7);
        assert_eq!(view.cells.len(), 42);
        assert_eq!(view.columns.len(), 7);
        assert_eq!(view.active_seat, Some(ColumnFourSeat::Seat1));
        assert_eq!(view.ply_count, 1);
        assert_eq!(view.terminal, TerminalView::NonTerminal);

        let landed = view
            .cells
            .iter()
            .find(|view_cell| view_cell.cell == cell(RowId::R1, ColumnId::C4))
            .unwrap();
        assert_eq!(landed.occupancy, "occupied");
        assert_eq!(landed.owner, Some(ColumnFourSeat::Seat0));
        assert_eq!(landed.piece_token_key.as_deref(), Some("first_piece_ring"));
        assert_eq!(landed.piece_shape_label.as_deref(), Some("ring piece"));

        let column_four = view
            .columns
            .iter()
            .find(|column| column.column == ColumnId::C4)
            .unwrap();
        assert_eq!(column_four.label, "Column 4");
        assert!(!column_four.is_full);
        assert_eq!(column_four.legal_action_segment.as_deref(), Some("drop/c4"));
        assert_eq!(
            column_four.landing_preview,
            Some(cell(RowId::R2, ColumnId::C4))
        );
    }

    #[test]
    fn legal_targets_equal_rule_engine_legal_columns() {
        let mut state = state();
        for _ in 0..6 {
            let actor = state.active_seat;
            apply_action(
                &mut state,
                ValidatedAction {
                    actor,
                    column: ColumnId::C1,
                },
            );
        }

        let view = project_view(&state, &viewer());
        let target_columns = view
            .legal_targets
            .iter()
            .map(|target| target.column)
            .collect::<Vec<_>>();

        assert_eq!(target_columns, legal_columns(&state));
        assert!(!target_columns.contains(&ColumnId::C1));
        assert!(view
            .legal_targets
            .iter()
            .all(|target| target.freshness_token == state.freshness_token));
    }

    #[test]
    fn terminal_win_and_draw_are_projected_without_hidden_fields() {
        let mut win_state = state();
        win_state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: ColumnFourSeat::Seat0,
            line: WinningLine {
                cells: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R1, ColumnId::C2),
                    cell(RowId::R1, ColumnId::C3),
                    cell(RowId::R1, ColumnId::C4),
                ],
            },
        });
        let win_view = project_view(&win_state, &viewer());
        assert_eq!(
            win_view.terminal,
            TerminalView::Win {
                winning_seat: ColumnFourSeat::Seat0,
                line: [
                    cell(RowId::R1, ColumnId::C1),
                    cell(RowId::R1, ColumnId::C2),
                    cell(RowId::R1, ColumnId::C3),
                    cell(RowId::R1, ColumnId::C4)
                ],
                rationale: OutcomeRationaleView {
                    result_kind: "win".to_owned(),
                    decisive_cause: "line_completed".to_owned(),
                    template_key: "column_four.line_completed".to_owned(),
                    decisive_rule_ids: vec!["CF-SCORE-001".to_owned(), "CF-END-001".to_owned()],
                    line_cells: vec![
                        cell(RowId::R1, ColumnId::C1),
                        cell(RowId::R1, ColumnId::C2),
                        cell(RowId::R1, ColumnId::C3),
                        cell(RowId::R1, ColumnId::C4)
                    ],
                    line_orientation: Some("horizontal".to_owned()),
                    board_full: false,
                },
            }
        );
        assert!(win_view
            .stable_summary()
            .contains("column_four.line_completed"));
        assert_eq!(win_view.active_seat, None);
        assert!(win_view.legal_targets.is_empty());

        let mut draw_state = state();
        draw_state.terminal_outcome = Some(TerminalOutcome::Draw);
        let draw_view = project_view(&draw_state, &viewer());
        assert_eq!(
            draw_view.terminal,
            TerminalView::Draw {
                rationale: OutcomeRationaleView {
                    result_kind: "draw".to_owned(),
                    decisive_cause: "full_board_no_line".to_owned(),
                    template_key: "column_four.full_board_draw".to_owned(),
                    decisive_rule_ids: vec!["CF-SCORE-001".to_owned(), "CF-END-005".to_owned()],
                    line_cells: Vec::new(),
                    line_orientation: None,
                    board_full: true,
                }
            }
        );
        assert_eq!(draw_view.status_label, "draw");
        assert_eq!(
            draw_view.private_view.status,
            "not_applicable_perfect_information"
        );
        assert!(draw_view.private_view.hidden_fields.is_empty());
        assert!(!draw_view.stable_summary().contains("debug"));
        assert!(!draw_view.stable_summary().contains("candidate"));
        assert!(!draw_view.stable_summary().contains("internal"));
    }

    #[test]
    fn stable_order_is_bottom_row_then_left_to_right_and_columns_left_to_right() {
        let view = project_view(&state(), &viewer());

        assert_eq!(
            view.cells.first().unwrap().cell,
            cell(RowId::R1, ColumnId::C1)
        );
        assert_eq!(view.cells[6].cell, cell(RowId::R1, ColumnId::C7));
        assert_eq!(view.cells[7].cell, cell(RowId::R2, ColumnId::C1));
        assert_eq!(
            view.cells.last().unwrap().cell,
            cell(RowId::R6, ColumnId::C7)
        );
        assert_eq!(view.columns.first().unwrap().column, ColumnId::C1);
        assert_eq!(view.columns.last().unwrap().column, ColumnId::C7);
    }
}
