use engine_core::{Diagnostic, EffectEnvelope};

use crate::{
    actions::{self, ValidatedAction, ValidatedClaim},
    effects::{claim_placed_effect, reaction_window_opened_effect, MaskedClaimsEffect},
    state::{MaskedClaimsState, PendingClaim, Phase},
};

pub fn apply_action(
    state: &mut MaskedClaimsState,
    action: ValidatedAction,
) -> Result<Vec<EffectEnvelope<MaskedClaimsEffect>>, Diagnostic> {
    match action {
        ValidatedAction::Claim(claim) => apply_claim(state, claim),
        ValidatedAction::Response(_) => Err(actions::wrong_phase_diagnostic()),
    }
}

fn apply_claim(
    state: &mut MaskedClaimsState,
    claim: ValidatedClaim,
) -> Result<Vec<EffectEnvelope<MaskedClaimsEffect>>, Diagnostic> {
    ensure_claim_still_legal(state, claim)?;

    let actor_index = claim.actor.index();
    let tile_index = state.hands[actor_index]
        .iter()
        .position(|tile| *tile == claim.tile)
        .expect("validated claim tile must be in actor hand");
    let tile = state.hands[actor_index].remove(tile_index);
    let responder = claim.actor.other();

    state.pedestal = Some(PendingClaim {
        claimant: claim.actor,
        tile,
        declared: claim.declared,
    });
    state.phase = Phase::Reaction {
        turn_index: claim.turn_index,
        responder,
    };
    state.active_seat = Some(responder);
    state.freshness_token = state.freshness_token.next();

    let effects = vec![
        claim_placed_effect(claim.turn_index, claim.actor, claim.declared),
        reaction_window_opened_effect(claim.turn_index, responder, claim.declared),
    ];
    state
        .effect_history
        .extend(effects.iter().map(|effect| format!("{:?}", effect.payload)));

    Ok(effects)
}

fn ensure_claim_still_legal(
    state: &MaskedClaimsState,
    claim: ValidatedClaim,
) -> Result<(), Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if !matches!(state.phase, Phase::Claim { .. }) {
        return Err(actions::wrong_phase_diagnostic());
    }
    if state.active_seat != Some(claim.actor) || state.claimant != claim.actor {
        return Err(actions::wrong_claimant_diagnostic());
    }
    if state.pedestal.is_some() {
        return Err(actions::unavailable_action_diagnostic());
    }
    if !state.hands[claim.actor.index()].contains(&claim.tile) {
        return Err(actions::mask_not_in_hand_diagnostic());
    }
    Ok(())
}
