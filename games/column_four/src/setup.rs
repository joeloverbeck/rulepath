use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};

use crate::{
    ids::ColumnFourSeat,
    state::ColumnFourState,
    variants::{Variant, VARIANT_ID},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::column_four_standard(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<ColumnFourState, Diagnostic> {
    if options.variant.id != VARIANT_ID {
        return Err(Diagnostic {
            code: "unsupported_variant".to_owned(),
            message: "column_four supports only column_four_standard".to_owned(),
        });
    }

    if seats.len() != options.variant.seat_count as usize {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "column_four requires exactly two seats".to_owned(),
        });
    }

    Ok(ColumnFourState {
        variant: options.variant.clone(),
        cells: ColumnFourState::empty_cells(),
        active_seat: ColumnFourSeat::from_index(options.variant.first_seat as usize).ok_or_else(
            || Diagnostic {
                code: "invalid_first_seat".to_owned(),
                message: "first seat is outside the declared seats".to_owned(),
            },
        )?,
        seats: [seats[0].clone(), seats[1].clone()],
        ply_count: 0,
        freshness_token: FreshnessToken(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_deterministic_empty_board_first_seat() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let options = SetupOptions::default();

        let left = setup_match(Seed(1), &seats, &options).expect("setup succeeds");
        let right = setup_match(Seed(1), &seats, &options).expect("setup succeeds");

        assert_eq!(left, right);
        assert_eq!(left.cells.len(), 42);
        assert!(left.cells.iter().all(|cell| cell.is_empty()));
        assert_eq!(left.active_seat, ColumnFourSeat::Seat0);
        assert_eq!(left.ply_count, 0);
        assert_eq!(left.freshness_token, FreshnessToken(0));
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let seats = vec![SeatId("seat-0".to_owned())];

        let diagnostic = setup_match(Seed(1), &seats, &SetupOptions::default())
            .expect_err("setup rejects missing seat");

        assert_eq!(diagnostic.code, "invalid_seat_count");
    }
}
