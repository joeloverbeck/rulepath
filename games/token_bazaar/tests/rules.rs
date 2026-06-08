use std::collections::BTreeSet;

use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed};
use token_bazaar::{
    apply_action, legal_action_tree, setup_match, validate_command, ContractId, ResourceCounts,
    TerminalOutcome, TokenBazaarSeat, TokenBazaarSlot,
};

fn state() -> token_bazaar::TokenBazaarState {
    let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
    setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
}

fn command(state: &token_bazaar::TokenBazaarState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn invalid_commands_reject_without_mutation() {
    let state = state();
    let original = token_bazaar::TokenBazaarSnapshot::from_state(&state).stable_summary();

    let stale = CommandEnvelope {
        freshness_token: FreshnessToken(99),
        ..command(&state, "collect/amber")
    };
    assert_eq!(
        validate_command(&state, &stale)
            .expect_err("stale command")
            .code,
        "stale_action"
    );
    assert_eq!(
        token_bazaar::TokenBazaarSnapshot::from_state(&state).stable_summary(),
        original
    );
}

#[test]
fn fulfilled_contract_is_removed_from_visible_and_queued_market() {
    let mut state = state();
    let action = validate_command(&state, &command(&state, "fulfill/slot_0")).unwrap();
    apply_action(&mut state, action);

    assert_eq!(state.fulfilled[0], vec![ContractId::BalancedWares]);
    assert!(!state.slots.contains(&Some(ContractId::BalancedWares)));
    assert!(!state.queue.contains(&ContractId::BalancedWares));
}

#[test]
fn terminal_exposes_no_normal_actions() {
    let mut state = state();
    state.terminal_outcome = Some(TerminalOutcome::Draw);

    let tree = legal_action_tree(
        &state,
        &Actor {
            seat_id: state.seats[0].clone(),
        },
    );

    assert!(tree.root.choices.is_empty());
}

#[test]
fn action_ids_are_stable_and_duplicate_free() {
    let mut state = state();
    state.inventories[0] = ResourceCounts::new(2, 2, 2);
    let tree = legal_action_tree(
        &state,
        &Actor {
            seat_id: state.seats[0].clone(),
        },
    );
    let ids = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>();
    let unique = ids.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(ids.len(), unique.len());
    assert!(ids.starts_with(&[
        "collect/amber",
        "collect/jade",
        "collect/iron",
        "collect/amber-jade",
        "collect/jade-iron",
        "collect/iron-amber",
    ]));
}

#[test]
fn empty_slot_rejects_fulfill() {
    let mut state = state();
    state.slots[TokenBazaarSlot::Slot0.index()] = None;

    assert_eq!(
        validate_command(&state, &command(&state, "fulfill/slot_0"))
            .expect_err("empty slot")
            .code,
        "empty_slot"
    );
}

#[test]
fn wrong_actor_rejects() {
    let state = state();
    let command = CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[TokenBazaarSeat::Seat1.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec!["collect/amber".to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };

    assert_eq!(
        validate_command(&state, &command)
            .expect_err("wrong actor")
            .code,
        "not_active_seat"
    );
}
