use engine_core::{
    ActionPath, ActionTree, Actor, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion,
    SeatId, Seed, StableSerialize, Viewer,
};

use crate::{
    apply_action, effects::ColumnFourEffect, legal_action_tree, project_view, setup_match,
    validate_command, CellId, CellOccupancy, ColumnFourSnapshot, ColumnFourState, SetupOptions,
    TerminalOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayHashes {
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub view_hash: HashValue,
    pub replay_hash: HashValue,
    pub outcome: Option<TerminalOutcome>,
    pub terminal: bool,
    pub projections: Vec<ReplayStepProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayStepProjection {
    pub step_index: usize,
    pub board: Vec<String>,
    pub effects: Vec<String>,
    pub terminal: Option<TerminalOutcome>,
    pub public_view_hash: HashValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnFourReplayJson {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed: u64,
    pub initial_snapshot: String,
    pub command_segments: Vec<String>,
}

impl ColumnFourReplayJson {
    pub fn to_json(&self) -> String {
        let commands = self
            .command_segments
            .iter()
            .map(|segment| format!("\"{}\"", escape_json(segment)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed\":{},\"initial_snapshot\":\"{}\",\"command_segments\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed,
            escape_json(&self.initial_snapshot),
            commands
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        object.reject_unknown(&[
            "schema_version",
            "game_id",
            "rules_version",
            "variant",
            "seed",
            "initial_snapshot",
            "command_segments",
        ])?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            game_id: object.required_string("game_id")?,
            rules_version: object.required_string("rules_version")?,
            variant: object.required_string("variant")?,
            seed: object.required_u64("seed")?,
            initial_snapshot: object.required_string("initial_snapshot")?,
            command_segments: parse_string_array(&object.required_raw("command_segments")?)?,
        })
    }
}

impl StableSerialize for ColumnFourReplayJson {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &ColumnFourState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(state: &ColumnFourState, segment: String) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn replay_commands(seed: u64, commands: &[String]) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = ColumnFourSnapshot::from_state(&state).stable_summary();
    let mut effects = Vec::new();
    let mut projections = Vec::new();

    for (index, segment) in commands.iter().enumerate() {
        let command = command_for_state(&state, segment.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        let step_effects = apply_action(&mut state, action);
        projections.push(project_step(index, &state, &step_effects));
        effects.extend(step_effects);
    }

    hashes_for_state(
        seed,
        &initial_snapshot,
        commands,
        &state,
        &effects,
        projections,
    )
}

pub fn hashes_for_state(
    seed: u64,
    initial_snapshot: &str,
    commands: &[String],
    state: &ColumnFourState,
    effects: &[EffectEnvelope<ColumnFourEffect>],
    projections: Vec<ReplayStepProjection>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    let replay = ColumnFourReplayJson {
        schema_version: 1,
        game_id: crate::ids::GAME_ID.to_owned(),
        rules_version: crate::ids::RULES_VERSION_LABEL.to_owned(),
        variant: crate::ids::VARIANT_ID.to_owned(),
        seed,
        initial_snapshot: initial_snapshot.to_owned(),
        command_segments: commands.to_vec(),
    };
    ReplayHashes {
        state_hash: ColumnFourSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash: replay.stable_hash(),
        outcome: state.terminal_outcome,
        terminal: state.terminal_outcome.is_some(),
        projections,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<ColumnFourEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<ColumnFourEffect>) -> String {
    match &effect.payload {
        ColumnFourEffect::DropAccepted { seat, column, ply } => {
            format!("DropAccepted:{}:{}:{ply}", seat.as_str(), column.as_str())
        }
        ColumnFourEffect::PieceLanded {
            seat,
            column,
            row,
            cell,
            display_from_anchor,
            display_to_anchor,
        } => format!(
            "PieceLanded:{}:{}:{}:{}:{display_from_anchor}:{display_to_anchor}",
            seat.as_str(),
            column.as_str(),
            row.as_str(),
            cell.as_string()
        ),
        ColumnFourEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "ActivePlayerChanged:{}:{}:{ply}",
            previous_seat.as_str(),
            active_seat.as_str()
        ),
        ColumnFourEffect::WinDetected { winning_seat, line } => format!(
            "WinDetected:{}:{}-{}-{}-{}",
            winning_seat.as_str(),
            line.cells[0].as_string(),
            line.cells[1].as_string(),
            line.cells[2].as_string(),
            line.cells[3].as_string()
        ),
        ColumnFourEffect::DrawDetected {
            final_ply,
            full_board,
        } => format!("DrawDetected:{final_ply}:{full_board}"),
        ColumnFourEffect::GameEnded {
            outcome,
            final_ply,
            terminal_hash_ref,
        } => format!(
            "GameEnded:{}:{final_ply}:{terminal_hash_ref}",
            outcome_summary(*outcome)
        ),
        ColumnFourEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            column,
            rationale,
        } => format!(
            "BotChoseAction:{level}:{policy_id}:{action_id}:{}:{rationale}",
            column.as_str()
        ),
    }
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn project_step(
    step_index: usize,
    state: &ColumnFourState,
    effects: &[EffectEnvelope<ColumnFourEffect>],
) -> ReplayStepProjection {
    ReplayStepProjection {
        step_index,
        board: CellId::ALL
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}",
                    cell.as_string(),
                    match state.occupancy(*cell) {
                        CellOccupancy::Empty => "empty",
                        CellOccupancy::Occupied(seat) => seat.as_str(),
                    }
                )
            })
            .collect(),
        effects: effects.iter().map(effect_stable_string).collect(),
        terminal: state.terminal_outcome,
        public_view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
    }
}

