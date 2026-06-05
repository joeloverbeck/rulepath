//! Generic Rulepath engine contracts.
//!
//! This crate is intentionally minimal at Gate 0. It defines contract-only
//! vocabulary and has no dependencies on other Rulepath crates.

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
            rules_version: RulesVersion(1),
        };

        assert_eq!(envelope.rules_version, RulesVersion(1));
        assert_eq!(envelope.actor.seat_id, SeatId("seat-a".to_owned()));
    }
}
