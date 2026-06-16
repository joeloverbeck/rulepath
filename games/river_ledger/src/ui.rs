use crate::{
    actions::RiverLedgerAction,
    ids::{
        RiverLedgerSeat, GAME_ID, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    },
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerActionPresentation {
    pub segment: String,
    pub label: String,
    pub helper_text: String,
    pub accessibility_label: String,
    pub display_rows: Vec<RiverLedgerActionDisplayRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerActionDisplayRow {
    pub label: String,
    pub value: String,
    pub tone: String,
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

pub fn action_presentation(
    action: RiverLedgerAction,
    required_to_call: u16,
    adds_to_pot: u16,
    raises_remaining: u8,
    accessibility_label: String,
) -> RiverLedgerActionPresentation {
    let display_rows = match action {
        RiverLedgerAction::Fold | RiverLedgerAction::Check => {
            vec![display_row("Adds", "0", "neutral")]
        }
        RiverLedgerAction::Call => vec![
            display_row("Call price", required_to_call.to_string(), "cost"),
            display_row("Adds", adds_to_pot.to_string(), "cost"),
        ],
        RiverLedgerAction::Bet => vec![
            display_row("Adds", adds_to_pot.to_string(), "cost"),
            display_row("Raises left", raises_remaining.to_string(), "limit"),
        ],
        RiverLedgerAction::Raise => vec![
            display_row("Call price", required_to_call.to_string(), "cost"),
            display_row("Adds", adds_to_pot.to_string(), "cost"),
            display_row("Raises left", raises_remaining.to_string(), "limit"),
        ],
    };

    RiverLedgerActionPresentation {
        segment: action.segment().to_owned(),
        label: action.label().to_owned(),
        helper_text: action_helper_text(action, required_to_call, adds_to_pot, raises_remaining),
        accessibility_label,
        display_rows,
    }
}

fn action_helper_text(
    action: RiverLedgerAction,
    required_to_call: u16,
    adds_to_pot: u16,
    raises_remaining: u8,
) -> String {
    match action {
        RiverLedgerAction::Fold => "Leave this hand; add no more to the ledger.".to_owned(),
        RiverLedgerAction::Check => "Stay in without adding to the ledger.".to_owned(),
        RiverLedgerAction::Call => {
            format!("Match the current price by adding {required_to_call}.")
        }
        RiverLedgerAction::Bet => {
            format!("Open this street by adding {adds_to_pot}; {raises_remaining} raises remain.")
        }
        RiverLedgerAction::Raise => format!(
            "Call {required_to_call} and add the street unit; {raises_remaining} raises remain after this choice."
        ),
    }
}

fn display_row(
    label: impl Into<String>,
    value: impl Into<String>,
    tone: impl Into<String>,
) -> RiverLedgerActionDisplayRow {
    RiverLedgerActionDisplayRow {
        label: label.into(),
        value: value.into(),
        tone: tone.into(),
    }
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

    use crate::{actions::RiverLedgerAction, ids::RiverLedgerSeat};

    use super::{action_presentation, seat_public_label, ui_metadata};

    #[test]
    fn action_presentation_rows_are_segment_relevant() {
        let fold = action_presentation(RiverLedgerAction::Fold, 2, 0, 3, "Fold".to_owned());
        assert_eq!(row_labels(&fold), vec!["Adds"]);

        let check = action_presentation(RiverLedgerAction::Check, 0, 0, 3, "Check".to_owned());
        assert_eq!(row_labels(&check), vec!["Adds"]);

        let call = action_presentation(RiverLedgerAction::Call, 2, 2, 3, "Call".to_owned());
        assert_eq!(row_labels(&call), vec!["Call price", "Adds"]);

        let bet = action_presentation(RiverLedgerAction::Bet, 0, 2, 3, "Bet".to_owned());
        assert_eq!(row_labels(&bet), vec!["Adds", "Raises left"]);

        let raise = action_presentation(RiverLedgerAction::Raise, 2, 4, 2, "Raise".to_owned());
        assert_eq!(
            row_labels(&raise),
            vec!["Call price", "Adds", "Raises left"]
        );
    }

    fn row_labels(presentation: &super::RiverLedgerActionPresentation) -> Vec<&str> {
        presentation
            .display_rows
            .iter()
            .map(|row| row.label.as_str())
            .collect()
    }

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