fn outcome_summary(outcome: TerminalOutcome) -> String {
    match outcome {
        TerminalOutcome::Draw => "draw".to_owned(),
        TerminalOutcome::Win { seat, line } => format!(
            "win:{}:{}-{}-{}-{}",
            seat.as_str(),
            line.cells[0].as_string(),
            line.cells[1].as_string(),
            line.cells[2].as_string(),
            line.cells[3].as_string()
        ),
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StrictJsonObject {
    fields: Vec<(String, String)>,
}

impl StrictJsonObject {
    fn parse(input: &str) -> Result<Self, String> {
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

    fn reject_unknown(&self, allowed: &[&str]) -> Result<(), String> {
        for (key, _) in &self.fields {
            if !allowed.contains(&key.as_str()) {
                return Err(format!("unknown field `{key}`"));
            }
        }
        Ok(())
    }

    fn required_raw(&self, key: &str) -> Result<String, String> {
        self.fields
            .iter()
            .find(|(candidate, _)| candidate == key)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| format!("missing field `{key}`"))
    }

    fn required_string(&self, key: &str) -> Result<String, String> {
        parse_json_string(&self.required_raw(key)?)
    }

    fn required_u32(&self, key: &str) -> Result<u32, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must be u32"))
    }

    fn required_u64(&self, key: &str) -> Result<u64, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must be u64"))
    }
}

fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    let body = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| "expected string array".to_owned())?;
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    body.split(',')
        .map(|value| parse_json_string(value.trim()))
        .collect()
}

fn split_key_value(field: &str) -> Result<(String, String), String> {
    let mut in_string = false;
    let mut previous_escape = false;
    for (index, ch) in field.char_indices() {
        match ch {
            '"' if !previous_escape => in_string = !in_string,
            ':' if !in_string => {
                let key = parse_json_string(field[..index].trim())?;
                return Ok((key, field[index + 1..].trim().to_owned()));
            }
            _ => {}
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    Err("expected key/value".to_owned())
}

fn split_top_level(input: &str, delimiter: char) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut previous_escape = false;

    for (index, ch) in input.char_indices() {
        match ch {
            '"' if !previous_escape => in_string = !in_string,
            '[' | '{' if !in_string => depth += 1,
            ']' | '}' if !in_string => depth -= 1,
            _ => {}
        }
        if ch == delimiter && depth == 0 && !in_string {
            result.push(input[start..index].to_owned());
            start = index + ch.len_utf8();
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    if depth != 0 || in_string {
        return Err("unterminated JSON value".to_owned());
    }
    result.push(input[start..].to_owned());
    Ok(result)
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let escaped = chars.next().ok_or_else(|| "dangling escape".to_owned())?;
            output.push(escaped);
        } else {
            output.push(ch);
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColumnFourSeat, ColumnId, RowId, WinningLine};

    #[test]
    fn replay_reproduces_hashes_outcome_and_projection_for_same_inputs() {
        let commands = vec![
            "drop/c1".to_owned(),
            "drop/c2".to_owned(),
            "drop/c1".to_owned(),
            "drop/c2".to_owned(),
            "drop/c1".to_owned(),
            "drop/c2".to_owned(),
            "drop/c1".to_owned(),
        ];
        let left = replay_commands(1, &commands);
        let right = replay_commands(1, &commands);

        assert_eq!(left, right);
        assert!(left.terminal);
        assert_eq!(
            left.outcome,
            Some(TerminalOutcome::Win {
                seat: ColumnFourSeat::Seat0,
                line: WinningLine {
                    cells: [
                        CellId::new(RowId::R1, ColumnId::C1),
                        CellId::new(RowId::R2, ColumnId::C1),
                        CellId::new(RowId::R3, ColumnId::C1),
                        CellId::new(RowId::R4, ColumnId::C1),
                    ]
                }
            })
        );
        assert_eq!(left.projections.len(), 7);
        assert!(left.projections[6]
            .board
            .contains(&"r4c1:seat_0".to_owned()));
        assert!(left.projections[6]
            .effects
            .iter()
            .any(|effect| effect.starts_with("WinDetected:seat_0")));
        assert_eq!(left.projections[6].public_view_hash, left.view_hash);
    }

    #[test]
    fn replay_json_uses_schema_fields_and_rejects_unknowns() {
        let replay = ColumnFourReplayJson {
            schema_version: 1,
            game_id: "column_four".to_owned(),
            rules_version: "column_four-rules-v1".to_owned(),
            variant: "column_four_standard".to_owned(),
            seed: 7,
            initial_snapshot: "snapshot".to_owned(),
            command_segments: vec!["drop/c4".to_owned()],
        };

        let json = replay.to_json();
        assert!(json.contains("\"game_id\":\"column_four\""));
        assert!(json.contains("\"rules_version\":\"column_four-rules-v1\""));
        assert!(json.contains("\"variant\":\"column_four_standard\""));
        assert_eq!(ColumnFourReplayJson::from_json(&json).unwrap(), replay);
        assert!(ColumnFourReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
    }

    #[test]
    fn effect_and_replay_hashes_are_stable_for_same_command_stream() {
        let commands = vec![
            "drop/c4".to_owned(),
            "drop/c3".to_owned(),
            "drop/c4".to_owned(),
        ];
        let left = replay_commands(44, &commands);
        let right = replay_commands(44, &commands);

        assert_eq!(left.state_hash, right.state_hash);
        assert_eq!(left.effect_hash, right.effect_hash);
        assert_eq!(left.action_tree_hash, right.action_tree_hash);
        assert_eq!(left.view_hash, right.view_hash);
        assert_eq!(left.replay_hash, right.replay_hash);
    }
}
