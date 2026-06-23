use engine_core::{ActionTree, ActionTreeEncodingVersion, HashValue, StableSerialize};

use crate::{visibility::PublicView, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID};

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

impl PublicReplayExport {
    pub fn new(viewer: impl Into<String>, steps: Vec<PublicReplayStep>) -> Self {
        Self {
            schema_version: 1,
            export_class: "viewer_scoped_observation".to_owned(),
            viewer: viewer.into(),
            game_id: GAME_ID.to_owned(),
            rules_version: RULES_VERSION_LABEL.to_owned(),
            variant: VARIANT_ID.to_owned(),
            steps,
        }
    }

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
        let schema_version = parse_u32_field(input, "schema_version")?;
        let export_class = parse_string_field(input, "export_class")?;
        let viewer = parse_string_field(input, "viewer")?;
        let game_id = parse_string_field(input, "game_id")?;
        let rules_version = parse_string_field(input, "rules_version")?;
        let variant = parse_string_field(input, "variant")?;
        let steps_raw = parse_array_field(input, "steps")?;
        let steps = if steps_raw.trim().is_empty() {
            Vec::new()
        } else {
            split_step_objects(&steps_raw)
                .into_iter()
                .map(PublicReplayStep::from_json)
                .collect::<Result<Vec<_>, _>>()?
        };

        Ok(Self {
            schema_version,
            export_class,
            viewer,
            game_id,
            rules_version,
            variant,
            steps,
        })
    }
}

impl StableSerialize for PublicReplayExport {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

pub fn action_tree_v1_bytes(tree: &ActionTree) -> Vec<u8> {
    tree.stable_bytes(ActionTreeEncodingVersion::V1)
}

pub fn action_tree_v1_hash(tree: &ActionTree) -> HashValue {
    tree.stable_hash(ActionTreeEncodingVersion::V1)
}

impl PublicReplayStep {
    pub fn from_view(
        step_index: usize,
        view: &PublicView,
        public_effects: Vec<String>,
        redacted_command_summary: impl Into<String>,
        terminal: bool,
    ) -> Self {
        Self {
            step_index,
            public_view_summary: view.stable_summary(),
            public_effects,
            redacted_command_summary: redacted_command_summary.into(),
            terminal,
        }
    }

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

    fn from_json(input: String) -> Result<Self, String> {
        let step_index = parse_usize_field(&input, "step_index")?;
        let public_view_summary = parse_string_field(&input, "public_view_summary")?;
        let redacted_command_summary = parse_string_field(&input, "redacted_command_summary")?;
        let terminal = parse_bool_field(&input, "terminal")?;
        let effects_raw = parse_array_field(&input, "public_effects")?;
        let public_effects = parse_string_array(&effects_raw)?;
        Ok(Self {
            step_index,
            public_view_summary,
            public_effects,
            redacted_command_summary,
            terminal,
        })
    }
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn unescape_json(value: &str) -> String {
    value.replace("\\\"", "\"").replace("\\\\", "\\")
}

fn parse_string_field(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":\"");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing string field `{key}`"))?
        + needle.len();
    let rest = &input[start..];
    let end = find_string_end(rest).ok_or_else(|| format!("unterminated field `{key}`"))?;
    Ok(unescape_json(&rest[..end]))
}

fn parse_u32_field(input: &str, key: &str) -> Result<u32, String> {
    parse_number_field(input, key)?
        .parse()
        .map_err(|_| format!("field `{key}` must be u32"))
}

fn parse_usize_field(input: &str, key: &str) -> Result<usize, String> {
    parse_number_field(input, key)?
        .parse()
        .map_err(|_| format!("field `{key}` must be usize"))
}

fn parse_number_field<'a>(input: &'a str, key: &str) -> Result<&'a str, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing number field `{key}`"))?
        + needle.len();
    let rest = &input[start..];
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    Ok(&rest[..end])
}

