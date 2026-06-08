use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{actor_seat, collect_gain, parse_action_segment, TokenBazaarAction},
    ids::{CollectBundleId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    state::{contract_spec, ResourceCounts, TokenBazaarState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidatedAction {
    Collect {
        actor: TokenBazaarSeat,
        bundle: CollectBundleId,
    },
    Exchange {
        actor: TokenBazaarSeat,
        pay: ResourceId,
        take: ResourceId,
    },
    Fulfill {
        actor: TokenBazaarSeat,
        slot: TokenBazaarSlot,
    },
    Pass {
        actor: TokenBazaarSeat,
    },
}

pub fn legal_actions(state: &TokenBazaarState) -> Vec<TokenBazaarAction> {
    if state.terminal_outcome.is_some() {
        return Vec::new();
    }

    let mut actions = Vec::new();
    actions.extend(
        CollectBundleId::ALL
            .into_iter()
            .filter(|bundle| can_collect(state, *bundle))
            .map(|bundle| TokenBazaarAction::Collect { bundle }),
    );
    actions.extend(ResourceId::ALL.into_iter().flat_map(|pay| {
        ResourceId::ALL
            .into_iter()
            .filter(move |take| can_exchange(state, pay, *take))
            .map(move |take| TokenBazaarAction::Exchange { pay, take })
    }));
    actions.extend(
        TokenBazaarSlot::ALL
            .into_iter()
            .filter(|slot| can_fulfill(state, *slot))
            .map(|slot| TokenBazaarAction::Fulfill { slot }),
    );

    if actions.is_empty() {
        actions.push(TokenBazaarAction::Pass);
    }

    actions
}

pub fn validate_command(
    state: &TokenBazaarState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if state.terminal_outcome.is_some() {
        return Err(diagnostic(
            "terminal_match",
            "the match is already finished",
        ));
    }

    if command.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "stale_action",
            "the action was submitted for an older decision point",
        ));
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    if actor != state.active_seat {
        return Err(wrong_seat_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(diagnostic(
            "invalid_action_path",
            "the action path is not available",
        ));
    };
    let action = parse_action_segment(segment)
        .ok_or_else(|| diagnostic("invalid_action_path", "the action path is not available"))?;

    match action {
        TokenBazaarAction::Collect { bundle } => validate_collect(state, actor, bundle),
        TokenBazaarAction::Exchange { pay, take } => validate_exchange(state, actor, pay, take),
        TokenBazaarAction::Fulfill { slot } => validate_fulfill(state, actor, slot),
        TokenBazaarAction::Pass => validate_pass(state, actor),
    }
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    diagnostic(
        "not_active_seat",
        "only the active seat may act at this decision point",
    )
}

pub fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

fn validate_collect(
    state: &TokenBazaarState,
    actor: TokenBazaarSeat,
    bundle: CollectBundleId,
) -> Result<ValidatedAction, Diagnostic> {
    if !can_collect(state, bundle) {
        return Err(diagnostic(
            "exhausted_supply",
            "the public supply cannot satisfy that collect bundle",
        ));
    }
    Ok(ValidatedAction::Collect { actor, bundle })
}

fn validate_exchange(
    state: &TokenBazaarState,
    actor: TokenBazaarSeat,
    pay: ResourceId,
    take: ResourceId,
) -> Result<ValidatedAction, Diagnostic> {
    if pay == take {
        return Err(diagnostic(
            "invalid_exchange",
            "exchange must pay and take different resources",
        ));
    }
    let inventory = state.inventory_for(actor);
    if inventory.get(pay) < 2 {
        return Err(diagnostic(
            "insufficient_cost",
            "the active seat lacks the resources required for that exchange",
        ));
    }
    if state.supply.get(take) < 1 {
        return Err(diagnostic(
            "exhausted_supply",
            "the public supply cannot provide the requested exchange resource",
        ));
    }

    Ok(ValidatedAction::Exchange { actor, pay, take })
}

fn validate_fulfill(
    state: &TokenBazaarState,
    actor: TokenBazaarSeat,
    slot: TokenBazaarSlot,
) -> Result<ValidatedAction, Diagnostic> {
    let Some(contract_id) = state.slot_contract(slot) else {
        return Err(diagnostic(
            "empty_slot",
            "the requested market slot is empty",
        ));
    };
    let contract = contract_spec(contract_id);
    if !has_at_least(state.inventory_for(actor), contract.cost) {
        return Err(diagnostic(
            "insufficient_cost",
            "the active seat lacks the resources required for that contract",
        ));
    }

    Ok(ValidatedAction::Fulfill { actor, slot })
}

fn validate_pass(
    state: &TokenBazaarState,
    actor: TokenBazaarSeat,
) -> Result<ValidatedAction, Diagnostic> {
    if legal_actions(state) != [TokenBazaarAction::Pass] {
        return Err(diagnostic(
            "pass_not_forced",
            "pass is legal only when no collect, exchange, or fulfill action is available",
        ));
    }
    Ok(ValidatedAction::Pass { actor })
}

fn can_collect(state: &TokenBazaarState, bundle: CollectBundleId) -> bool {
    has_at_least(state.supply, collect_gain(bundle))
}

fn can_exchange(state: &TokenBazaarState, pay: ResourceId, take: ResourceId) -> bool {
    pay != take
        && state.inventory_for(state.active_seat).get(pay) >= 2
        && state.supply.get(take) >= 1
}

fn can_fulfill(state: &TokenBazaarState, slot: TokenBazaarSlot) -> bool {
    let Some(contract_id) = state.slot_contract(slot) else {
        return false;
    };
    has_at_least(
        state.inventory_for(state.active_seat),
        contract_spec(contract_id).cost,
    )
}

fn has_at_least(available: ResourceCounts, required: ResourceCounts) -> bool {
    ResourceId::ALL
        .iter()
        .all(|resource| available.get(*resource) >= required.get(*resource))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ids::ContractId, setup::setup_match};
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
    };

    fn state() -> TokenBazaarState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn command(
        seat_index: usize,
        segment: &str,
        freshness_token: FreshnessToken,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId(format!("seat-{seat_index}")),
            },
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn legal_actions_cover_collect_exchange_fulfill_in_stable_order() {
        let mut state = state();
        state.inventories[0] = ResourceCounts::new(2, 2, 2);

        let actions = legal_actions(&state);

        assert_eq!(
            actions[..6],
            [
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::Amber
                },
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::Jade
                },
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::Iron
                },
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::AmberJade
                },
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::JadeIron
                },
                TokenBazaarAction::Collect {
                    bundle: CollectBundleId::IronAmber
                },
            ]
        );
        assert!(actions.contains(&TokenBazaarAction::Exchange {
            pay: ResourceId::Amber,
            take: ResourceId::Jade
        }));
        assert!(actions.contains(&TokenBazaarAction::Fulfill {
            slot: TokenBazaarSlot::Slot0
        }));
    }

    #[test]
    fn forced_pass_appears_only_when_stuck() {
        let mut state = state();
        state.supply = ResourceCounts::default();
        state.inventories[0] = ResourceCounts::default();
        state.slots = [None, None, None];

        assert_eq!(legal_actions(&state), vec![TokenBazaarAction::Pass]);

        state.supply = ResourceCounts::new(2, 0, 0);
        assert_ne!(legal_actions(&state), vec![TokenBazaarAction::Pass]);
    }

    #[test]
    fn validate_accepts_each_legal_family() {
        let mut state = state();
        state.inventories[0] = ResourceCounts::new(2, 2, 2);

        assert_eq!(
            validate_command(&state, &command(0, "collect/amber", state.freshness_token)).unwrap(),
            ValidatedAction::Collect {
                actor: TokenBazaarSeat::Seat0,
                bundle: CollectBundleId::Amber
            }
        );
        assert_eq!(
            validate_command(
                &state,
                &command(0, "exchange/amber/jade", state.freshness_token)
            )
            .unwrap(),
            ValidatedAction::Exchange {
                actor: TokenBazaarSeat::Seat0,
                pay: ResourceId::Amber,
                take: ResourceId::Jade
            }
        );
        assert_eq!(
            validate_command(&state, &command(0, "fulfill/slot_0", state.freshness_token)).unwrap(),
            ValidatedAction::Fulfill {
                actor: TokenBazaarSeat::Seat0,
                slot: TokenBazaarSlot::Slot0
            }
        );

        state.supply = ResourceCounts::default();
        state.inventories[0] = ResourceCounts::default();
        state.slots = [None, None, None];
        assert_eq!(
            validate_command(&state, &command(0, "pass", state.freshness_token)).unwrap(),
            ValidatedAction::Pass {
                actor: TokenBazaarSeat::Seat0
            }
        );
    }

    #[test]
    fn invalid_commands_reject_without_mutation() {
        let mut state = state();
        state.inventories[0] = ResourceCounts::new(1, 1, 1);

        let original = state.clone();
        assert_eq!(
            validate_command(&state, &command(0, "collect/amber", FreshnessToken(99)))
                .expect_err("stale")
                .code,
            "stale_action"
        );
        assert_eq!(state, original);

        assert_eq!(
            validate_command(&state, &command(1, "collect/amber", state.freshness_token))
                .expect_err("wrong actor")
                .code,
            "not_active_seat"
        );
        assert_eq!(state, original);

        state.supply = ResourceCounts::new(1, 14, 14);
        assert_eq!(
            validate_command(&state, &command(0, "collect/amber", state.freshness_token))
                .expect_err("exhausted collect")
                .code,
            "exhausted_supply"
        );

        state = original.clone();
        assert_eq!(
            validate_command(
                &state,
                &command(0, "exchange/amber/jade", state.freshness_token)
            )
            .expect_err("insufficient exchange")
            .code,
            "insufficient_cost"
        );

        let mut empty_slot = original.clone();
        empty_slot.slots[0] = None;
        assert_eq!(
            validate_command(
                &empty_slot,
                &command(0, "fulfill/slot_0", empty_slot.freshness_token)
            )
            .expect_err("empty slot")
            .code,
            "empty_slot"
        );

        let mut terminal = original.clone();
        terminal.terminal_outcome = Some(crate::state::TerminalOutcome::Draw);
        assert_eq!(
            validate_command(
                &terminal,
                &command(0, "collect/amber", terminal.freshness_token)
            )
            .expect_err("terminal")
            .code,
            "terminal_match"
        );

        let mut exhausted_exchange = original.clone();
        exhausted_exchange.inventories[0] = ResourceCounts::new(2, 0, 0);
        exhausted_exchange.supply = ResourceCounts::new(14, 0, 14);
        assert_eq!(
            validate_command(
                &exhausted_exchange,
                &command(0, "exchange/amber/jade", exhausted_exchange.freshness_token)
            )
            .expect_err("exhausted exchange")
            .code,
            "exhausted_supply"
        );
    }

    #[test]
    fn fulfill_requires_affordable_contract() {
        let mut state = state();
        state.slots[1] = Some(ContractId::AmberGuild);

        assert_eq!(
            validate_command(&state, &command(0, "fulfill/slot_1", state.freshness_token))
                .expect_err("insufficient contract")
                .code,
            "insufficient_cost"
        );
    }
}
