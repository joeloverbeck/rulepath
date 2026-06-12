//! UI metadata and presentation helpers for Event Frontier.

use crate::cards::{CardId, CardPresentationCatalog};

pub const DISPLAY_NAME: &str = "Event Frontier";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub table_label: String,
    pub event_deck_label: String,
    pub current_card_label: String,
    pub next_card_label: String,
    pub discard_label: String,
    pub face_down_label: String,
    pub face_down_summary: String,
    pub reduced_motion_token: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardFaceView {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub family: String,
    pub accessibility_label: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        table_label: "Event Frontier board".to_owned(),
        event_deck_label: "Event deck".to_owned(),
        current_card_label: "Current event".to_owned(),
        next_card_label: "Next public event".to_owned(),
        discard_label: "Resolved events".to_owned(),
        face_down_label: "Face-down event deck".to_owned(),
        face_down_summary: "Order hidden until cards become public.".to_owned(),
        reduced_motion_token: "event-frontier-reduced-motion".to_owned(),
    }
}

pub fn card_face(card: CardId) -> CardFaceView {
    let catalog = CardPresentationCatalog::parse(include_str!("../data/cards_presentation.toml"))
        .expect("checked Event Frontier card presentation metadata");
    let presentation = catalog
        .get(card)
        .expect("presentation catalog covers every Event Frontier card");
    CardFaceView {
        id: card.as_str().to_owned(),
        label: presentation.label.clone(),
        summary: presentation.summary.clone(),
        family: presentation.family.clone(),
        accessibility_label: presentation.accessibility_label.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_metadata_has_player_facing_labels() {
        let metadata = ui_metadata();
        assert_eq!(metadata.event_deck_label, "Event deck");
        assert_eq!(
            metadata.face_down_summary,
            "Order hidden until cards become public."
        );
        let combined = format!("{metadata:?}");
        assert!(!combined.contains("debug"));
        assert!(!combined.contains("candidate"));
        assert!(!combined.contains("internal"));
        assert!(!combined.contains("redacted"));
    }

    #[test]
    fn card_face_resolves_authored_presentation() {
        let face = card_face(CardId::HighMeadowFair);
        assert_eq!(face.id, "ef_high_meadow_fair");
        assert_eq!(face.label, "High Meadow Fair");
        assert!(face.summary.contains("Freeholders gain"));
        assert_eq!(face.family, "ordinary");
    }
}
