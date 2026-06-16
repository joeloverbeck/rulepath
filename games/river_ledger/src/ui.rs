use crate::ids::{
    RiverLedgerSeat, GAME_ID, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMetadata {
    pub game_id: String,
    pub display_name: String,
    pub surface_label: String,
    pub viewer_modes: Vec<String>,
    pub min_seats: u8,
    pub default_seats: u8,
    pub max_seats: u8,
    pub seat_metadata_label: String,
    pub action_hint_label: String,
    pub outcome_explanation_label: String,
    pub contribution_label: String,
    pub board_label: String,
    pub hidden_hole_label: String,
    pub reduced_motion_note: String,
    pub hand_rankings: Vec<HandRankingMetadata>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandRankingMetadata {
    pub category: String,
    pub label: String,
    pub definition: String,
}

pub fn ui_metadata() -> UiMetadata {
    UiMetadata {
        game_id: GAME_ID.to_owned(),
        display_name: "River Ledger".to_owned(),
        surface_label: "River Ledger table".to_owned(),
        viewer_modes: vec!["observer".to_owned(), "seat_private".to_owned()],
        min_seats: STANDARD_MIN_SEATS,
        default_seats: STANDARD_DEFAULT_SEATS,
        max_seats: STANDARD_MAX_SEATS,
        seat_metadata_label: "Seat order, button, blinds, active, and pending response".to_owned(),
        action_hint_label: "Legal betting actions and contribution obligations".to_owned(),
        outcome_explanation_label: "Showdown category, tie-break, allocation, and rationale"
            .to_owned(),
        contribution_label: "Contribution ledger".to_owned(),
        board_label: "Community board".to_owned(),
        hidden_hole_label: "Private cards hidden".to_owned(),
        reduced_motion_note: "Use immediate state changes when reduced motion is enabled"
            .to_owned(),
        hand_rankings: hand_rankings(),
    }
}

pub fn seat_public_label(seat: RiverLedgerSeat) -> String {
    format!("Seat {}", seat.index() + 1)
}

fn hand_rankings() -> Vec<HandRankingMetadata> {
    [
        (
            "straight_flush",
            "Straight flush",
            "Five cards in sequence, all sharing one suit.",
        ),
        (
            "four_of_a_kind",
            "Four of a kind",
            "Four cards with the same rank, plus one side card.",
        ),
        (
            "full_house",
            "Full house",
            "Three cards of one rank and two cards of another rank.",
        ),
        (
            "flush",
            "Flush",
            "Five cards sharing one suit, not in sequence.",
        ),
        (
            "straight",
            "Straight",
            "Five cards in sequence, with mixed suits allowed.",
        ),
        (
            "three_of_a_kind",
            "Three of a kind",
            "Three cards with the same rank, plus two side cards.",
        ),
        (
            "two_pair",
            "Two pair",
            "Two ranks paired, plus one side card.",
        ),
        (
            "one_pair",
            "One pair",
            "One paired rank, plus three side cards.",
        ),
        (
            "high_card",
            "High card",
            "No category match; highest ranks decide.",
        ),
    ]
    .into_iter()
    .map(|(category, label, definition)| HandRankingMetadata {
        category: category.to_owned(),
        label: label.to_owned(),
        definition: definition.to_owned(),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::ids::RiverLedgerSeat;

    use super::{seat_public_label, ui_metadata};

    #[test]
    fn seat_public_labels_match_catalog_display_form() {
        assert_eq!(
            seat_public_label(RiverLedgerSeat::from_index(0).expect("seat 0")),
            "Seat 1"
        );
        assert_eq!(
            seat_public_label(RiverLedgerSeat::from_index(5).expect("seat 5")),
            "Seat 6"
        );
    }

    #[test]
    fn hand_rankings_are_ordered_unique_and_inert() {
        let ui = ui_metadata();

        assert_eq!(
            ui.hand_rankings
                .iter()
                .map(|row| row.category.as_str())
                .collect::<Vec<_>>(),
            vec![
                "straight_flush",
                "four_of_a_kind",
                "full_house",
                "flush",
                "straight",
                "three_of_a_kind",
                "two_pair",
                "one_pair",
                "high_card",
            ]
        );

        let unique = ui
            .hand_rankings
            .iter()
            .map(|row| row.category.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(unique.len(), ui.hand_rankings.len());
        assert!(ui
            .hand_rankings
            .iter()
            .all(|row| !row.label.is_empty() && !row.definition.is_empty()));

        let serialized = format!("{:?}", ui.hand_rankings);
        for behavior_token in ["selector", "valid_if", "legal", "action", "effect"] {
            assert!(!serialized.contains(behavior_token), "{serialized}");
        }
    }
}
