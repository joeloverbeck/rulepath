use engine_core::{EffectEnvelope, VisibilityScope};

use crate::{
    ids::{CellId, DirectionalFlipSeat},
    rules::{Direction, Score},
    state::TerminalOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectionalFlipEffect {
    PlacementAccepted {
        seat: DirectionalFlipSeat,
        cell: CellId,
        ply: u8,
    },
    DiscPlaced {
        seat: DirectionalFlipSeat,
        cell: CellId,
        display_to_anchor: String,
    },
    DiscsFlipped {
        seat: DirectionalFlipSeat,
        flips: Vec<FlipEntry>,
    },
    PassTaken {
        seat: DirectionalFlipSeat,
        ply: u8,
        reason: String,
    },
    ActivePlayerChanged {
        previous_seat: DirectionalFlipSeat,
        active_seat: DirectionalFlipSeat,
        ply: u8,
    },
    GameEnded {
        outcome: TerminalOutcome,
        final_score: Score,
        final_ply: u8,
        reason: TerminalReason,
        terminal_hash_ref: String,
    },
    BotChoseAction {
        level: u8,
        policy_id: String,
        action_id: String,
        rationale: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlipEntry {
    pub cell: CellId,
    pub previous_owner: DirectionalFlipSeat,
    pub new_owner: DirectionalFlipSeat,
    pub direction: Direction,
    pub distance: u8,
    pub order_index: u8,
    pub display_anchor: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalReason {
    BoardFull,
    NoContinuation,
    DoubleForcedPass,
}

pub fn public_effect(payload: DirectionalFlipEffect) -> EffectEnvelope<DirectionalFlipEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

pub fn bot_chose_action_effect(
    level: u8,
    policy_id: impl Into<String>,
    action_id: impl Into<String>,
    rationale: impl Into<String>,
) -> EffectEnvelope<DirectionalFlipEffect> {
    public_effect(DirectionalFlipEffect::BotChoseAction {
        level,
        policy_id: policy_id.into(),
        action_id: action_id.into(),
        rationale: rationale.into(),
    })
}

pub fn display_to_anchor(cell: CellId) -> String {
    format!("cell:{}", cell.as_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::VisibilityScope;

    #[test]
    fn bot_chose_action_effect_is_public_safe_prose() {
        let effect = bot_chose_action_effect(
            2,
            "directional_flip_mobility_v1",
            "place/r3c4",
            "I can reduce the opponent's legal choices with this legal placement.",
        );

        assert_eq!(effect.visibility, VisibilityScope::Public);
        let DirectionalFlipEffect::BotChoseAction { rationale, .. } = effect.payload else {
            panic!("expected bot effect");
        };
        assert!(rationale.contains("legal placement"));
        assert!(!rationale.contains("candidate"));
        assert!(!rationale.contains("score"));
        assert!(!rationale.contains('['));
        assert!(!rationale.contains('{'));
    }
}
