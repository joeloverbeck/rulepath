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
#[repr(u8)]
pub enum StableBytesTypeTag {
    Bool = 1,
    U8 = 2,
    U16 = 3,
    U32 = 4,
    U64 = 5,
    I64 = 6,
    String = 7,
    Bytes = 8,
    Record = 9,
    Sequence = 10,
    Option = 11,
    Enum = 12,
}

impl StableBytesTypeTag {
    pub const fn as_byte(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StableBytesWriterError {
    FieldTagNotIncreasing { previous: u32, next: u32 },
    LengthOverflow { len: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StableBytesWriter {
    header: Vec<u8>,
    record: StableBytesRecordWriter,
}

impl StableBytesWriter {
    pub const MAGIC: [u8; 4] = *b"RPSB";
    pub const WRITER_VERSION: u16 = 1;

    pub fn new(
        domain: impl AsRef<[u8]>,
        surface_version: u32,
    ) -> Result<Self, StableBytesWriterError> {
        let domain = domain.as_ref();
        let mut header = Vec::new();
        header.extend_from_slice(&Self::MAGIC);
        header.extend_from_slice(&Self::WRITER_VERSION.to_le_bytes());
        append_len_prefixed_bytes(&mut header, domain)?;
        header.extend_from_slice(&surface_version.to_le_bytes());
        Ok(Self {
            header,
            record: StableBytesRecordWriter::new(),
        })
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        self.header.extend_from_slice(&self.record.into_bytes());
        self.header
    }

    pub fn write_bool_field(
        &mut self,
        tag: u32,
        value: bool,
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_bool_field(tag, value)
    }

    pub fn write_u8_field(&mut self, tag: u32, value: u8) -> Result<(), StableBytesWriterError> {
        self.record.write_u8_field(tag, value)
    }

    pub fn write_u16_field(&mut self, tag: u32, value: u16) -> Result<(), StableBytesWriterError> {
        self.record.write_u16_field(tag, value)
    }

    pub fn write_u32_field(&mut self, tag: u32, value: u32) -> Result<(), StableBytesWriterError> {
        self.record.write_u32_field(tag, value)
    }

    pub fn write_u64_field(&mut self, tag: u32, value: u64) -> Result<(), StableBytesWriterError> {
        self.record.write_u64_field(tag, value)
    }

    pub fn write_i64_field(&mut self, tag: u32, value: i64) -> Result<(), StableBytesWriterError> {
        self.record.write_i64_field(tag, value)
    }

    pub fn write_string_field(
        &mut self,
        tag: u32,
        value: &str,
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_string_field(tag, value)
    }

    pub fn write_bytes_field(
        &mut self,
        tag: u32,
        value: &[u8],
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_bytes_field(tag, value)
    }

    pub fn write_record_field(
        &mut self,
        tag: u32,
        build: impl FnOnce(&mut StableBytesRecordWriter) -> Result<(), StableBytesWriterError>,
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_record_field(tag, build)
    }

    pub fn write_sequence_field<I, B>(
        &mut self,
        tag: u32,
        elements: I,
    ) -> Result<(), StableBytesWriterError>
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        self.record.write_sequence_field(tag, elements)
    }

    pub fn write_none_field(&mut self, tag: u32) -> Result<(), StableBytesWriterError> {
        self.record.write_none_field(tag)
    }

    pub fn write_some_field(
        &mut self,
        tag: u32,
        value_type: StableBytesTypeTag,
        value_payload: &[u8],
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_some_field(tag, value_type, value_payload)
    }

    pub fn write_enum_field(
        &mut self,
        tag: u32,
        discriminant: u32,
    ) -> Result<(), StableBytesWriterError> {
        self.record.write_enum_field(tag, discriminant)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StableBytesRecordWriter {
    bytes: Vec<u8>,
    last_field_tag: Option<u32>,
}

impl Default for StableBytesRecordWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl StableBytesRecordWriter {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            last_field_tag: None,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn write_bool_field(
        &mut self,
        tag: u32,
        value: bool,
    ) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::Bool, &[u8::from(value)])
    }

    pub fn write_u8_field(&mut self, tag: u32, value: u8) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::U8, &[value])
    }

    pub fn write_u16_field(&mut self, tag: u32, value: u16) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::U16, &value.to_le_bytes())
    }

    pub fn write_u32_field(&mut self, tag: u32, value: u32) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::U32, &value.to_le_bytes())
    }

    pub fn write_u64_field(&mut self, tag: u32, value: u64) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::U64, &value.to_le_bytes())
    }

    pub fn write_i64_field(&mut self, tag: u32, value: i64) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::I64, &value.to_le_bytes())
    }

    pub fn write_string_field(
        &mut self,
        tag: u32,
        value: &str,
    ) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::String, value.as_bytes())
    }

    pub fn write_bytes_field(
        &mut self,
        tag: u32,
        value: &[u8],
    ) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::Bytes, value)
    }

    pub fn write_record_field(
        &mut self,
        tag: u32,
        build: impl FnOnce(&mut StableBytesRecordWriter) -> Result<(), StableBytesWriterError>,
    ) -> Result<(), StableBytesWriterError> {
        let mut record = StableBytesRecordWriter::new();
        build(&mut record)?;
        self.write_field(tag, StableBytesTypeTag::Record, &record.into_bytes())
    }

    pub fn write_sequence_field<I, B>(
        &mut self,
        tag: u32,
        elements: I,
    ) -> Result<(), StableBytesWriterError>
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        let mut payload = Vec::new();
        let mut count = 0u32;
        for element in elements {
            count = count
                .checked_add(1)
                .ok_or(StableBytesWriterError::LengthOverflow { len: usize::MAX })?;
            append_len_prefixed_bytes(&mut payload, element.as_ref())?;
        }
        let mut framed = count.to_le_bytes().to_vec();
        framed.extend_from_slice(&payload);
        self.write_field(tag, StableBytesTypeTag::Sequence, &framed)
    }

    pub fn write_none_field(&mut self, tag: u32) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::Option, &[0])
    }

    pub fn write_some_field(
        &mut self,
        tag: u32,
        value_type: StableBytesTypeTag,
        value_payload: &[u8],
    ) -> Result<(), StableBytesWriterError> {
        let mut payload = vec![1];
        payload.push(value_type.as_byte());
        append_len_prefixed_bytes(&mut payload, value_payload)?;
        self.write_field(tag, StableBytesTypeTag::Option, &payload)
    }

    pub fn write_enum_field(
        &mut self,
        tag: u32,
        discriminant: u32,
    ) -> Result<(), StableBytesWriterError> {
        self.write_field(tag, StableBytesTypeTag::Enum, &discriminant.to_le_bytes())
    }

    fn write_field(
        &mut self,
        tag: u32,
        type_tag: StableBytesTypeTag,
        payload: &[u8],
    ) -> Result<(), StableBytesWriterError> {
        if let Some(previous) = self.last_field_tag {
            if tag <= previous {
                return Err(StableBytesWriterError::FieldTagNotIncreasing {
                    previous,
                    next: tag,
                });
            }
        }
        self.last_field_tag = Some(tag);
        self.bytes.extend_from_slice(&tag.to_le_bytes());
        self.bytes.push(type_tag.as_byte());
        append_len_prefixed_bytes(&mut self.bytes, payload)?;
        Ok(())
    }
}

