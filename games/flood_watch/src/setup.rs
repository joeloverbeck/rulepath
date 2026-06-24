//! Deterministic setup for Flood Watch.

use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};
use game_stdlib::SeatCount;

use crate::{
    ids::{DistrictId, EventKind, STANDARD_SEAT_COUNT},
    state::{EventCard, FloodWatchState},
    variants::ScenarioVariant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: ScenarioVariant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: ScenarioVariant::standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<FloodWatchState, Diagnostic> {
    if SeatCount::new(seats.len()).map(SeatCount::get) != Ok(STANDARD_SEAT_COUNT as usize) {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "flood_watch requires exactly two seats".to_owned(),
        });
    }
    validate_variant(&options.variant)?;

    let mut event_deck = build_event_deck(&options.variant);
    let mut rng = SeededRng::from_seed(seed);
    shuffle_event_deck(&mut event_deck, &mut rng);

    Ok(FloodWatchState::new_after_setup(
        options.variant.clone(),
        [seats[0].clone(), seats[1].clone()],
        event_deck,
    ))
}

pub fn build_event_deck(variant: &ScenarioVariant) -> Vec<EventCard> {
    let mut cards = Vec::with_capacity(variant.event_composition.total_cards() as usize);

    for district in DistrictId::ALL {
        for copy_index in 1..=variant.event_composition.downpours_per_district {
            cards.push(EventCard {
                kind: EventKind::Downpour { district },
                copy_index,
            });
        }
        for copy_index in 1..=variant.event_composition.surges_per_district {
            cards.push(EventCard {
                kind: EventKind::StormSurge { district },
                copy_index,
            });
        }
    }
    for copy_index in 1..=variant.event_composition.reprieves {
        cards.push(EventCard {
            kind: EventKind::Reprieve,
            copy_index,
        });
    }

    cards
}

pub fn shuffle_event_deck<R: DeterministicRng>(cards: &mut [EventCard], rng: &mut R) {
    for index in (1..cards.len()).rev() {
        let swap_index =
            next_bounded_index_unbiased(rng, index + 1).expect("shuffle upper bound is nonzero");
        cards.swap(index, swap_index);
    }
}

fn validate_variant(variant: &ScenarioVariant) -> Result<(), Diagnostic> {
    if SeatCount::new(variant.seat_count as usize).map(SeatCount::get)
        != Ok(STANDARD_SEAT_COUNT as usize)
    {
        return Err(Diagnostic {
            code: "invalid_variant_seat_count".to_owned(),
            message: "flood_watch variants require exactly two seats".to_owned(),
        });
    }
    if variant.role_order.len() != STANDARD_SEAT_COUNT as usize {
        return Err(Diagnostic {
            code: "invalid_variant_roles".to_owned(),
            message: "flood_watch variants require exactly two roles".to_owned(),
        });
    }
    for level in variant.starting_levels {
        if level > variant.max_flood_level {
            return Err(Diagnostic {
                code: "invalid_starting_level".to_owned(),
                message: "flood_watch starting levels must not exceed the max flood level"
                    .to_owned(),
            });
        }
    }
    if variant.levee_cap == 0 || variant.action_budget == 0 || variant.draws_per_phase == 0 {
        return Err(Diagnostic {
            code: "invalid_variant_zero".to_owned(),
            message: "flood_watch budget, levee cap, and draw count must be nonzero".to_owned(),
        });
    }
    Ok(())
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
    use crate::ids::{FloodWatchRole, STANDARD_ACTION_BUDGET, STANDARD_DECK_SIZE};

    fn seats() -> [SeatId; 2] {
        [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
    }

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
            message: "flood_watch requires exactly two seats".to_owned(),
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
    fn setup_rejects_wrong_variant_seat_count() {
        let expected = Diagnostic {
            code: "invalid_variant_seat_count".to_owned(),
            message: "flood_watch variants require exactly two seats".to_owned(),
        };

        for seat_count in [0, 1, 3] {
            let mut options = SetupOptions::default();
            options.variant.seat_count = seat_count;

            assert_eq!(setup_match(Seed(0), &seats(), &options), Err(expected.clone()));
        }
    }

    #[test]
    fn setup_is_deterministic_for_same_seed_and_options() {
        let options = SetupOptions::default();
        let seats = seats();

        let first = setup_match(Seed(42), &seats, &options).expect("first setup succeeds");
        let second = setup_match(Seed(42), &seats, &options).expect("second setup succeeds");

        assert_eq!(first.event_deck_internal(), second.event_deck_internal());
        assert_eq!(first.stable_summary(), second.stable_summary());
    }

    #[test]
    fn setup_initializes_standard_state() {
        let options = SetupOptions::default();
        let state = setup_match(Seed(7), &seats(), &options).expect("setup succeeds");

        assert_eq!(
            state.roles,
            [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden]
        );
        assert_eq!(state.turn_number, 1);
        assert_eq!(state.active_seat, SeatId("seat_0".to_owned()));
        assert_eq!(state.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
        assert_eq!(
            state.remaining_composition().total_cards(),
            STANDARD_DECK_SIZE
        );
        assert_eq!(
            state.phase,
            crate::state::Phase::Action {
                budget_remaining: STANDARD_ACTION_BUDGET
            }
        );
        assert!(state.drawn.is_empty());
        assert!(state.forecast.is_none());
        assert!(state.terminal_outcome.is_none());
        assert_eq!(
            state
                .districts
                .iter()
                .map(|district| district.flood_level)
                .collect::<Vec<_>>(),
            vec![0, 1, 0, 1, 0]
        );
        assert!(state.districts.iter().all(|district| district.levees == 0));
    }

    #[test]
    fn fixed_seed_has_known_deck_prefix() {
        let options = SetupOptions::default();
        let state = setup_match(Seed(11), &seats(), &options).expect("setup succeeds");
        let prefix = state
            .event_deck_internal()
            .iter()
            .take(5)
            .map(EventCard::stable_id)
            .collect::<Vec<_>>();

        assert_eq!(
            prefix,
            vec![
                "storm_surge/district_gardens#1",
                "storm_surge/district_riverside#1",
                "downpour/district_terraces#1",
                "downpour/district_market#3",
                "downpour/district_gardens#1",
            ]
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
}
