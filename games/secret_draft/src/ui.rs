#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: &'static str,
    pub display_name: &'static str,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: crate::ids::GAME_ID,
        display_name: "Veiled Draft",
    }
}
