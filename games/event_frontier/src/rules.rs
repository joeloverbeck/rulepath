//! Rule application for Event Frontier card flow.

use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{
        operation_cost, validate_command, ChoicePosition, EventFrontierAction, OperationKind,
        OperationSelection, ValidatedAction,
    },
    cards::{expire_all_edicts, resolve_event_card, CardCatalog, CardId},
    effects::{
        public_effect, EventFrontierEffect, EventFrontierEffectEnvelope, SiteScoreBreakdown,
    },
    ids::{FactionId, SiteId},
    state::{
        epoch_for_card, is_reckoning, CardPhase, Eligibility, EventFrontierState, FirstChoice,
        TerminalOutcome, VictoryType,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppliedAction {
    pub effects: Vec<EventFrontierEffectEnvelope>,
}

pub fn initialize_card_phase(state: &mut EventFrontierState) -> Result<(), Diagnostic> {
    match state.deck.current {
        None => {
            state.card_phase = CardPhase::Terminal;
            Ok(())
        }
        Some(card) if is_reckoning(card) => {
            state.card_phase = CardPhase::Reckoning;
            Ok(())
        }
        Some(card) => {
            let faction = first_eligible_faction(state, card).ok_or_else(|| Diagnostic {
                code: "no_eligible_faction_at_setup".to_owned(),
                message: "event_frontier setup cannot begin with no eligible faction".to_owned(),
            })?;
            state.card_phase = CardPhase::AwaitingFirstChoice { faction };
            Ok(())
        }
    }
}

pub fn apply_command(
    state: &mut EventFrontierState,
    command: &CommandEnvelope,
) -> Result<AppliedAction, Diagnostic> {
    let validated = validate_command(state, command)?;
    apply_validated_action(state, validated)
}

pub fn apply_validated_action(
    state: &mut EventFrontierState,
    validated: ValidatedAction,
) -> Result<AppliedAction, Diagnostic> {
    let mut effects = Vec::new();
    let choice_label = validated.action.as_choice_label().to_owned();
    effects.push(public_effect(EventFrontierEffect::ChoiceTaken {
        faction: validated.actor_faction,
        choice: choice_label,
    }));

    match validated.position {
        ChoicePosition::First => apply_first_choice(state, validated, &mut effects)?,
        ChoicePosition::Second { first_choice } => {
            apply_second_choice(state, validated, first_choice, &mut effects)?
        }
    }

    state.freshness_token = state.freshness_token.next();
    Ok(AppliedAction { effects })
}

pub fn resolve_reckoning(state: &mut EventFrontierState) -> Result<AppliedAction, Diagnostic> {
    if state.card_phase != CardPhase::Reckoning || state.deck.current.is_none() {
        return Err(Diagnostic {
            code: "wrong_phase".to_owned(),
            message: "no Event Frontier Reckoning is ready to resolve".to_owned(),
        });
    }

    let round = state.reckoning_count.saturating_add(1);
    let mut effects = Vec::new();
    let instant = instant_victory(state);
    if let Some((winner, victory_type, decisive_rule, summary)) = instant {
        state.reckoning_count = round;
        set_terminal(state, winner, victory_type, decisive_rule);
        effects.push(public_effect(EventFrontierEffect::ReckoningResolved {
            round,
            victory_check: summary.clone(),
            site_breakdown: Vec::new(),
            income: (0, 0),
            expired_edicts: Vec::new(),
        }));
        push_terminal_effect(state, winner, victory_type, summary, &mut effects);
        state.freshness_token = state.freshness_token.next();
        return Ok(AppliedAction { effects });
    }

    let site_breakdown = score_sites(state);
    let income = apply_reckoning_income(state, &mut effects);
    let expiry_effects = expire_all_edicts(state);
    let expired_edicts = expiry_effects
        .iter()
        .filter_map(|effect| match &effect.payload {
            EventFrontierEffect::EdictExpired { edict } => Some(edict.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    effects.extend(expiry_effects);
    restore_all_eligibility(state, "reckoning_reset", &mut effects);
    state.reckoning_count = round;

    effects.push(public_effect(EventFrontierEffect::ReckoningResolved {
        round,
        victory_check: "none".to_owned(),
        site_breakdown,
        income,
        expired_edicts,
    }));

    if round >= 3 {
        let (winner, decisive_rule, summary) = final_fallback(state);
        set_terminal(state, winner, VictoryType::FinalFallback, decisive_rule);
        push_terminal_effect(
            state,
            winner,
            VictoryType::FinalFallback,
            summary,
            &mut effects,
        );
    } else {
        advance_to_next_card(state, "reckoning_resolved", &mut effects);
    }

    state.freshness_token = state.freshness_token.next();
    Ok(AppliedAction { effects })
}

pub fn advance_to_next_card(
    state: &mut EventFrontierState,
    reason: &str,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if let Some(card) = state.deck.current.take() {
        state.deck.discard.push(card);
        effects.push(public_effect(EventFrontierEffect::CardDiscarded {
            card,
            reason: reason.to_owned(),
        }));
    }

    state.deck.current = state.deck.next_public.take();
    state.deck.next_public = if state.deck.undrawn.is_empty() {
        None
    } else {
        Some(state.deck.undrawn.remove(0))
    };
    state.deck.epoch = state.deck.current.map(epoch_for_card).unwrap_or(0);

    if let Some(card) = state.deck.current {
        effects.push(public_effect(EventFrontierEffect::CardRevealed {
            card,
            next_public: state.deck.next_public,
        }));
    }

    prepare_current_card_after_advance(state, effects);
}

fn apply_first_choice(
    state: &mut EventFrontierState,
    validated: ValidatedAction,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> Result<(), Diagnostic> {
    match validated.action {
        EventFrontierAction::Event => {
            if let Some(card) = state.deck.current {
                effects.extend(resolve_event_card(state, card));
            }
            mark_ineligible(state, validated.actor_faction, "event_choice", effects);
            offer_second_or_cleanup(state, validated.actor_faction, FirstChoice::Event, effects);
        }
        EventFrontierAction::Operation {
            kind, selections, ..
        } => {
            apply_operation(state, validated.actor_faction, kind, &selections, effects)?;
            mark_ineligible(state, validated.actor_faction, "operation_choice", effects);
            offer_second_or_cleanup(
                state,
                validated.actor_faction,
                FirstChoice::Operation,
                effects,
            );
        }
        EventFrontierAction::Pass => {
            apply_pass_income(state, validated.actor_faction, effects);
            offer_second_or_cleanup(state, validated.actor_faction, FirstChoice::Pass, effects);
        }
    }
    Ok(())
}

fn apply_second_choice(
    state: &mut EventFrontierState,
    validated: ValidatedAction,
    first_choice: FirstChoice,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> Result<(), Diagnostic> {
    match validated.action {
        EventFrontierAction::Event => {
            if let Some(card) = state.deck.current {
                effects.extend(resolve_event_card(state, card));
            }
            mark_ineligible(state, validated.actor_faction, "event_choice", effects);
            advance_to_next_card(state, "resolved_after_second_choice", effects);
        }
        EventFrontierAction::Operation {
            kind, selections, ..
        } => {
            apply_operation(state, validated.actor_faction, kind, &selections, effects)?;
            mark_ineligible(state, validated.actor_faction, "operation_choice", effects);
            advance_to_next_card(state, "resolved_after_second_choice", effects);
        }
        EventFrontierAction::Pass => {
            apply_pass_income(state, validated.actor_faction, effects);
            let reason = if first_choice == FirstChoice::Pass {
                "double_pass"
            } else {
                "resolved_after_second_pass"
            };
            advance_to_next_card(state, reason, effects);
        }
    }
    Ok(())
}

fn apply_operation(
    state: &mut EventFrontierState,
    faction: FactionId,
    kind: OperationKind,
    selections: &[OperationSelection],
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> Result<(), Diagnostic> {
    spend_operation_cost(
        state,
        faction,
        operation_cost(state, faction, kind, selections),
        effects,
    );
    let mut sorted = selections.to_vec();
    sorted.sort();
    effects.push(public_effect(EventFrontierEffect::OpResolved {
        faction,
        op: kind.as_str().to_owned(),
        sites: sorted.iter().map(|selection| selection.site).collect(),
    }));
    for selection in sorted {
        apply_operation_selection(state, faction, kind, selection, effects)?;
    }
    Ok(())
}

fn instant_victory(
    state: &EventFrontierState,
) -> Option<(FactionId, VictoryType, &'static str, String)> {
    let charter_sites = charter_majority_site_count(state);
    let freeholder_caches = freeholder_cache_count(state);
    let charter_met = charter_sites >= state.variant.charter_site_threshold;
    let freeholder_met = freeholder_caches >= state.variant.freeholder_cache_threshold;

    match (charter_met, freeholder_met) {
        (true, true) => Some((
            FactionId::Freeholders,
            VictoryType::FreeholderInstant,
            "EF-END-003",
            format!("both_met:charter_sites={charter_sites}:freeholder_caches={freeholder_caches}"),
        )),
        (true, false) => Some((
            FactionId::Charter,
            VictoryType::CharterInstant,
            "EF-END-001",
            format!("charter_instant:charter_sites={charter_sites}"),
        )),
        (false, true) => Some((
            FactionId::Freeholders,
            VictoryType::FreeholderInstant,
            "EF-END-002",
            format!("freeholder_instant:caches={freeholder_caches}"),
        )),
        (false, false) => None,
    }
}

fn score_sites(state: &mut EventFrontierState) -> Vec<SiteScoreBreakdown> {
    let mut breakdown = Vec::new();
    for site_id in SiteId::ALL {
        let Some(site) = state.site(site_id) else {
            continue;
        };
        let charter_presence = charter_presence(site);
        let freeholder_presence = site.settlers;
        let awarded_to = if charter_presence > freeholder_presence {
            state.scores.charter = state.scores.charter.saturating_add(1);
            Some(FactionId::Charter)
        } else if freeholder_presence > charter_presence {
            state.scores.freeholders = state.scores.freeholders.saturating_add(1);
            Some(FactionId::Freeholders)
        } else {
            None
        };
        breakdown.push(SiteScoreBreakdown {
            site: site_id,
            charter_presence,
            freeholder_presence,
            awarded_to,
        });
    }
    breakdown
}

fn apply_reckoning_income(
    state: &mut EventFrontierState,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> (u8, u8) {
    let previous_funds = state.resources.funds;
    let previous_provisions = state.resources.provisions;
    let cap = state.variant.resource_cap;
    state.resources.funds = state.resources.funds.saturating_add(2).min(cap);
    state.resources.provisions = state.resources.provisions.saturating_add(2).min(cap);
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction: FactionId::Charter,
        previous: previous_funds,
        new: state.resources.funds,
        reason: "reckoning_income".to_owned(),
    }));
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction: FactionId::Freeholders,
        previous: previous_provisions,
        new: state.resources.provisions,
        reason: "reckoning_income".to_owned(),
    }));
    (state.resources.funds, state.resources.provisions)
}

fn final_fallback(state: &EventFrontierState) -> (FactionId, &'static str, String) {
    if state.scores.charter > state.scores.freeholders {
        (
            FactionId::Charter,
            "EF-END-004",
            format!(
                "final_fallback:charter:{}:freeholders:{}",
                state.scores.charter, state.scores.freeholders
            ),
        )
    } else {
        (
            FactionId::Freeholders,
            "EF-END-004",
            format!(
                "final_fallback_freeholder_tiebreak:charter:{}:freeholders:{}",
                state.scores.charter, state.scores.freeholders
            ),
        )
    }
}

fn set_terminal(
    state: &mut EventFrontierState,
    winner: FactionId,
    victory_type: VictoryType,
    decisive_rule: &'static str,
) {
    state.card_phase = CardPhase::Terminal;
    state.terminal_outcome = Some(TerminalOutcome::Winner {
        faction: winner,
        victory_type,
        scores: state.scores,
        decisive_rule,
    });
}

fn push_terminal_effect(
    state: &EventFrontierState,
    winner: FactionId,
    victory_type: VictoryType,
    summary: String,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    effects.push(public_effect(EventFrontierEffect::Terminal {
        winner,
        victory_type: victory_type.as_str().to_owned(),
        totals: (state.scores.charter, state.scores.freeholders),
        summary,
    }));
}

fn charter_majority_site_count(state: &EventFrontierState) -> u8 {
    state
        .sites
        .iter()
        .filter(|site| charter_presence(site) > site.settlers)
        .count() as u8
}

fn freeholder_cache_count(state: &EventFrontierState) -> u8 {
    state
        .sites
        .iter()
        .map(|site| site.cache_count)
        .fold(0u8, u8::saturating_add)
}

fn charter_presence(site: &crate::state::SiteState) -> u8 {
    site.agents.saturating_add(u8::from(site.depot))
}

fn spend_operation_cost(
    state: &mut EventFrontierState,
    faction: FactionId,
    cost: u8,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let (previous, new) = match faction {
        FactionId::Charter => {
            let previous = state.resources.funds;
            state.resources.funds = state.resources.funds.saturating_sub(cost);
            (previous, state.resources.funds)
        }
        FactionId::Freeholders => {
            let previous = state.resources.provisions;
            state.resources.provisions = state.resources.provisions.saturating_sub(cost);
            (previous, state.resources.provisions)
        }
    };
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction,
        previous,
        new,
        reason: "operation_cost".to_owned(),
    }));
}

