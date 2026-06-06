use engine_core::{Actor, FreshnessToken, StableSerialize, Viewer};

use crate::{
    actions::legal_action_tree,
    ids::{CellId, ThreeMarksSeat, GAME_ID, RULES_VERSION_LABEL, VARIANT_ID},
    state::{CellOccupancy, TerminalOutcome, ThreeMarksState},
    ui::{cell_layout, mark_token},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub game_id: String,
    pub display_name: String,
    pub variant_id: String,
    pub rules_version_label: String,
    pub board_rows: u8,
    pub board_columns: u8,
    pub cells: Vec<CellView>,
    pub active_seat: ThreeMarksSeat,
    pub ply_count: u8,
    pub status_label: String,
    pub freshness_token: FreshnessToken,
    pub legal_targets: Vec<LegalTargetView>,
    pub terminal: TerminalView,
    pub private_view: PrivateView,
    pub replay_step_index: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CellView {
    pub cell: CellId,
    pub row: u8,
    pub column: u8,
    pub occupancy: String,
    pub owner: Option<ThreeMarksSeat>,
    pub mark_token_key: Option<String>,
    pub mark_shape_label: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalTargetView {
    pub cell: CellId,
    pub action_segment: String,
    pub label: String,
    pub accessibility_label: String,
    pub freshness_token: FreshnessToken,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalView {
    NonTerminal,
    Win {
        winning_seat: ThreeMarksSeat,
        line: [CellId; 3],
    },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateView {
    pub status: String,
    pub hidden_fields: Vec<String>,
}

pub fn project_view(state: &ThreeMarksState, _viewer: &Viewer) -> PublicView {
    let terminal = match state.terminal_outcome {
        None => TerminalView::NonTerminal,
        Some(TerminalOutcome::Draw) => TerminalView::Draw,
        Some(TerminalOutcome::Win { seat, line }) => TerminalView::Win {
            winning_seat: seat,
            line: line.cells,
        },
    };
    let legal_targets = if state.terminal_outcome.is_none() {
        let actor = Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        };
        legal_action_tree(state, &actor)
            .root
            .choices
            .into_iter()
            .filter_map(|choice| {
                let cell = choice
                    .metadata
                    .iter()
                    .find(|entry| entry.key == "cell")
                    .and_then(|entry| CellId::parse(&entry.value))?;
                Some(LegalTargetView {
                    cell,
                    action_segment: choice.segment,
                    label: choice.label,
                    accessibility_label: choice.accessibility_label,
                    freshness_token: state.freshness_token,
                })
            })
            .collect()
    } else {
        Vec::new()
    };

    PublicView {
        schema_version: 1,
        rules_version: 1,
        game_id: GAME_ID.to_owned(),
        display_name: "Three Marks".to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        rules_version_label: RULES_VERSION_LABEL.to_owned(),
        board_rows: 3,
        board_columns: 3,
        cells: CellId::ALL
            .into_iter()
            .map(|cell| cell_view(state, cell))
            .collect(),
        active_seat: state.active_seat,
        ply_count: state.ply_count,
        status_label: status_label(&terminal, state.active_seat),
        freshness_token: state.freshness_token,
        legal_targets,
        terminal,
        private_view: PrivateView {
            status: "not_applicable_perfect_information".to_owned(),
            hidden_fields: Vec::new(),
        },
        replay_step_index: None,
    }
}

impl PublicView {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"board_rows\":{},\"board_columns\":{},\"cells\":[{}],\"active_seat\":\"{}\",\"ply_count\":{},\"status_label\":\"{}\",\"freshness_token\":{},\"legal_targets\":[{}],\"terminal_kind\":\"{}\",\"winning_seat\":{},\"winning_line\":[{}],\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"replay_step_index\":{}}}",
            self.schema_version,
            self.rules_version,
            escape_json(&self.game_id),
            escape_json(&self.display_name),
            escape_json(&self.variant_id),
            escape_json(&self.rules_version_label),
            self.board_rows,
            self.board_columns,
            string_array(&self.cells.iter().map(encode_cell).collect::<Vec<_>>()),
            self.active_seat.as_str(),
            self.ply_count,
            escape_json(&self.status_label),
            self.freshness_token.0,
            string_array(
                &self
                    .legal_targets
                    .iter()
                    .map(encode_legal_target)
                    .collect::<Vec<_>>()
            ),
            terminal_kind(&self.terminal),
            option_seat_json(terminal_winner(&self.terminal)),
            string_array(&terminal_line(&self.terminal)),
            escape_json(&self.private_view.status),
            string_array(&self.private_view.hidden_fields),
            self.replay_step_index
                .map_or_else(|| "null".to_owned(), |step| step.to_string())
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        object.reject_unknown(&[
            "schema_version",
            "rules_version",
            "game_id",
            "display_name",
            "variant_id",
            "rules_version_label",
            "board_rows",
            "board_columns",
            "cells",
            "active_seat",
            "ply_count",
            "status_label",
            "freshness_token",
            "legal_targets",
            "terminal_kind",
            "winning_seat",
            "winning_line",
            "private_view_status",
            "hidden_fields",
            "replay_step_index",
        ])?;

        let winning_seat = object.optional_seat("winning_seat")?;
        let winning_line = parse_cell_array(&object.required_raw("winning_line")?)?;
        let terminal = match object.required_string("terminal_kind")?.as_str() {
            "non_terminal" => TerminalView::NonTerminal,
            "draw" => TerminalView::Draw,
            "win" => TerminalView::Win {
                winning_seat: winning_seat.ok_or_else(|| "win requires winning_seat".to_owned())?,
                line: winning_line
                    .try_into()
                    .map_err(|_| "win requires exactly three line cells".to_owned())?,
            },
            other => return Err(format!("unknown terminal kind `{other}`")),
        };

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            rules_version: object.required_u32("rules_version")?,
            game_id: object.required_string("game_id")?,
            display_name: object.required_string("display_name")?,
            variant_id: object.required_string("variant_id")?,
            rules_version_label: object.required_string("rules_version_label")?,
            board_rows: object.required_u8("board_rows")?,
            board_columns: object.required_u8("board_columns")?,
            cells: parse_cells(&object.required_raw("cells")?)?,
            active_seat: object.required_seat("active_seat")?,
            ply_count: object.required_u8("ply_count")?,
            status_label: object.required_string("status_label")?,
            freshness_token: FreshnessToken(object.required_u64("freshness_token")?),
            legal_targets: parse_legal_targets(&object.required_raw("legal_targets")?)?,
            terminal,
            private_view: PrivateView {
                status: object.required_string("private_view_status")?,
                hidden_fields: parse_string_array(&object.required_raw("hidden_fields")?)?,
            },
            replay_step_index: object.optional_u32("replay_step_index")?,
        })
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

fn cell_view(state: &ThreeMarksState, cell: CellId) -> CellView {
    let layout = cell_layout(cell);
    match state.occupancy(cell) {
        CellOccupancy::Empty => CellView {
            cell,
            row: layout.row,
            column: layout.column,
            occupancy: "empty".to_owned(),
            owner: None,
            mark_token_key: None,
            mark_shape_label: None,
        },
        CellOccupancy::Occupied(owner) => {
            let token = mark_token(owner);
            CellView {
                cell,
                row: layout.row,
                column: layout.column,
                occupancy: "occupied".to_owned(),
                owner: Some(owner),
                mark_token_key: Some(token.token_key.to_owned()),
                mark_shape_label: Some(token.shape_label.to_owned()),
            }
        }
    }
}

fn status_label(terminal: &TerminalView, active_seat: ThreeMarksSeat) -> String {
    match terminal {
        TerminalView::NonTerminal => format!("{} to place", active_seat.as_str()),
        TerminalView::Win { winning_seat, .. } => format!("{} wins", winning_seat.as_str()),
        TerminalView::Draw => "draw".to_owned(),
    }
}

fn terminal_kind(terminal: &TerminalView) -> &'static str {
    match terminal {
        TerminalView::NonTerminal => "non_terminal",
        TerminalView::Win { .. } => "win",
        TerminalView::Draw => "draw",
    }
}

fn terminal_winner(terminal: &TerminalView) -> Option<ThreeMarksSeat> {
    match terminal {
        TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
        _ => None,
    }
}

fn terminal_line(terminal: &TerminalView) -> Vec<String> {
    match terminal {
        TerminalView::Win { line, .. } => {
            line.iter().map(|cell| cell.as_str().to_owned()).collect()
        }
        _ => Vec::new(),
    }
}

fn encode_cell(cell: &CellView) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        cell.cell.as_str(),
        cell.row,
        cell.column,
        cell.occupancy,
        cell.owner.map_or("none", ThreeMarksSeat::as_str),
        cell.mark_token_key.as_deref().unwrap_or("none"),
        cell.mark_shape_label.as_deref().unwrap_or("none")
    )
}

