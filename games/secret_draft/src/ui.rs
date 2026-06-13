use crate::ids::{DraftItemId, GAME_ID};

pub const SEAT_LABEL_AUDIT: &str =
    "Veiled Draft is factionless; keep existing player/seat naming for simultaneous commitments.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; Veiled Draft reveal narration already stays in its board surface.";

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
        icon_id: "secret_draft",
        theme_key: "secret-draft",
        accent_token: "--game-accent",
        secondary_accent_token: "--game-accent-2",
        shape_token: "folded-hidden-draft",
        accessibility_label: "Veiled Draft abstract folded hidden draft panel",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: &'static str,
    pub display_name: &'static str,
    pub table_label: String,
    pub visible_pool_label: String,
    pub drafted_label: String,
    pub pending_label: String,
    pub score_label: String,
    pub reveal_group_token: String,
    pub reduced_motion_token: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID,
        display_name: "Veiled Draft",
        table_label: "Veiled Draft table".to_owned(),
        visible_pool_label: "Visible draft pool".to_owned(),
        drafted_label: "Drafted items".to_owned(),
        pending_label: "Pending commitments".to_owned(),
        score_label: "Score".to_owned(),
        reveal_group_token: "secret-draft-reveal-batch".to_owned(),
        reduced_motion_token: "secret-draft-reduced-motion".to_owned(),
    }
}

pub fn item_accessibility_label(item: DraftItemId) -> String {
    format!(
        "{}, {} thread, value {}",
        item.label(),
        item.thread().as_str(),
        item.value()
    )
}

pub fn pending_copy(seat_0_committed: bool, seat_1_committed: bool) -> String {
    format!(
        "seat_0 {}; seat_1 {}",
        if seat_0_committed {
            "committed"
        } else {
            "waiting"
        },
        if seat_1_committed {
            "committed"
        } else {
            "waiting"
        }
    )
}

pub fn priority_copy(priority_seat: crate::ids::SecretDraftSeat) -> String {
    format!("{} has conflict priority", priority_seat.as_str())
}

pub fn action_preview_copy(item: DraftItemId) -> String {
    format!("Commit {} as a hidden draft choice", item.label())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_metadata_and_copy_are_viewer_safe() {
        let metadata = ui_metadata();
        assert_eq!(metadata.display_name, "Veiled Draft");
        assert_eq!(
            item_accessibility_label(DraftItemId::Tide3),
            "Tide Three, tide thread, value 3"
        );
        assert_eq!(
            pending_copy(true, false),
            "seat_0 committed; seat_1 waiting"
        );

        let combined = format!("{metadata:?}");
        assert!(!combined.contains("commitment_for_internal"));
        assert!(!combined.contains("candidate"));
        assert!(!combined.contains("debug"));
    }
}