fn apply_operation_selection(
    state: &mut EventFrontierState,
    faction: FactionId,
    kind: OperationKind,
    selection: OperationSelection,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> Result<(), Diagnostic> {
    match kind {
        OperationKind::Survey => {
            let site = state
                .site_mut(selection.site)
                .ok_or_else(action_unavailable)?;
            site.agents = site.agents.saturating_add(1).min(3);
            effects.push(public_effect(EventFrontierEffect::AgentPlaced {
                site: selection.site,
                new_count: site.agents,
            }));
        }
        OperationKind::Fortify => {
            let site = state
                .site_mut(selection.site)
                .ok_or_else(action_unavailable)?;
            site.depot = true;
            effects.push(public_effect(EventFrontierEffect::DepotBuilt {
                site: selection.site,
            }));
        }
        OperationKind::Writ => {
            let site = state
                .site_mut(selection.site)
                .ok_or_else(action_unavailable)?;
            site.cache_count = site.cache_count.saturating_sub(1);
            effects.push(public_effect(EventFrontierEffect::CacheRemoved {
                site: selection.site,
                new_count: site.cache_count,
            }));
            gain_resource(state, faction, 1, "writ_gain", effects);
        }
        OperationKind::Trek => {
            let destination = selection.destination.ok_or_else(action_unavailable)?;
            move_settler(state, selection.site, destination, effects)?;
        }
        OperationKind::Cache => {
            let site = state
                .site_mut(selection.site)
                .ok_or_else(action_unavailable)?;
            site.cache_count = site.cache_count.saturating_add(1).min(2);
            effects.push(public_effect(EventFrontierEffect::CacheLaid {
                site: selection.site,
                new_count: site.cache_count,
            }));
        }
        OperationKind::Rally => {
            let site = state
                .site_mut(selection.site)
                .ok_or_else(action_unavailable)?;
            site.settlers = site.settlers.saturating_add(1).min(3);
            effects.push(public_effect(EventFrontierEffect::SettlerRallied {
                site: selection.site,
                new_count: site.settlers,
            }));
        }
    }
    Ok(())
}

fn move_settler(
    state: &mut EventFrontierState,
    from: crate::SiteId,
    to: crate::SiteId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) -> Result<(), Diagnostic> {
    {
        let from_site = state.site_mut(from).ok_or_else(action_unavailable)?;
        from_site.settlers = from_site.settlers.saturating_sub(1);
    }
    let from_count = state.site(from).map(|site| site.settlers).unwrap_or(0);
    let to_count = {
        let to_site = state.site_mut(to).ok_or_else(action_unavailable)?;
        to_site.settlers = to_site.settlers.saturating_add(1).min(3);
        to_site.settlers
    };
    effects.push(public_effect(EventFrontierEffect::SettlerMoved {
        from,
        to,
        from_count,
        to_count,
    }));
    Ok(())
}

fn gain_resource(
    state: &mut EventFrontierState,
    faction: FactionId,
    amount: u8,
    reason: &str,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let cap = state.variant.resource_cap;
    let (previous, new) = match faction {
        FactionId::Charter => {
            let previous = state.resources.funds;
            state.resources.funds = state.resources.funds.saturating_add(amount).min(cap);
            (previous, state.resources.funds)
        }
        FactionId::Freeholders => {
            let previous = state.resources.provisions;
            state.resources.provisions = state.resources.provisions.saturating_add(amount).min(cap);
            (previous, state.resources.provisions)
        }
    };
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction,
        previous,
        new,
        reason: reason.to_owned(),
    }));
}

