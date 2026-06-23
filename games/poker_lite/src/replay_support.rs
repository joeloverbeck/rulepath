use engine_core::{
    ActionPath, ActionTree, ActionTreeEncodingVersion, Actor, CommandEnvelope, EffectEnvelope,
    HashValue, RulesVersion, SeatId, StableSerialize, Viewer, VisibilityScope,
};

use crate::{
    apply_action, filter_effects_for_viewer, legal_action_tree, project_view, setup_effects,
    setup_match, validate_command, Phase, PokerLiteEffect, PokerLiteSeat, PokerLiteState,
    SetupOptions, TerminalOutcome, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

pub type ReplayCommandPath = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PokerLiteInternalTrace {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed_evidence: u64,
    pub commands: Vec<ReplayCommand>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayCommand {
    pub actor: String,
    pub path: ReplayCommandPath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayResult {
    pub trace_hash: HashValue,
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub view_hash: HashValue,
    pub action_tree_hashes: Vec<HashValue>,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub final_state: PokerLiteState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayExport {
    pub schema_version: u32,
    pub export_class: String,
    pub viewer: String,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub steps: Vec<PublicReplayStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayStep {
    pub step_index: usize,
    pub public_view_summary: String,
    pub public_effects: Vec<String>,
    pub redacted_command_summary: String,
    pub terminal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayTimeline {
    pub viewer: String,
    pub steps: Vec<PublicReplayStep>,
}

impl PokerLiteInternalTrace {
    pub fn to_json(&self) -> String {
        let commands = self
            .commands
            .iter()
            .map(ReplayCommand::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed_evidence\":{},\"commands\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed_evidence,
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
            "seed_evidence",
            "commands",
        ])?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            game_id: object.required_string("game_id")?,
            rules_version: object.required_string("rules_version")?,
            variant: object.required_string("variant")?,
            seed_evidence: object.required_u64("seed_evidence")?,
            commands: parse_replay_commands(&object.required_raw("commands")?)?,
        })
    }
}

impl StableSerialize for PokerLiteInternalTrace {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

impl ReplayCommand {
    fn to_json(&self) -> String {
        let path = self
            .path
            .iter()
            .map(|segment| format!("\"{}\"", escape_json(segment)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"actor\":\"{}\",\"path\":[{}]}}",
            escape_json(&self.actor),
            path
        )
    }
}

impl PublicReplayExport {
    pub fn to_json(&self) -> String {
        let steps = self
            .steps
            .iter()
            .map(PublicReplayStep::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"export_class\":\"{}\",\"viewer\":\"{}\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"steps\":[{}]}}",
            self.schema_version,
            escape_json(&self.export_class),
            escape_json(&self.viewer),
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            steps
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        object.reject_unknown(&[
            "schema_version",
            "export_class",
            "viewer",
            "game_id",
            "rules_version",
            "variant",
            "steps",
        ])?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            export_class: object.required_string("export_class")?,
            viewer: object.required_string("viewer")?,
            game_id: object.required_string("game_id")?,
            rules_version: object.required_string("rules_version")?,
            variant: object.required_string("variant")?,
            steps: parse_public_steps(&object.required_raw("steps")?)?,
        })
    }
}

impl StableSerialize for PublicReplayExport {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

impl PublicReplayStep {
    fn to_json(&self) -> String {
        let effects = self
            .public_effects
            .iter()
            .map(|effect| format!("\"{}\"", escape_json(effect)))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"step_index\":{},\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"redacted_command_summary\":\"{}\",\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.public_view_summary),
            effects,
            escape_json(&self.redacted_command_summary),
            self.terminal
        )
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

pub fn command_for_state(
    state: &PokerLiteState,
    actor: PokerLiteSeat,
    action_path: ReplayCommandPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[actor.index()].clone(),
        },
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn generate_internal_full_trace() -> PokerLiteInternalTrace {
    trace_from_commands(
        11,
        &[
            (PokerLiteSeat::Seat0, "hold"),
            (PokerLiteSeat::Seat1, "hold"),
            (PokerLiteSeat::Seat1, "hold"),
            (PokerLiteSeat::Seat0, "hold"),
        ],
    )
}

pub fn trace_from_commands(
    seed: u64,
    commands: &[(PokerLiteSeat, &str)],
) -> PokerLiteInternalTrace {
    PokerLiteInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: seed,
        commands: commands
            .iter()
            .map(|(seat, segment)| ReplayCommand {
                actor: seat.as_str().to_owned(),
                path: vec![(*segment).to_owned()],
            })
            .collect(),
    }
}

pub fn replay_internal_full_trace(trace: &PokerLiteInternalTrace) -> ReplayResult {
    let mut state = setup_match(
        engine_core::Seed(trace.seed_evidence),
        &default_seats(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut effects = setup_effects(&state);
    let mut action_hashes = Vec::new();

    for replay_command in &trace.commands {
        let actor = PokerLiteSeat::parse(&replay_command.actor).expect("trace actor is valid");
        let actor_envelope = Actor {
            seat_id: state.seats[actor.index()].clone(),
        };
        action_hashes.push(action_tree_hash(&legal_action_tree(
            &state,
            &actor_envelope,
        )));
        let command = command_for_state(&state, actor, replay_command.path.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        effects.extend(apply_action(&mut state, action).expect("trace command applies"));
    }

    ReplayResult {
        trace_hash: trace.stable_hash(),
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        view_hash: view_hash(&state, &Viewer { seat_id: None }),
        action_tree_hashes: action_hashes,
        terminal_outcome: state.terminal_outcome,
        final_state: state,
    }
}

pub fn export_public_replay(trace: &PokerLiteInternalTrace, viewer: &Viewer) -> PublicReplayExport {
    let mut state = setup_match(
        engine_core::Seed(trace.seed_evidence),
        &default_seats(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut steps = Vec::new();
    let setup = setup_effects(&state);
    steps.push(public_step(
        0,
        &state,
        &setup,
        viewer,
        "initial_public_state".to_owned(),
    ));

    for (index, replay_command) in trace.commands.iter().enumerate() {
        let actor = PokerLiteSeat::parse(&replay_command.actor).expect("trace actor is valid");
        let command = command_for_state(&state, actor, replay_command.path.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        let effects = apply_action(&mut state, action).expect("trace command applies");
        steps.push(public_step(
            index + 1,
            &state,
            &effects,
            viewer,
            redacted_command_summary(actor),
        ));
    }

    PublicReplayExport {
        schema_version: 1,
        export_class: "viewer_scoped_observation_v1".to_owned(),
        viewer: viewer
            .seat_id
            .as_ref()
            .map_or_else(|| "observer".to_owned(), |seat| seat.0.clone()),
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        steps,
    }
}

pub fn import_public_export(export: &PublicReplayExport) -> PublicReplayTimeline {
    PublicReplayTimeline {
        viewer: export.viewer.clone(),
        steps: export.steps.clone(),
    }
}

pub fn state_hash(state: &PokerLiteState) -> HashValue {
    HashValue::from_stable_bytes(state.stable_internal_summary().as_bytes())
}

pub fn view_hash(state: &PokerLiteState, viewer: &Viewer) -> HashValue {
    project_view(state, viewer).stable_hash()
}

pub fn action_tree_hash(tree: &engine_core::ActionTree) -> HashValue {
    let summary = tree
        .root
        .choices
        .iter()
        .map(|choice| {
            format!(
                "{}:{}:{}:{}",
                choice.segment,
                choice.label,
                choice
                    .metadata
                    .iter()
                    .map(|entry| format!("{}={}", entry.key, entry.value))
                    .collect::<Vec<_>>()
                    .join("|"),
                choice.tags.join("|")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(
        format!("freshness={};{summary}", tree.freshness_token.0).as_bytes(),
    )
}

pub fn action_tree_v1_bytes(tree: &ActionTree) -> Vec<u8> {
    tree.stable_bytes(ActionTreeEncodingVersion::V1)
}

pub fn action_tree_v1_hash(tree: &ActionTree) -> HashValue {
    tree.stable_hash(ActionTreeEncodingVersion::V1)
}

pub fn effect_hash(effects: &[EffectEnvelope<PokerLiteEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<PokerLiteEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "public".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => format!("private:{}", seat.0),
    };
    format!("{}:{:?}", visibility, effect.payload)
}

fn public_step(
    step_index: usize,
    state: &PokerLiteState,
    effects: &[EffectEnvelope<PokerLiteEffect>],
    viewer: &Viewer,
    redacted_command_summary: String,
) -> PublicReplayStep {
    let filtered = filter_effects_for_viewer(effects, viewer);
    PublicReplayStep {
        step_index,
        public_view_summary: project_view(state, viewer).stable_summary(),
        public_effects: filtered.iter().map(effect_stable_string).collect(),
        redacted_command_summary,
        terminal: state.phase == Phase::Terminal,
    }
}

fn redacted_command_summary(actor: PokerLiteSeat) -> String {
    format!("{}:pledge_action_redacted", actor.as_str())
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn parse_replay_commands(raw: &str) -> Result<Vec<ReplayCommand>, String> {
    parse_object_array(raw)?
        .into_iter()
        .map(|object| {
            object.reject_unknown(&["actor", "path"])?;
            Ok(ReplayCommand {
                actor: object.required_string("actor")?,
                path: parse_string_array(&object.required_raw("path")?)?,
            })
        })
        .collect()
}

fn parse_public_steps(raw: &str) -> Result<Vec<PublicReplayStep>, String> {
    parse_object_array(raw)?
        .into_iter()
        .map(|object| {
            object.reject_unknown(&[
                "step_index",
                "public_view_summary",
                "public_effects",
                "redacted_command_summary",
                "terminal",
            ])?;
            Ok(PublicReplayStep {
                step_index: object.required_usize("step_index")?,
                public_view_summary: object.required_string("public_view_summary")?,
                public_effects: parse_string_array(&object.required_raw("public_effects")?)?,
                redacted_command_summary: object.required_string("redacted_command_summary")?,
                terminal: object.required_bool("terminal")?,
            })
        })
        .collect()
}

fn parse_object_array(raw: &str) -> Result<Vec<StrictJsonObject>, String> {
    let body = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| "expected object array".to_owned())?;
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(body, ',')?
        .into_iter()
        .map(|value| StrictJsonObject::parse(value.trim()))
        .collect()
}

fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    let body = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| "expected string array".to_owned())?;
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(body, ',')?
        .into_iter()
        .map(|value| parse_json_string(value.trim()))
        .collect()
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

    fn required_usize(&self, key: &str) -> Result<usize, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must be usize"))
    }

    fn required_bool(&self, key: &str) -> Result<bool, String> {
        match self.required_raw(key)?.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("field `{key}` must be bool")),
        }
    }
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
    Err("expected key/value field".to_owned())
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
        if depth < 0 {
            return Err("malformed JSON nesting".to_owned());
        }
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }

    if in_string || depth != 0 {
        return Err("malformed JSON nesting".to_owned());
    }
    result.push(input[start..].to_owned());
    Ok(result)
}

fn parse_json_string(raw: &str) -> Result<String, String> {
    let body = raw
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut result = String::new();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some(other) => return Err(format!("unsupported escape `\\{other}`")),
                None => return Err("unterminated escape".to_owned()),
            }
        } else {
            result.push(ch);
        }
    }
    Ok(result)
}
