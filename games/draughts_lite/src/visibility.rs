use engine_core::{FreshnessToken, StableSerialize, Viewer};
use game_stdlib::board_space::Coord;

use crate::{
    ids::{DraughtsLiteSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{CellOccupancy, DraughtsLiteState, PieceKind, TerminalOutcome},
    ui::{board_presentation, cell_layout, piece_label, piece_token},
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
    pub active_seat: Option<DraughtsLiteSeat>,
    pub ply_count: u32,
    pub command_count: u32,
    pub status_label: String,
    pub freshness_token: FreshnessToken,
    pub terminal: TerminalView,
    pub private_view: PrivateView,
    pub ui: UiMetadata,
    pub replay_step_index: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellView {
    pub cell: Coord,
    pub cell_id: String,
    pub row: u8,
    pub column: u8,
    pub playable: bool,
    pub presentation_token: String,
    pub accessibility_label: String,
    pub occupancy: String,
    pub owner: Option<DraughtsLiteSeat>,
    pub piece_id: Option<String>,
    pub piece_kind: Option<PieceKind>,
    pub piece_token_key: Option<String>,
    pub piece_shape_label: Option<String>,
    pub piece_pattern_label: Option<String>,
    pub piece_label: Option<String>,
    pub piece_accessibility_label: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win { winning_seat: DraughtsLiteSeat },
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
    pub playable_cell_token: String,
    pub non_playable_cell_token: String,
    pub first_man_token_key: String,
    pub first_man_shape_label: String,
    pub first_crown_token_key: String,
    pub first_crown_shape_label: String,
    pub second_man_token_key: String,
    pub second_man_shape_label: String,
    pub second_crown_token_key: String,
    pub second_crown_shape_label: String,
}

pub fn project_view(state: &DraughtsLiteState, _viewer: &Viewer) -> PublicView {
    let terminal = terminal_view(state.terminal_outcome);

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Draughts Lite".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        board_rows: state.board.rows(),
        board_columns: state.board.cols(),
        cells: state
            .board
            .row_major()
            .map(|cell| cell_view(state, cell))
            .collect(),
        active_seat: state
            .terminal_outcome
            .is_none()
            .then_some(state.active_seat),
        ply_count: state.ply_count,
        command_count: state.command_count,
        status_label: status_label(&terminal, state.active_seat),
        freshness_token: state.freshness_token,
        terminal,
        private_view: PrivateView {
            status: "not_applicable_perfect_information".to_owned(),
            hidden_fields: Vec::new(),
        },
        ui: ui_metadata(),
        replay_step_index: None,
    }
}

impl PublicView {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};rules={};game={};variant={};label={};rows={};columns={};cells={};active={};ply={};commands={};status={};freshness={};terminal={};private={};hidden={};ui={};replay={}",
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
            self.active_seat.map_or("none", DraughtsLiteSeat::as_str),
            self.ply_count,
            self.command_count,
            self.status_label,
            self.freshness_token.0,
            encode_terminal(&self.terminal),
            self.private_view.status,
            self.private_view.hidden_fields.join(","),
            encode_ui(&self.ui),
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

fn cell_view(state: &DraughtsLiteState, cell: Coord) -> CellView {
    let layout = cell_layout(cell);
    match state
        .occupancy(cell)
        .expect("view cell is inside the board")
    {
        CellOccupancy::Empty => CellView {
            cell,
            cell_id: layout.cell_id,
            row: layout.row,
            column: layout.column,
            playable: layout.playable,
            presentation_token: layout.presentation_token.to_owned(),
            accessibility_label: layout.accessibility_label,
            occupancy: "empty".to_owned(),
            owner: None,
            piece_id: None,
            piece_kind: None,
            piece_token_key: None,
            piece_shape_label: None,
            piece_pattern_label: None,
            piece_label: None,
            piece_accessibility_label: None,
        },
        CellOccupancy::Occupied(piece_id) => {
            let piece = *state
                .piece(piece_id)
                .expect("occupied view cell references a live piece");
            let token = piece_token(piece.owner, piece.kind);
            let label = piece_label(piece);
            CellView {
                cell,
                cell_id: layout.cell_id,
                row: layout.row,
                column: layout.column,
                playable: layout.playable,
                presentation_token: layout.presentation_token.to_owned(),
                accessibility_label: layout.accessibility_label,
                occupancy: "occupied".to_owned(),
                owner: Some(piece.owner),
                piece_id: Some(piece.id.stable_id()),
                piece_kind: Some(piece.kind),
                piece_token_key: Some(token.token_key.to_owned()),
                piece_shape_label: Some(token.shape_label.to_owned()),
                piece_pattern_label: Some(token.pattern_label.to_owned()),
                piece_label: Some(label.label),
                piece_accessibility_label: Some(label.accessibility_label),
            }
        }
    }
}

fn terminal_view(outcome: Option<TerminalOutcome>) -> TerminalView {
    match outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Win { seat }) => TerminalView::Win { winning_seat: seat },
    }
}