fn action_unavailable() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "that Event Frontier action is not available now".to_owned(),
    }
}

fn offer_second_or_cleanup(
    state: &mut EventFrontierState,
    first_faction: FactionId,
    first_choice: FirstChoice,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let second_faction = other_faction(first_faction);
    if state.eligibility_for(second_faction) == Eligibility::Eligible {
        state.card_phase = CardPhase::AwaitingSecondChoice {
            first_faction,
            second_faction,
            first_choice,
        };
    } else {
        advance_to_next_card(state, "no_second_eligible_faction", effects);
    }
}

fn prepare_current_card_after_advance(
    state: &mut EventFrontierState,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    loop {
        let Some(card) = state.deck.current else {
            state.card_phase = CardPhase::Terminal;
            return;
        };
        if is_reckoning(card) {
            state.card_phase = CardPhase::Reckoning;
            return;
        }
        if let Some(faction) = first_eligible_faction(state, card) {
            state.card_phase = CardPhase::AwaitingFirstChoice { faction };
            return;
        }

        restore_all_eligibility(state, "no_eligible_card_discard", effects);
        if let Some(discarded) = state.deck.current.take() {
            state.deck.discard.push(discarded);
            effects.push(public_effect(EventFrontierEffect::CardDiscarded {
                card: discarded,
                reason: "no_eligible_faction".to_owned(),
            }));
        }
        state.deck.current = state.deck.next_public.take();
        state.deck.next_public = if state.deck.undrawn.is_empty() {
            None
        } else {
            Some(state.deck.undrawn.remove(0))
        };
        state.deck.epoch = state.deck.current.map(epoch_for_card).unwrap_or(0);
        if let Some(revealed) = state.deck.current {
            effects.push(public_effect(EventFrontierEffect::CardRevealed {
                card: revealed,
                next_public: state.deck.next_public,
            }));
        }
    }
}

