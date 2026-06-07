use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{
    ids::CellId,
    rules::{
        actor_seat, legal_placements, Direction, FlipRun, Placement, FORCED_PASS_SEGMENT,
        PLACE_SEGMENT_PREFIX,
    },
    state::DirectionalFlipState,
};

pub const PASS_REASON_NO_MOVES: &str = "no_legal_placements";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectionPreview {
    pub direction: Direction,
    pub cells: Vec<CellId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementPreview {
    pub preview_id: String,
    pub action_segment: String,
    pub target_cell: CellId,
    pub row: u8,
    pub column: u8,
    pub accessibility_label: String,
    pub ordered_flips: Vec<CellId>,
    pub direction_groups: Vec<DirectionPreview>,
    pub explanation: String,
}

pub fn legal_action_tree(state: &DirectionalFlipState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let placements = legal_placements(state, state.active_seat);
    let choices = if placements.is_empty() {
        vec![forced_pass_choice()]
    } else {
        placements.into_iter().map(placement_choice).collect()
    };

    ActionTree::flat(state.freshness_token, choices)
}

pub fn placement_preview(placement: &Placement) -> PlacementPreview {
    let target_id = placement.cell.as_string();
    let ordered_flips = placement.ordered_flips();
    let direction_groups = placement
        .flip_runs
        .iter()
        .map(direction_preview)
        .collect::<Vec<_>>();
    let flip_count = ordered_flips.len();

    PlacementPreview {
        preview_id: format!("preview:{target_id}"),
        action_segment: place_segment(placement.cell),
        target_cell: placement.cell,
        row: (placement.cell.row.index() + 1) as u8,
        column: (placement.cell.column.index() + 1) as u8,
        accessibility_label: format!(
            "Place at {target_id}, flipping {flip_count} {}",
            plural_disc(flip_count)
        ),
        ordered_flips,
        direction_groups,
        explanation: placement_explanation(placement),
    }
}

fn placement_choice(placement: Placement) -> ActionChoice {
    let preview = placement_preview(&placement);
    let target_id = preview.target_cell.as_string();
    let mut choice = ActionChoice::leaf(
        preview.action_segment.clone(),
        format!("Place at {target_id}"),
        preview.accessibility_label.clone(),
    );
    choice.metadata = placement_metadata(&preview);
    choice.tags = vec![
        "flat".to_owned(),
        "placement".to_owned(),
        "cell".to_owned(),
        "preview".to_owned(),
    ];
    choice.preview = ActionPreview::Available;
    choice
}

fn forced_pass_choice() -> ActionChoice {
    let mut choice = ActionChoice::leaf(
        FORCED_PASS_SEGMENT,
        "Forced pass",
        "Pass because no legal placements are available",
    );
    choice.metadata = vec![
        metadata("action_kind", "forced_pass"),
        metadata("reason_code", PASS_REASON_NO_MOVES),
        metadata(
            "explanation",
            "No legal placements are available, so this seat must pass.",
        ),
    ];
    choice.tags = vec!["flat".to_owned(), "forced_pass".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn placement_metadata(preview: &PlacementPreview) -> Vec<ActionMetadata> {
    vec![
        metadata("action_kind", "place"),
        metadata("cell_id", preview.target_cell.as_string()),
        metadata("row", preview.row.to_string()),
        metadata("column", preview.column.to_string()),
        metadata("preview_id", preview.preview_id.clone()),
        metadata("ordered_flip_cells", cell_list(&preview.ordered_flips)),
        metadata(
            "direction_groups",
            direction_group_list(&preview.direction_groups),
        ),
        metadata("explanation", preview.explanation.clone()),
    ]
}

fn direction_preview(run: &FlipRun) -> DirectionPreview {
    DirectionPreview {
        direction: run.direction,
        cells: run.cells.clone(),
    }
}

fn placement_explanation(placement: &Placement) -> String {
    let target_id = placement.cell.as_string();
    let groups = placement
        .flip_runs
        .iter()
        .map(|run| {
            format!(
                "{} {}",
                cell_list(&run.cells),
                direction_word(run.direction)
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    format!("Places at {target_id} and flips {groups}.")
}

fn place_segment(cell: CellId) -> String {
    format!("{PLACE_SEGMENT_PREFIX}{}", cell.as_string())
}

fn cell_list(cells: &[CellId]) -> String {
    cells
        .iter()
        .map(|cell| cell.as_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn direction_group_list(groups: &[DirectionPreview]) -> String {
    groups
        .iter()
        .map(|group| format!("{}:{}", group.direction.as_str(), cell_list(&group.cells)))
        .collect::<Vec<_>>()
        .join("|")
}

fn direction_word(direction: Direction) -> &'static str {
    match direction {
        Direction::North => "northward",
        Direction::Northeast => "northeastward",
        Direction::East => "eastward",
        Direction::Southeast => "southeastward",
        Direction::South => "southward",
        Direction::Southwest => "southwestward",
        Direction::West => "westward",
        Direction::Northwest => "northwestward",
    }
}

fn plural_disc(count: usize) -> &'static str {
    if count == 1 {
        "disc"
    } else {
        "discs"
    }
}

fn metadata(key: impl Into<String>, value: impl Into<String>) -> ActionMetadata {
    ActionMetadata {
        key: key.into(),
        value: value.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ids::{ColumnId, DirectionalFlipSeat, RowId},
        rules::{apply_action, ValidatedAction},
        setup::setup_match,
        CellOccupancy, DirectionalFlipState, TerminalOutcome,
    };
    use engine_core::{Actor, SeatId, Seed};

    fn state() -> DirectionalFlipState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn actor(state: &DirectionalFlipState, seat: DirectionalFlipSeat) -> Actor {
        Actor {
            seat_id: state.seats[seat.index()].clone(),
        }
    }

    fn metadata_value<'a>(choice: &'a ActionChoice, key: &str) -> Option<&'a str> {
        choice
            .metadata
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.value.as_str())
    }

    #[test]
    fn action_tree_lists_only_legal_placements_in_row_major_order() {
        let state = state();

        let tree = legal_action_tree(&state, &actor(&state, DirectionalFlipSeat::Seat0));
        let segments = tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            segments,
            vec!["place/r3c4", "place/r4c3", "place/r5c6", "place/r6c5"]
        );
        assert!(tree
            .root
            .choices
            .iter()
            .all(|choice| metadata_value(choice, "action_kind") == Some("place")));
        assert!(tree
            .root
            .choices
            .iter()
            .all(|choice| choice.preview == ActionPreview::Available));
    }

    #[test]
    fn action_tree_is_empty_for_terminal_or_non_active_actor() {
        let mut state = state();

        let non_active = legal_action_tree(&state, &actor(&state, DirectionalFlipSeat::Seat1));
        assert!(non_active.root.choices.is_empty());

        state.terminal_outcome = Some(TerminalOutcome::Draw);
        let terminal = legal_action_tree(&state, &actor(&state, DirectionalFlipSeat::Seat0));
        assert!(terminal.root.choices.is_empty());
    }

    #[test]
    fn forced_pass_is_sole_action_when_no_placement_exists() {
        let mut state = state();
        state.cells = DirectionalFlipState::empty_cells();

        let tree = legal_action_tree(&state, &actor(&state, DirectionalFlipSeat::Seat0));

        assert_eq!(tree.root.choices.len(), 1);
        let choice = &tree.root.choices[0];
        assert_eq!(choice.segment, "pass/forced");
        assert_eq!(metadata_value(choice, "action_kind"), Some("forced_pass"));
        assert_eq!(
            metadata_value(choice, "reason_code"),
            Some(PASS_REASON_NO_MOVES)
        );
    }

    #[test]
    fn placement_preview_metadata_contains_viewer_safe_fields() {
        let state = state();
        let tree = legal_action_tree(&state, &actor(&state, DirectionalFlipSeat::Seat0));
        let choice = tree
            .root
            .choices
            .iter()
            .find(|choice| choice.segment == "place/r3c4")
            .expect("opening move exists");

        assert_eq!(metadata_value(choice, "cell_id"), Some("r3c4"));
        assert_eq!(metadata_value(choice, "row"), Some("3"));
        assert_eq!(metadata_value(choice, "column"), Some("4"));
        assert_eq!(metadata_value(choice, "preview_id"), Some("preview:r3c4"));
        assert_eq!(metadata_value(choice, "ordered_flip_cells"), Some("r4c4"));
        assert_eq!(
            metadata_value(choice, "direction_groups"),
            Some("south:r4c4")
        );
        assert_eq!(
            metadata_value(choice, "explanation"),
            Some("Places at r3c4 and flips r4c4 southward.")
        );

        let keys = choice
            .metadata
            .iter()
            .map(|entry| entry.key.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            keys,
            vec![
                "action_kind",
                "cell_id",
                "row",
                "column",
                "preview_id",
                "ordered_flip_cells",
                "direction_groups",
                "explanation",
            ]
        );
    }

    #[test]
    fn preview_flip_set_equals_apply_flip_set_for_all_legal_targets() {
        let state = state();

        for placement in legal_placements(&state, DirectionalFlipSeat::Seat0) {
            let preview = placement_preview(&placement);
            let preview_flips = preview
                .ordered_flips
                .iter()
                .map(|cell| cell.as_string())
                .collect::<Vec<_>>();

            let mut applied = state.clone();
            apply_action(&mut applied, ValidatedAction::Place(placement));
            let applied_flips = preview
                .ordered_flips
                .iter()
                .filter(|cell| {
                    state.occupancy(**cell) != CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
                        && applied.occupancy(**cell)
                            == CellOccupancy::Occupied(DirectionalFlipSeat::Seat0)
                })
                .map(|cell| cell.as_string())
                .collect::<Vec<_>>();

            assert_eq!(preview_flips, applied_flips);
        }
    }

    #[test]
    fn preview_preserves_direction_group_order() {
        let mut state = state();
        state.cells = DirectionalFlipState::empty_cells();
        state.set_occupancy(
            CellId::new(RowId::R1, ColumnId::C4),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0),
        );
        state.set_occupancy(
            CellId::new(RowId::R2, ColumnId::C4),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1),
        );
        state.set_occupancy(
            CellId::new(RowId::R3, ColumnId::C4),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1),
        );
        state.set_occupancy(
            CellId::new(RowId::R4, ColumnId::C1),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat0),
        );
        state.set_occupancy(
            CellId::new(RowId::R4, ColumnId::C2),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1),
        );
        state.set_occupancy(
            CellId::new(RowId::R4, ColumnId::C3),
            CellOccupancy::Occupied(DirectionalFlipSeat::Seat1),
        );

        let placement = legal_placements(&state, DirectionalFlipSeat::Seat0)
            .into_iter()
            .find(|placement| placement.cell == CellId::new(RowId::R4, ColumnId::C4))
            .expect("target is legal");
        let preview = placement_preview(&placement);

        assert_eq!(
            preview
                .direction_groups
                .iter()
                .map(|group| group.direction.as_str())
                .collect::<Vec<_>>(),
            vec!["north", "west"]
        );
        assert_eq!(
            preview
                .ordered_flips
                .iter()
                .map(|cell| cell.as_string())
                .collect::<Vec<_>>(),
            vec!["r3c4", "r2c4", "r4c3", "r4c2"]
        );
    }
}
