//! UI metadata and presentation helpers for Flood Watch.

use std::collections::BTreeSet;

use crate::{
    ids::{DistrictId, EventKind},
    variants::{parse_flat_toml, parse_string_list, reject_unknown_keys, required_string},
};

pub const SEAT_LABEL_AUDIT: &str =
    "Flood Watch is cooperative and role-first; role labels remain authored in the Rust view.";
pub const TURN_REPORT_ADOPTION: &str =
    "Adopted by ACTCONMAT-006; Flood Watch reports viewer-filtered automation and storm bursts near the board.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub display_name: String,
    pub event_deck_label: String,
    pub forecast_label: String,
    pub drawn_label: String,
    pub face_down_label: String,
    pub face_down_summary: String,
    pub reduced_motion_token: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        display_name: "Flood Watch".to_owned(),
        event_deck_label: "Storm deck".to_owned(),
        forecast_label: "Forecast".to_owned(),
        drawn_label: "Resolved storm cards".to_owned(),
        face_down_label: "Storm deck".to_owned(),
        face_down_summary: "Remaining storm cards stay face down. The count is public.".to_owned(),
        reduced_motion_token: "flood-watch-reduced-motion".to_owned(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardFaceView {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub details: Option<String>,
    pub family: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPresentation {
    pub kind: EventKind,
    pub label: String,
    pub summary: String,
    pub details: Option<String>,
    pub family: String,
    pub accessibility_label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardPresentationCatalog {
    pub cards: Vec<CardPresentation>,
}

impl CardPresentationCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(
            &values,
            &[
                "event_ids",
                "labels",
                "summaries",
                "details",
                "families",
                "accessibility_labels",
            ],
        )?;

        let ids = parse_event_kind_list(&required_string(&values, "event_ids")?)?;
        let labels = parse_non_empty_string_list(&required_string(&values, "labels")?, "labels")?;
        let summaries =
            parse_non_empty_string_list(&required_string(&values, "summaries")?, "summaries")?;
        let details = values
            .get("details")
            .map(|value| parse_optional_string_list(value, "details"))
            .transpose()?;
        let families =
            parse_non_empty_string_list(&required_string(&values, "families")?, "families")?;
        let accessibility_labels = parse_non_empty_string_list(
            &required_string(&values, "accessibility_labels")?,
            "accessibility_labels",
        )?;

        validate_complete_unique_ids(&ids)?;
        let len = ids.len();
        for (field, field_len) in [
            ("labels", labels.len()),
            ("summaries", summaries.len()),
            ("details", details.as_ref().map_or(len, Vec::len)),
            ("families", families.len()),
            ("accessibility_labels", accessibility_labels.len()),
        ] {
            if field_len != len {
                return Err(format!("{field} must contain {len} entries"));
            }
        }

        let cards = ids
            .into_iter()
            .enumerate()
            .map(|(index, kind)| CardPresentation {
                kind,
                label: labels[index].clone(),
                summary: summaries[index].clone(),
                details: details.as_ref().and_then(|entries| entries[index].clone()),
                family: families[index].clone(),
                accessibility_label: accessibility_labels[index].clone(),
            })
            .collect();

        Ok(Self { cards })
    }

    pub fn get(&self, kind: EventKind) -> Option<&CardPresentation> {
        self.cards.iter().find(|card| card.kind == kind)
    }
}

pub fn card_face(kind: EventKind) -> CardFaceView {
    let catalog = CardPresentationCatalog::parse(include_str!("../data/cards_presentation.toml"))
        .expect("checked Flood Watch card presentation metadata");
    let presentation = catalog
        .get(kind)
        .expect("presentation catalog covers every Flood Watch event kind");
    CardFaceView {
        id: kind.id(),
        label: presentation.label.clone(),
        summary: presentation.summary.clone(),
        details: presentation.details.clone(),
        family: presentation.family.clone(),
        accessibility_label: presentation.accessibility_label.clone(),
    }
}

fn parse_event_kind_list(value: &str) -> Result<Vec<EventKind>, String> {
    parse_string_list(value)
        .into_iter()
        .map(|part| EventKind::parse(&part).ok_or_else(|| format!("unknown event `{part}`")))
        .collect()
}

fn parse_non_empty_string_list(value: &str, field: &str) -> Result<Vec<String>, String> {
    let entries = parse_string_list(value);
    if let Some(empty_index) = entries.iter().position(|entry| entry.trim().is_empty()) {
        return Err(format!("{field} entry {empty_index} must not be empty"));
    }
    Ok(entries)
}

fn parse_optional_string_list(value: &str, field: &str) -> Result<Vec<Option<String>>, String> {
    Ok(parse_non_empty_string_list(value, field)?
        .into_iter()
        .map(|entry| {
            let trimmed = entry.trim();
            if trimmed == "-" {
                None
            } else {
                Some(trimmed.to_owned())
            }
        })
        .collect())
}

fn validate_complete_unique_ids(ids: &[EventKind]) -> Result<(), String> {
    let expected = expected_event_kinds();
    if ids.len() != expected.len() {
        return Err(format!(
            "event_ids must contain {} presentation rows",
            expected.len()
        ));
    }

    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(*id) {
            return Err(format!("duplicate presentation row for `{}`", id.id()));
        }
    }
    for expected_id in expected {
        if !seen.contains(&expected_id) {
            return Err(format!(
                "missing presentation row for `{}`",
                expected_id.id()
            ));
        }
    }
    Ok(())
}