fn parse_bool_field(input: &str, key: &str) -> Result<bool, String> {
    let needle = format!("\"{key}\":");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing bool field `{key}`"))?
        + needle.len();
    if input[start..].starts_with("true") {
        Ok(true)
    } else if input[start..].starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field `{key}` must be bool"))
    }
}

fn parse_array_field(input: &str, key: &str) -> Result<String, String> {
    let needle = format!("\"{key}\":[");
    let start = input
        .find(&needle)
        .ok_or_else(|| format!("missing array field `{key}`"))?
        + needle.len();
    let rest = &input[start..];
    let end = find_matching_array_end(rest).ok_or_else(|| format!("unterminated array `{key}`"))?;
    Ok(rest[..end].to_owned())
}

fn find_string_end(input: &str) -> Option<usize> {
    let mut escaped = false;
    for (index, ch) in input.char_indices() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Some(index);
        }
    }
    None
}

fn find_matching_array_end(input: &str) -> Option<usize> {
    let mut string = false;
    let mut escaped = false;
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        if string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                string = false;
            }
            continue;
        }
        match ch {
            '"' => string = true,
            '[' | '{' => depth += 1,
            ']' if depth == 0 => return Some(index),
            ']' | '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

fn split_step_objects(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = None;
    let mut string = false;
    let mut escaped = false;
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        if string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                string = false;
            }
            continue;
        }
        match ch {
            '"' => string = true,
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    parts.push(input[start.expect("object start")..=index].to_owned());
                }
            }
            _ => {}
        }
    }
    parts
}

fn parse_string_array(input: &str) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    let mut index = 0usize;

    while index < input.len() {
        while input[index..].starts_with(char::is_whitespace) {
            index += input[index..]
                .chars()
                .next()
                .expect("whitespace char")
                .len_utf8();
        }
        if index == input.len() {
            break;
        }

        if !input[index..].starts_with('"') {
            return Err("expected string array item".to_owned());
        }
        index += 1;
        let value_start = index;
        let value_end = find_string_end(&input[index..])
            .ok_or_else(|| "unterminated string array item".to_owned())?
            + index;
        values.push(unescape_json(&input[value_start..value_end]));
        index = value_end + 1;

        while index < input.len() && input[index..].starts_with(char::is_whitespace) {
            index += input[index..]
                .chars()
                .next()
                .expect("whitespace char")
                .len_utf8();
        }
        if index == input.len() {
            break;
        }
        if !input[index..].starts_with(',') {
            return Err("expected comma between string array items".to_owned());
        }
        index += 1;
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::{validate_command, ValidatedAction},
        apply_action, project_view,
        setup::{setup_match, SetupOptions},
    };
    use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};

    fn command(
        state: &crate::MaskedClaimsState,
        seat: &str,
        segments: Vec<&str>,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId(seat.to_owned()),
            },
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn public_export_redacts_claim_command_and_round_trips() {
        let mut state = setup_match(
            Seed(7),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let tile = state.hands[0][0];
        let ValidatedAction::Claim(claim) = validate_command(
            &state,
            &command(&state, "seat_0", vec!["claim", tile.as_str(), "5"]),
        )
        .expect("claim validates") else {
            panic!("expected claim");
        };
        let effects =
            apply_action(&mut state, ValidatedAction::Claim(claim)).expect("claim applies");
        let view = project_view(&state, &Viewer { seat_id: None });
        let step = PublicReplayStep::from_view(
            0,
            &view,
            effects
                .iter()
                .map(|effect| format!("{:?}", effect.payload))
                .collect(),
            "claim/grade-5",
            false,
        );
        let export = PublicReplayExport::new("observer", vec![step]);
        let json = export.to_json();

        assert!(!json.contains(tile.as_str()));
        assert!(!json.contains(&tile.label()));
        assert!(json.contains("claim/grade-5"));
        assert_eq!(PublicReplayExport::from_json(&json).unwrap(), export);
    }
}
