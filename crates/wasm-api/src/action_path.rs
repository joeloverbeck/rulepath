//! Parsing of browser-supplied action-path strings.
//!
//! The UI submits a chosen action as a `>`-delimited, percent-encoded path
//! (e.g. `place>col%203`); this turns it into the engine `ActionPath` of
//! decoded segments. Glob-imported at the crate root.

use engine_core::ActionPath;

pub(crate) fn parse_action_path(action_path: &str) -> ActionPath {
    ActionPath {
        segments: if action_path.is_empty() {
            Vec::new()
        } else {
            action_path.split('>').map(percent_decode_segment).collect()
        },
    }
}

fn percent_decode_segment(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) =
                (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
            {
                decoded.push((high << 4) | low);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(decoded).unwrap_or_else(|_| segment.to_owned())
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