fn first_eligible_faction(state: &EventFrontierState, card: CardId) -> Option<FactionId> {
    let printed = printed_first_eligible(card)?;
    if state.eligibility_for(printed) == Eligibility::Eligible {
        return Some(printed);
    }
    let other = other_faction(printed);
    (state.eligibility_for(other) == Eligibility::Eligible).then_some(other)
}

fn printed_first_eligible(card: CardId) -> Option<FactionId> {
    let catalog = CardCatalog::parse(include_str!("../data/cards.toml")).ok()?;
    catalog
        .cards
        .iter()
        .find_map(|data| (data.id == card).then_some(data.first_eligible))
}

fn other_faction(faction: FactionId) -> FactionId {
    match faction {
        FactionId::Charter => FactionId::Freeholders,
        FactionId::Freeholders => FactionId::Charter,
    }
}

fn mark_ineligible(
    state: &mut EventFrontierState,
    faction: FactionId,
    reason: &str,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    if state.eligibility_for(faction) == Eligibility::Ineligible {
        return;
    }
    state.set_eligibility(faction, Eligibility::Ineligible);
    effects.push(public_effect(EventFrontierEffect::EligibilityChanged {
        faction,
        eligible: false,
        reason: reason.to_owned(),
    }));
}

fn restore_all_eligibility(
    state: &mut EventFrontierState,
    reason: &str,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    for faction in FactionId::ALL {
        if state.eligibility_for(faction) == Eligibility::Ineligible {
            state.set_eligibility(faction, Eligibility::Eligible);
            effects.push(public_effect(EventFrontierEffect::EligibilityChanged {
                faction,
                eligible: true,
                reason: reason.to_owned(),
            }));
        }
    }
}

