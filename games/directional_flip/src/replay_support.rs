use engine_core::{
    ActionPath, ActionPreview, ActionTree, Actor, CommandEnvelope, EffectEnvelope, HashValue,
    RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};

use crate::{
    apply_action,
    effects::{DirectionalFlipEffect, FlipEntry, TerminalReason},
    legal_action_tree, project_view, setup_match, validate_command, CellId, CellOccupancy,
    Direction, DirectionalFlipSnapshot, DirectionalFlipState, Score, SetupOptions, TerminalOutcome,
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
pub struct DirectionalFlipReplayJson {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed: u64,
    pub initial_snapshot: String,
    pub command_segments: Vec<String>,
}

impl DirectionalFlipReplayJson {
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

impl StableSerialize for DirectionalFlipReplayJson {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &DirectionalFlipState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(state: &DirectionalFlipState, segment: String) -> CommandEnvelope {
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
    let initial_snapshot = DirectionalFlipSnapshot::from_state(&state).stable_summary();
    replay_from_state(seed, initial_snapshot, commands, &mut state)
}

pub fn replay_from_state(
    seed: u64,
    initial_snapshot: String,
    commands: &[String],
    state: &mut DirectionalFlipState,
) -> ReplayHashes {
    let mut effects = Vec::new();
    let mut projections = Vec::new();

    for (index, segment) in commands.iter().enumerate() {
        let command = command_for_state(state, segment.clone());
        let action = validate_command(state, &command).expect("trace command validates");
        let step_effects = apply_action(state, action);
        projections.push(project_step(index, state, &step_effects));
        effects.extend(step_effects);
    }

    hashes_for_state(
        seed,
        &initial_snapshot,
        commands,
        state,
        &effects,
        projections,
    )
}

pub fn hashes_for_state(
    seed: u64,
    initial_snapshot: &str,
    commands: &[String],
    state: &DirectionalFlipState,
    effects: &[EffectEnvelope<DirectionalFlipEffect>],
    projections: Vec<ReplayStepProjection>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    let replay = DirectionalFlipReplayJson {
        schema_version: 1,
        game_id: crate::ids::GAME_ID.to_owned(),
        rules_version: crate::ids::RULES_VERSION_LABEL.to_owned(),
        variant: crate::ids::VARIANT_ID.to_owned(),
        seed,
        initial_snapshot: initial_snapshot.to_owned(),
        command_segments: commands.to_vec(),
    };
    ReplayHashes {
        state_hash: DirectionalFlipSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash: replay.stable_hash(),
        outcome: state.terminal_outcome,
        terminal: state.terminal_outcome.is_some(),
        projections,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<DirectionalFlipEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<DirectionalFlipEffect>) -> String {
    match &effect.payload {
        DirectionalFlipEffect::PlacementAccepted { seat, cell, ply } => {
            format!(
                "PlacementAccepted:{}:{}:{ply}",
                seat.as_str(),
                cell.as_string()
            )
        }
        DirectionalFlipEffect::DiscPlaced {
            seat,
            cell,
            display_to_anchor,
        } => format!(
            "DiscPlaced:{}:{}:{display_to_anchor}",
            seat.as_str(),
            cell.as_string()
        ),
        DirectionalFlipEffect::DiscsFlipped { seat, flips } => format!(
            "DiscsFlipped:{}:{}",
            seat.as_str(),
            flips
                .iter()
                .map(flip_entry_stable_string)
                .collect::<Vec<_>>()
                .join("|")
        ),
        DirectionalFlipEffect::PassTaken { seat, ply, reason } => {
            format!("PassTaken:{}:{ply}:{reason}", seat.as_str())
        }
        DirectionalFlipEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "ActivePlayerChanged:{}:{}:{ply}",
            previous_seat.as_str(),
            active_seat.as_str()
        ),
        DirectionalFlipEffect::GameEnded {
            outcome,
            final_score,
            final_ply,
            reason,
            terminal_hash_ref,
        } => format!(
            "GameEnded:{}:{}:{final_ply}:{}:{terminal_hash_ref}",
            outcome_summary(*outcome),
            score_summary(*final_score),
            reason_summary(*reason)
        ),
        DirectionalFlipEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            rationale,
        } => format!("BotChoseAction:{level}:{policy_id}:{action_id}:{rationale}"),
    }
}

pub fn action_tree_hash(tree: &ActionTree) -> HashValue {
    let bytes = tree
        .root
        .choices
        .iter()
        .map(|choice| {
            let metadata = choice
                .metadata
                .iter()
                .map(|entry| format!("{}={}", entry.key, entry.value))
                .collect::<Vec<_>>()
                .join(",");
            let tags = choice.tags.join(",");
            format!(
                "{}|{}|{}|{}|{}|{}",
                choice.segment,
                choice.label,
                choice.accessibility_label,
                preview_summary(choice.preview),
                metadata,
                tags
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn project_step(
    step_index: usize,
    state: &DirectionalFlipState,
    effects: &[EffectEnvelope<DirectionalFlipEffect>],
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

fn flip_entry_stable_string(entry: &FlipEntry) -> String {
    format!(
        "{}:{}>{}:{}:{}:{}:{}",
        entry.cell.as_string(),
        entry.previous_owner.as_str(),
        entry.new_owner.as_str(),
        direction_summary(entry.direction),
        entry.distance,
        entry.order_index,
        entry.display_anchor
    )
}

fn outcome_summary(outcome: TerminalOutcome) -> String {
    match outcome {
        TerminalOutcome::Draw => "draw".to_owned(),
        TerminalOutcome::Win { seat } => format!("win:{}", seat.as_str()),
    }
}

fn score_summary(score: Score) -> String {
    format!("seat_0={},seat_1={}", score.seat_0, score.seat_1)
}

fn reason_summary(reason: TerminalReason) -> &'static str {
    match reason {
        TerminalReason::BoardFull => "board_full",
        TerminalReason::NoContinuation => "no_continuation",
        TerminalReason::DoubleForcedPass => "double_forced_pass",
    }
}

fn direction_summary(direction: Direction) -> &'static str {
    direction.as_str()
}

fn preview_summary(preview: ActionPreview) -> &'static str {
    match preview {
        ActionPreview::Unavailable => "unavailable",
        ActionPreview::Available => "available",
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
    use crate::{ColumnId, DirectionalFlipSeat, RowId};

    #[test]
    fn replay_reproduces_hashes_outcome_and_projection_for_same_inputs() {
        let commands = first_legal_segments(19, 7);
        let left = replay_commands(19, &commands);
        let right = replay_commands(19, &commands);

        assert_eq!(left, right);
        assert!(!left.terminal);
        assert_eq!(left.projections.len(), 7);
        assert!(left.projections[0]
            .board
            .iter()
            .any(|cell| cell.starts_with("r3c4:seat_0")));
        assert!(left.projections[0]
            .effects
            .iter()
            .any(|effect| effect.starts_with("DiscsFlipped:seat_0")));
        assert_eq!(left.projections[6].public_view_hash, left.view_hash);
    }

    #[test]
    fn replay_json_uses_schema_fields_and_rejects_unknowns() {
        let replay = DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: "directional_flip".to_owned(),
            rules_version: "directional_flip-rules-v1".to_owned(),
            variant: "directional_flip_standard".to_owned(),
            seed: 7,
            initial_snapshot: "snapshot".to_owned(),
            command_segments: vec!["place/r3c4".to_owned()],
        };

        let json = replay.to_json();
        assert!(json.contains("\"game_id\":\"directional_flip\""));
        assert!(json.contains("\"rules_version\":\"directional_flip-rules-v1\""));
        assert!(json.contains("\"variant\":\"directional_flip_standard\""));
        assert_eq!(DirectionalFlipReplayJson::from_json(&json).unwrap(), replay);
        assert!(DirectionalFlipReplayJson::from_json("{\"debug\":\"nope\"}").is_err());
    }

    #[test]
    fn effect_and_replay_hashes_are_stable_for_same_command_stream() {
        let commands = first_legal_segments(44, 5);
        let left = replay_commands(44, &commands);
        let right = replay_commands(44, &commands);

        assert_eq!(left.state_hash, right.state_hash);
        assert_eq!(left.effect_hash, right.effect_hash);
        assert_eq!(left.action_tree_hash, right.action_tree_hash);
        assert_eq!(left.view_hash, right.view_hash);
        assert_eq!(left.replay_hash, right.replay_hash);
    }

    #[test]
    fn forced_pass_step_projection_and_terminal_hash_are_stable() {
        let mut left_state = no_move_state();
        let mut right_state = no_move_state();
        let initial_snapshot = DirectionalFlipSnapshot::from_state(&left_state).stable_summary();
        let commands = vec!["pass/forced".to_owned(), "pass/forced".to_owned()];

        let left = replay_from_state(55, initial_snapshot.clone(), &commands, &mut left_state);
        let right = replay_from_state(55, initial_snapshot, &commands, &mut right_state);

        assert_eq!(left, right);
        assert!(left.terminal);
        assert_eq!(left.outcome, Some(TerminalOutcome::Draw));
        assert_eq!(left.projections.len(), 2);
        assert_eq!(left.projections[1].terminal, Some(TerminalOutcome::Draw));
        assert!(left.projections[0]
            .effects
            .iter()
            .any(|effect| effect == "PassTaken:seat_0:1:no_legal_placements"));
        assert!(left.projections[1]
            .effects
            .iter()
            .any(|effect| effect.starts_with("GameEnded:draw:seat_0=1,seat_1=1")));
        assert_eq!(left.projections[1].public_view_hash, left.view_hash);
    }

    #[test]
    fn replay_json_and_effect_strings_do_not_export_hidden_state_names() {
        let commands = first_legal_segments(2, 3);
        let hashes = replay_commands(2, &commands);
        let replay = DirectionalFlipReplayJson {
            schema_version: 1,
            game_id: crate::ids::GAME_ID.to_owned(),
            rules_version: crate::ids::RULES_VERSION_LABEL.to_owned(),
            variant: crate::ids::VARIANT_ID.to_owned(),
            seed: 2,
            initial_snapshot: "snapshot".to_owned(),
            command_segments: commands,
        };
        let export = format!(
            "{}\n{}",
            replay.to_json(),
            hashes
                .projections
                .iter()
                .flat_map(|projection| projection.effects.iter())
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );

        assert!(!export.contains("consecutive_forced_passes"));
        assert!(!export.contains("DirectionalFlipState"));
        assert!(!export.contains("rng"));
        assert!(!export.contains("private"));
        assert!(!export.contains("hidden"));
    }

    fn first_legal_segments(seed: u64, count: usize) -> Vec<String> {
        let mut state =
            setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
        let mut commands = Vec::new();

        for _ in 0..count {
            let actor = actor_for_state(&state);
            let tree = legal_action_tree(&state, &actor);
            let segment = tree
                .root
                .choices
                .first()
                .expect("choice exists")
                .segment
                .clone();
            let command = command_for_state(&state, segment.clone());
            let action = validate_command(&state, &command).unwrap();
            apply_action(&mut state, action);
            commands.push(segment);
        }

        commands
    }

    fn no_move_state() -> DirectionalFlipState {
        let mut state = setup_match(Seed(55), &default_seats(), &SetupOptions::default()).unwrap();
        state.cells = DirectionalFlipState::empty_cells();
        state.active_seat = DirectionalFlipSeat::Seat0;
        state.set_occupancy(
            CellId::new(RowId::R1, ColumnId::C1),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0),
        );
        state.set_occupancy(
            CellId::new(RowId::R8, ColumnId::C8),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1),
        );
        state
    }
}
