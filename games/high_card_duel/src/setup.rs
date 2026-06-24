use engine_core::{DeterministicRng, Diagnostic, FreshnessToken, SeatId, Seed, SeededRng};
use game_stdlib::SeatCount;

use crate::{
    ids::{canonical_deck, CardId, HighCardDuelSeat},
    state::{HighCardDuelState, Phase, Score},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::high_card_duel_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<HighCardDuelState, Diagnostic> {
    if SeatCount::new(seats.len()).map(SeatCount::get) != Ok(options.variant.seat_count as usize) {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "high_card_duel requires exactly two seats".to_owned(),
        });
    }

    let mut rng = SeededRng::from_seed(seed);
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, &mut rng);

    let mut hands = [Vec::new(), Vec::new()];
    for _ in 0..options.variant.hand_size {
        for hand in &mut hands {
            let card = deck.pop().ok_or_else(|| Diagnostic {
                code: "invalid_deck_exhausted".to_owned(),
                message: "high_card_duel setup deck exhausted during initial deal".to_owned(),
            })?;
            hand.push(card);
        }
    }

    Ok(HighCardDuelState {
        variant: options.variant.clone(),
        seats: [seats[0].clone(), seats[1].clone()],
        round_number: 1,
        phase: Phase::LeadCommit,
        lead_seat: HighCardDuelSeat::Seat0,
        score: Score::default(),
        hands,
        commitments: [None, None],
        revealed_history: Vec::new(),
        deck,
        terminal_outcome: None,
        freshness_token: FreshnessToken(0),
    })
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [CardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index = rng
            .next_index_unbiased_v1(index + 1)
            .expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRng {
        values: Vec<u64>,
        draws: usize,
    }

    impl FixedRng {
        fn new(values: Vec<u64>) -> Self {
            Self { values, draws: 0 }
        }
    }

    impl DeterministicRng for FixedRng {
        fn next_u64(&mut self) -> u64 {
            self.draws += 1;
            self.values.remove(0)
        }
    }

    fn seat_ids(count: usize) -> Vec<SeatId> {
        (0..count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect()
    }

    fn invalid_seat_count_diagnostic() -> Diagnostic {
        Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "high_card_duel requires exactly two seats".to_owned(),
        }
    }

    #[test]
    fn setup_accepts_exact_variant_seat_count() {
        let seats = seat_ids(2);
        let state =
            setup_match(Seed(11), &seats, &SetupOptions::default()).expect("setup succeeds");

        assert_eq!(state.seats, [seats[0].clone(), seats[1].clone()]);
    }

    #[test]
    fn setup_rejects_non_two_seat_counts_with_exact_diagnostic() {
        for count in [0, 1, 3] {
            assert_eq!(
                setup_match(Seed(11), &seat_ids(count), &SetupOptions::default()),
                Err(invalid_seat_count_diagnostic()),
                "{count}"
            );
        }
    }

    #[test]
    fn bounded_index_rejects_high_residue_band() {
        let range = u128::from(u64::MAX) + 1;
        let accepted_zone_for_three = range - (range % 3);
        let rejected = accepted_zone_for_three as u64;
        let mut rng = FixedRng::new(vec![rejected, 4]);

        assert_eq!(rng.next_index_unbiased_v1(3), Some(1));
        assert_eq!(rng.draws, 2);
    }

    #[test]
    fn bounded_index_rejects_empty_bound() {
        let mut rng = FixedRng::new(vec![0]);

        assert_eq!(rng.next_index_unbiased_v1(0), None);
        assert_eq!(rng.draws, 0);
    }
}