fn encode_legal_target(target: &LegalTargetView) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        target.cell.as_str(),
        target.action_segment,
        target.label,
        target.accessibility_label,
        target.freshness_token.0
    )
}

fn parse_cells(raw: &str) -> Result<Vec<CellView>, String> {
    parse_string_array(raw)?
        .into_iter()
        .map(|entry| {
            let parts = split_encoded(&entry, 7)?;
            Ok(CellView {
                cell: CellId::parse(&parts[0]).ok_or_else(|| "invalid cell".to_owned())?,
                row: parse_u8(&parts[1], "row")?,
                column: parse_u8(&parts[2], "column")?,
                occupancy: parts[3].clone(),
                owner: parse_optional_seat(&parts[4])?,
                mark_token_key: optional_string(&parts[5]),
                mark_shape_label: optional_string(&parts[6]),
            })
        })
        .collect()
}

fn parse_legal_targets(raw: &str) -> Result<Vec<LegalTargetView>, String> {
    parse_string_array(raw)?
        .into_iter()
        .map(|entry| {
            let parts = split_encoded(&entry, 5)?;
            Ok(LegalTargetView {
                cell: CellId::parse(&parts[0]).ok_or_else(|| "invalid target cell".to_owned())?,
                action_segment: parts[1].clone(),
                label: parts[2].clone(),
                accessibility_label: parts[3].clone(),
                freshness_token: FreshnessToken(
                    parts[4]
                        .parse()
                        .map_err(|_| "freshness token must be u64".to_owned())?,
                ),
            })
        })
        .collect()
}

