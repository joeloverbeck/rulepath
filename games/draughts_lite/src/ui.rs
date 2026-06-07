use game_stdlib::board_space::Coord;

use crate::{
    ids::{is_playable_cell, DraughtsLiteSeat, BOARD_COLS, BOARD_ROWS},
    state::{Piece, PieceKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PieceTokenMetadata {
    pub seat: DraughtsLiteSeat,
    pub kind: PieceKind,
    pub token_key: &'static str,
    pub shape_label: &'static str,
    pub pattern_label: &'static str,
    pub color_role: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellLayoutMetadata {
    pub cell: Coord,
    pub cell_id: String,
    pub row: u8,
    pub column: u8,
    pub playable: bool,
    pub presentation_token: &'static str,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PieceLabelMetadata {
    pub label: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardPresentationMetadata {
    pub board_label: &'static str,
    pub row_count: u8,
    pub column_count: u8,
    pub playable_cell_token: &'static str,
    pub non_playable_cell_token: &'static str,
}

pub fn board_presentation() -> BoardPresentationMetadata {
    BoardPresentationMetadata {
        board_label: "Draughts Lite board",
        row_count: BOARD_ROWS,
        column_count: BOARD_COLS,
        playable_cell_token: "playable_dark_square",
        non_playable_cell_token: "non_playable_light_square",
    }
}

pub fn piece_token(seat: DraughtsLiteSeat, kind: PieceKind) -> PieceTokenMetadata {
    match (seat, kind) {
        (DraughtsLiteSeat::Seat0, PieceKind::Man) => PieceTokenMetadata {
            seat,
            kind,
            token_key: "first_man_ring",
            shape_label: "ring man",
            pattern_label: "single-ring pattern",
            color_role: "first-player",
        },
        (DraughtsLiteSeat::Seat0, PieceKind::Crown) => PieceTokenMetadata {
            seat,
            kind,
            token_key: "first_crown_double_ring",
            shape_label: "double-ring crowned piece",
            pattern_label: "double-ring pattern",
            color_role: "first-player",
        },
        (DraughtsLiteSeat::Seat1, PieceKind::Man) => PieceTokenMetadata {
            seat,
            kind,
            token_key: "second_man_cross",
            shape_label: "cross man",
            pattern_label: "cross-hatch pattern",
            color_role: "second-player",
        },
        (DraughtsLiteSeat::Seat1, PieceKind::Crown) => PieceTokenMetadata {
            seat,
            kind,
            token_key: "second_crown_star_cross",
            shape_label: "star-cross crowned piece",
            pattern_label: "star-cross pattern",
            color_role: "second-player",
        },
    }
}

pub fn cell_layout(cell: Coord) -> CellLayoutMetadata {
    let playable = is_playable_cell(cell);
    CellLayoutMetadata {
        cell,
        cell_id: cell.id(),
        row: cell.row(),
        column: cell.col(),
        playable,
        presentation_token: if playable {
            "playable_dark_square"
        } else {
            "non_playable_light_square"
        },
        accessibility_label: format!(
            "{} cell {}",
            if playable { "Playable" } else { "Non-playable" },
            cell.id()
        ),
    }
}

pub fn piece_label(piece: Piece) -> PieceLabelMetadata {
    let kind = match piece.kind {
        PieceKind::Man => "man",
        PieceKind::Crown => "crowned piece",
    };
    PieceLabelMetadata {
        label: format!("{} {}", piece.owner.as_str(), kind),
        accessibility_label: format!("{} {} at {}", piece.owner.as_str(), kind, piece.cell.id()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PieceId;

    fn coord(row: u8, col: u8) -> Coord {
        Coord::checked(row, col).unwrap()
    }

    #[test]
    fn cell_metadata_marks_playable_and_non_playable_cells() {
        let playable = cell_layout(coord(1, 2));
        let non_playable = cell_layout(coord(1, 1));

        assert!(playable.playable);
        assert_eq!(playable.cell_id, "r1c2");
        assert_eq!(playable.presentation_token, "playable_dark_square");
        assert!(!non_playable.playable);
        assert_eq!(non_playable.presentation_token, "non_playable_light_square");
    }

    #[test]
    fn piece_tokens_use_non_color_only_metadata() {
        let first_man = piece_token(DraughtsLiteSeat::Seat0, PieceKind::Man);
        let first_crown = piece_token(DraughtsLiteSeat::Seat0, PieceKind::Crown);
        let second_man = piece_token(DraughtsLiteSeat::Seat1, PieceKind::Man);
        let second_crown = piece_token(DraughtsLiteSeat::Seat1, PieceKind::Crown);

        assert_eq!(first_man.shape_label, "ring man");
        assert_eq!(first_crown.pattern_label, "double-ring pattern");
        assert_eq!(second_man.shape_label, "cross man");
        assert_eq!(second_crown.pattern_label, "star-cross pattern");
    }

    #[test]
    fn piece_labels_include_owner_kind_and_cell() {
        let piece = Piece {
            id: PieceId::new(DraughtsLiteSeat::Seat1, 3).unwrap(),
            owner: DraughtsLiteSeat::Seat1,
            kind: PieceKind::Crown,
            cell: coord(6, 5),
        };

        let label = piece_label(piece);

        assert_eq!(label.label, "seat_1 crowned piece");
        assert_eq!(label.accessibility_label, "seat_1 crowned piece at r6c5");
    }
}
