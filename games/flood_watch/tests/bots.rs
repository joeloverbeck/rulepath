use engine_core::{SeatId, Seed};
use game_test_support::no_leak::{
    assert_pairwise_no_leak, ExposureExpectation, LeakProbe,
};
use flood_watch::{
    setup_match, validate_bot_decision, DistrictId, EventCard, EventKind, FloodWatchLevel1Bot,
    FloodWatchRandomBot, FloodWatchState, Phase, ScenarioVariant, SetupOptions, ACTION_BAIL,
    ACTION_REINFORCE,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn card(kind: EventKind, copy_index: u8) -> EventCard {
    EventCard { kind, copy_index }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BotSurface {
    InputDebug,
    DecisionDebug,
    Rationale,
}

#[test]
fn level1_bot_surfaces_do_not_leak_hidden_future_deck_cards() {
    let state = setup_match(Seed(12), &seats(), &SetupOptions::default()).unwrap();
    let probes = state
        .event_deck_internal()
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, card)| LeakProbe {
            source_seat: index,
            canary_id: card.stable_id(),
            canary: card.clone(),
        })
        .collect::<Vec<_>>();

    assert_pairwise_no_leak(
        [state.active_seat.clone()],
        [BotSurface::InputDebug, BotSurface::DecisionDebug, BotSurface::Rationale],
        probes,
        |seat, surface| bot_surface_text(&state, seat, *surface),
        |_source, _seat, _surface, _canary_id| ExposureExpectation::MustBeAbsent,
        |snapshot, card| snapshot_contains_event(snapshot, card),
    )
    .expect("Flood Watch bot no-leak matrix has no failures");
}

fn bot_surface_text(state: &FloodWatchState, seat: &SeatId, surface: BotSurface) -> String {
    match surface {
        BotSurface::InputDebug => format!("{:?}", FloodWatchLevel1Bot::input_for(state, seat)),
        BotSurface::DecisionDebug => format!(
            "{:?}",
            FloodWatchLevel1Bot::new(Seed(23))
                .select_decision(state, seat)
                .expect("level1 decision")
        ),
        BotSurface::Rationale => FloodWatchLevel1Bot::new(Seed(23))
            .select_decision(state, seat)
            .expect("level1 decision")
            .rationale,
    }
}

fn snapshot_contains_event(snapshot: &str, card: &EventCard) -> bool {
    snapshot.contains(&card.stable_id())
        || (!matches!(card.kind, EventKind::Reprieve) && snapshot.contains(&card.kind.id()))
        || snapshot.contains(&format!("{:?}", card.kind))
}

#[test]
fn random_and_level1_bots_choose_legal_actions_for_both_roles() {
    let mut state = setup_match(Seed(9), &seats(), &SetupOptions::default()).unwrap();
    for seat_index in 0..2 {
        state.active_seat = state.seats[seat_index].clone();
        state.phase = Phase::Action {
            budget_remaining: state.variant.action_budget,
        };
        let seat = state.seats[seat_index].clone();

        let random = FloodWatchRandomBot::new(Seed(17))
            .select_decision(&state, &seat)
            .expect("random decision");
        validate_bot_decision(&state, &seat, &random).expect("random validates");

        let level1 = FloodWatchLevel1Bot::new(Seed(23))
            .select_decision(&state, &seat)
            .expect("level1 decision");
        validate_bot_decision(&state, &seat, &level1).expect("level1 validates");
        assert_public_safe(&level1.rationale);
    }
}

#[test]
fn seeded_bots_are_deterministic() {
    let state = setup_match(Seed(12), &seats(), &SetupOptions::default()).unwrap();
    let seat = state.seats[0].clone();
    let random = FloodWatchRandomBot::new(Seed(42));
    let level1 = FloodWatchLevel1Bot::new(Seed(42));

    assert_eq!(
        random.select_decision(&state, &seat),
        random.select_decision(&state, &seat)
    );
    assert_eq!(
        level1.select_decision(&state, &seat),
        level1.select_decision(&state, &seat)
    );
}

#[test]
fn level1_rescues_imminent_loss_before_reinforcing() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state
        .district_mut(DistrictId::Market)
        .expect("district")
        .flood_level = 2;

    let decision = FloodWatchLevel1Bot::new(Seed(1))
        .select_decision(&state, &state.seats[0])
        .expect("decision");

    assert_eq!(
        decision.action_path.segments,
        vec![format!("{ACTION_BAIL}/{}", DistrictId::Market.as_str())]
    );
    assert!(decision.rationale.contains("one step from shared loss"));
}

#[test]
fn level1_mitigates_public_forecast_threat() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.forecast = Some(card(
        EventKind::StormSurge {
            district: DistrictId::Terraces,
        },
        1,
    ));

    let decision = FloodWatchLevel1Bot::new(Seed(1))
        .select_decision(&state, &state.seats[0])
        .expect("decision");

    assert_eq!(
        decision.action_path.segments,
        vec![format!(
            "{ACTION_REINFORCE}/{}",
            DistrictId::Terraces.as_str()
        )]
    );
    assert!(decision.rationale.contains("public forecast"));
    assert_public_safe(&decision.rationale);
}

#[test]
fn level1_is_invariant_to_hidden_deck_order_with_same_public_view() {
    let deck_a = vec![
        card(
            EventKind::Downpour {
                district: DistrictId::Riverside,
            },
            1,
        ),
        card(
            EventKind::StormSurge {
                district: DistrictId::Gardens,
            },
            1,
        ),
        card(EventKind::Reprieve, 1),
    ];
    let deck_b = vec![deck_a[2].clone(), deck_a[1].clone(), deck_a[0].clone()];
    let state_a = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck_a);
    let state_b = FloodWatchState::new_after_setup(ScenarioVariant::standard(), seats(), deck_b);
    let bot = FloodWatchLevel1Bot::new(Seed(77));

    let left = bot
        .select_decision(&state_a, &state_a.seats[0])
        .expect("left decision");
    let right = bot
        .select_decision(&state_b, &state_b.seats[0])
        .expect("right decision");

    assert_eq!(left, right);
    assert_public_safe(&left.rationale);
}

#[test]
fn inactive_bot_has_no_legal_actions() {
    let state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let err = FloodWatchLevel1Bot::new(Seed(1))
        .select_decision(&state, &state.seats[1])
        .expect_err("inactive bot rejected");

    assert_eq!(err.code, "no_legal_actions");
}

fn assert_public_safe(rationale: &str) {
    let lower = rationale.to_lowercase();
    for forbidden in [
        "storm_surge/",
        "downpour/",
        "#1",
        "hidden",
        "deck order",
        "sample",
        "mcts",
        "monte carlo",
        "machine learning",
        "reinforcement learning",
        "debug",
        "hash",
    ] {
        assert!(
            !lower.contains(forbidden),
            "rationale contained forbidden text `{forbidden}`: {rationale}"
        );
    }
}
