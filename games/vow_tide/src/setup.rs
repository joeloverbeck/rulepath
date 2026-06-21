use engine_core::{Diagnostic, SeatId, Seed};

use crate::{
    ids::{hand_schedule_for_seats, supported_seat_count, VowTideSeat},
    state::VowTideState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::vow_tide_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<VowTideState, Diagnostic> {
    if !supported_seat_count(seats.len()) {
        return Err(invalid_seat_count_diagnostic(seats.len()));
    }

    let schedule = hand_schedule_for_seats(seats.len())
        .expect("validated Vow Tide seat counts always have a schedule");

    Ok(VowTideState::new_empty_hand(
        options.variant.clone(),
        seats.to_vec(),
        schedule,
        VowTideSeat::Seat0,
        seed,
    ))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "VT_INVALID_SEAT_COUNT".to_owned(),
        message: format!("vow_tide supports 3 to 7 seats; received {actual}"),
    }
}
