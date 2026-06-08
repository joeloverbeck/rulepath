use engine_core::{
    ActionPath, ActionTree, Actor, CommandEnvelope, EffectEnvelope, HashValue, RulesVersion,
    SeatId, Seed, StableSerialize, Viewer,
};

use crate::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, TerminalOutcome,
    TokenBazaarEffect, TokenBazaarSnapshot, TokenBazaarState, GAME_ID, RULES_VERSION_LABEL,
    VARIANT_ID,
};

pub type ReplayCommandPath = Vec<String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayResult {
    pub seed: u64,
    pub command_paths: Vec<ReplayCommandPath>,
    pub initial_state_hash: HashValue,
    pub final_state_hash: HashValue,
    pub effect_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub public_view_hash: HashValue,
    pub replay_hash: HashValue,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub terminal: bool,
    pub final_state: TokenBazaarState,
    pub projections: Vec<ReplayStepProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayStepProjection {
    pub step_index: usize,
    pub command_summary: String,
    pub effects: Vec<String>,
    pub state_hash: HashValue,
    pub action_tree_hash: HashValue,
    pub public_view_hash: HashValue,
    pub terminal: Option<TerminalOutcome>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayExport {
    pub schema_version: u32,
    pub export_class: String,
    pub game_id: String,
    pub rules_version: String,
    pub variant: String,
    pub seed: u64,
    pub command_paths: Vec<ReplayCommandPath>,
    pub steps: Vec<PublicReplayStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayStep {
    pub step_index: usize,
    pub public_view_summary: String,
    pub public_effects: Vec<String>,
    pub command_summary: String,
    pub terminal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayTimeline {
    pub game_id: String,
    pub variant: String,
    pub steps: Vec<PublicReplayStep>,
}

impl PublicReplayExport {
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
        let steps = self
            .steps
            .iter()
            .map(PublicReplayStep::to_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"export_class\":\"{}\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"seed\":{},\"command_paths\":[{}],\"steps\":[{}]}}",
            self.schema_version,
            escape_json(&self.export_class),
            escape_json(&self.game_id),
            escape_json(&self.rules_version),
            escape_json(&self.variant),
            self.seed,
            commands,
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
            "{{\"step_index\":{},\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"command_summary\":\"{}\",\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.public_view_summary),
            effects,
            escape_json(&self.command_summary),
            self.terminal
        )
    }
}

pub fn default_seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

pub fn actor_for_state(state: &TokenBazaarState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

pub fn command_for_state(
    state: &TokenBazaarState,
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

pub fn replay_commands(seed: u64, command_paths: &[ReplayCommandPath]) -> ReplayResult {
    let mut state =
        setup_match(Seed(seed), &default_seats(), &Default::default()).expect("setup succeeds");
    let initial_state_hash = state_hash(&state);
    let mut effects = Vec::new();
    let mut projections = Vec::new();

    for (index, path) in command_paths.iter().enumerate() {
        let command = command_for_state(&state, path.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        let step_effects = apply_action(&mut state, action);
        projections.push(project_step(index, path, &state, &step_effects));
        effects.extend(step_effects);
    }

    let actor = actor_for_hash(&state);
    let export = export_public_replay(seed, command_paths);
    ReplayResult {
        seed,
        command_paths: command_paths.to_vec(),
        initial_state_hash,
        final_state_hash: state_hash(&state),
        effect_hash: effect_hash(&effects),
        action_tree_hash: action_tree_hash(&legal_action_tree(&state, &actor)),
        public_view_hash: project_view(&state, &Viewer { seat_id: None }).stable_hash(),
        replay_hash: export.stable_hash(),
        terminal_outcome: state.terminal_outcome,
        terminal: state.terminal_outcome.is_some(),
        final_state: state,
        projections,
    }
}

pub fn export_public_replay(seed: u64, command_paths: &[ReplayCommandPath]) -> PublicReplayExport {
    let mut state =
        setup_match(Seed(seed), &default_seats(), &Default::default()).expect("setup succeeds");
    let mut steps = Vec::new();
    steps.push(public_step(
        0,
        &state,
        &[],
        "initial_public_state".to_owned(),
    ));

    for (index, path) in command_paths.iter().enumerate() {
        let command = command_for_state(&state, path.clone());
        let action = validate_command(&state, &command).expect("trace command validates");
        let effects = apply_action(&mut state, action);
        steps.push(public_step(index + 1, &state, &effects, path.join("/")));
    }

    PublicReplayExport {
        schema_version: 1,
        export_class: "token_bazaar_public_replay_v1".to_owned(),
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed,
        command_paths: command_paths.to_vec(),
        steps,
    }
}

pub fn import_public_export(export: &PublicReplayExport) -> PublicReplayTimeline {
    PublicReplayTimeline {
        game_id: export.game_id.clone(),
        variant: export.variant.clone(),
        steps: export.steps.clone(),
    }
}

pub fn state_hash(state: &TokenBazaarState) -> HashValue {
    TokenBazaarSnapshot::from_state(state).stable_hash()
}

pub fn effect_hash(effects: &[EffectEnvelope<TokenBazaarEffect>]) -> HashValue {
    let bytes = effects
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

pub fn effect_stable_string(effect: &EffectEnvelope<TokenBazaarEffect>) -> String {
    format!("public:{}", effect.payload.stable_summary())
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
            format!("{}:{metadata}", choice.segment)
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn project_step(
    step_index: usize,
    path: &ReplayCommandPath,
    state: &TokenBazaarState,
    effects: &[EffectEnvelope<TokenBazaarEffect>],
) -> ReplayStepProjection {
    ReplayStepProjection {
        step_index,
        command_summary: path.join("/"),
        effects: effects.iter().map(effect_stable_string).collect(),
        state_hash: state_hash(state),
        action_tree_hash: action_tree_hash(&legal_action_tree(state, &actor_for_hash(state))),
        public_view_hash: project_view(state, &Viewer { seat_id: None }).stable_hash(),
        terminal: state.terminal_outcome,
    }
}

fn public_step(
    step_index: usize,
    state: &TokenBazaarState,
    effects: &[EffectEnvelope<TokenBazaarEffect>],
    command_summary: String,
) -> PublicReplayStep {
    PublicReplayStep {
        step_index,
        public_view_summary: project_view(state, &Viewer { seat_id: None }).stable_summary(),
        public_effects: effects.iter().map(effect_stable_string).collect(),
        command_summary,
        terminal: state.terminal_outcome.is_some(),
    }
}

fn actor_for_hash(state: &TokenBazaarState) -> Actor {
    if state.terminal_outcome.is_some() {
        Actor {
            seat_id: SeatId("terminal".to_owned()),
        }
    } else {
        actor_for_state(state)
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::FreshnessToken;

    fn terminal_collect_commands() -> Vec<ReplayCommandPath> {
        [
            "collect/amber",
            "collect/amber",
            "collect/amber",
            "collect/amber",
            "collect/amber",
            "collect/amber",
            "collect/jade",
            "collect/jade",
            "collect/jade",
            "collect/jade",
            "collect/jade",
            "collect/jade",
            "collect/iron",
            "collect/iron",
            "collect/iron",
            "collect/iron",
        ]
        .into_iter()
        .map(|segment| vec![segment.to_owned()])
        .collect()
    }

    #[test]
    fn command_stream_reproduces_hashes_and_terminal_outcome() {
        let commands = terminal_collect_commands();
        let left = replay_commands(9, &commands);
        let right = replay_commands(9, &commands);

        assert_eq!(left.final_state_hash, right.final_state_hash);
        assert_eq!(left.effect_hash, right.effect_hash);
        assert_eq!(left.action_tree_hash, right.action_tree_hash);
        assert_eq!(left.public_view_hash, right.public_view_hash);
        assert_eq!(left.replay_hash, right.replay_hash);
        assert_eq!(left.terminal_outcome, Some(TerminalOutcome::Draw));
        assert!(left.terminal);
        assert_eq!(left.final_state.turns_taken, [8, 8]);
        assert_eq!(left.projections.len(), commands.len());
    }

    #[test]
    fn public_export_import_is_lossless_and_public_safe() {
        let commands = terminal_collect_commands();
        let export = export_public_replay(11, &commands);
        let timeline = import_public_export(&export);

        assert_eq!(timeline.game_id, GAME_ID);
        assert_eq!(timeline.variant, VARIANT_ID);
        assert_eq!(timeline.steps, export.steps);
        assert_eq!(export.steps.len(), commands.len() + 1);
        assert!(export.steps.last().expect("terminal step").terminal);
        let json = export.to_json();
        assert_eq!(export.stable_bytes(), json.clone().into_bytes());
        assert!(!json.contains("debug"));
        assert!(!json.contains("candidate"));
        assert!(!json.contains("valuation"));
        assert!(!json.contains("internal"));
    }

    #[test]
    fn invalid_and_stale_commands_reject_without_mutation_during_replay() {
        let state =
            setup_match(Seed(1), &default_seats(), &Default::default()).expect("setup succeeds");
        let original = TokenBazaarSnapshot::from_state(&state).stable_summary();

        let stale = CommandEnvelope {
            freshness_token: FreshnessToken(99),
            ..command_for_state(&state, vec!["collect/amber".to_owned()])
        };
        assert_eq!(
            validate_command(&state, &stale)
                .expect_err("stale command")
                .code,
            "stale_action"
        );
        assert_eq!(
            TokenBazaarSnapshot::from_state(&state).stable_summary(),
            original
        );

        let invalid = command_for_state(&state, vec!["fulfill/slot_1".to_owned()]);
        assert_eq!(
            validate_command(&state, &invalid)
                .expect_err("insufficient command")
                .code,
            "insufficient_cost"
        );
        assert_eq!(
            TokenBazaarSnapshot::from_state(&state).stable_summary(),
            original
        );
    }
}
