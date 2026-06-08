use engine_core::{Diagnostic, SeatId};

use crate::{
    ids::{DraftItemId, STANDARD_SEAT_COUNT},
    state::SecretDraftState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::secret_draft_standard(),
        }
    }
}

pub fn setup_match(
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<SecretDraftState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "secret_draft requires exactly two seats".to_owned(),
        });
    }

    Ok(SecretDraftState::new_with_empty_commitments(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        DraftItemId::ALL.to_vec(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let options = SetupOptions::default();

        assert!(setup_match(&[SeatId("seat_0".to_owned())], &options).is_err());
    }

    #[test]
    fn setup_starts_with_empty_commitments_and_stable_pool() {
        let options = SetupOptions::default();
        let state = setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &options,
        )
        .expect("setup succeeds");

        assert_eq!(state.visible_pool, DraftItemId::ALL);
        assert!(state.empty_commitments());
        assert_eq!(state.round_number, 1);
        assert_eq!(state.priority_seat, crate::ids::SecretDraftSeat::Seat0);
        assert_eq!(state.scores, [0, 0]);
        assert_eq!(state.fallback_awards, [0, 0]);
        assert_eq!(state.priority_conflict_wins, [0, 0]);
        assert!(state.terminal_outcome.is_none());
    }

    #[test]
    fn setup_is_deterministic() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
        let first = setup_match(&seats, &options).expect("first setup succeeds");
        let second = setup_match(&seats, &options).expect("second setup succeeds");

        assert_eq!(first, second);
        assert_eq!(first.stable_summary(), second.stable_summary());
    }
}
