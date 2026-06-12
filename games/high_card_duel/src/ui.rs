pub const SEAT_LABEL_AUDIT: &str =
    "High Card Duel is factionless; keep existing player/seat naming for hidden-commitment views.";
pub const TURN_REPORT_AUDIT: &str =
    "Not adopted by ACTCONMAT-006; High Card Duel reveal narration already stays in its board surface.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub table_label: String,
    pub card_back_token: String,
    pub own_card_token: String,
    pub revealed_card_token: String,
    pub empty_commitment_token: String,
    pub face_down_commitment_token: String,
    pub commit_action_label: String,
    pub observer_disabled_reason: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        table_label: "Wayfarer duel table".to_owned(),
        card_back_token: "trail-badge-back".to_owned(),
        own_card_token: "duel-rune-own-card".to_owned(),
        revealed_card_token: "duel-rune-revealed-card".to_owned(),
        empty_commitment_token: "commitment-empty".to_owned(),
        face_down_commitment_token: "commitment-face-down".to_owned(),
        commit_action_label: "Commit a trail badge face-down".to_owned(),
        observer_disabled_reason: "Observer view has no private commit actions".to_owned(),
    }
}

pub fn card_accessibility_label(rank: u8) -> String {
    format!("Private duel card, rank {rank}")
}

pub fn revealed_card_accessibility_label(rank: u8) -> String {
    format!("Revealed duel card, rank {rank}")
}

pub fn face_down_commitment_label() -> &'static str {
    "Face-down commitment"
}
