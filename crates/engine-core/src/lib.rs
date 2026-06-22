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

/// Structural error for canonical seat IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CanonicalSeatIdError {
    MissingPrefix,
    EmptyIndex,
    LeadingZero,
    InvalidCharacter { ch: char },
    NonAsciiDigit { ch: char },
    Overflow,
}

impl SeatId {
    /// Constructs a canonical seat ID using grammar version 1:
    /// `seat_<unsigned-zero-based-decimal>`.
    pub fn from_zero_based_index(index: u32) -> Self {
        Self(format!("seat_{index}"))
    }

    /// Parses grammar version 1 canonical seat IDs:
    /// `seat_<unsigned-zero-based-decimal>`.
    ///
    /// This strict parser rejects whitespace, signs, empty suffixes,
    /// non-digits, non-ASCII digits, `u32` overflow, and non-canonical leading
    /// zero spellings except `seat_0`. Legacy aliases belong at import
    /// boundaries, not in the kernel.
    pub fn parse_canonical(input: &str) -> Result<Self, CanonicalSeatIdError> {
        let index = parse_canonical_seat_index(input)?;
        Ok(Self::from_zero_based_index(index))
    }

    /// Extracts the zero-based index from a grammar version 1 canonical seat
    /// ID.
    pub fn canonical_zero_based_index(&self) -> Result<u32, CanonicalSeatIdError> {
        parse_canonical_seat_index(&self.0)
    }
}

fn parse_canonical_seat_index(input: &str) -> Result<u32, CanonicalSeatIdError> {
    let suffix = input
        .strip_prefix("seat_")
        .ok_or(CanonicalSeatIdError::MissingPrefix)?;
    if suffix.is_empty() {
        return Err(CanonicalSeatIdError::EmptyIndex);
    }
    if suffix.len() > 1 && suffix.starts_with('0') {
        return Err(CanonicalSeatIdError::LeadingZero);
    }
    for ch in suffix.chars() {
        if ch.is_ascii_digit() {
            continue;
        }
        if ch.is_numeric() {
            return Err(CanonicalSeatIdError::NonAsciiDigit { ch });
        }
        return Err(CanonicalSeatIdError::InvalidCharacter { ch });
    }
    suffix
        .parse::<u32>()
        .map_err(|_| CanonicalSeatIdError::Overflow)
}

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

    #[test]
    fn canonical_seat_id_round_trips_zero_based_indices() {
        for index in [0, 1, 7, 42, u32::MAX] {
            let seat_id = SeatId::from_zero_based_index(index);

            assert_eq!(seat_id.0, format!("seat_{index}"));
            assert_eq!(SeatId::parse_canonical(&seat_id.0), Ok(seat_id.clone()));
            assert_eq!(seat_id.canonical_zero_based_index(), Ok(index));
        }
    }

    #[test]
    fn canonical_seat_id_rejects_non_canonical_spellings() {
        let cases = [
            ("seat-", CanonicalSeatIdError::MissingPrefix),
            (" seat_1", CanonicalSeatIdError::MissingPrefix),
            ("seat_", CanonicalSeatIdError::EmptyIndex),
            ("seat_01", CanonicalSeatIdError::LeadingZero),
            (
                "seat_+1",
                CanonicalSeatIdError::InvalidCharacter { ch: '+' },
            ),
            (
                "seat_-1",
                CanonicalSeatIdError::InvalidCharacter { ch: '-' },
            ),
            (
                "seat_1 ",
                CanonicalSeatIdError::InvalidCharacter { ch: ' ' },
            ),
            ("seat_a", CanonicalSeatIdError::InvalidCharacter { ch: 'a' }),
            ("seat_１", CanonicalSeatIdError::NonAsciiDigit { ch: '１' }),
            ("seat_4294967296", CanonicalSeatIdError::Overflow),
        ];

        for (input, expected) in cases {
            assert_eq!(
                SeatId::parse_canonical(input),
                Err(expected),
                "{input} rejects"
            );
            assert_eq!(
                SeatId(input.to_owned()).canonical_zero_based_index(),
                Err(expected),
                "{input} extracts"
            );
        }
    }
}
