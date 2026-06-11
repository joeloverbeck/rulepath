use engine_core::{FreshnessToken, SeatId, Seed};
use flood_watch::{
    setup_match, DistrictId, FloodWatchRole, Phase, ScenarioVariant, SetupOptions,
    STANDARD_ACTION_BUDGET, STANDARD_DECK_SIZE, STANDARD_DRAWS_PER_PHASE, STANDARD_LEVEE_CAP,
    STANDARD_MAX_FLOOD_LEVEL,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

#[test]
fn standard_setup_initializes_bounded_state() {
    let state = setup_match(Seed(100), &seats(), &SetupOptions::default()).unwrap();

    assert_eq!(state.variant, ScenarioVariant::standard());
    assert_eq!(state.seats, seats());
    assert_eq!(
        state.roles,
        [FloodWatchRole::Pumpwright, FloodWatchRole::LeveeWarden]
    );
    assert_eq!(state.turn_number, 1);
    assert_eq!(state.active_seat, SeatId("seat_0".to_owned()));
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.freshness_token, FreshnessToken(0));
    assert_eq!(state.districts.len(), DistrictId::ALL.len());
    assert_eq!(
        state
            .districts
            .iter()
            .map(|district| district.flood_level)
            .collect::<Vec<_>>(),
        vec![0, 1, 0, 1, 0]
    );
    assert!(state
        .districts
        .iter()
        .all(|district| district.flood_level <= STANDARD_MAX_FLOOD_LEVEL));
    assert!(state
        .districts
        .iter()
        .all(|district| district.levees <= STANDARD_LEVEE_CAP));
    assert_eq!(state.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
    assert_eq!(
        state.remaining_composition().total_cards(),
        STANDARD_DECK_SIZE
    );
    assert!(state.drawn.is_empty());
    assert!(state.forecast.is_none());
    assert!(state.terminal_outcome.is_none());
}

#[test]
fn deluge_setup_initializes_scenario_constants() {
    let options = SetupOptions {
        variant: ScenarioVariant::deluge(),
    };
    let state = setup_match(Seed(100), &seats(), &options).unwrap();

    assert_eq!(state.variant, ScenarioVariant::deluge());
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: STANDARD_ACTION_BUDGET
        }
    );
    assert_eq!(state.variant.draws_per_phase, STANDARD_DRAWS_PER_PHASE);
    assert_eq!(state.variant.levee_cap, STANDARD_LEVEE_CAP);
    assert_eq!(
        state
            .districts
            .iter()
            .map(|district| district.flood_level)
            .collect::<Vec<_>>(),
        vec![1, 1, 1, 2, 1]
    );
    assert_eq!(
        state.undrawn_deck_len(),
        state.variant.event_composition.total_cards() as usize
    );
    assert_eq!(state.variant.event_composition.total_cards(), 27);
}

#[test]
fn setup_rejects_wrong_seat_count() {
    let err = setup_match(
        Seed(1),
        &[SeatId("seat_0".to_owned())],
        &SetupOptions::default(),
    )
    .unwrap_err();

    assert_eq!(err.code, "invalid_seat_count");
}
