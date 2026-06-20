use engine_core::{Diagnostic, SeatId, Seed};

use crate::{ids::STANDARD_SEAT_COUNT, state::BriarCircuitState, variants::Variant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::briar_circuit_standard(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<BriarCircuitState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(invalid_seat_count_diagnostic(seats.len()));
    }

    Ok(BriarCircuitState::new_empty_hand(
        options.variant.clone(),
        [
            seats[0].clone(),
            seats[1].clone(),
            seats[2].clone(),
            seats[3].clone(),
        ],
    ))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "BC_UNSUPPORTED_SEAT_COUNT".to_owned(),
        message: format!("briar_circuit requires exactly four seats; received {actual}"),
    }
}
