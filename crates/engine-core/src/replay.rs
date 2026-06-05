use crate::{
    CommandEnvelope, EffectEnvelope, GameId, PlayerId, RulesVersion, SchemaVersion, SeatId, Seed,
    Viewer, VisibilityScope,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct EngineVersion(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DataVersion(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EffectCursor(pub u64);

impl EffectCursor {
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoggedEffect<T> {
    pub cursor: EffectCursor,
    pub envelope: EffectEnvelope<T>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLog<T> {
    entries: Vec<LoggedEffect<T>>,
    next_cursor: EffectCursor,
}

impl<T> Default for EffectLog<T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            next_cursor: EffectCursor(1),
        }
    }
}

impl<T> EffectLog<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, envelope: EffectEnvelope<T>) -> EffectCursor {
        let cursor = self.next_cursor;
        self.entries.push(LoggedEffect { cursor, envelope });
        self.next_cursor = self.next_cursor.next();
        cursor
    }

    pub fn since(&self, cursor: EffectCursor, viewer: &Viewer) -> Vec<&LoggedEffect<T>> {
        self.entries
            .iter()
            .filter(|entry| entry.cursor > cursor && visible_to(&entry.envelope.visibility, viewer))
            .collect()
    }

    pub fn next_cursor(&self) -> EffectCursor {
        self.next_cursor
    }
}

fn visible_to(scope: &VisibilityScope, viewer: &Viewer) -> bool {
    match scope {
        VisibilityScope::Public => true,
        VisibilityScope::PrivateToSeat(seat_id) => viewer.seat_id.as_ref() == Some(seat_id),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HashValue(pub u64);

impl HashValue {
    pub fn from_stable_bytes(bytes: &[u8]) -> Self {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self(hash)
    }
}

pub trait StableSerialize {
    fn stable_bytes(&self) -> Vec<u8>;

    fn stable_hash(&self) -> HashValue {
        HashValue::from_stable_bytes(&self.stable_bytes())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HashSurface {
    State,
    Effect,
    ActionTree,
    View,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewHash {
    pub viewer: Viewer,
    pub hash: HashValue,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ReplayHashSet {
    pub state_hashes: Vec<HashValue>,
    pub effect_hashes: Vec<HashValue>,
    pub action_tree_hashes: Vec<HashValue>,
    pub view_hashes: Vec<ViewHash>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub command_index: usize,
    pub state_hash: HashValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatAssignment {
    pub seat_id: SeatId,
    pub player_id: PlayerId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayRecord {
    pub game_id: GameId,
    pub rules_version: RulesVersion,
    pub engine_version: EngineVersion,
    pub data_version: DataVersion,
    pub schema_version: SchemaVersion,
    pub seed: Seed,
    pub seats: Vec<SeatAssignment>,
    pub options_stable_bytes: Vec<u8>,
    pub commands: Vec<CommandEnvelope>,
    pub checkpoints: Vec<Checkpoint>,
    pub hashes: ReplayHashSet,
    pub build_source: Option<String>,
    pub migration_notes: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UnknownFieldPolicy {
    Reject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SerializationContract {
    pub stable_order_required: bool,
    pub unknown_field_policy: UnknownFieldPolicy,
}

impl SerializationContract {
    pub const STRICT: Self = Self {
        stable_order_required: true,
        unknown_field_policy: UnknownFieldPolicy::Reject,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionPath, Actor, FreshnessToken};

    #[derive(Clone)]
    struct Fixture {
        fields: Vec<(&'static str, u8)>,
    }

    impl StableSerialize for Fixture {
        fn stable_bytes(&self) -> Vec<u8> {
            let mut fields = self.fields.clone();
            fields.sort_by_key(|(key, _)| *key);

            let mut bytes = Vec::new();
            for (key, value) in fields {
                bytes.extend_from_slice(key.as_bytes());
                bytes.push(b'=');
                bytes.push(value);
                bytes.push(b';');
            }
            bytes
        }
    }

    #[test]
    fn stable_serialization_produces_stable_hash() {
        let left = Fixture {
            fields: vec![("b", 2), ("a", 1)],
        };
        let right = Fixture {
            fields: vec![("a", 1), ("b", 2)],
        };

        assert_eq!(left.stable_bytes(), right.stable_bytes());
        assert_eq!(left.stable_hash(), right.stable_hash());
    }

    #[test]
    fn effect_log_cursors_are_monotonic_and_viewer_filtered() {
        let mut log = EffectLog::new();
        let public_cursor = log.push(EffectEnvelope {
            visibility: VisibilityScope::Public,
            payload: "public",
        });
        let private_cursor = log.push(EffectEnvelope {
            visibility: VisibilityScope::PrivateToSeat(SeatId("seat-a".to_owned())),
            payload: "private",
        });

        assert!(private_cursor > public_cursor);
        assert_eq!(log.next_cursor(), EffectCursor(3));

        let public_viewer = Viewer { seat_id: None };
        assert_eq!(log.since(EffectCursor(0), &public_viewer).len(), 1);

        let seat_viewer = Viewer {
            seat_id: Some(SeatId("seat-a".to_owned())),
        };
        let visible = log.since(EffectCursor(0), &seat_viewer);
        assert_eq!(visible.len(), 2);
        assert_eq!(visible[1].envelope.payload, "private");
    }

    #[test]
    fn replay_record_carries_command_stream_and_hashes() {
        let command = CommandEnvelope {
            actor: Actor {
                seat_id: SeatId("seat-a".to_owned()),
            },
            action_path: ActionPath {
                segments: vec!["step".to_owned()],
            },
            freshness_token: FreshnessToken(0),
            rules_version: RulesVersion(1),
        };
        let record = ReplayRecord {
            game_id: GameId("demo".to_owned()),
            rules_version: RulesVersion(1),
            engine_version: EngineVersion(1),
            data_version: DataVersion(1),
            schema_version: SchemaVersion(1),
            seed: Seed(7),
            seats: vec![SeatAssignment {
                seat_id: SeatId("seat-a".to_owned()),
                player_id: PlayerId("player-a".to_owned()),
            }],
            options_stable_bytes: Vec::new(),
            commands: vec![command],
            checkpoints: vec![Checkpoint {
                command_index: 0,
                state_hash: HashValue(10),
            }],
            hashes: ReplayHashSet {
                state_hashes: vec![HashValue(10)],
                effect_hashes: vec![HashValue(11)],
                action_tree_hashes: vec![HashValue(12)],
                view_hashes: vec![ViewHash {
                    viewer: Viewer { seat_id: None },
                    hash: HashValue(13),
                }],
            },
            build_source: None,
            migration_notes: Vec::new(),
        };

        assert_eq!(record.commands.len(), 1);
        assert_eq!(record.hashes.view_hashes[0].hash, HashValue(13));
        assert_eq!(
            SerializationContract::STRICT.unknown_field_policy,
            UnknownFieldPolicy::Reject
        );
    }
}
