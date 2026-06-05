use engine_core::{FreshnessToken, StableSerialize};

use crate::{
    actions::legal_additions,
    ids::RaceSeat,
    state::{option_seat_json, CounterValue, RaceState, StrictJsonObject},
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
    pub freshness_token: FreshnessToken,
    pub legal_additions: Vec<u8>,
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
            "{{\"schema_version\":{},\"rules_version\":{},\"counter\":{},\"target\":{},\"max_add\":{},\"active_seat\":\"{}\",\"winner\":{},\"freshness_token\":{},\"legal_additions\":[{}]}}",
            self.schema_version,
            self.rules_version,
            self.counter.0,
            self.target,
            self.max_add,
            self.active_seat.as_str(),
            option_seat_json(self.winner),
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

#[cfg(test)]
mod tests {
    use engine_core::{SeatId, Seed};

    use super::*;
    use crate::{setup_match, SetupOptions};

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
}
