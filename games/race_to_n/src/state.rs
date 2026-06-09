use engine_core::{FreshnessToken, SeatId};

use crate::{ids::RaceSeat, variants::Variant};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CounterValue(pub u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaceState {
    pub variant: Variant,
    pub counter: CounterValue,
    pub active_seat: RaceSeat,
    pub seats: [SeatId; 2],
    pub winner: Option<RaceSeat>,
    pub terminal_advance: Option<TerminalAdvance>,
    pub freshness_token: FreshnessToken,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TerminalAdvance {
    pub counter_before: CounterValue,
    pub addition: u8,
    pub counter_after: CounterValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaceSnapshot {
    pub schema_version: u32,
    pub rules_version: u32,
    pub variant: Variant,
    pub counter: CounterValue,
    pub active_seat: RaceSeat,
    pub seats: [SeatId; 2],
    pub winner: Option<RaceSeat>,
    pub terminal_advance: Option<TerminalAdvance>,
    pub freshness_token: FreshnessToken,
}

impl RaceSnapshot {
    pub fn from_state(state: &RaceState) -> Self {
        Self {
            schema_version: 1,
            rules_version: 1,
            variant: state.variant.clone(),
            counter: state.counter,
            active_seat: state.active_seat,
            seats: state.seats.clone(),
            winner: state.winner,
            terminal_advance: state.terminal_advance,
            freshness_token: state.freshness_token,
        }
    }

    pub fn into_state(self) -> RaceState {
        RaceState {
            variant: self.variant,
            counter: self.counter,
            active_seat: self.active_seat,
            seats: self.seats,
            winner: self.winner,
            terminal_advance: self.terminal_advance,
            freshness_token: self.freshness_token,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"rules_version\":{},\"variant_id\":\"{}\",\"target\":{},\"max_add\":{},\"seat_count\":{},\"first_seat\":{},\"ending\":\"{}\",\"counter\":{},\"active_seat\":\"{}\",\"seat_0\":\"{}\",\"seat_1\":\"{}\",\"winner\":{},\"terminal_counter_before\":{},\"terminal_addition\":{},\"terminal_counter_after\":{},\"freshness_token\":{}}}",
            self.schema_version,
            self.rules_version,
            escape_json(&self.variant.id),
            self.variant.target,
            self.variant.max_add,
            self.variant.seat_count,
            self.variant.first_seat,
            escape_json(&self.variant.ending),
            self.counter.0,
            self.active_seat.as_str(),
            escape_json(&self.seats[0].0),
            escape_json(&self.seats[1].0),
            option_seat_json(self.winner),
            option_u8_json(self.terminal_advance.map(|advance| advance.counter_before.0)),
            option_u8_json(self.terminal_advance.map(|advance| advance.addition)),
            option_u8_json(self.terminal_advance.map(|advance| advance.counter_after.0)),
            self.freshness_token.0
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        object.reject_unknown(&[
            "schema_version",
            "rules_version",
            "variant_id",
            "target",
            "max_add",
            "seat_count",
            "first_seat",
            "ending",
            "counter",
            "active_seat",
            "seat_0",
            "seat_1",
            "winner",
            "terminal_counter_before",
            "terminal_addition",
            "terminal_counter_after",
            "freshness_token",
        ])?;
        let terminal_advance = terminal_advance_from_object(&object)?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            rules_version: object.required_u32("rules_version")?,
            variant: Variant {
                id: object.required_string("variant_id")?,
                display_name: "Race to 21".to_owned(),
                target: object.required_u8("target")?,
                max_add: object.required_u8("max_add")?,
                seat_count: object.required_u8("seat_count")?,
                first_seat: object.required_u8("first_seat")?,
                ending: object.required_string("ending")?,
            },
            counter: CounterValue(object.required_u8("counter")?),
            active_seat: object.required_seat("active_seat")?,
            seats: [
                SeatId(object.required_string("seat_0")?),
                SeatId(object.required_string("seat_1")?),
            ],
            winner: object.optional_seat("winner")?,
            terminal_advance,
            freshness_token: FreshnessToken(object.required_u64("freshness_token")?),
        })
    }
}

impl engine_core::StableSerialize for RaceSnapshot {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaceReplayJson {
    pub schema_version: u32,
    pub rules_version: u32,
    pub seed: u64,
    pub initial_snapshot: RaceSnapshot,
    pub command_segments: Vec<String>,
}

impl RaceReplayJson {
    pub fn to_json(&self) -> String {
        let commands = self
            .command_segments
            .iter()
            .map(|segment| format!("\"{}\"", escape_json(segment)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"rules_version\":{},\"seed\":{},\"initial_snapshot\":{},\"command_segments\":[{}]}}",
            self.schema_version,
            self.rules_version,
            self.seed,
            self.initial_snapshot.to_json(),
            commands
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        object.reject_unknown(&[
            "schema_version",
            "rules_version",
            "seed",
            "initial_snapshot",
            "command_segments",
        ])?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            rules_version: object.required_u32("rules_version")?,
            seed: object.required_u64("seed")?,
            initial_snapshot: RaceSnapshot::from_json(&object.required_raw("initial_snapshot")?)?,
            command_segments: object.required_string_array("command_segments")?,
        })
    }
}

impl engine_core::StableSerialize for RaceReplayJson {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub(crate) fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(crate) fn option_seat_json(seat: Option<RaceSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

pub(crate) fn option_u8_json(value: Option<u8>) -> String {
    value.map_or_else(|| "null".to_owned(), |value| value.to_string())
}

fn terminal_advance_from_object(
    object: &StrictJsonObject,
) -> Result<Option<TerminalAdvance>, String> {
    let counter_before = object.optional_u8("terminal_counter_before")?;
    let addition = object.optional_u8("terminal_addition")?;
    let counter_after = object.optional_u8("terminal_counter_after")?;
    match (counter_before, addition, counter_after) {
        (None, None, None) => Ok(None),
        (Some(counter_before), Some(addition), Some(counter_after)) => Ok(Some(TerminalAdvance {
            counter_before: CounterValue(counter_before),
            addition,
            counter_after: CounterValue(counter_after),
        })),
        _ => Err("terminal advance fields must be all null or all present".to_owned()),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StrictJsonObject {
    fields: Vec<(String, String)>,
}

impl StrictJsonObject {
    pub(crate) fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        let body = trimmed
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .ok_or_else(|| "expected JSON object".to_owned())?;
        let mut fields = Vec::new();
        for field in split_top_level(body, ',')? {
            if field.trim().is_empty() {
                continue;
            }
            let (key, value) = split_key_value(&field)?;
            if fields.iter().any(|(existing, _)| existing == &key) {
                return Err(format!("duplicate field `{key}`"));
            }
            fields.push((key, value.trim().to_owned()));
        }
        Ok(Self { fields })
    }

    pub(crate) fn reject_unknown(&self, allowed: &[&str]) -> Result<(), String> {
        for (key, _) in &self.fields {
            if !allowed.contains(&key.as_str()) {
                return Err(format!("unknown field `{key}`"));
            }
        }
        Ok(())
    }

    pub(crate) fn required_raw(&self, key: &str) -> Result<String, String> {
        self.fields
            .iter()
            .find(|(candidate, _)| candidate == key)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| format!("missing field `{key}`"))
    }

    pub(crate) fn required_string(&self, key: &str) -> Result<String, String> {
        parse_json_string(&self.required_raw(key)?)
    }

    pub(crate) fn required_u8(&self, key: &str) -> Result<u8, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must fit u8"))
    }

    pub(crate) fn required_u32(&self, key: &str) -> Result<u32, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must be u32"))
    }

    pub(crate) fn required_u64(&self, key: &str) -> Result<u64, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must be u64"))
    }

    pub(crate) fn required_seat(&self, key: &str) -> Result<RaceSeat, String> {
        RaceSeat::parse(&self.required_string(key)?)
            .ok_or_else(|| format!("field `{key}` must be a seat"))
    }

    pub(crate) fn optional_seat(&self, key: &str) -> Result<Option<RaceSeat>, String> {
        let raw = self.required_raw(key)?;
        if raw == "null" {
            return Ok(None);
        }
        RaceSeat::parse(&parse_json_string(&raw)?)
            .map(Some)
            .ok_or_else(|| format!("field `{key}` must be a seat or null"))
    }

    pub(crate) fn optional_u8(&self, key: &str) -> Result<Option<u8>, String> {
        let raw = self.required_raw(key)?;
        if raw == "null" {
            return Ok(None);
        }
        raw.parse()
            .map(Some)
            .map_err(|_| format!("field `{key}` must fit u8 or null"))
    }

    pub(crate) fn optional_string(&self, key: &str) -> Result<Option<String>, String> {
        let raw = self.required_raw(key)?;
        if raw == "null" {
            return Ok(None);
        }
        parse_json_string(&raw).map(Some)
    }

    pub(crate) fn required_string_array(&self, key: &str) -> Result<Vec<String>, String> {
        let raw = self.required_raw(key)?;
        let body = raw
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
            .ok_or_else(|| format!("field `{key}` must be an array"))?;
        if body.trim().is_empty() {
            return Ok(Vec::new());
        }
        split_top_level(body, ',')?
            .into_iter()
            .map(|entry| parse_json_string(entry.trim()))
            .collect()
    }
}

fn split_key_value(field: &str) -> Result<(String, String), String> {
    let mut in_string = false;
    let mut escaped = false;
    let mut nested = 0usize;
    for (index, ch) in field.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => nested += 1,
            '}' | ']' if !in_string => nested = nested.saturating_sub(1),
            ':' if !in_string && nested == 0 => {
                let key = parse_json_string(field[..index].trim())?;
                return Ok((key, field[index + 1..].trim().to_owned()));
            }
            _ => {}
        }
    }
    Err("expected object field".to_owned())
}

fn split_top_level(input: &str, delimiter: char) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut nested = 0usize;
    for (index, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => nested += 1,
            '}' | ']' if !in_string => {
                nested = nested
                    .checked_sub(1)
                    .ok_or_else(|| "unbalanced JSON".to_owned())?;
            }
            ch if ch == delimiter && !in_string && nested == 0 => {
                parts.push(input[start..index].to_owned());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if in_string || nested != 0 {
        return Err("unbalanced JSON".to_owned());
    }
    parts.push(input[start..].to_owned());
    Ok(parts)
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            output.push(ch);
        }
    }
    if escaped {
        return Err("unterminated escape".to_owned());
    }
    Ok(output)
}
