use engine_core::{EffectEnvelope, VisibilityScope};

use crate::{
    actions::ResponseChoice,
    ids::{Grade, MaskedClaimsSeat},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaskedClaimsEffect {
    ClaimPlaced {
        turn: u8,
        claimant: MaskedClaimsSeat,
        declared_grade: Grade,
        log: String,
    },
    ReactionWindowOpened {
        turn: u8,
        responder: MaskedClaimsSeat,
        declared_grade: Grade,
        choices: Vec<String>,
        log: String,
    },
}

pub fn public_effect(payload: MaskedClaimsEffect) -> EffectEnvelope<MaskedClaimsEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

pub fn claim_placed_effect(
    turn_index: u8,
    claimant: MaskedClaimsSeat,
    declared_grade: Grade,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::ClaimPlaced {
        turn: display_turn(turn_index),
        claimant,
        declared_grade,
        log: format!(
            "{} placed a face-down mask and claimed grade {}.",
            claimant.as_str(),
            declared_grade.as_str()
        ),
    })
}

pub fn reaction_window_opened_effect(
    turn_index: u8,
    responder: MaskedClaimsSeat,
    declared_grade: Grade,
) -> EffectEnvelope<MaskedClaimsEffect> {
    let choices = ResponseChoice::ALL
        .into_iter()
        .map(|choice| choice.action_segment().to_owned())
        .collect::<Vec<_>>();
    public_effect(MaskedClaimsEffect::ReactionWindowOpened {
        turn: display_turn(turn_index),
        responder,
        declared_grade,
        choices,
        log: format!(
            "{} may accept or challenge because a grade {} claim is pending on the pedestal.",
            responder.as_str(),
            declared_grade.as_str()
        ),
    })
}

const fn display_turn(turn_index: u8) -> u8 {
    turn_index.saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::MaskTileId;

    #[test]
    fn reaction_window_log_names_responder_choices_and_reason() {
        let effect = reaction_window_opened_effect(0, MaskedClaimsSeat::Seat1, Grade::Jeweled);
        let rendered = format!("{effect:?}");

        assert!(rendered.contains("seat_1"));
        assert!(rendered.contains("accept"));
        assert!(rendered.contains("challenge"));
        assert!(rendered.contains("claim is pending"));
        assert!(rendered.contains("4"));
    }

    #[test]
    fn claim_and_window_effects_do_not_include_tile_ids() {
        let effects = vec![
            claim_placed_effect(0, MaskedClaimsSeat::Seat0, Grade::Master),
            reaction_window_opened_effect(0, MaskedClaimsSeat::Seat1, Grade::Master),
        ];
        let rendered = format!("{effects:?}");

        for tile in MaskTileId::ALL {
            assert!(!rendered.contains(tile.as_str()));
            assert!(!rendered.contains(&tile.label()));
        }
    }
}
