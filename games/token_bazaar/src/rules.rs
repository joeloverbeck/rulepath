use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{actor_seat, collect_gain, parse_action_segment, TokenBazaarAction},
    effects::{public_effect, TokenBazaarEffect},
    ids::{CollectBundleId, ResourceId, TokenBazaarSeat, TokenBazaarSlot},
    state::{contract_spec, ResourceCounts, TerminalOutcome, TerminalTrigger, TokenBazaarState},
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

pub fn apply_action(
    state: &mut TokenBazaarState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<TokenBazaarEffect>> {
    let actor = action.actor();
    let mut effects = match action {
        ValidatedAction::Collect { actor, bundle } => apply_collect(state, actor, bundle),
        ValidatedAction::Exchange { actor, pay, take } => apply_exchange(state, actor, pay, take),
        ValidatedAction::Fulfill { actor, slot } => apply_fulfill(state, actor, slot),
        ValidatedAction::Pass { actor } => vec![public_effect(TokenBazaarEffect::PassAccepted {
            seat: actor,
        })],
    };

    finish_turn(state, actor, &mut effects);
    effects
}

pub fn determine_terminal_outcome(state: &TokenBazaarState) -> TerminalOutcome {
    if state.scores[0] > state.scores[1] {
        return TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat0,
        };
    }
    if state.scores[1] > state.scores[0] {
        return TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat1,
        };
    }

    let fulfilled_counts = state.fulfilled_counts();
    if fulfilled_counts[0] > fulfilled_counts[1] {
        return TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat0,
        };
    }
    if fulfilled_counts[1] > fulfilled_counts[0] {
        return TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat1,
        };
    }

    let inventory_totals = state.inventory_totals();
    if inventory_totals[0] > inventory_totals[1] {
        TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat0,
        }
    } else if inventory_totals[1] > inventory_totals[0] {
        TerminalOutcome::Win {
            seat: TokenBazaarSeat::Seat1,
        }
    } else {
        TerminalOutcome::Draw
    }
}

pub fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

