use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};

use crate::{
    ids::{canonical_deck, CrestCardId, STANDARD_SEAT_COUNT},
    state::PokerLiteState,
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::poker_lite_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<PokerLiteState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "poker_lite requires exactly two seats".to_owned(),
        });
    }

    let mut rng = SeededRng::from_seed(seed);
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, &mut rng);

    let mut dealt = deck.into_iter();
    let private_cards = [
        dealt.next().ok_or_else(setup_deck_exhausted)?,
        dealt.next().ok_or_else(setup_deck_exhausted)?,
    ];
    let center_card = dealt.next().ok_or_else(setup_deck_exhausted)?;
    let deck_tail = dealt.collect::<Vec<_>>();

    Ok(PokerLiteState::new_after_deal(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        private_cards,
        center_card,
        deck_tail,
    ))
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [CrestCardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

fn setup_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "invalid_deck_exhausted".to_owned(),
        message: "poker_lite setup deck exhausted during initial deal".to_owned(),
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
    use crate::ids::PokerLiteSeat;
    use crate::state::{Phase, PledgeRoundState};

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let options = SetupOptions::default();

        assert!(setup_match(Seed(0), &[SeatId("seat_0".to_owned())], &options).is_err());
    }

    #[test]
    fn setup_is_deterministic_for_same_seed_and_options() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];

        let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
        let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

        assert_eq!(
            first.private_cards_internal(),
            second.private_cards_internal()
        );
        assert_eq!(first.center_card_internal(), second.center_card_internal());
        assert_eq!(first.deck_tail_internal(), second.deck_tail_internal());
        assert_eq!(first.contributions, second.contributions);
        assert_eq!(
            first.stable_internal_summary(),
            second.stable_internal_summary()
        );
    }

    #[test]
    fn setup_deals_private_center_tail_and_opening_accounting() {
        let options = SetupOptions::default();
        let seats = [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())];
        let state = setup_match(Seed(7), &seats, &options).expect("setup succeeds");

        assert_eq!(state.phase, Phase::PledgeRound { round_index: 0 });
        assert_eq!(state.active_seat, Some(PokerLiteSeat::Seat0));
        assert_eq!(state.contributions, [1, 1]);
        assert_eq!(state.shared_pool, 2);
        assert_eq!(state.round, PledgeRoundState::initial());
        assert!(!state.center_visible);
        assert_eq!(state.deck_tail_internal().len(), 3);
        assert!(state.terminal_outcome.is_none());

        let mut all_dealt = vec![
            state.private_card_for_internal(PokerLiteSeat::Seat0),
            state.private_card_for_internal(PokerLiteSeat::Seat1),
            state.center_card_internal(),
        ];
        all_dealt.extend_from_slice(state.deck_tail_internal());
        all_dealt.sort();
        assert_eq!(all_dealt, CrestCardId::ALL);
    }
}
