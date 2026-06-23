use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};
use game_stdlib::SeatCount;

use crate::{
    ids::{canonical_masks, MaskTileId, STANDARD_HAND_SIZE, STANDARD_SEAT_COUNT},
    state::MaskedClaimsState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::masked_claims_standard(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SetupDeal {
    pub hands: [Vec<MaskTileId>; 2],
    pub reserve: Vec<MaskTileId>,
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<MaskedClaimsState, Diagnostic> {
    if SeatCount::new(seats.len()).map(SeatCount::get) != Ok(STANDARD_SEAT_COUNT as usize) {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "masked_claims requires exactly two seats".to_owned(),
        });
    }

    let mut rng = SeededRng::from_seed(seed);
    let deal = deal_setup(&mut rng)?;

    Ok(MaskedClaimsState::new_after_deal(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        deal.hands,
        deal.reserve,
        rng,
    ))
}

pub(crate) fn deal_setup<R: DeterministicRng>(rng: &mut R) -> Result<SetupDeal, Diagnostic> {
    let mut masks = canonical_masks();
    shuffle_masks(&mut masks, rng);

    let mut dealt = masks.into_iter();
    let mut hands = [Vec::new(), Vec::new()];
    for _ in 0..STANDARD_HAND_SIZE {
        hands[0].push(dealt.next().ok_or_else(setup_masks_exhausted)?);
    }
    for _ in 0..STANDARD_HAND_SIZE {
        hands[1].push(dealt.next().ok_or_else(setup_masks_exhausted)?);
    }

    Ok(SetupDeal {
        hands,
        reserve: dealt.collect(),
    })
}

pub fn shuffle_masks<R: DeterministicRng>(masks: &mut [MaskTileId], rng: &mut R) {
    for index in (1..masks.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        masks.swap(index, swap_index);
    }
}

fn setup_masks_exhausted() -> Diagnostic {
    Diagnostic {
        code: "invalid_masks_exhausted".to_owned(),
        message: "masked_claims setup masks exhausted during deal".to_owned(),
    }
}

fn next_bounded_index_unbiased<R: DeterministicRng>(
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
    use crate::{
        ids::{MaskedClaimsSeat, STANDARD_MASK_COUNT, STANDARD_RESERVE_SIZE},
        state::Phase,
    };

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

    fn seat_ids(count: usize) -> Vec<SeatId> {
        (0..count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect()
    }

    fn invalid_seat_count_diagnostic() -> Diagnostic {
        Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "masked_claims requires exactly two seats".to_owned(),
        }
    }

    #[test]
    fn setup_rejects_wrong_seat_counts_with_exact_diagnostic() {
        let options = SetupOptions::default();

        for count in [0, 1, 3] {
            assert_eq!(
                setup_match(Seed(0), &seat_ids(count), &options),
                Err(invalid_seat_count_diagnostic()),
                "{count}"
            );
        }
    }

    #[test]
    fn setup_accepts_exact_standard_seat_count() {
        let options = SetupOptions::default();
        let seats = seat_ids(2);
        let state = setup_match(Seed(0), &seats, &options).expect("setup succeeds");

        assert_eq!(state.seats, [seats[0].clone(), seats[1].clone()]);
    }

    #[test]
    fn setup_is_deterministic_for_same_seed_and_options() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];

        let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
        let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

        assert_eq!(first.hands_internal(), second.hands_internal());
        assert_eq!(first.reserve_internal(), second.reserve_internal());
        assert_eq!(
            first.stable_internal_summary(),
            second.stable_internal_summary()
        );
    }

    #[test]
    fn setup_deals_private_hands_reserve_and_first_claimant() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
        let state = setup_match(Seed(7), &seats, &options).expect("setup succeeds");

        assert_eq!(state.phase, Phase::Claim { turn_index: 0 });
        assert_eq!(state.turn_index, 0);
        assert_eq!(state.claimant, MaskedClaimsSeat::Seat0);
        assert_eq!(state.active_seat, Some(MaskedClaimsSeat::Seat0));
        assert_eq!(
            state.hand_for_internal(MaskedClaimsSeat::Seat0).len(),
            STANDARD_HAND_SIZE as usize
        );
        assert_eq!(
            state.hand_for_internal(MaskedClaimsSeat::Seat1).len(),
            STANDARD_HAND_SIZE as usize
        );
        assert_eq!(
            state.reserve_internal().len(),
            STANDARD_RESERVE_SIZE as usize
        );
        assert!(state.pedestal.is_none());
        assert!(state.terminal_outcome.is_none());
        assert_eq!(state.scores, [0, 0]);

        let mut all_dealt = state.hand_for_internal(MaskedClaimsSeat::Seat0).to_vec();
        all_dealt.extend_from_slice(state.hand_for_internal(MaskedClaimsSeat::Seat1));
        all_dealt.extend_from_slice(state.reserve_internal());
        all_dealt.sort();
        assert_eq!(all_dealt, MaskTileId::ALL);
        assert_eq!(all_dealt.len(), STANDARD_MASK_COUNT as usize);
    }

    #[test]
    fn fixed_seed_has_known_setup_order() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
        let state = setup_match(Seed(11), &seats, &options).expect("setup succeeds");

        assert_eq!(
            state
                .hand_for_internal(MaskedClaimsSeat::Seat0)
                .iter()
                .map(|mask| mask.as_str())
                .collect::<Vec<_>>()
                .join(","),
            "mask_g4_c,mask_g3_b,mask_g5_b,mask_g1_b,mask_g2_c"
        );
        assert_eq!(
            state
                .hand_for_internal(MaskedClaimsSeat::Seat1)
                .iter()
                .map(|mask| mask.as_str())
                .collect::<Vec<_>>()
                .join(","),
            "mask_g2_b,mask_g1_a,mask_g4_a,mask_g3_a,mask_g1_c"
        );
        assert_eq!(
            state
                .reserve_internal()
                .iter()
                .map(|mask| mask.as_str())
                .collect::<Vec<_>>()
                .join(","),
            "mask_g4_b,mask_g5_a,mask_g3_c,mask_g5_c,mask_g2_a"
        );
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
