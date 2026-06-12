use engine_core::{SeatId, Seed};
use event_frontier::{setup_match, EventFrontierState};
use event_frontier::{
    validate_bot_decision, CardId, CardPhase, EventCharterLevel1Bot, EventFreeholdersLevel1Bot,
    EventFrontierRandomBot, FactionId, SetupOptions, ACTION_EVENT, ACTION_OPERATION,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn seat_for(faction: FactionId) -> SeatId {
    match faction {
        FactionId::Charter => SeatId("seat_0".to_owned()),
        FactionId::Freeholders => SeatId("seat_1".to_owned()),
    }
}

fn setup(seed: u64) -> EventFrontierState {
    setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup")
}

fn make_freeholders_first(state: &mut EventFrontierState) {
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Freeholders,
    };
}

fn make_charter_first(state: &mut EventFrontierState) {
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
}

#[test]
fn level0_and_level1_bots_choose_validating_actions_over_many_seeds() {
    for seed in 0..32 {
        let mut state = setup(seed);
        state.deck.current = Some(CardId::LastLight);
        make_charter_first(&mut state);
        let charter_seat = seat_for(FactionId::Charter);
        let random = EventFrontierRandomBot::new(Seed(seed + 100))
            .select_decision(&state, &charter_seat)
            .expect("random decision");
        validate_bot_decision(&state, &charter_seat, &random).expect("random legal");

        let charter = EventCharterLevel1Bot::new(Seed(seed + 200))
            .select_decision(&state, &charter_seat)
            .expect("charter decision");
        validate_bot_decision(&state, &charter_seat, &charter).expect("charter legal");

        let mut freeholder_state = setup(seed);
        freeholder_state.deck.current = Some(CardId::LastLight);
        make_freeholders_first(&mut freeholder_state);
        let freeholder_seat = seat_for(FactionId::Freeholders);
        let freeholder = EventFreeholdersLevel1Bot::new(Seed(seed + 300))
            .select_decision(&freeholder_state, &freeholder_seat)
            .expect("freeholder decision");
        validate_bot_decision(&freeholder_state, &freeholder_seat, &freeholder)
            .expect("freeholder legal");
    }
}

#[test]
fn level1_bots_are_deterministic_for_same_public_inputs() {
    let state = setup(3);
    let seat = seat_for(FactionId::Charter);
    let bot = EventCharterLevel1Bot::new(Seed(42));

    let left = bot.select_decision(&state, &seat).expect("left decision");
    let right = bot.select_decision(&state, &seat).expect("right decision");

    assert_eq!(left, right);
}

#[test]
fn favorable_public_events_match_the_decision_table() {
    let mut charter_state = setup(5);
    charter_state.deck.current = Some(CardId::BorderSurvey);
    charter_state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let charter = EventCharterLevel1Bot::new(Seed(1))
        .select_decision(&charter_state, &seat_for(FactionId::Charter))
        .expect("charter event");
    assert_eq!(charter.action_path.segments, vec![ACTION_EVENT]);

    let mut freeholder_state = setup(5);
    freeholder_state.deck.current = Some(CardId::HighMeadowFair);
    make_freeholders_first(&mut freeholder_state);
    let freeholder = EventFreeholdersLevel1Bot::new(Seed(1))
        .select_decision(&freeholder_state, &seat_for(FactionId::Freeholders))
        .expect("freeholder event");
    assert_eq!(freeholder.action_path.segments, vec![ACTION_EVENT]);
}

#[test]
fn faction_level1_policies_rank_distinct_operation_families() {
    let mut charter_state = setup(2);
    charter_state.deck.current = Some(CardId::LastLight);
    charter_state.resources.funds = 3;
    charter_state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let charter = EventCharterLevel1Bot::new(Seed(1))
        .select_decision(&charter_state, &seat_for(FactionId::Charter))
        .expect("charter op");
    let charter_path = charter.action_path.segments[0].as_str();
    assert!(charter_path.starts_with(ACTION_OPERATION));
    assert!(
        charter_path.contains("/survey/")
            || charter_path.contains("/fortify/")
            || charter_path.contains("/writ/")
    );

    let mut freeholder_state = setup(2);
    freeholder_state.deck.current = Some(CardId::LastLight);
    freeholder_state.resources.provisions = 3;
    make_freeholders_first(&mut freeholder_state);
    let freeholder = EventFreeholdersLevel1Bot::new(Seed(1))
        .select_decision(&freeholder_state, &seat_for(FactionId::Freeholders))
        .expect("freeholder op");
    let freeholder_path = freeholder.action_path.segments[0].as_str();
    assert!(freeholder_path.starts_with(ACTION_OPERATION));
    assert!(
        freeholder_path.contains("/cache/")
            || freeholder_path.contains("/trek/")
            || freeholder_path.contains("/rally/")
    );
    assert_ne!(charter_path, freeholder_path);
}

#[test]
fn bot_inputs_and_explanations_do_not_expose_undrawn_deck_order() {
    let mut state = setup(1);
    state.deck.current = Some(CardId::LastLight);
    make_charter_first(&mut state);
    let hidden = state.deck.undrawn[0].as_str();
    let seat = seat_for(FactionId::Charter);
    let input = EventCharterLevel1Bot::input_for(&state, &seat);
    let decision = EventCharterLevel1Bot::new(Seed(1))
        .select_decision(&state, &seat)
        .expect("decision");

    assert!(!format!("{input:?}").contains(hidden));
    assert!(!decision.rationale.contains(hidden));
    assert!(!format!("{decision:?}").contains(hidden));
}
