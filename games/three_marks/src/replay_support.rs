use engine_core::{
    ActionPath, ActionTree, Actor, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion,
    SeatId, Seed, StableSerialize, Viewer,
};

use crate::{
    apply_action,
    bots::ThreeMarksLevel1Bot,
    effects::{RejectionReason, ThreeMarksEffect},
    ids::CellId,
    legal_action_tree, project_view, setup_match, validate_command, validate_command_with_effects,
    CellOccupancy, SetupOptions, TerminalOutcome, ThreeMarksSnapshot, ThreeMarksState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayHashes {
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub view_hash: HashValue,
    pub replay_hash: HashValue,
    pub diagnostic_hash: Option<HashValue>,
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThreeMarksReplayJson {
    pub schema_version: u32,
    pub rules_version: u32,
    pub seed: u64,
    pub initial_snapshot: String,
    pub command_segments: Vec<String>,
}

impl ThreeMarksReplayJson {
    pub fn to_json(&self) -> String {
        let commands = self
            .command_segments
            .iter()
            .map(|segment| format!("\"{}\"", escape_json(segment)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"rules_version\":{},\"seed\":{},\"initial_snapshot\":\"{}\",\"command_segments\":[{}]}}",
            self.schema_version,
            self.rules_version,
            self.seed,
            escape_json(&self.initial_snapshot),
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
            initial_snapshot: object.required_string("initial_snapshot")?,
            command_segments: parse_string_array(&object.required_raw("command_segments")?)?,
        })
    }
}

impl StableSerialize for ThreeMarksReplayJson {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &ThreeMarksState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(state: &ThreeMarksState, segment: String) -> CommandEnvelope {
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
    let initial_snapshot = ThreeMarksSnapshot::from_state(&state).stable_summary();
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
        None,
        projections,
    )
}

pub fn replay_bot_action(seed: u64) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = ThreeMarksSnapshot::from_state(&state).stable_summary();
    let decision = ThreeMarksLevel1Bot::new()
        .select_decision(&state, state.active_seat)
        .expect("bot action selected");
    let command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: decision.action_path.clone(),
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    };
    let action = validate_command(&state, &command).expect("bot action validates");
    let mut effects = decision.effects;
    let step_effects = apply_action(&mut state, action);
    effects.extend(step_effects.clone());
    let projections = vec![project_step(0, &state, &effects)];
    let commands = vec![decision.action_path.segments[0].clone()];

    hashes_for_state(
        seed,
        &initial_snapshot,
        &commands,
        &state,
        &effects,
        None,
        projections,
    )
}

pub fn replay_invalid(seed: u64, invalid: &str, stale: &str) -> ReplayHashes {
    let state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = ThreeMarksSnapshot::from_state(&state).stable_summary();
    let invalid_rejected =
        validate_command_with_effects(&state, &command_for_state(&state, invalid.to_owned()))
            .expect_err("invalid command rejected");
    let stale_command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: ActionPath {
            segments: vec![stale.to_owned()],
        },
        freshness_token: state.freshness_token.next(),
        rules_version: RulesVersion(1),
    };
    let stale_rejected =
        validate_command_with_effects(&state, &stale_command).expect_err("stale command rejected");
    let diagnostic_hash = HashValue::from_stable_bytes(
        format!(
            "{}:{}|{}:{}",
            invalid_rejected.diagnostic.code,
            invalid_rejected.diagnostic.message,
            stale_rejected.diagnostic.code,
            stale_rejected.diagnostic.message
        )
        .as_bytes(),
    );
    let mut effects = invalid_rejected.effects;
    effects.extend(stale_rejected.effects);

    hashes_for_state(
        seed,
        &initial_snapshot,
        &[],
        &state,
        &effects,
        Some(diagnostic_hash),
        vec![project_step(0, &state, &effects)],
    )
}

pub fn replay_diagnostic(seed: u64, commands: &[String], diagnostic: &str) -> ReplayHashes {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = ThreeMarksSnapshot::from_state(&state).stable_summary();
    let mut effects = Vec::new();

    for segment in commands {
        let command = command_for_state(&state, segment.clone());
        let action = validate_command(&state, &command).expect("setup command validates");
        effects.extend(apply_action(&mut state, action));
    }

    let rejected =
        validate_command_with_effects(&state, &command_for_state(&state, diagnostic.to_owned()))
            .expect_err("diagnostic command rejected");
    let diagnostic_hash = HashValue::from_stable_bytes(
        format!(
            "{}:{}",
            rejected.diagnostic.code, rejected.diagnostic.message
        )
        .as_bytes(),
    );
    effects.extend(rejected.effects);

    hashes_for_state(
        seed,
        &initial_snapshot,
        commands,
        &state,
        &effects,
        Some(diagnostic_hash),
        vec![project_step(commands.len(), &state, &effects)],
    )
}