impl ValidatedAction {
    pub const fn actor(self) -> TokenBazaarSeat {
        match self {
            Self::Collect { actor, .. }
            | Self::Exchange { actor, .. }
            | Self::Fulfill { actor, .. }
            | Self::Pass { actor } => actor,
        }
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

fn apply_collect(
    state: &mut TokenBazaarState,
    actor: TokenBazaarSeat,
    bundle: CollectBundleId,
) -> Vec<EffectEnvelope<TokenBazaarEffect>> {
    let gain = collect_gain(bundle);
    subtract_counts(&mut state.supply, gain);
    add_counts(&mut state.inventories[actor.index()], gain);

    vec![public_effect(TokenBazaarEffect::ResourceCollected {
        seat: actor,
        bundle,
        gain,
        inventory_after: state.inventories[actor.index()],
        supply_after: state.supply,
    })]
}

fn apply_exchange(
    state: &mut TokenBazaarState,
    actor: TokenBazaarSeat,
    pay: ResourceId,
    take: ResourceId,
) -> Vec<EffectEnvelope<TokenBazaarEffect>> {
    let mut cost = ResourceCounts::default();
    cost.set(pay, 2);
    let mut gain = ResourceCounts::default();
    gain.set(take, 1);

    subtract_counts(&mut state.inventories[actor.index()], cost);
    add_counts(&mut state.supply, cost);
    subtract_counts(&mut state.supply, gain);
    add_counts(&mut state.inventories[actor.index()], gain);

    vec![public_effect(TokenBazaarEffect::ResourceExchanged {
        seat: actor,
        paid_resource: pay,
        taken_resource: take,
        cost,
        gain,
        inventory_after: state.inventories[actor.index()],
        supply_after: state.supply,
    })]
}

fn apply_fulfill(
    state: &mut TokenBazaarState,
    actor: TokenBazaarSeat,
    slot: TokenBazaarSlot,
) -> Vec<EffectEnvelope<TokenBazaarEffect>> {
    let contract_id = state
        .slot_contract(slot)
        .expect("validated fulfill action must target occupied slot");
    let contract = contract_spec(contract_id);

    subtract_counts(&mut state.inventories[actor.index()], contract.cost);
    add_counts(&mut state.supply, contract.cost);
    state.scores[actor.index()] += u32::from(contract.points);
    state.fulfilled[actor.index()].push(contract_id);
    state.slots[slot.index()] = None;

    let mut effects = vec![public_effect(TokenBazaarEffect::ContractFulfilled {
        seat: actor,
        slot,
        contract: contract_id,
        cost: contract.cost,
        points: contract.points,
        score_after: state.scores[actor.index()],
        fulfilled_count_after: state.fulfilled[actor.index()].len() as u8,
    })];

    if let Some(next_contract) = state.queue.first().copied() {
        state.queue.remove(0);
        state.slots[slot.index()] = Some(next_contract);
        effects.push(public_effect(TokenBazaarEffect::SlotRefilled {
            slot,
            contract: next_contract,
            remaining_queue_len: state.queue.len() as u8,
        }));
    } else {
        effects.push(public_effect(TokenBazaarEffect::SlotEmptied {
            slot,
            remaining_queue_len: 0,
        }));
    }

    effects
}

fn finish_turn(
    state: &mut TokenBazaarState,
    actor: TokenBazaarSeat,
    effects: &mut Vec<EffectEnvelope<TokenBazaarEffect>>,
) {
    state.turns_taken[actor.index()] = state.turns_taken[actor.index()].saturating_add(1);
    state.freshness_token = state.freshness_token.next();

    if let Some(trigger) = terminal_trigger(state) {
        let outcome = determine_terminal_outcome(state);
        state.terminal_outcome = Some(outcome);
        state.terminal_trigger = Some(trigger);
        effects.push(public_effect(TokenBazaarEffect::Terminal {
            outcome,
            scores: state.scores,
            fulfilled_counts: state.fulfilled_counts(),
            inventory_totals: state.inventory_totals(),
        }));
        return;
    }

    let previous_seat = actor;
    state.active_seat = actor.other();
    effects.push(public_effect(TokenBazaarEffect::TurnAdvanced {
        previous_seat,
        active_seat: state.active_seat,
        turns_taken: state.turns_taken,
    }));
}

fn terminal_trigger(state: &TokenBazaarState) -> Option<TerminalTrigger> {
    if terminal_by_turn_cap(state) {
        Some(TerminalTrigger::TurnCap)
    } else if terminal_by_market_exhaustion(state) {
        Some(TerminalTrigger::MarketExhaustion)
    } else {
        None
    }
}

fn terminal_by_turn_cap(state: &TokenBazaarState) -> bool {
    state
        .turns_taken
        .iter()
        .all(|turns| *turns >= state.variant.turns_per_seat)
}

fn terminal_by_market_exhaustion(state: &TokenBazaarState) -> bool {
    state.queue.is_empty() && state.slots.iter().all(Option::is_none)
}

fn add_counts(target: &mut ResourceCounts, delta: ResourceCounts) {
    target.amber = target.amber.saturating_add(delta.amber);
    target.jade = target.jade.saturating_add(delta.jade);
    target.iron = target.iron.saturating_add(delta.iron);
}

fn subtract_counts(target: &mut ResourceCounts, delta: ResourceCounts) {
    target.amber = target
        .amber
        .checked_sub(delta.amber)
        .expect("validated action cannot underflow amber");
    target.jade = target
        .jade
        .checked_sub(delta.jade)
        .expect("validated action cannot underflow jade");
    target.iron = target
        .iron
        .checked_sub(delta.iron)
        .expect("validated action cannot underflow iron");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{effects::TokenBazaarEffect, ids::ContractId, setup::setup_match};
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
    };

    fn state() -> TokenBazaarState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn resource_total(state: &TokenBazaarState) -> u16 {
        state.supply.total() + state.inventories[0].total() + state.inventories[1].total()
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

    #[test]
    fn apply_collect_updates_accounting_and_turn() {
        let mut state = state();

        let effects = apply_action(
            &mut state,
            ValidatedAction::Collect {
                actor: TokenBazaarSeat::Seat0,
                bundle: CollectBundleId::AmberJade,
            },
        );

        assert_eq!(state.supply, ResourceCounts::new(13, 13, 14));
        assert_eq!(state.inventories[0], ResourceCounts::new(2, 2, 1));
        assert_eq!(state.active_seat, TokenBazaarSeat::Seat1);
        assert_eq!(state.turns_taken, [1, 0]);
        assert_eq!(resource_total(&state), 48);
        assert!(matches!(
            effects[0].payload,
            TokenBazaarEffect::ResourceCollected {
                bundle: CollectBundleId::AmberJade,
                inventory_after: ResourceCounts {
                    amber: 2,
                    jade: 2,
                    iron: 1
                },
                supply_after: ResourceCounts {
                    amber: 13,
                    jade: 13,
                    iron: 14
                },
                ..
            }
        ));
        assert!(matches!(
            effects[1].payload,
            TokenBazaarEffect::TurnAdvanced {
                active_seat: TokenBazaarSeat::Seat1,
                turns_taken: [1, 0],
                ..
            }
        ));
    }

    #[test]
    fn apply_exchange_returns_paid_supply_and_takes_requested_resource() {
        let mut state = state();
        state.inventories[0] = ResourceCounts::new(2, 1, 1);
        state.supply = ResourceCounts::new(13, 14, 14);

        let effects = apply_action(
            &mut state,
            ValidatedAction::Exchange {
                actor: TokenBazaarSeat::Seat0,
                pay: ResourceId::Amber,
                take: ResourceId::Iron,
            },
        );

        assert_eq!(state.inventories[0], ResourceCounts::new(0, 1, 2));
        assert_eq!(state.supply, ResourceCounts::new(15, 14, 13));
        assert_eq!(state.turns_taken, [1, 0]);
        assert_eq!(resource_total(&state), 48);
        assert!(matches!(
            effects[0].payload,
            TokenBazaarEffect::ResourceExchanged {
                paid_resource: ResourceId::Amber,
                taken_resource: ResourceId::Iron,
                inventory_after: ResourceCounts {
                    amber: 0,
                    jade: 1,
                    iron: 2
                },
                supply_after: ResourceCounts {
                    amber: 15,
                    jade: 14,
                    iron: 13
                },
                ..
            }
        ));
    }

    #[test]
    fn apply_fulfill_scores_refills_and_advances() {
        let mut state = state();

        let effects = apply_action(
            &mut state,
            ValidatedAction::Fulfill {
                actor: TokenBazaarSeat::Seat0,
                slot: TokenBazaarSlot::Slot0,
            },
        );

        assert_eq!(state.inventories[0], ResourceCounts::default());
        assert_eq!(state.supply, ResourceCounts::new(15, 15, 15));
        assert_eq!(state.scores, [3, 0]);
        assert_eq!(state.fulfilled[0], vec![ContractId::BalancedWares]);
        assert_eq!(state.slots[0], Some(ContractId::JadeGuild));
        assert_eq!(state.queue.len(), 6);
        assert_eq!(state.active_seat, TokenBazaarSeat::Seat1);
        assert_eq!(resource_total(&state), 48);
        assert!(matches!(
            effects[0].payload,
            TokenBazaarEffect::ContractFulfilled {
                contract: ContractId::BalancedWares,
                score_after: 3,
                fulfilled_count_after: 1,
                ..
            }
        ));
        assert!(matches!(
            effects[1].payload,
            TokenBazaarEffect::SlotRefilled {
                contract: ContractId::JadeGuild,
                remaining_queue_len: 6,
                ..
            }
        ));
    }

    #[test]
    fn empty_slot_when_queue_exhausted_and_terminal_when_market_empty() {
        let mut state = state();
        state.slots = [Some(ContractId::BalancedWares), None, None];
        state.queue.clear();

        let effects = apply_action(
            &mut state,
            ValidatedAction::Fulfill {
                actor: TokenBazaarSeat::Seat0,
                slot: TokenBazaarSlot::Slot0,
            },
        );

        assert_eq!(state.slots, [None, None, None]);
        assert_eq!(
            state.terminal_outcome,
            Some(TerminalOutcome::Win {
                seat: TokenBazaarSeat::Seat0
            })
        );
        assert_eq!(
            state.terminal_trigger,
            Some(TerminalTrigger::MarketExhaustion)
        );
        assert!(matches!(
            effects[1].payload,
            TokenBazaarEffect::SlotEmptied {
                slot: TokenBazaarSlot::Slot0,
                remaining_queue_len: 0
            }
        ));
        assert!(matches!(
            effects[2].payload,
            TokenBazaarEffect::Terminal { .. }
        ));
        assert!(!effects
            .iter()
            .any(|effect| matches!(effect.payload, TokenBazaarEffect::TurnAdvanced { .. })));
    }

    #[test]
    fn turn_cap_terminal_uses_tie_breaks() {
        let mut state = state();
        state.supply = ResourceCounts::default();
        state.inventories[0] = ResourceCounts::default();
        state.inventories[1] = ResourceCounts::default();
        state.slots = [None, None, None];
        state.turns_taken = [7, 8];

        let effects = apply_action(
            &mut state,
            ValidatedAction::Pass {
                actor: TokenBazaarSeat::Seat0,
            },
        );

        assert_eq!(state.turns_taken, [8, 8]);
        assert_eq!(state.terminal_outcome, Some(TerminalOutcome::Draw));
        assert_eq!(state.terminal_trigger, Some(TerminalTrigger::TurnCap));
        assert!(matches!(
            effects.last().expect("terminal effect").payload,
            TokenBazaarEffect::Terminal {
                outcome: TerminalOutcome::Draw,
                ..
            }
        ));
    }

    #[test]
    fn terminal_tie_break_order_is_score_fulfilled_inventory_draw() {
        let mut score = state();
        score.scores = [4, 3];
        assert_eq!(
            determine_terminal_outcome(&score),
            TerminalOutcome::Win {
                seat: TokenBazaarSeat::Seat0
            }
        );

        let mut fulfilled = state();
        fulfilled.fulfilled[1].push(ContractId::BalancedWares);
        assert_eq!(
            determine_terminal_outcome(&fulfilled),
            TerminalOutcome::Win {
                seat: TokenBazaarSeat::Seat1
            }
        );

        let mut inventory = state();
        inventory.inventories[1] = ResourceCounts::new(2, 1, 1);
        assert_eq!(
            determine_terminal_outcome(&inventory),
            TerminalOutcome::Win {
                seat: TokenBazaarSeat::Seat1
            }
        );

        assert_eq!(determine_terminal_outcome(&state()), TerminalOutcome::Draw);
    }
}
