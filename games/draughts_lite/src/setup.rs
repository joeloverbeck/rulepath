use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};
use game_stdlib::board_space::Coord;
use game_stdlib::SeatCountRange;

use crate::{
    ids::{
        board_dimensions, is_playable_cell, DraughtsLiteSeat, PieceId, STANDARD_PIECES_PER_SEAT,
        VARIANT_ID,
    },
    state::{sorted_pieces, CellOccupancy, DraughtsLiteState, Piece, PieceKind},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::draughts_lite_standard(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<DraughtsLiteState, Diagnostic> {
    if options.variant.id != VARIANT_ID {
        return Err(Diagnostic {
            code: "unsupported_variant".to_owned(),
            message: "draughts_lite supports only draughts_lite_standard".to_owned(),
        });
    }

    let seat_count = options.variant.seat_count as usize;
    if SeatCountRange::inclusive(seat_count, seat_count)
        .expect("standard draughts_lite seat count is nonzero")
        .validate(seats.len())
        .is_err()
    {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "draughts_lite requires exactly two seats".to_owned(),
        });
    }

    let board = board_dimensions();
    let mut cells = DraughtsLiteState::empty_cells();
    let mut pieces = Vec::with_capacity(24);

    place_home_rows(
        board,
        &mut cells,
        &mut pieces,
        DraughtsLiteSeat::Seat0,
        1..=3,
    );
    place_home_rows(
        board,
        &mut cells,
        &mut pieces,
        DraughtsLiteSeat::Seat1,
        6..=8,
    );

    Ok(DraughtsLiteState {
        variant: options.variant.clone(),
        board,
        cells,
        pieces: sorted_pieces(pieces),
        active_seat: DraughtsLiteSeat::from_index(options.variant.first_seat as usize).ok_or_else(
            || Diagnostic {
                code: "invalid_first_seat".to_owned(),
                message: "first seat is outside the declared seats".to_owned(),
            },
        )?,
        seats: [seats[0].clone(), seats[1].clone()],
        ply_count: 0,
        command_count: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: FreshnessToken(0),
    })
}

fn place_home_rows(
    board: game_stdlib::board_space::Dimensions,
    cells: &mut [CellOccupancy; 64],
    pieces: &mut Vec<Piece>,
    owner: DraughtsLiteSeat,
    rows: std::ops::RangeInclusive<u8>,
) {
    let mut ordinal = 1;
    for row in rows {
        for col in 1..=board.cols() {
            let cell = Coord::checked(row, col).expect("home-row cell is one-based");
            if !is_playable_cell(cell) {
                continue;
            }
            let id = PieceId::new(owner, ordinal).expect("standard setup has 12 pieces per seat");
            cells[cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(id);
            pieces.push(Piece {
                id,
                owner,
                kind: PieceKind::Man,
                cell,
            });
            ordinal += 1;
        }
    }
    debug_assert_eq!(ordinal - 1, STANDARD_PIECES_PER_SEAT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_deterministic_standard_opening_first_seat() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let options = SetupOptions::default();

        let left = setup_match(Seed(1), &seats, &options).expect("setup succeeds");
        let right = setup_match(Seed(99), &seats, &options).expect("setup ignores seed");

        assert_eq!(left, right);
        assert_eq!(left.board.rows(), 8);
        assert_eq!(left.board.cols(), 8);
        assert_eq!(left.cells.len(), 64);
        assert_eq!(left.pieces.len(), 24);
        assert_eq!(left.active_seat, DraughtsLiteSeat::Seat0);
        assert_eq!(left.ply_count, 0);
        assert_eq!(left.command_count, 0);
        assert_eq!(left.terminal_outcome, None);
        assert_eq!(left.freshness_token, FreshnessToken(0));
    }

    #[test]
    fn setup_places_twelve_men_per_seat_on_playable_home_rows() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();

        for seat in DraughtsLiteSeat::ALL {
            let pieces: Vec<_> = state.pieces_for_seat(seat).collect();
            assert_eq!(pieces.len(), 12);
            assert!(pieces.iter().all(|piece| piece.kind == PieceKind::Man));
            assert!(pieces.iter().all(|piece| is_playable_cell(piece.cell)));
        }

        for piece in state.pieces_for_seat(DraughtsLiteSeat::Seat0) {
            assert!((1..=3).contains(&piece.cell.row()));
        }
        for piece in state.pieces_for_seat(DraughtsLiteSeat::Seat1) {
            assert!((6..=8).contains(&piece.cell.row()));
        }

        assert_eq!(
            state.occupancy(Coord::checked(1, 2).unwrap()),
            Some(CellOccupancy::Occupied(
                PieceId::new(DraughtsLiteSeat::Seat0, 1).unwrap()
            ))
        );
        assert_eq!(
            state.occupancy(Coord::checked(8, 7).unwrap()),
            Some(CellOccupancy::Occupied(
                PieceId::new(DraughtsLiteSeat::Seat1, 12).unwrap()
            ))
        );
    }

    #[test]
    fn setup_leaves_middle_rows_and_unplayable_cells_empty() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();

        for cell in state.board.row_major() {
            let occupancy = state.occupancy(cell).unwrap();
            if (4..=5).contains(&cell.row()) || !is_playable_cell(cell) {
                assert_eq!(occupancy, CellOccupancy::Empty, "cell {}", cell.id());
            }
        }
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let seats = vec![SeatId("seat-0".to_owned())];

        let diagnostic = setup_match(Seed(1), &seats, &SetupOptions::default())
            .expect_err("setup rejects missing seat");

        assert_eq!(diagnostic.code, "invalid_seat_count");
        assert_eq!(
            diagnostic.message,
            "draughts_lite requires exactly two seats"
        );
    }

    #[test]
    fn setup_rejects_unknown_variant() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let mut options = SetupOptions::default();
        options.variant.id = "other".to_owned();

        let diagnostic = setup_match(Seed(1), &seats, &options).expect_err("setup rejects variant");

        assert_eq!(diagnostic.code, "unsupported_variant");
    }
}
