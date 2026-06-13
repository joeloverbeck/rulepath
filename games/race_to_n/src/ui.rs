//! UI metadata for Race to 21.

pub const SEAT_LABEL_AUDIT: &str =
    "Race to 21 is factionless; keep existing first-player/second-player labels.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Race to 21 has no automated non-interactive burst surface in scope.";

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
        icon_id: "race_to_n",
        theme_key: "race-to-n",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "ascending-step-path",
        accessibility_label: "Race to 21 abstract ascending step path",
    }
}
