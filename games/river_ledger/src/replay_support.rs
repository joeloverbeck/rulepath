use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, StableSerialize, Viewer,
};

use crate::{
    apply_action, filter_effects_for_viewer, legal_action_tree, project_view, setup_effects,
    setup_match, validate_command, RiverLedgerState, SetupOptions, GAME_ID, RULES_VERSION_LABEL,
    VARIANT_ID,
};

pub type ReplayCommandPath = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerInternalTrace {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed_evidence: u64,
    pub seat_count: usize,
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
    pub final_state: RiverLedgerState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayImportDiagnostic {
    pub code: String,
    pub message: String,
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

impl RiverLedgerInternalTrace {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed_evidence\":{},\"seat_count\":{},\"commands\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed_evidence,
            self.seat_count,
            self.commands
                .iter()
                .map(ReplayCommand::to_json)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl StableSerialize for RiverLedgerInternalTrace {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

impl ReplayCommand {
    fn to_json(&self) -> String {
        format!(
            "{{\"actor\":\"{}\",\"path\":[{}]}}",
            escape_json(&self.actor),
            self.path
                .iter()
                .map(|segment| format!("\"{}\"", escape_json(segment)))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl PublicReplayExport {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"export_class\":\"{}\",\"viewer\":\"{}\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"steps\":[{}]}}",
            self.schema_version,
            escape_json(&self.export_class),
            escape_json(&self.viewer),
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.steps
                .iter()
                .map(PublicReplayStep::to_json)
                .collect::<Vec<_>>()
                .join(",")
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
        format!(
            "{{\"step_index\":{},\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"redacted_command_summary\":\"{}\",\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.public_view_summary),
            self.public_effects
                .iter()
                .map(|effect| format!("\"{}\"", escape_json(effect)))
                .collect::<Vec<_>>()
                .join(","),
            escape_json(&self.redacted_command_summary),
            self.terminal
        )
    }
}

pub fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

pub fn trace_from_commands(
    seed: u64,
    seat_count: usize,
    commands: &[(usize, &str)],
) -> RiverLedgerInternalTrace {
    RiverLedgerInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: seed,
        seat_count,
        commands: commands
            .iter()
            .map(|(seat, segment)| ReplayCommand {
                actor: format!("seat_{seat}"),
                path: vec![(*segment).to_owned()],
            })
            .collect(),
    }
}

pub fn command_for_state(
    state: &RiverLedgerState,
    actor_seat: &str,
    action_path: ReplayCommandPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: SeatId(actor_seat.to_owned()),
        },
        action_path: ActionPath {
            segments: action_path,
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

pub fn replay_internal_full_trace(trace: &RiverLedgerInternalTrace) -> ReplayResult {
    replay_internal_full_trace_result(trace).expect("trace version is supported")
}

pub fn replay_internal_full_trace_result(
    trace: &RiverLedgerInternalTrace,
) -> Result<ReplayResult, ReplayImportDiagnostic> {
    validate_trace_version(trace)?;
    let mut state = setup_match(
        engine_core::Seed(trace.seed_evidence),
        &seats(trace.seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let effects = setup_effects(&state);
    let mut action_hashes = Vec::new();

    for command in &trace.commands {
        let envelope = command_for_state(&state, &command.actor, command.path.clone());
        action_hashes.push(action_tree_hash(&legal_action_tree(
            &state,
            &envelope.actor,
        )));
        let action = validate_command(&state, &envelope).expect("trace command validates");
        apply_action(&mut state, action).expect("trace command applies");
    }

    Ok(ReplayResult {
        trace_hash: trace.stable_hash(),
        state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        view_hash: crate::view_hash(&project_view(&state, &Viewer { seat_id: None })),
        action_tree_hashes: action_hashes,
        final_state: state,
    })
}

pub fn export_public_replay(
    trace: &RiverLedgerInternalTrace,
    viewer: &Viewer,
) -> PublicReplayExport {
    validate_trace_version(trace).expect("trace version is supported");
    let mut state = setup_match(
        engine_core::Seed(trace.seed_evidence),
        &seats(trace.seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut steps = Vec::new();
    push_step(
        &mut steps,
        0,
        &state,
        &filter_effects_for_viewer(&setup_effects(&state), viewer),
        "setup",
        viewer,
    );

    for (index, command) in trace.commands.iter().enumerate() {
        let envelope = command_for_state(&state, &command.actor, command.path.clone());
        let action = validate_command(&state, &envelope).expect("trace command validates");
        apply_action(&mut state, action).expect("trace command applies");
        push_step(
            &mut steps,
            index + 1,
            &state,
            &[],
            &redacted_command_summary(command),
            viewer,
        );
    }

    PublicReplayExport {
        schema_version: 1,
        export_class: "viewer_scoped_public_replay".to_owned(),
        viewer: viewer
            .seat_id
            .as_ref()
            .map(|seat| seat.0.clone())
            .unwrap_or_else(|| "observer".to_owned()),
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        steps,
    }
}

fn validate_trace_version(trace: &RiverLedgerInternalTrace) -> Result<(), ReplayImportDiagnostic> {
    if trace.rules_version == RULES_VERSION_LABEL {
        return Ok(());
    }
    Err(ReplayImportDiagnostic {
        code: "river_ledger_rules_version_mismatch".to_owned(),
        message: format!(
            "River Ledger replay uses {}; expected {}",
            trace.rules_version, RULES_VERSION_LABEL
        ),
    })
}

pub fn import_public_export(export: &PublicReplayExport) -> PublicReplayTimeline {
    PublicReplayTimeline {
        viewer: export.viewer.clone(),
        steps: export.steps.clone(),
    }
}

pub fn state_hash(state: &RiverLedgerState) -> HashValue {
    HashValue::from_stable_bytes(state.stable_internal_summary().as_bytes())
}

pub fn action_tree_hash(tree: &engine_core::ActionTree) -> HashValue {
    HashValue::from_stable_bytes(format!("{tree:?}").as_bytes())
}

pub fn effect_hash(effects: &[engine_core::EffectEnvelope<crate::RiverLedgerEffect>]) -> HashValue {
    HashValue::from_stable_bytes(format!("{effects:?}").as_bytes())
}

fn push_step(
    steps: &mut Vec<PublicReplayStep>,
    step_index: usize,
    state: &RiverLedgerState,
    effects: &[engine_core::EffectEnvelope<crate::RiverLedgerEffect>],
    command_summary: &str,
    viewer: &Viewer,
) {
    steps.push(PublicReplayStep {
        step_index,
        public_view_summary: project_view(state, viewer).stable_summary(),
        public_effects: effects.iter().map(|effect| format!("{effect:?}")).collect(),
        redacted_command_summary: command_summary.to_owned(),
        terminal: state.terminal_outcome.is_some(),
    });
}

fn redacted_command_summary(command: &ReplayCommand) -> String {
    format!("{}:{}", command.actor, command.path.join("/"))
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
