//! UI metadata and presentation helpers for Event Frontier.

use crate::{
    cards::{CardId, CardPresentationCatalog, SitePresentationCatalog},
    ids::{FactionId, SiteId},
    variants::{parse_flat_toml, parse_string_list, reject_unknown_keys, required_string},
};

pub const DISPLAY_NAME: &str = "Event Frontier";
pub const TURN_REPORT_ADOPTION: &str =
    "Adopted by ACTCONMAT-006; Event Frontier reports viewer-filtered event/reckoning bursts near the board.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogThemeMetadata {
    pub icon_id: &'static str,
    pub theme_key: &'static str,
    pub accent_token: &'static str,
    pub secondary_accent_token: &'static str,
    pub shape_token: &'static str,
    pub accessibility_label: &'static str,
}

pub fn catalog_theme() -> CatalogThemeMetadata {
    CatalogThemeMetadata {
        icon_id: "event_frontier",
        theme_key: "event-frontier",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "frontier-peak-event-spark",
        accessibility_label: "Event Frontier abstract frontier ridge with event spark",
    }
}

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
    pub seat_labels: Vec<SeatDisplayLabel>,
    pub faction_labels: Vec<FactionDisplayLabel>,
    pub action_affordance_templates: Vec<ActionAffordanceTemplate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatDisplayLabel {
    pub seat: String,
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactionDisplayLabel {
    pub faction: String,
    pub label: String,
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
pub struct ActionAffordanceTemplate {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionAffordanceTemplateCatalog {
    pub templates: Vec<ActionAffordanceTemplate>,
}

impl ActionAffordanceTemplateCatalog {
    pub fn parse(input: &str) -> Result<Self, String> {
        let values = parse_flat_toml(input)?;
        reject_unknown_keys(&values, &["template_ids", "texts"])?;

        let ids = parse_non_empty_string_list(
            &required_string(&values, "template_ids")?,
            "template_ids",
        )?;
        let texts = parse_non_empty_string_list(&required_string(&values, "texts")?, "texts")?;
        if ids.len() != texts.len() {
            return Err(format!("texts must contain {} entries", ids.len()));
        }
        for id in &ids {
            if id.contains(' ') {
                return Err(format!("template id `{id}` must be a stable token"));
            }
        }
        for (index, id) in ids.iter().enumerate() {
            if ids.iter().take(index).any(|seen| seen == id) {
                return Err(format!("duplicate action affordance template `{id}`"));
            }
        }

        Ok(Self {
            templates: ids
                .into_iter()
                .zip(texts)
                .map(|(id, text)| ActionAffordanceTemplate { id, text })
                .collect(),
        })
    }

    pub fn get(&self, id: &str) -> Option<&ActionAffordanceTemplate> {
        self.templates.iter().find(|template| template.id == id)
    }
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
        seat_labels: seat_display_labels(),
        faction_labels: faction_display_labels(),
        action_affordance_templates: action_affordance_templates(),
    }
}

pub fn seat_display_labels() -> Vec<SeatDisplayLabel> {
    [
        ("seat_0", FactionId::Charter),
        ("seat_1", FactionId::Freeholders),
    ]
    .into_iter()
    .map(|(seat, faction)| SeatDisplayLabel {
        seat: seat.to_owned(),
        label: faction.label().to_owned(),
    })
    .collect()
}

pub fn faction_display_labels() -> Vec<FactionDisplayLabel> {
    FactionId::ALL
        .into_iter()
        .map(|faction| FactionDisplayLabel {
            faction: faction.as_str().to_owned(),
            label: faction.label().to_owned(),
        })
        .collect()
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
        details: presentation.details.clone(),
        family: presentation.family.clone(),
        accessibility_label: presentation.accessibility_label.clone(),
    }
}

pub fn site_label(site: SiteId) -> String {
    let catalog = SitePresentationCatalog::parse(include_str!("../data/sites_presentation.toml"))
        .expect("checked Event Frontier site presentation metadata");
    catalog
        .get(site)
        .expect("presentation catalog covers every Event Frontier site")
        .label
        .clone()
}

pub fn site_accessibility_label(site: SiteId) -> String {
    let catalog = SitePresentationCatalog::parse(include_str!("../data/sites_presentation.toml"))
        .expect("checked Event Frontier site presentation metadata");
    catalog
        .get(site)
        .expect("presentation catalog covers every Event Frontier site")
        .accessibility_label
        .clone()
}

pub fn action_affordance_templates() -> Vec<ActionAffordanceTemplate> {
    ActionAffordanceTemplateCatalog::parse(include_str!("../data/action_affordance_templates.toml"))
        .expect("checked Event Frontier action affordance templates")
        .templates
}

fn parse_non_empty_string_list(value: &str, field: &str) -> Result<Vec<String>, String> {
    let entries = parse_string_list(value);
    if let Some(empty_index) = entries.iter().position(|entry| entry.trim().is_empty()) {
        return Err(format!("{field} entry {empty_index} must not be empty"));
    }
    Ok(entries)
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
        assert_eq!(metadata.seat_labels[0].label, "Charter");
        assert_eq!(metadata.seat_labels[1].label, "Freeholders");
        assert_eq!(metadata.faction_labels[0].faction, "faction_charter");
        let combined = format!("{metadata:?}");
        assert!(!combined.contains("debug"));
        assert!(!combined.contains("candidate"));
        assert!(!combined.contains("internal"));
        assert!(!combined.contains("redacted"));
        assert!(metadata
            .action_affordance_templates
            .iter()
            .any(|template| template.id == "acting_forfeits_next_card"));
    }

    #[test]
    fn card_face_resolves_authored_presentation() {
        let face = card_face(CardId::HighMeadowFair);
        assert_eq!(face.id, "ef_high_meadow_fair");
        assert_eq!(face.label, "High Meadow Fair");
        assert!(face.summary.contains("Freeholders gain"));
        assert!(face.details.as_deref().unwrap().contains("High Meadow"));
        assert_eq!(face.family, "ordinary");
    }

    #[test]
    fn site_labels_resolve_authored_presentation() {
        assert_eq!(site_label(SiteId::GranitePass), "Granite Pass");
        assert_eq!(
            site_accessibility_label(SiteId::HighMeadow),
            "High Meadow site"
        );
    }

    #[test]
    fn action_affordance_templates_resolve_and_fail_closed() {
        let templates = ActionAffordanceTemplateCatalog::parse(include_str!(
            "../data/action_affordance_templates.toml"
        ))
        .unwrap();
        assert_eq!(
            templates.get("acting_forfeits_next_card").unwrap().text,
            "Acting now forfeits your eligibility for the next card."
        );
        assert_eq!(
            templates.get("base_one_resource_per_site").unwrap().text,
            "Operations cost one matching resource per selected site before public edict modifiers."
        );
        assert!(ActionAffordanceTemplateCatalog::parse(
            "template_ids = \"acting_forfeits_next_card\"\ntexts = \"x\"\ntrigger = \"bad\"\n"
        )
        .is_err());
        assert!(ActionAffordanceTemplateCatalog::parse(
            "template_ids = \"acting_forfeits_next_card,acting_forfeits_next_card\"\ntexts = \"x,y\"\n"
        )
        .is_err());
    }
}
