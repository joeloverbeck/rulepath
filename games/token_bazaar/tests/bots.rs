use engine_core::{Actor, CommandEnvelope, RulesVersion, SeatId, Seed};
use token_bazaar::{
    action_from_decision, setup_match, validate_command, BotDecision, ContractId, ResourceCounts,
    TokenBazaarLevel1Bot, TokenBazaarRandomBot, TokenBazaarSeat, TokenBazaarSlot, TokenBazaarState,
    LEVEL1_POLICY_ID, RANDOM_POLICY_ID,
};

fn state() -> TokenBazaarState {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
}

fn command_for_decision(state: &TokenBazaarState, decision: &BotDecision) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: decision.action_path.clone(),
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn random_bot_selection_validates_through_normal_command_path() {
    let state = state();
    let decision = TokenBazaarRandomBot::new(Seed(7))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("random bot chooses");

    assert_eq!(decision.policy_id, RANDOM_POLICY_ID);
    validate_command(&state, &command_for_decision(&state, &decision))
        .expect("random bot action validates");
}

#[test]
fn level1_selection_validates_across_public_states() {
    let mut states = vec![state()];
    let mut exchange_state = state();
    exchange_state.inventories[0] = ResourceCounts::new(2, 1, 1);
    exchange_state.supply = ResourceCounts::new(13, 14, 14);
    states.push(exchange_state);

    for state in states {
        let decision = TokenBazaarLevel1Bot::new(Seed(3))
            .select_decision(&state, state.active_seat)
            .expect("level1 bot chooses");
        validate_command(&state, &command_for_decision(&state, &decision))
            .expect("level1 bot action validates");
        assert_public_safe_rationale(&decision.rationale);
    }
}

#[test]
fn level1_decision_is_deterministic_for_fixed_state_and_seed() {
    let state = state();
    let left = TokenBazaarLevel1Bot::new(Seed(9))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("left decision");
    let right = TokenBazaarLevel1Bot::new(Seed(9))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("right decision");

    assert_eq!(left, right);
}

#[test]
fn level1_fulfills_affordable_contract() {
    let state = state();
    let decision = TokenBazaarLevel1Bot::new(Seed(1))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("level1 bot chooses");

    assert_eq!(decision.policy_id, LEVEL1_POLICY_ID);
    assert_eq!(decision.action_path.segments, vec!["fulfill/slot_0"]);
    assert!(decision.rationale.contains("affordable visible contract"));
    assert!(matches!(
        action_from_decision(&decision),
        Some(token_bazaar::TokenBazaarAction::Fulfill {
            slot: TokenBazaarSlot::Slot0
        })
    ));
}

#[test]
fn level1_collects_toward_unaffordable_visible_contract() {
    let mut state = state();
    state.slots = [Some(ContractId::AmberFocus), None, None];
    state.queue.clear();
    state.inventories[0] = ResourceCounts::new(1, 1, 1);

    let decision = TokenBazaarLevel1Bot::new(Seed(1))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("level1 bot chooses");

    assert_eq!(decision.action_path.segments, vec!["collect/amber"]);
    assert!(decision.rationale.contains("Amber Focus"));
    validate_command(&state, &command_for_decision(&state, &decision))
        .expect("collect action validates");
}

#[test]
fn level1_fallback_reaches_forced_pass() {
    let mut state = state();
    state.supply = ResourceCounts::default();
    state.inventories[0] = ResourceCounts::default();
    state.slots = [None, None, None];
    state.queue.clear();

    let decision = TokenBazaarLevel1Bot::new(Seed(1))
        .select_decision(&state, TokenBazaarSeat::Seat0)
        .expect("level1 bot chooses pass");

    assert_eq!(decision.action_path.segments, vec!["pass"]);
    validate_command(&state, &command_for_decision(&state, &decision)).expect("pass validates");
}

#[test]
fn inactive_bot_has_no_legal_actions() {
    let state = state();
    let diagnostic = TokenBazaarLevel1Bot::new(Seed(1))
        .select_decision(&state, TokenBazaarSeat::Seat1)
        .expect_err("inactive bot cannot choose");

    assert_eq!(diagnostic.code, "no_legal_actions");
}

fn assert_public_safe_rationale(rationale: &str) {
    assert!(!rationale.contains("candidate"));
    assert!(!rationale.contains("debug"));
    assert!(!rationale.contains("valuation"));
    assert!(!rationale.contains("internal"));
    assert!(!rationale.contains('['));
    assert!(!rationale.contains('{'));
}
