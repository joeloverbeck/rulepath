use engine_core::{SeatId, Seed};
use frontier_control::{
    command_for_decision, setup_match, validate_bot_decision, FactionId, FrontierGarrisonLevel1Bot,
    FrontierProspectorLevel1Bot, FrontierRandomBot, SetupOptions,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn seat_for(state: &frontier_control::FrontierControlState, faction: FactionId) -> SeatId {
    state
        .seats
        .iter()
        .find(|seat| state.faction_for_seat(seat) == Some(faction))
        .unwrap()
        .clone()
}

#[test]
fn random_and_level1_bots_select_legal_public_actions() {
    let state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    let prospector = seat_for(&state, FactionId::Prospectors);

    let random = FrontierRandomBot::new(Seed(9))
        .select_decision(&state, &prospector)
        .unwrap();
    validate_bot_decision(&state, &prospector, &random).unwrap();

    let level1 = FrontierProspectorLevel1Bot::new(Seed(9))
        .select_decision(&state, &prospector)
        .unwrap();
    validate_bot_decision(&state, &prospector, &level1).unwrap();
    assert!(level1.rationale.contains("Prospectors"));
}

#[test]
fn level1_garrison_policy_is_legal_and_faction_named() {
    let mut state = setup_match(&seats(), &SetupOptions::default()).unwrap();
    state.active_faction = FactionId::Garrison;
    let garrison = seat_for(&state, FactionId::Garrison);
    let decision = FrontierGarrisonLevel1Bot::new(Seed(3))
        .select_decision(&state, &garrison)
        .unwrap();

    validate_bot_decision(&state, &garrison, &decision).unwrap();
    assert!(decision.rationale.contains("Garrison"));
    let command = command_for_decision(&state, &garrison, &decision);
    assert_eq!(command.actor.seat_id, garrison);
}
