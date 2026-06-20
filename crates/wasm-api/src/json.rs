//! Generic JSON output primitives shared across the bridge: string escaping,
//! diagnostic/error object rendering, and small array helpers.
//!
//! The bridge hand-rolls its JSON output, so these leaf helpers are the
//! foundation every per-game formatter builds on. They are glob-imported
//! (`use crate::json::*;`) at the crate root.

use engine_core::Diagnostic;

pub(crate) fn diagnostic_json(diagnostic: Diagnostic) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(&diagnostic.code),
        escape_json(&diagnostic.message)
    )
}

pub(crate) fn diagnostic_string(code: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{}\",\"message\":\"{}\"}}",
        escape_json(code),
        escape_json(message)
    )
}

pub(crate) fn unsupported_variant_json(game_id: &str, message: &str) -> String {
    diagnostic_string("unsupported_variant", &format!("{game_id}: {message}"))
}

pub(crate) fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(crate) fn string_array_json(values: &[&str]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}
