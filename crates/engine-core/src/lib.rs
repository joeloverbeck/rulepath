//! Generic Rulepath engine contracts.
//!
//! This crate is intentionally minimal at Gate 0. It defines contract-only
//! vocabulary and has no dependencies on other Rulepath crates.

mod action;
mod game;
mod replay;
mod rng;

pub use action::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, FreshnessToken,
};
pub use game::Game;
pub use replay::{
    Checkpoint, DataVersion, EffectCursor, EffectLog, EngineVersion, HashSurface, HashValue,
    LoggedEffect, ReplayHashSet, ReplayRecord, SeatAssignment, SerializationContract,
    StableSerialize, UnknownFieldPolicy, ViewHash,
};
pub use rng::{DeterministicRng, SeededRng};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GameId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatchId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SeatId(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerId(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RulesVersion(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SchemaVersion(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Seed(pub u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VisibilityScope {
    Public,
    PrivateToSeat(SeatId),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub seat_id: SeatId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Viewer {
    pub seat_id: Option<SeatId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionPath {
    pub segments: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandEnvelope {
    pub actor: Actor,
    pub action_path: ActionPath,
    pub freshness_token: FreshnessToken,
    pub rules_version: RulesVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectEnvelope<T> {
    pub visibility: VisibilityScope,
    pub payload: T,
}

impl<T> EffectEnvelope<T> {
    /// Builds a public effect envelope without filtering, reveal, or
    /// serialization behavior.
    pub fn public(payload: T) -> Self {
        Self {
            visibility: VisibilityScope::Public,
            payload,
        }
    }

    /// Builds a seat-private effect envelope for an already-typed seat without
    /// filtering, reveal, or serialization behavior.
    pub fn private_to(seat_id: SeatId, payload: T) -> Self {
        Self {
            visibility: VisibilityScope::PrivateToSeat(seat_id),
            payload,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_envelope_keeps_contract_fields() {
        let envelope = CommandEnvelope {
            actor: Actor {
                seat_id: SeatId("seat-a".to_owned()),
            },
            action_path: ActionPath {
                segments: vec!["choose".to_owned()],
            },
            freshness_token: FreshnessToken(7),
            rules_version: RulesVersion(1),
        };

        assert_eq!(envelope.rules_version, RulesVersion(1));
        assert_eq!(envelope.actor.seat_id, SeatId("seat-a".to_owned()));
        assert_eq!(envelope.freshness_token, FreshnessToken(7));
    }

    #[test]
    fn effect_envelope_public_matches_literal_scope_and_payload() {
        let envelope = EffectEnvelope::public("payload".to_owned());

        assert_eq!(
            envelope,
            EffectEnvelope {
                visibility: VisibilityScope::Public,
                payload: "payload".to_owned(),
            }
        );
    }

    #[test]
    fn effect_envelope_private_to_matches_literal_scope_and_payload() {
        let seat_id = SeatId("seat_0".to_owned());
        let envelope = EffectEnvelope::private_to(seat_id.clone(), vec!["payload".to_owned()]);

        assert_eq!(
            envelope,
            EffectEnvelope {
                visibility: VisibilityScope::PrivateToSeat(seat_id),
                payload: vec!["payload".to_owned()],
            }
        );
    }

    #[test]
    fn effect_envelope_constructors_move_non_copy_payloads() {
        #[derive(Debug, Eq, PartialEq)]
        struct NonCopyPayload(String);

        let payload = NonCopyPayload("owned".to_owned());
        let envelope = EffectEnvelope::public(payload);

        assert_eq!(envelope.payload, NonCopyPayload("owned".to_owned()));
        assert_eq!(envelope.visibility, VisibilityScope::Public);
    }
}
