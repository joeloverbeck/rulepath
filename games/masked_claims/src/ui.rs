use crate::ids::{Grade, GAME_ID, VARIANT_ID};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: String,
    pub variant_id: String,
    pub display_name: String,
    pub grade_labels: Vec<String>,
    pub claim_preview_template: String,
    pub reaction_prompt_template: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID.to_owned(),
        variant_id: VARIANT_ID.to_owned(),
        display_name: "Masked Claims".to_owned(),
        grade_labels: Grade::ALL
            .into_iter()
            .map(|grade| grade.label().to_owned())
            .collect(),
        claim_preview_template: "Claim grade {grade}; accepted claims score that grade.".to_owned(),
        reaction_prompt_template: "A claim is pending; the responder may accept or challenge."
            .to_owned(),
    }
}

pub fn grade_label(grade: Grade) -> &'static str {
    grade.label()
}

pub fn grade_accessibility_label(grade: Grade) -> String {
    format!("Grade {} {}", grade.as_str(), grade.label())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::MaskTileId;

    #[test]
    fn ui_metadata_is_viewer_safe() {
        let rendered = format!("{:?}", ui_metadata());
        for tile in MaskTileId::ALL {
            assert!(!rendered.contains(tile.as_str()));
            assert!(!rendered.contains(&tile.label()));
        }
    }
}
