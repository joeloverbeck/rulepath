use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};
use game_stdlib::SeatCountRange;

use crate::{
    ids::{
        active_points_for_seat_count, supported_seat_count, StarPoint, StarZone,
        STANDARD_MAX_SEATS, STANDARD_MIN_SEATS, STANDARD_PEGS_PER_SEAT, VARIANT_ID,
    },
    state::{SeatAssignment, StarPeg, StarPegId, StarbridgeState},
    topology::home_spaces,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::starbridge_classic(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<StarbridgeState, Diagnostic> {
    if options.variant.id != VARIANT_ID {
        return Err(Diagnostic {
            code: "unsupported_variant".to_owned(),
            message: format!("starbridge_crossing supports only {VARIANT_ID}"),
        });
    }

    validate_seat_count(seats.len(), &options.variant)?;
    if options.variant.pegs_per_seat != STANDARD_PEGS_PER_SEAT {
        return Err(Diagnostic {
            code: "invalid_variant_peg_count".to_owned(),
            message: format!(
                "starbridge_crossing requires exactly {STANDARD_PEGS_PER_SEAT} pegs per seat"
            ),
        });
    }

    let active_points = active_points_for_seat_count(seats.len()).ok_or_else(|| Diagnostic {
        code: "invalid_seat_count".to_owned(),
        message: unsupported_seat_message(seats.len()),
    })?;
    let assignments = seats
        .iter()
        .cloned()
        .zip(active_points.iter().copied())
        .enumerate()
        .map(|(seat_index, (seat_id, home))| SeatAssignment {
            seat_id,
            seat_index: u8::try_from(seat_index).expect("supported seat count fits u8"),
            home,
            target: home.opposite(),
        })
        .collect::<Vec<_>>();

    let mut occupancy = StarbridgeState::empty_occupancy();
    let mut pegs = Vec::with_capacity(seats.len() * usize::from(STANDARD_PEGS_PER_SEAT));
    for assignment in &assignments {
        place_home_pegs(
            assignment.seat_index,
            assignment.home,
            &mut occupancy,
            &mut pegs,
        );
    }

    Ok(StarbridgeState {
        variant: options.variant.clone(),
        seats: assignments,
        occupancy,
        pegs,
        active_seat_index: 0,
        finish_ranks: Vec::new(),
        terminal_status: None,
        ply_count: 0,
        command_count: 0,
        freshness_token: FreshnessToken(0),
    })
}

fn validate_seat_count(seat_count: usize, variant: &Variant) -> Result<(), Diagnostic> {
    if SeatCountRange::inclusive(
        usize::from(STANDARD_MIN_SEATS),
        usize::from(STANDARD_MAX_SEATS),
    )
    .expect("standard starbridge seat range is valid")
    .validate(seat_count)
    .is_err()
        || !supported_seat_count(seat_count)
        || !variant.supports_seat_count(seat_count)
    {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: unsupported_seat_message(seat_count),
        });
    }
    Ok(())
}

fn unsupported_seat_message(seat_count: usize) -> String {
    format!("starbridge_crossing supports exactly 2, 3, 4, or 6 seats; got {seat_count}")
}

fn place_home_pegs(
    seat_index: u8,
    home: StarPoint,
    occupancy: &mut [Option<StarPegId>],
    pegs: &mut Vec<StarPeg>,
) {
    let mut home_spaces = home_spaces(home).collect::<Vec<_>>();
    home_spaces.sort_by_key(|space| space.id);
    debug_assert_eq!(home_spaces.len(), usize::from(STANDARD_PEGS_PER_SEAT));

    for (ordinal, space) in home_spaces.into_iter().enumerate() {
        debug_assert_eq!(space.zone, StarZone::Home(home));
        let id = StarPegId::new(
            seat_index,
            u8::try_from(ordinal).expect("standard peg ordinal fits u8"),
        );
        occupancy[usize::from(space.id.index())] = Some(id);
        pegs.push(StarPeg {
            id,
            owner_seat_index: seat_index,
            space: space.id,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_rejects_unknown_variant() {
        let seats = vec![
            SeatId::from_zero_based_index(0),
            SeatId::from_zero_based_index(1),
        ];
        let mut options = SetupOptions::default();
        options.variant.id = "other".to_owned();

        let diagnostic = setup_match(Seed(1), &seats, &options).expect_err("setup rejects variant");

        assert_eq!(diagnostic.code, "unsupported_variant");
    }
}
