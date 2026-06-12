//! Rule application for Event Frontier card flow.

use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{validate_command, ChoicePosition, EventFrontierAction, ValidatedAction},
    cards::{CardCatalog, CardId},
    effects::{public_effect, EventFrontierEffect, EventFrontierEffectEnvelope},
    ids::FactionId,
    state::{
        epoch_for_card, is_reckoning, CardPhase, Eligibility, EventFrontierState, FirstChoice,
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
            mark_ineligible(state, validated.actor_faction, "event_choice", effects);
            offer_second_or_cleanup(state, validated.actor_faction, FirstChoice::Event, effects);
        }
        EventFrontierAction::OperationPlaceholder => {
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
        EventFrontierAction::LimitedOperationPlaceholder => {
            return Err(Diagnostic {
                code: "action_unavailable".to_owned(),
                message: "limited operations are only available to the second eligible faction"
                    .to_owned(),
            });
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
            mark_ineligible(state, validated.actor_faction, "event_choice", effects);
            advance_to_next_card(state, "resolved_after_second_choice", effects);
        }
        EventFrontierAction::OperationPlaceholder
        | EventFrontierAction::LimitedOperationPlaceholder => {
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

        assert_eq!(state.resources.provisions, 4);
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
                previous: 3,
                new: 4,
                ..
            }
        )));
    }
}
