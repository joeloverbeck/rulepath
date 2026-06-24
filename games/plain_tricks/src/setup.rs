use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};
use game_stdlib::SeatCount;

use crate::{
    ids::{canonical_deck, PlainTricksSeat, TrickCardId, STANDARD_HAND_SIZE, STANDARD_SEAT_COUNT},
    state::PlainTricksState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::plain_tricks_standard(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RoundDeal {
    pub hands: [Vec<TrickCardId>; 2],
    pub tail: Vec<TrickCardId>,
    pub leader: PlainTricksSeat,
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<PlainTricksState, Diagnostic> {
    if SeatCount::new(seats.len()).map(SeatCount::get) != Ok(STANDARD_SEAT_COUNT as usize) {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "plain_tricks requires exactly two seats".to_owned(),
        });
    }

    let mut rng = SeededRng::from_seed(seed);
    let deal = deal_round(&mut rng, 0)?;

    Ok(PlainTricksState::new_after_deal(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        0,
        deal.leader,
        deal.hands,
        deal.tail,
        rng,
    ))
}

pub(crate) fn deal_round<R: DeterministicRng>(
    rng: &mut R,
    round_index: u8,
) -> Result<RoundDeal, Diagnostic> {
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, rng);

    let mut dealt = deck.into_iter();
    let mut hands = [Vec::new(), Vec::new()];
    for _ in 0..STANDARD_HAND_SIZE {
        hands[0].push(dealt.next().ok_or_else(setup_deck_exhausted)?);
    }
    for _ in 0..STANDARD_HAND_SIZE {
        hands[1].push(dealt.next().ok_or_else(setup_deck_exhausted)?);
    }

    Ok(RoundDeal {
        hands,
        tail: dealt.collect(),
        leader: round_leader(round_index),
    })
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [TrickCardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index = rng
            .next_index_unbiased_v1(index + 1)
            .expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

pub const fn round_leader(round_index: u8) -> PlainTricksSeat {
    match round_index {
        0 => PlainTricksSeat::Seat0,
        1 => PlainTricksSeat::Seat1,
        _ => PlainTricksSeat::Seat0,
    }
}

fn setup_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "invalid_deck_exhausted".to_owned(),
        message: "plain_tricks setup deck exhausted during round deal".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{STANDARD_CARD_COUNT, STANDARD_TAIL_SIZE};

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
    fn setup_rejects_wrong_seat_count() {
        let options = SetupOptions::default();
        let expected = Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "plain_tricks requires exactly two seats".to_owned(),
        };

        assert_eq!(setup_match(Seed(0), &[], &options), Err(expected.clone()));
        assert_eq!(
            setup_match(Seed(0), &[SeatId("seat_0".to_owned())], &options),
            Err(expected.clone())
        );
        assert_eq!(
            setup_match(
                Seed(0),
                &[
                    SeatId("seat_0".to_owned()),
                    SeatId("seat_1".to_owned()),
                    SeatId("seat_2".to_owned()),
                ],
                &options,
            ),
            Err(expected)
        );
    }

    #[test]
    fn setup_is_deterministic_for_same_seed_and_options() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];

        let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
        let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

        assert_eq!(first.hands_internal(), second.hands_internal());
        assert_eq!(first.tail_internal(), second.tail_internal());
        assert_eq!(
            first.stable_internal_summary(),
            second.stable_internal_summary()
        );
    }

    #[test]
    fn setup_deals_private_hands_tail_and_first_leader() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
        let state = setup_match(Seed(7), &seats, &options).expect("setup succeeds");

        assert_eq!(state.round_index, 0);
        assert_eq!(state.trick_index, 0);
        assert_eq!(state.round_leader, PlainTricksSeat::Seat0);
        assert_eq!(state.current_leader, PlainTricksSeat::Seat0);
        assert_eq!(state.active_seat, Some(PlainTricksSeat::Seat0));
        assert_eq!(
            state.hand_for_internal(PlainTricksSeat::Seat0).len(),
            STANDARD_HAND_SIZE as usize
        );
        assert_eq!(
            state.hand_for_internal(PlainTricksSeat::Seat1).len(),
            STANDARD_HAND_SIZE as usize
        );
        assert_eq!(state.tail_internal().len(), STANDARD_TAIL_SIZE as usize);
        assert!(state.terminal_outcome.is_none());

        let mut all_dealt = state.hand_for_internal(PlainTricksSeat::Seat0).to_vec();
        all_dealt.extend_from_slice(state.hand_for_internal(PlainTricksSeat::Seat1));
        all_dealt.extend_from_slice(state.tail_internal());
        all_dealt.sort();
        assert_eq!(all_dealt, TrickCardId::ALL);
        assert_eq!(all_dealt.len(), STANDARD_CARD_COUNT as usize);
    }

    #[test]
    fn deal_round_rotates_second_round_leader_from_continuing_rng_stream() {
        let mut first_rng = SeededRng::from_seed(Seed(11));
        let first_round = deal_round(&mut first_rng, 0).expect("round 1 deal succeeds");
        let second_round = deal_round(&mut first_rng, 1).expect("round 2 deal succeeds");

        let mut replay_rng = SeededRng::from_seed(Seed(11));
        let replay_first = deal_round(&mut replay_rng, 0).expect("replay round 1 deal succeeds");
        let replay_second = deal_round(&mut replay_rng, 1).expect("replay round 2 deal succeeds");

        assert_eq!(first_round.leader, PlainTricksSeat::Seat0);
        assert_eq!(second_round.leader, PlainTricksSeat::Seat1);
        assert_eq!(first_round, replay_first);
        assert_eq!(second_round, replay_second);
        assert_ne!(first_round.hands, second_round.hands);
        assert_eq!(first_round.tail.len(), STANDARD_TAIL_SIZE as usize);
        assert_eq!(second_round.tail.len(), STANDARD_TAIL_SIZE as usize);
    }

    #[test]
    fn bounded_index_rejects_high_residue_band() {
        let range = u128::from(u64::MAX) + 1;
        let accepted_zone_for_three = range - (range % 3);
        let rejected = accepted_zone_for_three as u64;
        let mut rng = FixedRng::new(vec![rejected, 4]);

        assert_eq!(rng.next_index_unbiased_v1(3), Some(1));
    }

    #[test]
    fn bounded_index_rejects_empty_bound() {
        let mut rng = FixedRng::new(vec![0]);

        assert_eq!(rng.next_index_unbiased_v1(0), None);
    }
}