fn ui_metadata() -> UiMetadata {
    let board = board_presentation();
    let first_man = piece_token(DraughtsLiteSeat::Seat0, PieceKind::Man);
    let first_crown = piece_token(DraughtsLiteSeat::Seat0, PieceKind::Crown);
    let second_man = piece_token(DraughtsLiteSeat::Seat1, PieceKind::Man);
    let second_crown = piece_token(DraughtsLiteSeat::Seat1, PieceKind::Crown);

    UiMetadata {
        board_label: board.board_label.to_owned(),
        row_count: board.row_count,
        column_count: board.column_count,
        playable_cell_token: board.playable_cell_token.to_owned(),
        non_playable_cell_token: board.non_playable_cell_token.to_owned(),
        first_man_token_key: first_man.token_key.to_owned(),
        first_man_shape_label: first_man.shape_label.to_owned(),
        first_crown_token_key: first_crown.token_key.to_owned(),
        first_crown_shape_label: first_crown.shape_label.to_owned(),
        second_man_token_key: second_man.token_key.to_owned(),
        second_man_shape_label: second_man.shape_label.to_owned(),
        second_crown_token_key: second_crown.token_key.to_owned(),
        second_crown_shape_label: second_crown.shape_label.to_owned(),
    }
}

fn status_label(terminal: &TerminalView, active_seat: DraughtsLiteSeat) -> String {
    match terminal {
        TerminalView::NonTerminal => format!("{} to move", active_seat.as_str()),
        TerminalView::Win { winning_seat } => format!("{} wins", winning_seat.as_str()),
    }
}

fn encode_cell(cell: &CellView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        cell.cell_id,
        cell.row,
        cell.column,
        cell.playable,
        cell.presentation_token,
        cell.accessibility_label,
        cell.occupancy,
        cell.owner.map_or("none", DraughtsLiteSeat::as_str),
        cell.piece_id.as_deref().unwrap_or("none"),
        cell.piece_kind.map_or("none", PieceKind::as_str),
        cell.piece_token_key.as_deref().unwrap_or("none"),
        cell.piece_shape_label.as_deref().unwrap_or("none"),
        cell.piece_pattern_label.as_deref().unwrap_or("none"),
        cell.piece_label.as_deref().unwrap_or("none"),
        cell.piece_accessibility_label.as_deref().unwrap_or("none")
    )
}

fn encode_terminal(terminal: &TerminalView) -> String {
    match terminal {
        TerminalView::NonTerminal => "non_terminal".to_owned(),
        TerminalView::Win { winning_seat } => format!("win:{}", winning_seat.as_str()),
    }
}

