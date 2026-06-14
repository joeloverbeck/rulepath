use crate::ids::{GAME_ID, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: String,
    pub display_name: String,
    pub surface_label: String,
    pub min_seats: u8,
    pub default_seats: u8,
    pub max_seats: u8,
    pub contribution_label: String,
    pub board_label: String,
    pub hidden_hole_label: String,
    pub reduced_motion_note: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID.to_owned(),
        display_name: "River Ledger".to_owned(),
        surface_label: "River Ledger table".to_owned(),
        min_seats: STANDARD_MIN_SEATS,
        default_seats: STANDARD_DEFAULT_SEATS,
        max_seats: STANDARD_MAX_SEATS,
        contribution_label: "Contribution ledger".to_owned(),
        board_label: "Community board".to_owned(),
        hidden_hole_label: "Private cards hidden".to_owned(),
        reduced_motion_note: "Use immediate state changes when reduced motion is enabled"
            .to_owned(),
    }
}
