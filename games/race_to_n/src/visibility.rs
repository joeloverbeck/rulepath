use engine_core::{FreshnessToken, StableSerialize};

use crate::{
    actions::legal_additions,
    ids::RaceSeat,
    state::{option_seat_json, option_u8_json, CounterValue, RaceState, StrictJsonObject},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub schema_version: u32,
    pub rules_version: u32,
    pub counter: CounterValue,
    pub target: u8,
    pub max_add: u8,
    pub active_seat: RaceSeat,
    pub winner: Option<RaceSeat>,
    pub outcome_rationale: Option<OutcomeRationaleView>,
    pub freshness_token: FreshnessToken,
    pub legal_additions: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeRationaleView {
    pub result_kind: String,
    pub decisive_cause: String,
    pub template_key: String,
    pub decisive_rule_ids: Vec<String>,
    pub counter_before: u8,
    pub addition: u8,
    pub counter_after: u8,
    pub target: u8,
    pub max_add: u8,
}

pub fn project_view(state: &RaceState) -> PublicView {
    PublicView {
        schema_version: 1,
        rules_version: 1,
        counter: state.counter,
        target: state.variant.target,
        max_add: state.variant.max_add,
        active_seat: state.active_seat,
        winner: state.winner,
        outcome_rationale: state
            .winner
            .and_then(|winning_seat| terminal_rationale(state, winning_seat)),
        freshness_token: state.freshness_token,
        legal_additions: legal_additions(state),
    }
}

impl PublicView {
    pub fn to_json(&self) -> String {
        let legal_additions = self
            .legal_additions
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":{},\"rules_version\":{},\"counter\":{},\"target\":{},\"max_add\":{},\"active_seat\":\"{}\",\"winner\":{},\"outcome_result_kind\":{},\"outcome_decisive_cause\":{},\"outcome_template_key\":{},\"outcome_decisive_rule_ids\":[{}],\"outcome_counter_before\":{},\"outcome_addition\":{},\"outcome_counter_after\":{},\"outcome_target\":{},\"outcome_max_add\":{},\"freshness_token\":{},\"legal_additions\":[{}]}}",
            self.schema_version,
            self.rules_version,
            self.counter.0,
            self.target,
            self.max_add,
            self.active_seat.as_str(),
            option_seat_json(self.winner),
            optional_string_json(self.outcome_rationale.as_ref().map(|rationale| rationale.result_kind.as_str())),
            optional_string_json(self.outcome_rationale.as_ref().map(|rationale| rationale.decisive_cause.as_str())),
            optional_string_json(self.outcome_rationale.as_ref().map(|rationale| rationale.template_key.as_str())),
            string_array(
                &self
                    .outcome_rationale
                    .as_ref()
                    .map(|rationale| rationale.decisive_rule_ids.clone())
                    .unwrap_or_default()
            ),
            option_u8_json(self.outcome_rationale.as_ref().map(|rationale| rationale.counter_before)),
            option_u8_json(self.outcome_rationale.as_ref().map(|rationale| rationale.addition)),
            option_u8_json(self.outcome_rationale.as_ref().map(|rationale| rationale.counter_after)),
            option_u8_json(self.outcome_rationale.as_ref().map(|rationale| rationale.target)),
            option_u8_json(self.outcome_rationale.as_ref().map(|rationale| rationale.max_add)),
            self.freshness_token.0,
            legal_additions
        )
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let object = StrictJsonObject::parse(input)?;
        // deny_unknown_fields equivalent: all browser-facing hand-authored test
        // inputs are parsed through this explicit allowlist.
        object.reject_unknown(&[
            "schema_version",
            "rules_version",
            "counter",
            "target",
            "max_add",
            "active_seat",
            "winner",
            "outcome_result_kind",
            "outcome_decisive_cause",
            "outcome_template_key",
            "outcome_decisive_rule_ids",
            "outcome_counter_before",
            "outcome_addition",
            "outcome_counter_after",
            "outcome_target",
            "outcome_max_add",
            "freshness_token",
            "legal_additions",
        ])?;

        Ok(Self {
            schema_version: object.required_u32("schema_version")?,
            rules_version: object.required_u32("rules_version")?,
            counter: CounterValue(object.required_u8("counter")?),
            target: object.required_u8("target")?,
            max_add: object.required_u8("max_add")?,
            active_seat: object.required_seat("active_seat")?,
            winner: object.optional_seat("winner")?,
            outcome_rationale: parse_outcome_rationale(&object)?,
            freshness_token: FreshnessToken(object.required_u64("freshness_token")?),
            legal_additions: parse_u8_array(&object.required_raw("legal_additions")?)?,
        })
    }
}

