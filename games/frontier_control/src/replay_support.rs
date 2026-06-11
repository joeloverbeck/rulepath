//! Replay/export support for Frontier Control.

use engine_core::{CommandEnvelope, StableSerialize, Viewer};

use crate::{
    effects::FrontierControlEffectEnvelope,
    ids::{GAME_ID, RULES_VERSION_LABEL},
    state::FrontierControlState,
    visibility::{filter_effects_for_viewer, project_view, public_effect_text},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceNotApplicable {
    pub hidden_information_redaction: String,
    pub stochastic_game_rule_events: String,
    pub private_view_hashes: String,
    pub preview_hashes: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicReplayExport {
    pub schema_version: u32,
    pub game_id: String,
    pub rules_version_label: String,
    pub variant: String,
    pub not_applicable: TraceNotApplicable,
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
pub struct ImportedPublicReplay {
    pub raw_json: String,
}

pub fn trace_not_applicable() -> TraceNotApplicable {
    TraceNotApplicable {
        hidden_information_redaction:
            "not applicable: Frontier Control is perfect information and all game state is public"
                .to_owned(),
        stochastic_game_rule_events:
            "not applicable: Frontier Control game rules use no randomness".to_owned(),
        private_view_hashes: "not applicable: Frontier Control has one public view for all viewers"
            .to_owned(),
        preview_hashes: "not applicable: Frontier Control has no Rust preview surface in this gate"
            .to_owned(),
    }
}

pub fn public_replay_step(
    step_index: usize,
    state_after_step: &FrontierControlState,
    command: &CommandEnvelope,
    effects: &[FrontierControlEffectEnvelope],
    viewer: &Viewer,
) -> PublicReplayStep {
    PublicReplayStep {
        step_index,
        public_view_summary: project_view(state_after_step, viewer).stable_summary(),
        public_effects: filter_effects_for_viewer(effects, viewer)
            .iter()
            .map(|effect| public_effect_text(&effect.payload))
            .collect(),
        command_summary: command_summary(command),
        terminal: state_after_step.terminal_outcome.is_some(),
    }
}

pub fn export_public_replay(
    variant: impl Into<String>,
    steps: Vec<PublicReplayStep>,
) -> PublicReplayExport {
    PublicReplayExport {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        variant: variant.into(),
        not_applicable: trace_not_applicable(),
        steps,
    }
}

pub fn import_public_export(export: &PublicReplayExport) -> ImportedPublicReplay {
    ImportedPublicReplay {
        raw_json: export.to_json(),
    }
}

pub fn import_public_export_json(input: &str) -> Result<ImportedPublicReplay, String> {
    reject_unknown_json_keys(
        input,
        &[
            "schema_version",
            "game_id",
            "rules_version_label",
            "variant",
            "not_applicable",
            "hidden_information_redaction",
            "stochastic_game_rule_events",
            "private_view_hashes",
            "preview_hashes",
            "steps",
            "step_index",
            "public_view_summary",
            "public_effects",
            "command_summary",
            "terminal",
        ],
    )?;
    Ok(ImportedPublicReplay {
        raw_json: input.to_owned(),
    })
}

impl PublicReplayExport {
    pub fn stable_summary(&self) -> String {
        format!(
            "schema={};game={};rules={};variant={};na={};steps={}",
            self.schema_version,
            self.game_id,
            self.rules_version_label,
            self.variant,
            self.not_applicable.stable_summary(),
            self.steps
                .iter()
                .map(PublicReplayStep::stable_summary)
                .collect::<Vec<_>>()
                .join("|")
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version_label\":\"{}\",\"variant\":\"{}\",\"not_applicable\":{},\"steps\":[{}]}}",
            self.schema_version,
            escape_json(&self.game_id),
            escape_json(&self.rules_version_label),
            escape_json(&self.variant),
            self.not_applicable.to_json(),
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
        self.stable_summary().into_bytes()
    }
}

impl TraceNotApplicable {
    pub fn stable_summary(&self) -> String {
        format!(
            "hidden={};stochastic={};private={};preview={}",
            self.hidden_information_redaction,
            self.stochastic_game_rule_events,
            self.private_view_hashes,
            self.preview_hashes
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"hidden_information_redaction\":\"{}\",\"stochastic_game_rule_events\":\"{}\",\"private_view_hashes\":\"{}\",\"preview_hashes\":\"{}\"}}",
            escape_json(&self.hidden_information_redaction),
            escape_json(&self.stochastic_game_rule_events),
            escape_json(&self.private_view_hashes),
            escape_json(&self.preview_hashes)
        )
    }
}

impl PublicReplayStep {
    pub fn stable_summary(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.step_index,
            self.public_view_summary,
            self.public_effects.join(","),
            self.command_summary,
            self.terminal
        )
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"step_index\":{},\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"command_summary\":\"{}\",\"terminal\":{}}}",
            self.step_index,
            escape_json(&self.public_view_summary),
            encode_string_array(&self.public_effects),
            escape_json(&self.command_summary),
            self.terminal
        )
    }
}

impl ImportedPublicReplay {
    pub fn stable_summary(&self) -> String {
        self.raw_json.clone()
    }
}

impl StableSerialize for ImportedPublicReplay {
    fn stable_bytes(&self) -> Vec<u8> {
        self.stable_summary().into_bytes()
    }
}

fn command_summary(command: &CommandEnvelope) -> String {
    format!(
        "{}:{}:{}",
        command.actor.seat_id.0,
        command.action_path.segments.join("/"),
        command.freshness_token.0
    )
}

fn reject_unknown_json_keys(input: &str, allowed: &[&str]) -> Result<(), String> {
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'"' {
            index += 1;
            continue;
        }
        let key_start = index + 1;
        let Some(key_end) = input[key_start..]
            .find('"')
            .map(|offset| key_start + offset)
        else {
            return Err("unterminated JSON string".to_owned());
        };
        let mut cursor = key_end + 1;
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor < bytes.len() && bytes[cursor] == b':' {
            let key = &input[key_start..key_end];
            if !allowed.contains(&key) {
                return Err(format!("unknown public replay field `{key}`"));
            }
        }
        index = key_end + 1;
    }
    Ok(())
}

fn encode_string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn escape_json(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![ch],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use engine_core::{ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId};

    use crate::{
        effects::{public_effect, FrontierControlEffect},
        setup::{setup_match, SetupOptions},
    };

    use super::*;

    fn state() -> FrontierControlState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn command() -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId("seat_0".to_owned()),
            },
            action_path: ActionPath {
                segments: vec!["end_turn".to_owned()],
            },
            freshness_token: FreshnessToken(0),
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn replay_export_import_is_lossless_and_marks_not_applicable_surfaces() {
        let state = state();
        let command = command();
        let effects = vec![public_effect(FrontierControlEffect::TurnEnded {
            faction: crate::ids::FactionId::Prospectors,
            round: 1,
        })];
        let step = public_replay_step(0, &state, &command, &effects, &Viewer { seat_id: None });
        let export = export_public_replay(state.variant.id.clone(), vec![step]);
        let imported = import_public_export(&export);

        assert_eq!(imported.raw_json, export.to_json());
        assert!(export
            .not_applicable
            .hidden_information_redaction
            .contains("perfect information"));
        assert!(import_public_export_json(&imported.raw_json).is_ok());
        assert!(import_public_export_json("{\"unknown\":true}").is_err());
    }
}