fn append_len_prefixed_bytes(
    bytes: &mut Vec<u8>,
    payload: &[u8],
) -> Result<(), StableBytesWriterError> {
    let len = u32::try_from(payload.len())
        .map_err(|_| StableBytesWriterError::LengthOverflow { len: payload.len() })?;
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(payload);
    Ok(())
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
    fn stable_bytes_writer_header_matches_v1_contract() {
        let writer = StableBytesWriter::new("demo", 7).expect("header");

        assert_eq!(
            writer.into_bytes(),
            vec![b'R', b'P', b'S', b'B', 1, 0, 4, 0, 0, 0, b'd', b'e', b'm', b'o', 7, 0, 0, 0]
        );
    }

    #[test]
    fn stable_bytes_writer_primitives_match_golden_bytes() {
        let mut writer = StableBytesWriter::new("prim", 1).expect("writer");

        writer.write_bool_field(1, true).expect("bool");
        writer.write_u8_field(2, 0xab).expect("u8");
        writer.write_u16_field(3, 0x1234).expect("u16");
        writer.write_u32_field(4, 0x1234_5678).expect("u32");
        writer
            .write_u64_field(5, 0x0123_4567_89ab_cdef)
            .expect("u64");
        writer.write_i64_field(6, -2).expect("i64");
        writer.write_string_field(7, "a|b").expect("string");
        writer.write_bytes_field(8, &[0, 255]).expect("bytes");

        assert_eq!(
            writer.into_bytes(),
            vec![
                b'R', b'P', b'S', b'B', 1, 0, 4, 0, 0, 0, b'p', b'r', b'i', b'm', 1, 0, 0, 0, 1, 0,
                0, 0, 1, 1, 0, 0, 0, 1, 2, 0, 0, 0, 2, 1, 0, 0, 0, 0xab, 3, 0, 0, 0, 3, 2, 0, 0, 0,
                0x34, 0x12, 4, 0, 0, 0, 4, 4, 0, 0, 0, 0x78, 0x56, 0x34, 0x12, 5, 0, 0, 0, 5, 8, 0,
                0, 0, 0xef, 0xcd, 0xab, 0x89, 0x67, 0x45, 0x23, 0x01, 6, 0, 0, 0, 6, 8, 0, 0, 0,
                0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 7, 0, 0, 0, 7, 3, 0, 0, 0, b'a',
                b'|', b'b', 8, 0, 0, 0, 8, 2, 0, 0, 0, 0, 255,
            ]
        );
    }

    #[test]
    fn stable_bytes_writer_nested_sequence_option_and_enum_framing_is_golden() {
        let mut writer = StableBytesWriter::new("shape", 2).expect("writer");

        writer
            .write_record_field(1, |record| record.write_string_field(1, "x"))
            .expect("record");
        writer
            .write_sequence_field(2, [b"a|b".as_slice(), b"a".as_slice(), b"b".as_slice()])
            .expect("sequence");
        writer.write_none_field(3).expect("none");
        writer
            .write_some_field(4, StableBytesTypeTag::String, b"a|b")
            .expect("some");
        writer.write_enum_field(5, 2).expect("enum");

        assert_eq!(
            writer.into_bytes(),
            vec![
                b'R', b'P', b'S', b'B', 1, 0, 5, 0, 0, 0, b's', b'h', b'a', b'p', b'e', 2, 0, 0, 0,
                1, 0, 0, 0, 9, 10, 0, 0, 0, 1, 0, 0, 0, 7, 1, 0, 0, 0, b'x', 2, 0, 0, 0, 10, 21, 0,
                0, 0, 3, 0, 0, 0, 3, 0, 0, 0, b'a', b'|', b'b', 1, 0, 0, 0, b'a', 1, 0, 0, 0, b'b',
                3, 0, 0, 0, 11, 1, 0, 0, 0, 0, 4, 0, 0, 0, 11, 9, 0, 0, 0, 1, 7, 3, 0, 0, 0, b'a',
                b'|', b'b', 5, 0, 0, 0, 12, 4, 0, 0, 0, 2, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn stable_bytes_writer_rejects_duplicate_and_non_increasing_tags() {
        let mut duplicate = StableBytesWriter::new("order", 1).expect("writer");
        duplicate.write_u8_field(2, 1).expect("first");
        assert_eq!(
            duplicate.write_u8_field(2, 2),
            Err(StableBytesWriterError::FieldTagNotIncreasing {
                previous: 2,
                next: 2,
            })
        );

        let mut descending = StableBytesWriter::new("order", 1).expect("writer");
        descending.write_u8_field(3, 1).expect("first");
        assert_eq!(
            descending.write_u8_field(1, 2),
            Err(StableBytesWriterError::FieldTagNotIncreasing {
                previous: 3,
                next: 1,
            })
        );
    }

    #[test]
    fn stable_bytes_writer_sequence_lengths_resist_delimiter_collisions() {
        let mut left = StableBytesWriter::new("seq", 1).expect("left");
        left.write_sequence_field(1, [b"a|b".as_slice(), b"c".as_slice()])
            .expect("left sequence");

        let mut right = StableBytesWriter::new("seq", 1).expect("right");
        right
            .write_sequence_field(1, [b"a".as_slice(), b"b|c".as_slice()])
            .expect("right sequence");

        assert_eq!("a|b|c", ["a|b", "c"].join("|"));
        assert_eq!("a|b|c", ["a", "b|c"].join("|"));
        assert_ne!(left.into_bytes(), right.into_bytes());
    }

    #[test]
    fn stable_bytes_writer_repeated_input_is_deterministic() {
        fn encode() -> Vec<u8> {
            let mut writer = StableBytesWriter::new("det", 3).expect("writer");
            writer.write_u32_field(1, 42).expect("u32");
            writer.write_string_field(2, "same").expect("string");
            writer.into_bytes()
        }

        assert_eq!(encode(), encode());
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