fn apply_pass_income(
    state: &mut EventFrontierState,
    faction: FactionId,
    effects: &mut Vec<EventFrontierEffectEnvelope>,
) {
    let cap = state.variant.resource_cap;
    let (previous, new) = match faction {
        FactionId::Charter => {
            let previous = state.resources.funds;
            state.resources.funds = state.resources.funds.saturating_add(1).min(cap);
            (previous, state.resources.funds)
        }
        FactionId::Freeholders => {
            let previous = state.resources.provisions;
            state.resources.provisions = state.resources.provisions.saturating_add(1).min(cap);
            (previous, state.resources.provisions)
        }
    };
    effects.push(public_effect(EventFrontierEffect::ResourcesChanged {
        faction,
        previous,
        new,
        reason: "pass_income".to_owned(),
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actions::ACTION_PASS, setup_match, SetupOptions};
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
    };

    fn seats() -> [SeatId; 2] {
        [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
    }

    fn command(seat: &str, segment: &str, freshness_token: FreshnessToken) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId(seat.to_owned()),
            },
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn pass_income_preserves_eligibility_and_advances_to_second_choice() {
        let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).expect("setup");
        let freshness_token = state.freshness_token;
        let result = apply_command(&mut state, &command("seat_1", ACTION_PASS, freshness_token))
            .expect("pass applies");

        assert_eq!(state.resources.provisions, 5);
        assert_eq!(
            state.eligibility_for(FactionId::Freeholders),
            Eligibility::Eligible
        );
        assert_eq!(
            state.card_phase,
            CardPhase::AwaitingSecondChoice {
                first_faction: FactionId::Freeholders,
                second_faction: FactionId::Charter,
                first_choice: FirstChoice::Pass,
            }
        );
        assert!(result.effects.iter().any(|effect| matches!(
            effect.payload,
            EventFrontierEffect::ResourcesChanged {
                faction: FactionId::Freeholders,
                previous: 4,
                new: 5,
                ..
            }
        )));
    }
}
