use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use flood_watch::{
    apply_command, legal_action_metadata, legal_action_tree, setup_match, DistrictId,
    FloodWatchRole, Phase, ScenarioVariant, SetupOptions, ACTION_END_TURN, ACTION_FORECAST,
    ACTION_REINFORCE, STANDARD_ACTION_BUDGET, STANDARD_DECK_SIZE, STANDARD_DRAWS_PER_PHASE,
    STANDARD_LEVEE_CAP, STANDARD_MAX_FLOOD_LEVEL,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(
    state: &flood_watch::FloodWatchState,
    seat: &str,
    segments: Vec<&str>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: segments.into_iter().map(str::to_owned).collect(),
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
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

#[test]
fn active_tree_contains_legal_budgeted_choices_and_teammate_waits() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).unwrap();
    let tree = legal_action_tree(&state, &actor("seat_0"));
    let segments = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>();

    assert!(segments.contains(&"bail/district_old_docks"));
    assert!(segments.contains(&"bail/district_terraces"));
    assert!(!segments.contains(&"bail/district_riverside"));
    assert!(segments.contains(&"reinforce/district_riverside"));
    assert!(segments.contains(&ACTION_FORECAST));
    assert!(segments.contains(&ACTION_END_TURN));

    let rendered = format!("{:?}", tree);
    assert!(rendered.contains("remaining_budget"));
    assert!(rendered.contains("role_amount"));
    assert!(!rendered.contains("storm_surge/"));
    assert!(!rendered.contains("downpour/"));

    assert!(legal_action_tree(&state, &actor("seat_1"))
        .root
        .choices
        .is_empty());
    assert!(legal_action_metadata(&state, &actor("seat_1"))
        .iter()
        .any(|entry| entry.key == "action_status" && entry.value == "waiting"));
}

#[test]
fn budget_decrements_and_role_powers_apply_in_rust() {
    let mut state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();

    let cmd = command(&state, "seat_0", vec!["bail", "district_old_docks"]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.district(DistrictId::OldDocks).unwrap().flood_level, 0);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );

    state.active_seat = SeatId("seat_1".to_owned());
    state.phase = Phase::Action {
        budget_remaining: 3,
    };
    let cmd = command(
        &state,
        "seat_1",
        vec![ACTION_REINFORCE, "district_riverside"],
    );
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.district(DistrictId::Riverside).unwrap().levees, 2);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );
}

#[test]
fn forecast_reveals_top_card_without_drawing_and_then_becomes_unavailable() {
    let mut state = setup_match(Seed(11), &seats(), &SetupOptions::default()).unwrap();
    let top = state.top_undrawn_card().cloned().unwrap();

    let cmd = command(&state, "seat_0", vec![ACTION_FORECAST]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(!applied.environment_pending);
    assert_eq!(state.forecast, Some(top));
    assert_eq!(state.undrawn_deck_len(), STANDARD_DECK_SIZE as usize);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 2
        }
    );

    let cmd = command(&state, "seat_0", vec![ACTION_FORECAST]);
    let err = apply_command(&mut state, &cmd).unwrap_err();
    assert_eq!(err.code, "forecast_unavailable");
    assert!(!err.message.contains("storm_surge/"));
    assert!(!err.message.contains("downpour/"));
}

#[test]
fn final_budget_action_and_end_turn_mark_environment_pending() {
    let mut state = setup_match(Seed(3), &seats(), &SetupOptions::default()).unwrap();
    state.phase = Phase::Action {
        budget_remaining: 1,
    };

    let cmd = command(&state, "seat_0", vec!["reinforce", "district_market"]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(applied.environment_pending);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 0
        }
    );

    let mut state = setup_match(Seed(3), &seats(), &SetupOptions::default()).unwrap();
    let cmd = command(&state, "seat_0", vec![ACTION_END_TURN]);
    let applied = apply_command(&mut state, &cmd).unwrap();
    assert!(applied.environment_pending);
    assert_eq!(
        state.phase,
        Phase::Action {
            budget_remaining: 0
        }
    );
}

#[test]
fn diagnostics_are_fail_closed_and_deck_safe() {
    let mut state = setup_match(Seed(5), &seats(), &SetupOptions::default()).unwrap();

    let mut stale = command(&state, "seat_0", vec![ACTION_END_TURN]);
    stale.freshness_token = FreshnessToken(99);
    assert_eq!(
        apply_command(&mut state.clone(), &stale).unwrap_err().code,
        "stale_action"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_1", vec![ACTION_END_TURN])
        )
        .unwrap_err()
        .code,
        "wrong_seat"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_x", vec![ACTION_END_TURN])
        )
        .unwrap_err()
        .code,
        "wrong_actor"
    );
    assert_eq!(
        apply_command(
            &mut state.clone(),
            &command(&state, "seat_0", vec!["bail", "district_riverside"]),
        )
        .unwrap_err()
        .code,
        "dry_bail"
    );

    state
        .district_mut(DistrictId::Riverside)
        .expect("district")
        .levees = STANDARD_LEVEE_CAP;
    let cap = apply_command(
        &mut state.clone(),
        &command(&state, "seat_0", vec!["reinforce", "district_riverside"]),
    )
    .unwrap_err();
    assert_eq!(cap.code, "levee_cap");
    assert!(!format!("{:?}", cap).contains("storm_surge/"));
    assert!(!format!("{:?}", cap).contains("downpour/"));

    let mut empty_budget = state.clone();
    empty_budget.phase = Phase::Action {
        budget_remaining: 0,
    };
    assert_eq!(
        apply_command(
            &mut empty_budget,
            &command(&state, "seat_0", vec![ACTION_END_TURN]),
        )
        .unwrap_err()
        .code,
        "out_of_budget"
    );
}
