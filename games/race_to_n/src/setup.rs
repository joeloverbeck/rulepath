use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};

use crate::{
    ids::RaceSeat,
    state::{CounterValue, RaceState},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::race_to_21(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<RaceState, Diagnostic> {
    if seats.len() != options.variant.seat_count as usize {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "race_to_n requires exactly two seats".to_owned(),
        });
    }

    Ok(RaceState {
        variant: options.variant.clone(),
        counter: CounterValue(0),
        active_seat: RaceSeat::from_index(options.variant.first_seat as usize).ok_or_else(
            || Diagnostic {
                code: "invalid_first_seat".to_owned(),
                message: "first seat is outside the declared seats".to_owned(),
            },
        )?,
        seats: [seats[0].clone(), seats[1].clone()],
        winner: None,
        freshness_token: FreshnessToken(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_deterministic_for_same_inputs() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let options = SetupOptions::default();

        let left = setup_match(Seed(1), &seats, &options).expect("setup succeeds");
        let right = setup_match(Seed(1), &seats, &options).expect("setup succeeds");

        assert_eq!(left, right);
        assert_eq!(left.counter, CounterValue(0));
        assert_eq!(left.active_seat, RaceSeat::Seat0);
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