impl StableSerialize for PublicView {
    fn stable_bytes(&self) -> Vec<u8> {
        self.to_json().into_bytes()
    }
}

fn terminal_rationale(state: &RaceState, _winning_seat: RaceSeat) -> Option<OutcomeRationaleView> {
    let advance = state.terminal_advance?;
    Some(OutcomeRationaleView {
        result_kind: "win".to_owned(),
        decisive_cause: "exact_target_reached".to_owned(),
        template_key: "race_to_n.exact_target_reached".to_owned(),
        decisive_rule_ids: vec!["R-SCORE-001".to_owned(), "R-END-001".to_owned()],
        counter_before: advance.counter_before.0,
        addition: advance.addition,
        counter_after: advance.counter_after.0,
        target: state.variant.target,
        max_add: state.variant.max_add,
    })
}

fn parse_outcome_rationale(
    object: &StrictJsonObject,
) -> Result<Option<OutcomeRationaleView>, String> {
    let result_kind = object.optional_string("outcome_result_kind")?;
    let decisive_cause = object.optional_string("outcome_decisive_cause")?;
    let template_key = object.optional_string("outcome_template_key")?;
    let decisive_rule_ids = parse_string_array(&object.required_raw("outcome_decisive_rule_ids")?)?;
    let counter_before = object.optional_u8("outcome_counter_before")?;
    let addition = object.optional_u8("outcome_addition")?;
    let counter_after = object.optional_u8("outcome_counter_after")?;
    let target = object.optional_u8("outcome_target")?;
    let max_add = object.optional_u8("outcome_max_add")?;

    match (
        result_kind,
        decisive_cause,
        template_key,
        counter_before,
        addition,
        counter_after,
        target,
        max_add,
    ) {
        (None, None, None, None, None, None, None, None) if decisive_rule_ids.is_empty() => {
            Ok(None)
        }
        (
            Some(result_kind),
            Some(decisive_cause),
            Some(template_key),
            Some(counter_before),
            Some(addition),
            Some(counter_after),
            Some(target),
            Some(max_add),
        ) => Ok(Some(OutcomeRationaleView {
            result_kind,
            decisive_cause,
            template_key,
            decisive_rule_ids,
            counter_before,
            addition,
            counter_after,
            target,
            max_add,
        })),
        _ => Err("outcome rationale fields must be all null or all present".to_owned()),
    }
}

fn parse_u8_array(raw: &str) -> Result<Vec<u8>, String> {
    let body = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| "expected number array".to_owned())?;
    if body.trim().is_empty() {
        return Ok(Vec::new());
    }
    body.split(',')
        .map(|value| {
            value
                .trim()
                .parse()
                .map_err(|_| "array entry must fit u8".to_owned())
        })
        .collect()
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

fn string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn optional_string_json(value: Option<&str>) -> String {
    value.map_or_else(
        || "null".to_owned(),
        |value| format!("\"{}\"", escape_json(value)),
    )
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

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Seed};

    use super::*;
    use crate::{rules::apply_action, setup_match, SetupOptions, ValidatedAction};

    #[test]
    fn project_view_yields_public_view_type_and_expected_fields() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        let view: PublicView = project_view(&state);

        assert_eq!(view.counter, CounterValue(0));
        assert_eq!(view.active_seat, RaceSeat::Seat0);
        assert_eq!(view.legal_additions, vec![1, 2, 3]);
        assert!(view.to_json().contains("\"schema_version\":1"));
        assert!(!view.to_json().contains("seat-0"));
    }

    #[test]
    fn terminal_view_names_exact_target_rationale() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let mut state = setup_match(Seed(1), &seats, &SetupOptions::default()).unwrap();
        state.counter = CounterValue(18);
        apply_action(
            &mut state,
            ValidatedAction {
                actor: RaceSeat::Seat0,
                amount: 3,
            },
        );

        let view = project_view(&state);
        assert_eq!(view.winner, Some(RaceSeat::Seat0));
        assert_eq!(
            view.outcome_rationale,
            Some(OutcomeRationaleView {
                result_kind: "win".to_owned(),
                decisive_cause: "exact_target_reached".to_owned(),
                template_key: "race_to_n.exact_target_reached".to_owned(),
                decisive_rule_ids: vec!["R-SCORE-001".to_owned(), "R-END-001".to_owned()],
                counter_before: 18,
                addition: 3,
                counter_after: 21,
                target: 21,
                max_add: 3,
            })
        );
        let parsed = PublicView::from_json(&view.to_json()).expect("terminal view round trips");
        assert_eq!(parsed, view);
    }
}
