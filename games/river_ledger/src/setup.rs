use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};

use crate::{
    cards::{canonical_deck, Card},
    ids::{
        RiverLedgerSeat, MAX_STARTING_STACK, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
        STANDARD_STARTING_STACK,
    },
    state::{RiverLedgerState, SeatRoles},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
    pub button_index: usize,
    pub starting_stacks: Option<Vec<u16>>,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::river_ledger_standard(),
            button_index: 0,
            starting_stacks: None,
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<RiverLedgerState, Diagnostic> {
    validate_seat_count(seats.len())?;
    let starting_stacks =
        validate_starting_stacks(seats.len(), options.starting_stacks.as_deref())?;

    let button = RiverLedgerSeat::from_index(options.button_index % seats.len())
        .expect("button modulo valid seat count");
    let small_blind = button
        .next_in_count(seats.len() as u8)
        .expect("small blind");
    let big_blind = small_blind
        .next_in_count(seats.len() as u8)
        .expect("big blind");
    let active_seat = big_blind
        .next_in_count(seats.len() as u8)
        .expect("preflop active seat");

    let mut rng = SeededRng::from_seed(seed);
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, &mut rng);

    let mut dealt = deck.into_iter();
    let mut private_hands = Vec::with_capacity(seats.len());
    for _ in seats {
        let first = dealt.next().ok_or_else(setup_deck_exhausted)?;
        let second = dealt.next().ok_or_else(setup_deck_exhausted)?;
        private_hands.push([first, second]);
    }

    let community_deck = [
        dealt.next().ok_or_else(setup_deck_exhausted)?,
        dealt.next().ok_or_else(setup_deck_exhausted)?,
        dealt.next().ok_or_else(setup_deck_exhausted)?,
        dealt.next().ok_or_else(setup_deck_exhausted)?,
        dealt.next().ok_or_else(setup_deck_exhausted)?,
    ];
    let deck_tail = dealt.collect::<Vec<_>>();

    Ok(RiverLedgerState::new_after_setup(
        options.variant.clone(),
        seats.to_vec(),
        SeatRoles {
            button,
            small_blind,
            big_blind,
            active_seat,
        },
        starting_stacks,
        private_hands,
        community_deck,
        deck_tail,
    ))
}

pub fn validate_seat_count(count: usize) -> Result<(), Diagnostic> {
    if (STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize).contains(&count) {
        return Ok(());
    }

    Err(Diagnostic {
        code: "invalid_seat_count".to_owned(),
        message: format!(
            "river_ledger requires between {STANDARD_MIN_SEATS} and {STANDARD_MAX_SEATS} seats"
        ),
    })
}

pub fn validate_starting_stacks(
    seat_count: usize,
    configured: Option<&[u16]>,
) -> Result<Vec<u16>, Diagnostic> {
    validate_seat_count(seat_count)?;

    let stacks = configured
        .map(|values| values.to_vec())
        .unwrap_or_else(|| vec![STANDARD_STARTING_STACK; seat_count]);

    if stacks.len() != seat_count {
        return Err(Diagnostic {
            code: "invalid_starting_stack_count".to_owned(),
            message: format!(
                "river_ledger starting stack count must equal seat count ({seat_count})"
            ),
        });
    }

    for (index, stack) in stacks.iter().copied().enumerate() {
        if stack == 0 || stack > MAX_STARTING_STACK {
            return Err(Diagnostic {
                code: "invalid_starting_stack".to_owned(),
                message: format!(
                    "river_ledger starting stack for seat_{index} must be between 1 and {MAX_STARTING_STACK}"
                ),
            });
        }
    }

    let total = stacks
        .iter()
        .try_fold(0u32, |total, stack| total.checked_add(u32::from(*stack)));
    if total.is_none() {
        return Err(Diagnostic {
            code: "invalid_starting_stack_overflow".to_owned(),
            message: "river_ledger starting stacks overflow the accounting total".to_owned(),
        });
    }

    Ok(stacks)
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [Card], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

fn setup_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "invalid_deck_exhausted".to_owned(),
        message: "river_ledger setup deck exhausted during initial deal".to_owned(),
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
}
