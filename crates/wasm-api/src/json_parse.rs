//! Generic JSON parsing primitives for replay-document import.
//!
//! The bridge accepts hand-authored / exported replay JSON, so it needs to read
//! it back without pulling in a full parser dependency. These helpers walk a
//! `&str` directly: field lookup (`string_field`, `number_field`, `bool_field`,
//! `string_array_field`, `array_items`), structural validation
//! (`validate_json_object`, `reject_unknown_root_fields`), and the low-level
//! string/number/bracket scanners they build on. They are the parsing
//! counterpart to the output helpers in [`crate::json`] and are glob-imported at
//! the crate root.

pub(crate) fn validate_json_object(input: &str) -> Result<(), String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("malformed JSON object".to_owned());
    }
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;
    for ch in trimmed.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            return Err("malformed JSON nesting".to_owned());
        }
    }
    if depth != 0 || in_string {
        return Err("malformed JSON nesting".to_owned());
    }
    Ok(())
}

pub(crate) fn reject_unknown_root_fields(input: &str, allowed: &[&str]) -> Result<(), String> {
    for key in top_level_keys(input)? {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("unknown field `{key}`"));
        }
    }
    Ok(())
}

fn top_level_keys(input: &str) -> Result<Vec<String>, String> {
    let body = input
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed JSON object".to_owned())?;
    let mut keys = Vec::new();
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        let skipped = body[index..].len() - rest.len();
        index += skipped;
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err("malformed JSON field".to_owned());
        }
        index += body[index..].len() - after_key.len() + 1;
        index = skip_json_value(body, index)?;
        keys.push(key);
    }
    Ok(keys)
}

fn skip_json_value(input: &str, mut index: usize) -> Result<usize, String> {
    while input[index..].starts_with(char::is_whitespace) {
        index += input[index..].chars().next().unwrap().len_utf8();
    }
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0_i32;
    for (offset, ch) in input[index..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => {
                if depth == 0 {
                    return Ok(index + offset);
                }
                depth -= 1;
            }
            ',' if depth == 0 => return Ok(index + offset + 1),
            _ => {}
        }
    }
    Ok(input.len())
}

pub(crate) fn string_field(input: &str, key: &str) -> Result<String, String> {
    let start = field_value_start(input, key)?;
    parse_string_at(input, start).ok_or_else(|| format!("field `{key}` must be a string"))
}

pub(crate) fn number_field(input: &str, key: &str) -> Result<u64, String> {
    let start = field_value_start(input, key)?;
    parse_number_at(input, start).ok_or_else(|| format!("field `{key}` must be a number"))
}

pub(crate) fn bool_field(input: &str, key: &str) -> Result<bool, String> {
    let start = field_value_start(input, key)?;
    let tail = input[start..].trim_start();
    if tail.starts_with("true") {
        Ok(true)
    } else if tail.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field `{key}` must be a boolean"))
    }
}

pub(crate) fn string_array_field(input: &str, key: &str) -> Result<Vec<String>, String> {
    array_items(input, key)?
        .into_iter()
        .map(|item| parse_json_string(item.trim()))
        .collect()
}

pub(crate) fn array_items(input: &str, key: &str) -> Result<Vec<String>, String> {
    let start = field_value_start(input, key)?;
    let open = input[start..]
        .find('[')
        .ok_or_else(|| format!("field `{key}` must be an array"))?
        + start;
    let close = matching_bracket(input, open, '[', ']')?;
    let body = &input[open + 1..close];
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    split_top_level(body, ',')
}

fn field_value_start(input: &str, key: &str) -> Result<usize, String> {
    let trimmed = input.trim();
    let body = trimmed
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .ok_or_else(|| "malformed JSON object".to_owned())?;
    let body_start = input
        .find('{')
        .ok_or_else(|| "malformed JSON object".to_owned())?
        + 1;
    let mut index = 0;
    while index < body.len() {
        let rest = body[index..].trim_start();
        if rest.is_empty() {
            break;
        }
        index += body[index..].len() - rest.len();
        if body[index..].starts_with(',') {
            index += 1;
            continue;
        }
        let (field_key, next) = parse_json_string_at(body, index)?;
        index = next;
        let after_key = body[index..].trim_start();
        if !after_key.starts_with(':') {
            return Err(format!("malformed field `{field_key}`"));
        }
        let value_start = index + body[index..].len() - after_key.len() + 1;
        if field_key == key {
            return Ok(body_start + value_start);
        }
        index = skip_json_value(body, value_start)?;
    }
    Err(format!("missing `{key}`"))
}

fn matching_bracket(
    input: &str,
    open: usize,
    open_ch: char,
    close_ch: char,
) -> Result<usize, String> {
    let mut depth = 0_u32;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in input[open..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        if ch == '"' {
            in_string = true;
        } else if ch == open_ch {
            depth += 1;
        } else if ch == close_ch {
            depth = depth
                .checked_sub(1)
                .ok_or_else(|| "unbalanced JSON".to_owned())?;
            if depth == 0 {
                return Ok(open + offset);
            }
        }
    }
    Err("unbalanced JSON".to_owned())
}

fn split_top_level(input: &str, delimiter: char) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut nested = 0usize;
    for (index, ch) in input.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' | '[' if !in_string => nested += 1,
            '}' | ']' if !in_string => {
                nested = nested
                    .checked_sub(1)
                    .ok_or_else(|| "unbalanced JSON".to_owned())?;
            }
            ch if ch == delimiter && !in_string && nested == 0 => {
                parts.push(input[start..index].to_owned());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    if in_string || nested != 0 {
        return Err("unbalanced JSON".to_owned());
    }
    parts.push(input[start..].to_owned());
    Ok(parts)
}

fn parse_string_at(input: &str, start: usize) -> Option<String> {
    let tail = input[start..].trim_start();
    parse_json_string_prefix(tail).map(|(value, _)| value).ok()
}

fn parse_json_string_at(input: &str, start: usize) -> Result<(String, usize), String> {
    let (value, consumed) = parse_json_string_prefix(&input[start..])?;
    Ok((value, start + consumed))
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .trim()
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for ch in body.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else {
            output.push(ch);
        }
    }
    if escaped {
        return Err("unterminated escape".to_owned());
    }
    Ok(output)
}

fn parse_json_string_prefix(input: &str) -> Result<(String, usize), String> {
    let tail = input
        .strip_prefix('"')
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut escaped = false;
    for (index, ch) in tail.char_indices() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Ok((output, index + 2));
        } else {
            output.push(ch);
        }
    }
    Err("unterminated JSON string".to_owned())
}

fn parse_number_at(input: &str, start: usize) -> Option<u64> {
    let tail = input[start..].trim_start();
    let digits = tail
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}
