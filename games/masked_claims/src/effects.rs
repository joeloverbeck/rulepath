use engine_core::EffectEnvelope;

use crate::{
    actions::ResponseChoice,
    ids::{Grade, MaskTileId, MaskedClaimsSeat},
    state::TerminalOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ChallengeOutcome {
    Honest,
    Exposed,
}

impl ChallengeOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Honest => "honest",
            Self::Exposed => "exposed",
        }
    }
}

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
    ClaimAccepted {
        turn: u8,
        claimant: MaskedClaimsSeat,
        declared_grade: Grade,
        score_delta: u8,
        log: String,
    },
    ChallengeDeclared {
        turn: u8,
        responder: MaskedClaimsSeat,
        log: String,
    },
    MaskRevealed {
        turn: u8,
        tile_id: String,
        actual_grade: Grade,
        log: String,
    },
    ChallengeResolved {
        turn: u8,
        outcome: ChallengeOutcome,
        claimant: MaskedClaimsSeat,
        responder: MaskedClaimsSeat,
        claimant_award: u8,
        responder_award: u8,
        log: String,
    },
    ScoreChanged {
        seat: MaskedClaimsSeat,
        delta: u8,
        total: u8,
        reason: String,
    },
    TurnAdvanced {
        turn: u8,
        claimant: MaskedClaimsSeat,
        log: String,
    },
    Terminal {
        outcome: TerminalOutcome,
        final_scores: [u8; 2],
        tiebreak_summary: String,
        log: String,
    },
}

pub fn public_effect(payload: MaskedClaimsEffect) -> EffectEnvelope<MaskedClaimsEffect> {
    EffectEnvelope::public(payload)
}

pub fn claim_accepted_effect(
    turn_index: u8,
    claimant: MaskedClaimsSeat,
    declared_grade: Grade,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::ClaimAccepted {
        turn: display_turn(turn_index),
        claimant,
        declared_grade,
        score_delta: declared_grade.value(),
        log: format!(
            "{}'s grade {} claim was accepted.",
            claimant.as_str(),
            declared_grade.as_str()
        ),
    })
}

pub fn challenge_declared_effect(
    turn_index: u8,
    responder: MaskedClaimsSeat,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::ChallengeDeclared {
        turn: display_turn(turn_index),
        responder,
        log: format!("{} challenged the pending claim.", responder.as_str()),
    })
}

pub fn mask_revealed_effect(
    turn_index: u8,
    tile: MaskTileId,
    actual_grade: Grade,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::MaskRevealed {
        turn: display_turn(turn_index),
        tile_id: tile.as_str().to_owned(),
        actual_grade,
        log: format!(
            "{} was revealed as grade {}.",
            tile.label(),
            actual_grade.as_str()
        ),
    })
}

pub fn challenge_resolved_effect(
    turn_index: u8,
    outcome: ChallengeOutcome,
    claimant: MaskedClaimsSeat,
    responder: MaskedClaimsSeat,
    claimant_award: u8,
    responder_award: u8,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::ChallengeResolved {
        turn: display_turn(turn_index),
        outcome,
        claimant,
        responder,
        claimant_award,
        responder_award,
        log: format!(
            "Challenge resolved as {}; awards are {} to {} and {} to {}.",
            outcome.as_str(),
            claimant_award,
            claimant.as_str(),
            responder_award,
            responder.as_str()
        ),
    })
}

pub fn score_changed_effect(
    seat: MaskedClaimsSeat,
    delta: u8,
    total: u8,
    reason: impl Into<String>,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::ScoreChanged {
        seat,
        delta,
        total,
        reason: reason.into(),
    })
}

pub fn turn_advanced_effect(
    turn_index: u8,
    claimant: MaskedClaimsSeat,
) -> EffectEnvelope<MaskedClaimsEffect> {
    public_effect(MaskedClaimsEffect::TurnAdvanced {
        turn: display_turn(turn_index),
        claimant,
        log: format!(
            "Turn {} begins; {} is the claimant.",
            display_turn(turn_index),
            claimant.as_str()
        ),
    })
}

pub fn terminal_effect(
    outcome: TerminalOutcome,
    final_scores: [u8; 2],
) -> EffectEnvelope<MaskedClaimsEffect> {
    let tiebreak_summary = terminal_tiebreak_summary(outcome);
    public_effect(MaskedClaimsEffect::Terminal {
        outcome,
        final_scores,
        tiebreak_summary: tiebreak_summary.clone(),
        log: format!(
            "Match complete with final scores {}-{}; {}.",
            final_scores[0], final_scores[1], tiebreak_summary
        ),
    })
}

fn terminal_tiebreak_summary(outcome: TerminalOutcome) -> String {
    match outcome {
        TerminalOutcome::ScoreWin { winner, .. } => {
            format!("{} wins on final score", winner.as_str())
        }
        TerminalOutcome::TiebreakWin {
            winner, tiebreak, ..
        } => {
            format!("{} wins on {}", winner.as_str(), tiebreak)
        }
        TerminalOutcome::Draw { .. } => "all tiebreakers are equal; draw".to_owned(),
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
    use engine_core::VisibilityScope;

    #[test]
    fn public_effect_constructor_preserves_public_scope_and_payload() {
        let payload = MaskedClaimsEffect::ClaimPlaced {
            turn: 1,
            claimant: MaskedClaimsSeat::Seat0,
            declared_grade: Grade::Master,
            log: "seat_0 placed a redacted claim.".to_owned(),
        };
        let effect = public_effect(payload.clone());

        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert_eq!(effect.payload, payload);
    }

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

    #[test]
    fn accept_and_terminal_effects_do_not_include_hidden_tile_ids() {
        let effects = vec![
            claim_accepted_effect(0, MaskedClaimsSeat::Seat0, Grade::Master),
            score_changed_effect(MaskedClaimsSeat::Seat0, 5, 5, "accepted_claim"),
            terminal_effect(
                TerminalOutcome::ScoreWin {
                    winner: MaskedClaimsSeat::Seat0,
                    scores: [5, 0],
                },
                [5, 0],
            ),
        ];
        let rendered = format!("{effects:?}");

        for tile in MaskTileId::ALL {
            assert!(!rendered.contains(tile.as_str()));
            assert!(!rendered.contains(&tile.label()));
        }
    }

    #[test]
    fn mask_revealed_is_the_reveal_effect_that_carries_tile_identity() {
        let effect = mask_revealed_effect(0, MaskTileId::MaskG4A, Grade::Jeweled);
        let rendered = format!("{effect:?}");

        assert!(rendered.contains("mask_g4_a"));
        assert!(rendered.contains("Jeweled A"));
    }
}
