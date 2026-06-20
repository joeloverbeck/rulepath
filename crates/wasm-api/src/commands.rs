//! Serialization helpers for replayed command logs.
//!
//! Shared across the per-game replay-document exporters: validate that a
//! command log is single-segment (`single_segment_commands`) and render one
//! command-record JSON object (`command_record_json`). Glob-imported at the
//! crate root.

use crate::json::{diagnostic_string, escape_json};
use crate::AppliedCommand;

pub(crate) fn single_segment_commands(commands: &[AppliedCommand]) -> Result<Vec<String>, String> {
    commands
        .iter()
        .map(|command| {
            if command.action_path.len() != 1 {
                return Err(diagnostic_string(
                    "unsupported_replay_action_path",
                    "this game exports one-segment action paths only",
                ));
            }
            Ok(command.action_path[0].clone())
        })
        .collect()
}

pub(crate) fn command_record_json(index: usize, command: &AppliedCommand) -> String {
    let action_path = command
        .action_path
        .iter()
        .map(|segment| format!("\"{}\"", escape_json(segment)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"index\":{},\"actor_seat\":\"{}\",\"action_path\":[{}],\"freshness_token\":\"{}\",\"expect\":\"applied\"}}",
        index,
        escape_json(&command.actor_seat),
        action_path,
        command.freshness_token
    )
}