fn parse_cell_array(raw: &str) -> Result<Vec<CellId>, String> {
    parse_string_array(raw)?
        .into_iter()
        .map(|entry| CellId::parse(&entry).ok_or_else(|| "invalid line cell".to_owned()))
        .collect()
}

fn string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    let body = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| "expected string array".to_owned())?;
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    body.split(',')
        .map(|value| parse_json_string(value.trim()))
        .collect()
}

fn split_encoded(entry: &str, expected: usize) -> Result<Vec<String>, String> {
    let parts = entry.split('|').map(str::to_owned).collect::<Vec<_>>();
    if parts.len() != expected {
        return Err(format!("expected {expected} encoded fields"));
    }
    Ok(parts)
}

fn optional_string(value: &str) -> Option<String> {
    if value == "none" {
        None
    } else {
        Some(value.to_owned())
    }
}

fn parse_optional_seat(value: &str) -> Result<Option<ThreeMarksSeat>, String> {
    if value == "none" {
        Ok(None)
    } else {
        ThreeMarksSeat::parse(value)
            .map(Some)
            .ok_or_else(|| "invalid optional seat".to_owned())
    }
}

fn parse_u8(value: &str, label: &str) -> Result<u8, String> {
    value.parse().map_err(|_| format!("{label} must fit u8"))
}

fn option_seat_json(seat: Option<ThreeMarksSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
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

    fn required_u8(&self, key: &str) -> Result<u8, String> {
        self.required_raw(key)?
            .parse()
            .map_err(|_| format!("field `{key}` must fit u8"))
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

    fn required_seat(&self, key: &str) -> Result<ThreeMarksSeat, String> {
        ThreeMarksSeat::parse(&self.required_string(key)?)
            .ok_or_else(|| format!("field `{key}` must be a seat"))
    }

    fn optional_seat(&self, key: &str) -> Result<Option<ThreeMarksSeat>, String> {
        let raw = self.required_raw(key)?;
        if raw == "null" {
            return Ok(None);
        }
        ThreeMarksSeat::parse(&parse_json_string(&raw)?)
            .map(Some)
            .ok_or_else(|| format!("field `{key}` must be a seat or null"))
    }

    fn optional_u32(&self, key: &str) -> Result<Option<u32>, String> {
        let raw = self.required_raw(key)?;
        if raw == "null" {
            return Ok(None);
        }
        raw.parse()
            .map(Some)
            .map_err(|_| format!("field `{key}` must be u32 or null"))
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
    Err("expected key/value".to_owned())
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
        previous_escape = ch == '\\' && !previous_escape;
        if ch != '\\' {
            previous_escape = false;
        }
    }
    if depth != 0 || in_string {
        return Err("unterminated JSON value".to_owned());
    }
    result.push(input[start..].to_owned());
    Ok(result)
}

fn parse_json_string(input: &str) -> Result<String, String> {
    let body = input
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "expected JSON string".to_owned())?;
    let mut output = String::new();
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let escaped = chars.next().ok_or_else(|| "dangling escape".to_owned())?;
            output.push(escaped);
        } else {
            output.push(ch);
        }
    }
    Ok(output)
}