pub fn replay_stale(seed: u64, stale: &str) -> ReplayHashes {
    let state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default()).unwrap();
    let initial_snapshot = ThreeMarksSnapshot::from_state(&state).stable_summary();
    let stale_command = CommandEnvelope {
        actor: actor_for_state(&state),
        action_path: ActionPath {
            segments: vec![stale.to_owned()],
        },
        freshness_token: state.freshness_token.next(),
        rules_version: RulesVersion(1),
    };
    let rejected =
        validate_command_with_effects(&state, &stale_command).expect_err("stale command rejected");
    let diagnostic_hash = HashValue::from_stable_bytes(
        format!(
            "{}:{}",
            rejected.diagnostic.code, rejected.diagnostic.message
        )
        .as_bytes(),
    );
    let effects = rejected.effects;

    hashes_for_state(
        seed,
        &initial_snapshot,
        &[],
        &state,
        &effects,
        Some(diagnostic_hash),
        vec![project_step(0, &state, &effects)],
    )
}

pub fn hashes_for_state(
    seed: u64,
    initial_snapshot: &str,
    commands: &[String],
    state: &ThreeMarksState,
    effects: &[EffectEnvelope<ThreeMarksEffect>],
    diagnostic_hash: Option<HashValue>,
    projections: Vec<ReplayStepProjection>,
) -> ReplayHashes {
    let actor = actor_for_state(state);
    let replay = ThreeMarksReplayJson {
        schema_version: 1,
        rules_version: 1,
        seed,
        initial_snapshot: initial_snapshot.to_owned(),
        command_segments: commands.to_vec(),
    };
    ReplayHashes {
        state_hash: ThreeMarksSnapshot::from_state(state).stable_hash(),
        effect_hash: effect_hash(effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor)),
        view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash: replay.stable_hash(),
        diagnostic_hash,
        outcome: state.terminal_outcome,
        terminal: state.terminal_outcome.is_some(),
        projections,
    }
}

pub fn effect_hash(effects: &[EffectEnvelope<ThreeMarksEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<ThreeMarksEffect>) -> String {
    match &effect.payload {
        ThreeMarksEffect::SetupComplete {
            game_id,
            variant_id,
            rules_version,
            seats,
        } => format!(
            "SetupComplete:{game_id}:{variant_id}:{rules_version}:{}:{}",
            seats[0], seats[1]
        ),
        ThreeMarksEffect::MarkPlaced {
            seat,
            cell,
            ply,
            occupancy_summary,
        } => format!(
            "MarkPlaced:{}:{}:{ply}:{occupancy_summary}",
            seat.as_str(),
            cell.as_str()
        ),
        ThreeMarksEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "ActivePlayerChanged:{}:{}:{ply}",
            previous_seat.as_str(),
            active_seat.as_str()
        ),
        ThreeMarksEffect::PlacementRejected { reason, label } => {
            format!("PlacementRejected:{}:{label}", rejection_reason(*reason))
        }
        ThreeMarksEffect::LineCompleted { winning_seat, line } => format!(
            "LineCompleted:{}:{}-{}-{}",
            winning_seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
        ThreeMarksEffect::DrawReached {
            final_ply,
            full_board,
        } => format!("DrawReached:{final_ply}:{full_board}"),
        ThreeMarksEffect::GameEnded {
            outcome,
            final_ply,
            terminal_hash_ref,
        } => format!(
            "GameEnded:{}:{final_ply}:{terminal_hash_ref}",
            outcome_summary(*outcome)
        ),
        ThreeMarksEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            cell,
            explanation,
        } => format!(
            "BotChoseAction:{level}:{policy_id}:{action_id}:{}:{explanation}",
            cell.as_str()
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
    state: &ThreeMarksState,
    effects: &[EffectEnvelope<ThreeMarksEffect>],
) -> ReplayStepProjection {
    ReplayStepProjection {
        step_index,
        board: CellId::ALL
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}",
                    cell.as_str(),
                    match state.occupancy(*cell) {
                        CellOccupancy::Empty => "empty",
                        CellOccupancy::Occupied(seat) => seat.as_str(),
                    }
                )
            })
            .collect(),
        effects: effects.iter().map(effect_stable_string).collect(),
        terminal: state.terminal_outcome,
    }
}

fn outcome_summary(outcome: TerminalOutcome) -> String {
    match outcome {
        TerminalOutcome::Draw => "draw".to_owned(),
        TerminalOutcome::Win { seat, line } => format!(
            "win:{}:{}-{}-{}",
            seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
    }
}

fn rejection_reason(reason: RejectionReason) -> &'static str {
    match reason {
        RejectionReason::Occupied => "occupied",
        RejectionReason::Stale => "stale",
        RejectionReason::InvalidCell => "invalid_cell",
        RejectionReason::WrongActor => "wrong_actor",
        RejectionReason::Terminal => "terminal",
        RejectionReason::UnknownActor => "unknown_actor",
        RejectionReason::InvalidPath => "invalid_path",
        RejectionReason::InvalidAction => "invalid_action",
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
