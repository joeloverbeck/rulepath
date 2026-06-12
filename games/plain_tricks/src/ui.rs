use crate::ids::{TrickCardId, TrickRank, TrickSuit, GAME_ID};

pub const SEAT_LABEL_AUDIT: &str =
    "Plain Tricks is factionless; keep existing player/seat naming for hand ownership.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: String,
    pub display_name: String,
    pub table_label: String,
    pub own_hand_label: String,
    pub opponent_hand_label: String,
    pub current_trick_label: String,
    pub trick_history_label: String,
    pub score_label: String,
    pub play_action_label: String,
    pub observer_disabled_reason: String,
    pub reduced_motion_note: String,
    pub rules_summary: Vec<String>,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID.to_owned(),
        display_name: "Plain Tricks".to_owned(),
        table_label: "Plain Tricks table".to_owned(),
        own_hand_label: "Your hand".to_owned(),
        opponent_hand_label: "Opponent hand count".to_owned(),
        current_trick_label: "Current trick".to_owned(),
        trick_history_label: "Resolved tricks".to_owned(),
        score_label: "Tricks won".to_owned(),
        play_action_label: "Play a card".to_owned(),
        observer_disabled_reason: "Observer view has no private card actions".to_owned(),
        reduced_motion_note: "Use simple card placement changes when reduced motion is enabled"
            .to_owned(),
        rules_summary: vec![
            "Lead any card.".to_owned(),
            "Follow the led suit when able.".to_owned(),
            "A follower without the led suit may play any card.".to_owned(),
            "Highest card in the led suit wins the trick.".to_owned(),
        ],
    }
}

pub fn card_accessibility_label(card: TrickCardId) -> String {
    format!(
        "{} card, rank {}",
        card.suit().label(),
        card.rank().as_str()
    )
}

pub fn suit_label(suit: TrickSuit) -> &'static str {
    suit.label()
}

pub fn rank_label(rank: TrickRank) -> &'static str {
    rank.as_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui_copy_uses_neutral_terms() {
        let source = include_str!("ui.rs").to_ascii_lowercase();
        let forbidden = [
            "wh".to_owned() + "ist",
            "he".to_owned() + "arts",
            "sp".to_owned() + "ades",
            "br".to_owned() + "idge",
            "casi".to_owned() + "no",
            "pay".to_owned() + "out",
        ];
        for forbidden in forbidden {
            assert!(!source.contains(&forbidden), "forbidden term {forbidden}");
        }
    }
}