fn expected_event_kinds() -> Vec<EventKind> {
    let mut kinds = Vec::new();
    for district in DistrictId::ALL {
        kinds.push(EventKind::Downpour { district });
    }
    for district in DistrictId::ALL {
        kinds.push(EventKind::StormSurge { district });
    }
    kinds.push(EventKind::Reprieve);
    kinds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_metadata_is_complete_and_fail_closed() {
        let catalog =
            CardPresentationCatalog::parse(include_str!("../data/cards_presentation.toml"))
                .expect("presentation parses");
        assert_eq!(catalog.cards.len(), 11);
        assert_eq!(
            catalog
                .get(EventKind::StormSurge {
                    district: DistrictId::Market
                })
                .expect("market surge")
                .label,
            "Storm Surge at Market"
        );
        assert!(catalog
            .get(EventKind::StormSurge {
                district: DistrictId::Market
            })
            .expect("market surge")
            .details
            .as_deref()
            .unwrap()
            .contains("two separate Market rise steps"));
        assert!(CardPresentationCatalog::parse(
            "event_ids = \"reprieve\"\nlabels = \"Reprieve\"\nsummaries = \"No rise.\"\nfamilies = \"reprieve\"\naccessibility_labels = \"Reprieve.\"\ntrigger = \"bad\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "event_ids = \"reprieve\"\nlabels = \"Reprieve\"\nsummaries = \"No rise.\"\ndetails = \"No rise.\"\nfamilies = \"reprieve\"\naccessibility_labels = \"Reprieve.\"\nselector = \"bad\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "event_ids = \"reprieve\"\nlabels = \"Reprieve\"\nsummaries = \"No rise.\"\ndetails = \"A,B\"\nfamilies = \"reprieve\"\naccessibility_labels = \"Reprieve.\"\n"
        )
        .is_err());
        assert!(CardPresentationCatalog::parse(
            "event_ids = \"reprieve,reprieve\"\nlabels = \"A,B\"\nsummaries = \"A,B\"\nfamilies = \"reprieve,reprieve\"\naccessibility_labels = \"A,B\"\n"
        )
        .is_err());
    }

    #[test]
    fn ui_metadata_and_card_faces_are_player_facing() {
        let metadata = ui_metadata();
        assert_eq!(metadata.event_deck_label, "Storm deck");
        assert!(!format!("{metadata:?}").contains("debug"));

        let face = card_face(EventKind::Downpour {
            district: DistrictId::Riverside,
        });
        assert_eq!(face.id, "downpour/district_riverside");
        assert_eq!(face.label, "Downpour at Riverside");
        assert!(face.summary.contains("Riverside rises"));
        assert!(face.details.as_deref().unwrap().contains("Riverside flood"));
    }
}
