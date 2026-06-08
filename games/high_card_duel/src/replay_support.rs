use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize, Viewer, VisibilityScope,
};

use crate::{
    active_commit_seat, apply_action, legal_action_tree, project_view, setup_match,
    validate_command, HighCardDuelEffect, HighCardDuelSeat, HighCardDuelState, Phase, SetupOptions,
    TerminalOutcome, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID,
};

pub type ReplayCommandPath = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighCardDuelInternalTrace {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed: u64,
    pub command_paths: Vec<ReplayCommandPath>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalReplayResult {
    pub trace_hash: HashValue,
    pub state_hash: HashValue,
    pub effect_hash: HashValue,
    pub revealed_sequence: Vec<String>,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub final_state: HighCardDuelState,
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

impl HighCardDuelInternalTrace {
    pub fn to_json(&self) -> String {
        let commands = self
            .command_paths
            .iter()
            .map(|path| {
                let segments = path
                    .iter()
                    .map(|segment| format!("\"{}\"", escape_json(segment)))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{segments}]")
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed\":{},\"command_paths\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed,
            commands
        )
    }
}

impl StableSerialize for HighCardDuelInternalTrace {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
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
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &HighCardDuelState) -> Actor {
    let seat = active_commit_seat(state).expect("non-terminal state has an active commit seat");
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

pub fn command_for_state(
    state: &HighCardDuelState,
    action_path: ReplayCommandPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn generate_internal_full_trace(seed: u64) -> HighCardDuelInternalTrace {
    let mut state = setup_match(Seed(seed), &default_seats(), &SetupOptions::default())
        .expect("setup succeeds");
    let mut command_paths = Vec::new();

    while state.phase != Phase::Terminal {
        let actor = actor_for_state(&state);
        let tree = legal_action_tree(&state, &actor);
        let choice = tree
            .root
            .choices
            .first()
            .expect("non-terminal generated trace has a legal choice");
        let path = vec![choice.segment.clone()];
        let command = command_for_state(&state, path.clone());
        let action = validate_command(&state, &command).expect("generated command validates");
        apply_action(&mut state, action);
        command_paths.push(path);
    }

    HighCardDuelInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed,
        command_paths,
    }
}

pub fn replay_internal_full_trace(trace: &HighCardDuelInternalTrace) -> InternalReplayResult {
    let mut state = setup_match(Seed(trace.seed), &default_seats(), &SetupOptions::default())
        .expect("setup succeeds");
    let mut effects = Vec::new();

    for path in &trace.command_paths {
        let command = command_for_state(&state, path.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        effects.extend(apply_action(&mut state, action));
    }

    InternalReplayResult {
        trace_hash: trace.stable_hash(),
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        revealed_sequence: revealed_sequence(&state),
        terminal_outcome: state.terminal_outcome,
        final_state: state,
    }
}

pub fn export_public_observer_replay(trace: &HighCardDuelInternalTrace) -> PublicReplayExport {
    let mut state = setup_match(Seed(trace.seed), &default_seats(), &SetupOptions::default())
        .expect("setup succeeds");
    let mut steps = Vec::new();

    steps.push(public_step(
        0,
        &state,
        &[],
        "initial_public_state".to_owned(),
    ));

    for (index, path) in trace.command_paths.iter().enumerate() {
        let command = command_for_state(&state, path.clone());
        let actor = active_commit_seat(&state).expect("trace command has active actor");
        let action = validate_command(&state, &command).expect("trace command validates");
        let effects = apply_action(&mut state, action);
        steps.push(public_step(
            index + 1,
            &state,
            &effects,
            redacted_command_summary(actor),
        ));
    }

    PublicReplayExport {
        schema_version: 1,
        export_class: "public_observer_projection_v1".to_owned(),
        viewer: "observer".to_owned(),
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

pub fn state_hash(state: &HighCardDuelState) -> HashValue {
    HashValue::from_stable_bytes(internal_state_summary(state).as_bytes())
}

pub fn effect_hash(effects: &[EffectEnvelope<HighCardDuelEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<HighCardDuelEffect>) -> String {
    let visibility = match &effect.visibility {
        VisibilityScope::Public => "public".to_owned(),
        VisibilityScope::PrivateToSeat(seat) => format!("private:{}", seat.0),
    };
    format!("{}:{}", visibility, effect.payload.public_payload_text())
}

pub fn public_effect_stable_string(effect: &EffectEnvelope<HighCardDuelEffect>) -> Option<String> {
    matches!(effect.visibility, VisibilityScope::Public)
        .then(|| effect.payload.public_payload_text())
}

fn public_step(
    step_index: usize,
    state: &HighCardDuelState,
    effects: &[EffectEnvelope<HighCardDuelEffect>],
    redacted_command_summary: String,
) -> PublicReplayStep {
    PublicReplayStep {
        step_index,
        public_view_summary: project_view(state, &Viewer { seat_id: None }).stable_summary(),
        public_effects: effects
            .iter()
            .filter_map(public_effect_stable_string)
            .collect(),
        redacted_command_summary,
        terminal: state.phase == Phase::Terminal,
    }
}

fn redacted_command_summary(actor: HighCardDuelSeat) -> String {
    format!("{}:commit_redacted", actor.as_str())
}

fn revealed_sequence(state: &HighCardDuelState) -> Vec<String> {
    state
        .revealed_history
        .iter()
        .map(|round| {
            format!(
                "{}:{}:{}:{}",
                round.round_number,
                round.seat_0_card.stable_id(),
                round.seat_1_card.stable_id(),
                round.winner.map_or("draw", HighCardDuelSeat::as_str)
            )
        })
        .collect()
}

fn internal_state_summary(state: &HighCardDuelState) -> String {
    format!(
        "game={};variant={};round={};phase={};lead={};score={}-{};hands={}|{};commitments={}|{};revealed={};deck={};terminal={};freshness={}",
        GAME_ID,
        state.variant.id,
        state.round_number,
        state.phase.as_str(),
        state.lead_seat.as_str(),
        state.score.seat_0,
        state.score.seat_1,
        state.hands[HighCardDuelSeat::Seat0.index()]
            .iter()
            .map(|card| card.stable_id())
            .collect::<Vec<_>>()
            .join(","),
        state.hands[HighCardDuelSeat::Seat1.index()]
            .iter()
            .map(|card| card.stable_id())
            .collect::<Vec<_>>()
            .join(","),
        state.commitments[HighCardDuelSeat::Seat0.index()]
            .map_or_else(|| "none".to_owned(), |card| card.stable_id()),
        state.commitments[HighCardDuelSeat::Seat1.index()]
            .map_or_else(|| "none".to_owned(), |card| card.stable_id()),
        revealed_sequence(state).join(","),
        state
            .deck
            .iter()
            .map(|card| card.stable_id())
            .collect::<Vec<_>>()
            .join(","),
        terminal_summary(state.terminal_outcome),
        state.freshness_token.0
    )
}

fn terminal_summary(outcome: Option<TerminalOutcome>) -> String {
    match outcome {
        None => "none".to_owned(),
        Some(TerminalOutcome::Draw) => "draw".to_owned(),
        Some(TerminalOutcome::Win { seat }) => format!("win:{}", seat.as_str()),
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
