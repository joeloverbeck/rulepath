use engine_core::{DeterministicRng, Diagnostic, FreshnessToken, SeatId, Seed, SeededRng};

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
    if seats.len() != options.variant.seat_count as usize {
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
        freshness_token: FreshnessToken(0),
    })
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [CardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

/// Returns an unbiased index in `0..upper_bound` by rejecting the high residue
/// band above the largest multiple of `upper_bound` that fits in `u64`.
pub fn next_bounded_index_unbiased<R: DeterministicRng>(
    rng: &mut R,
    upper_bound: usize,
) -> Option<usize> {
    if upper_bound == 0 {
        return None;
    }

    let upper = upper_bound as u128;
    let range = u128::from(u64::MAX) + 1;
    let accepted_zone = range - (range % upper);

    loop {
        let value = u128::from(rng.next_u64());
        if value < accepted_zone {
            return Some((value % upper) as usize);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRng {
        values: Vec<u64>,
    }

    impl FixedRng {
        fn new(values: Vec<u64>) -> Self {
            Self { values }
        }
    }

    impl DeterministicRng for FixedRng {
        fn next_u64(&mut self) -> u64 {
            self.values.remove(0)
        }
    }

    #[test]
    fn bounded_index_rejects_high_residue_band() {
        let range = u128::from(u64::MAX) + 1;
        let accepted_zone_for_three = range - (range % 3);
        let rejected = accepted_zone_for_three as u64;
        let mut rng = FixedRng::new(vec![rejected, 4]);

        assert_eq!(next_bounded_index_unbiased(&mut rng, 3), Some(1));
    }

    #[test]
    fn bounded_index_rejects_empty_bound() {
        let mut rng = FixedRng::new(vec![0]);

        assert_eq!(next_bounded_index_unbiased(&mut rng, 0), None);
    }
}
