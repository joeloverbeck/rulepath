//! UI metadata skeleton for Flood Watch.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub display_name: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        display_name: "Flood Watch".to_owned(),
    }
}