fn encode_ui(ui: &UiMetadata) -> String {
    format!(
        "{}|{}x{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ui.board_label,
        ui.row_count,
        ui.column_count,
        ui.playable_cell_token,
        ui.non_playable_cell_token,
        ui.first_man_token_key,
        ui.first_man_shape_label,
        ui.first_crown_token_key,
        ui.first_crown_shape_label,
        ui.second_man_token_key,
        ui.second_man_shape_label,
        ui.second_crown_token_key,
        ui.second_crown_shape_label
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        setup::{setup_match, SetupOptions},
        PieceId,
    };
    use engine_core::{SeatId, Seed};

    fn seats() -> Vec<SeatId> {
        vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
    }

    fn viewer() -> Viewer {
        Viewer { seat_id: None }
    }

    fn state() -> DraughtsLiteState {
        setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap()
    }

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    #[test]
    fn public_view_contains_all_cells_pieces_and_ui_metadata() {
        let view = project_view(&state(), &viewer());

        assert_eq!(view.game_id, "draughts_lite");
        assert_eq!(view.display_name, "Draughts Lite");
        assert_eq!(view.variant_id, "draughts_lite_standard");
        assert_eq!(view.rules_version_label, "draughts_lite-rules-v1");
        assert_eq!(view.board_rows, 8);
        assert_eq!(view.board_columns, 8);
        assert_eq!(view.cells.len(), 64);
        assert_eq!(view.cells.iter().filter(|cell| cell.playable).count(), 32);
        assert_eq!(
            view.cells
                .iter()
                .filter(|cell| cell.occupancy == "occupied")
                .count(),
            24
        );
        assert_eq!(view.active_seat, Some(DraughtsLiteSeat::Seat0));
        assert_eq!(view.terminal, TerminalView::NonTerminal);
        assert_eq!(view.ui.board_label, "Draughts Lite board");
        assert_eq!(view.ui.first_man_token_key, "first_man_ring");
        assert_eq!(view.ui.second_crown_shape_label, "star-cross crowned piece");

        let occupied = view
            .cells
            .iter()
            .find(|cell| cell.cell == coord(1, 2))
            .unwrap();
        assert_eq!(occupied.occupancy, "occupied");
        assert_eq!(occupied.owner, Some(DraughtsLiteSeat::Seat0));
        let first_piece_id = PieceId::new(DraughtsLiteSeat::Seat0, 1)
            .unwrap()
            .stable_id();
        assert_eq!(occupied.piece_id.as_deref(), Some(first_piece_id.as_str()));
        assert_eq!(occupied.piece_kind, Some(PieceKind::Man));
        assert_eq!(occupied.piece_shape_label.as_deref(), Some("ring man"));
        assert_eq!(
            occupied.piece_accessibility_label.as_deref(),
            Some("seat_0 man at r1c2")
        );

        let non_playable = view
            .cells
            .iter()
            .find(|cell| cell.cell == coord(1, 1))
            .unwrap();
        assert!(!non_playable.playable);
        assert_eq!(non_playable.occupancy, "empty");
        assert_eq!(non_playable.presentation_token, "non_playable_light_square");
    }

    #[test]
    fn perfect_information_private_view_has_no_hidden_fields_or_internals() {
        let view = project_view(&state(), &viewer());

        assert_eq!(
            view.private_view.status,
            "not_applicable_perfect_information"
        );
        assert!(view.private_view.hidden_fields.is_empty());

        let summary = view.stable_summary();
        assert!(!summary.contains("debug"));
        assert!(!summary.contains("candidate"));
        assert!(!summary.contains("internal"));
        assert!(!summary.contains("seed"));
        assert!(!summary.contains("rng"));
    }

    #[test]
    fn terminal_win_is_projected_without_active_seat_or_legal_state() {
        let mut state = state();
        state.terminal_outcome = Some(TerminalOutcome::Win {
            seat: DraughtsLiteSeat::Seat1,
        });

        let view = project_view(&state, &viewer());

        assert_eq!(
            view.terminal,
            TerminalView::Win {
                winning_seat: DraughtsLiteSeat::Seat1
            }
        );
        assert_eq!(view.active_seat, None);
        assert_eq!(view.status_label, "seat_1 wins");
    }

    #[test]
    fn stable_order_is_row_major_across_all_sixty_four_cells() {
        let view = project_view(&state(), &viewer());

        assert_eq!(view.cells.first().unwrap().cell, coord(1, 1));
        assert_eq!(view.cells[7].cell, coord(1, 8));
        assert_eq!(view.cells[8].cell, coord(2, 1));
        assert_eq!(view.cells.last().unwrap().cell, coord(8, 8));
    }
}
