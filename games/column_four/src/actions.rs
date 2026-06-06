use engine_core::{ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor};

use crate::{
    ids::{ColumnFourSeat, ColumnId},
    rules::legal_columns,
    state::ColumnFourState,
};

pub const ACTION_SEGMENT_PREFIX: &str = "drop/";

pub fn legal_action_tree(state: &ColumnFourState, actor: &Actor) -> ActionTree {
    if state.terminal_outcome.is_some() || actor_seat(state, actor) != Some(state.active_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_columns(state)
            .into_iter()
            .map(action_choice)
            .collect(),
    )
}

pub fn actor_seat(state: &ColumnFourState, actor: &Actor) -> Option<ColumnFourSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(ColumnFourSeat::from_index)
}

pub fn parse_drop_segment(segment: &str) -> Option<ColumnId> {
    ColumnId::parse(segment.strip_prefix(ACTION_SEGMENT_PREFIX)?)
}

fn action_choice(column: ColumnId) -> ActionChoice {
    let column_id = column.as_str();
    let mut choice = ActionChoice::leaf(
        format!("{ACTION_SEGMENT_PREFIX}{column_id}"),
        format!("Column {}", column.index() + 1),
        format!("Drop a piece in column {}", column.index() + 1),
    );
    choice.metadata = vec![ActionMetadata {
        key: "column".to_owned(),
        value: column_id.to_owned(),
    }];
    choice.tags = vec![
        "flat".to_owned(),
        "placement".to_owned(),
        "column".to_owned(),
    ];
    choice.preview = ActionPreview::Available;
    choice
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rules::apply_action, setup::setup_match, ValidatedAction};
    use engine_core::{
        ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed,
    };

    fn state() -> ColumnFourState {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        setup_match(Seed(1), &seats, &Default::default()).expect("setup succeeds")
    }

    fn command(
        seat_index: usize,
        segment: &str,
        freshness_token: FreshnessToken,
    ) -> CommandEnvelope {
        CommandEnvelope {
            actor: Actor {
                seat_id: SeatId(format!("seat-{seat_index}")),
            },
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn legal_action_tree_lists_only_non_full_columns_for_active_actor() {
        let mut state = state();
        for _ in 0..6 {
            let actor = state.active_seat;
            apply_action(
                &mut state,
                ValidatedAction {
                    actor,
                    column: ColumnId::C1,
                },
            );
        }

        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[state.active_seat.index()].clone(),
            },
        );
        let segments = tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>();

        assert_eq!(segments.len(), 6);
        assert!(!segments.contains(&"drop/c1"));
        assert!(segments.contains(&"drop/c2"));
        assert!(segments.contains(&"drop/c7"));
    }

    #[test]
    fn legal_action_tree_is_empty_for_non_active_actor() {
        let state = state();
        let tree = legal_action_tree(
            &state,
            &Actor {
                seat_id: state.seats[1].clone(),
            },
        );

        assert!(tree.root.choices.is_empty());
    }

    #[test]
    fn diagnostics_cover_invalid_submissions() {
        let mut state = state();

        assert_eq!(
            crate::rules::validate_command(&state, &command(0, "drop/c1", FreshnessToken(99)))
                .expect_err("stale command")
                .code,
            "stale_action"
        );
        assert_eq!(
            crate::rules::validate_command(&state, &command(1, "drop/c1", state.freshness_token))
                .expect_err("wrong actor")
                .code,
            "not_active_seat"
        );
        assert_eq!(
            crate::rules::validate_command(
                &state,
                &CommandEnvelope {
                    actor: Actor {
                        seat_id: state.seats[0].clone(),
                    },
                    action_path: ActionPath {
                        segments: vec!["drop/c1".to_owned(), "extra".to_owned()],
                    },
                    freshness_token: state.freshness_token,
                    rules_version: RulesVersion(1),
                },
            )
            .expect_err("invalid path")
            .code,
            "invalid_action_path"
        );
        assert_eq!(
            crate::rules::validate_command(&state, &command(0, "drop/c8", state.freshness_token))
                .expect_err("unknown column")
                .code,
            "unknown_column"
        );

        for _ in 0..6 {
            let actor = state.active_seat;
            apply_action(
                &mut state,
                ValidatedAction {
                    actor,
                    column: ColumnId::C1,
                },
            );
        }
        assert_eq!(
            crate::rules::validate_command(
                &state,
                &command(state.active_seat.index(), "drop/c1", state.freshness_token),
            )
            .expect_err("full column")
            .code,
            "full_column"
        );

        state.terminal_outcome = Some(crate::state::TerminalOutcome::Draw);
        assert_eq!(
            crate::rules::validate_command(
                &state,
                &command(state.active_seat.index(), "drop/c2", state.freshness_token),
            )
            .expect_err("terminal match")
            .code,
            "terminal_match"
        );
    }
}
