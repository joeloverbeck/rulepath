use crate::ids::{CrestCardId, GAME_ID};

pub const SEAT_LABEL_AUDIT: &str =
    "Crest Ledger is factionless; keep existing player/seat naming for hidden-card ownership.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Crest Ledger reveal narration already stays in its board surface.";

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
        icon_id: "crest_ledger",
        theme_key: "crest-ledger",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "center-pool-card-slabs",
        accessibility_label: "Crest Ledger abstract center pool with card slabs",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: String,
    pub display_name: String,
    pub surface_label: String,
    pub shared_pool_label: String,
    pub hidden_center_label: String,
    pub hidden_private_label: String,
    pub hold_label: String,
    pub press_label: String,
    pub lift_label: String,
    pub match_label: String,
    pub yield_label: String,
    pub reduced_motion_note: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID.to_owned(),
        display_name: "Crest Ledger".to_owned(),
        surface_label: "Crest Ledger board".to_owned(),
        shared_pool_label: "Shared pool".to_owned(),
        hidden_center_label: "Center crest hidden".to_owned(),
        hidden_private_label: "Private crest hidden".to_owned(),
        hold_label: "Hold".to_owned(),
        press_label: "Press".to_owned(),
        lift_label: "Lift".to_owned(),
        match_label: "Match".to_owned(),
        yield_label: "Yield".to_owned(),
        reduced_motion_note: "Use simple reveal changes when reduced motion is enabled".to_owned(),
    }
}

pub fn card_accessibility_label(card: CrestCardId) -> String {
    format!("{} crest", card.label())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui_copy_uses_neutral_terms() {
        let source = include_str!("ui.rs").to_ascii_lowercase();
        let forbidden = [
            "casi".to_owned() + "no",
            "po".to_owned() + "ker",
            "ch".to_owned() + "ip",
            "pay".to_owned() + "out",
            "an".to_owned() + "te",
            "bl".to_owned() + "ind",
            "ra".to_owned() + "ke",
        ];
        for forbidden in forbidden {
            assert!(!source.contains(&forbidden), "forbidden term {forbidden}");
        }
    }
}
