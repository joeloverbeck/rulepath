use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};
use game_stdlib::SeatCountRange;

use crate::{
    ids::{CellId, ColumnId, DirectionalFlipSeat, RowId},
    state::{CellOccupancy, DirectionalFlipState},
    variants::{Variant, VARIANT_ID},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::directional_flip_standard(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<DirectionalFlipState, Diagnostic> {
    if options.variant.id != VARIANT_ID {
        return Err(Diagnostic {
            code: "unsupported_variant".to_owned(),
            message: "directional_flip supports only directional_flip_standard".to_owned(),
        });
    }

    let seat_count = options.variant.seat_count as usize;
    if SeatCountRange::inclusive(seat_count, seat_count)
        .expect("standard directional_flip seat count is nonzero")
        .validate(seats.len())
        .is_err()
    {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "directional_flip requires exactly two seats".to_owned(),
        });
    }

    let mut cells = DirectionalFlipState::empty_cells();
    cells[CellId::new(RowId::R4, ColumnId::C5).index()] =
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat0);
    cells[CellId::new(RowId::R5, ColumnId::C4).index()] =
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat0);
    cells[CellId::new(RowId::R4, ColumnId::C4).index()] =
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat1);
    cells[CellId::new(RowId::R5, ColumnId::C5).index()] =
        CellOccupancy::Occupied(DirectionalFlipSeat::Seat1);

    Ok(DirectionalFlipState {
        variant: options.variant.clone(),
        cells,
        active_seat: DirectionalFlipSeat::from_index(options.variant.first_seat as usize)
            .ok_or_else(|| Diagnostic {
                code: "invalid_first_seat".to_owned(),
                message: "first seat is outside the declared seats".to_owned(),
            })?,
        seats: [seats[0].clone(), seats[1].clone()],
        ply_count: 0,
        consecutive_forced_passes: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: FreshnessToken(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_deterministic_standard_opening_first_seat() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let options = SetupOptions::default();

        let left = setup_match(Seed(1), &seats, &options).expect("setup succeeds");
        let right = setup_match(Seed(1), &seats, &options).expect("setup succeeds");

        assert_eq!(left, right);
        assert_eq!(left.cells.len(), 64);
        assert_eq!(left.active_seat, DirectionalFlipSeat::Seat0);
        assert_eq!(left.ply_count, 0);
        assert_eq!(left.consecutive_forced_passes, 0);
        assert_eq!(left.terminal_outcome, None);
        assert_eq!(left.freshness_token, FreshnessToken(0));
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let seats = vec![SeatId("seat-0".to_owned())];

        let diagnostic = setup_match(Seed(1), &seats, &SetupOptions::default())
            .expect_err("setup rejects missing seat");

        assert_eq!(diagnostic.code, "invalid_seat_count");
        assert_eq!(
            diagnostic.message,
            "directional_flip requires exactly two seats"
        );
    }
}
