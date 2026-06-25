use engine_core::{Diagnostic, SeatId, Seed};
use game_stdlib::SeatCountRange;

use crate::{
    ids::{BlackglassSeat, STANDARD_SEAT_COUNT},
    state::BlackglassPactState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::blackglass_pact_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<BlackglassPactState, Diagnostic> {
    validate_standard_seat_count(seats.len())?;

    Ok(BlackglassPactState::new_admitted_setup(
        options.variant.clone(),
        [
            seats[0].clone(),
            seats[1].clone(),
            seats[2].clone(),
            seats[3].clone(),
        ],
        BlackglassSeat::North,
        0,
        seed,
    ))
}

pub fn validate_standard_seat_count(actual: usize) -> Result<(), Diagnostic> {
    SeatCountRange::inclusive(STANDARD_SEAT_COUNT as usize, STANDARD_SEAT_COUNT as usize)
        .expect("standard Blackglass seat count range is valid")
        .validate(actual)
        .map(|_| ())
        .map_err(|_| invalid_seat_count_diagnostic(actual))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "BP_UNSUPPORTED_SEAT_COUNT".to_owned(),
        message: format!("blackglass_pact requires exactly four seats; received {actual}"),
    }
}
