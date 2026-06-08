use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    ids::{DraftItemId, SecretDraftSeat},
    state::{Phase, SecretDraftState},
};

pub const COMMIT_SEGMENT_PREFIX: &str = "commit/";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretDraftAction {
    pub actor: SecretDraftSeat,
    pub item: DraftItemId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: SecretDraftSeat,
    pub item: DraftItemId,
}

pub fn legal_action_tree(state: &SecretDraftState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.seat_committed(actor_seat)
    {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        state
            .visible_pool
            .iter()
            .map(|item| commit_choice(state, actor_seat, *item))
            .collect(),
    )
}

pub fn legal_action_metadata(state: &SecretDraftState, actor: &Actor) -> Vec<ActionMetadata> {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return vec![metadata("action_status", "not_seated")];
    };
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return vec![metadata("action_status", "terminal")];
    }
    if state.seat_committed(actor_seat) {
        return vec![
            metadata("action_status", "pending"),
            metadata("actor_seat", actor_seat.as_str()),
            metadata("waiting_for", actor_seat.other().as_str()),
        ];
    }
    vec![
        metadata("action_status", "available"),
        metadata("actor_seat", actor_seat.as_str()),
    ]
}

pub fn actor_seat(state: &SecretDraftState, actor: &Actor) -> Option<SecretDraftSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(SecretDraftSeat::from_index)
}

pub fn commit_segment(item: DraftItemId) -> String {
    format!("{COMMIT_SEGMENT_PREFIX}{}", item.as_str())
}

pub fn parse_commit_segment(segment: &str) -> Option<DraftItemId> {
    DraftItemId::parse(segment.strip_prefix(COMMIT_SEGMENT_PREFIX)?)
}

pub fn validate_command(
    state: &SecretDraftState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(terminal_diagnostic());
    }
    if state.seat_committed(actor) {
        return Err(already_committed_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(malformed_action_diagnostic());
    };
    let item = parse_commit_segment(segment).ok_or_else(malformed_action_diagnostic)?;
    if !state.visible_pool.contains(&item) {
        return Err(unavailable_item_diagnostic());
    }

    Ok(ValidatedAction { actor, item })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "this actor is not seated in this secret_draft match".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "commit actions are not available after the match is complete".to_owned(),
    }
}

pub fn already_committed_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "already_committed".to_owned(),
        message: "this seat already has a hidden commitment pending for this round".to_owned(),
    }
}

pub fn unavailable_item_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "item_unavailable".to_owned(),
        message: "the requested item is unavailable for this commitment".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "commit actions require exactly one commit item segment".to_owned(),
    }
}

fn commit_choice(
    state: &SecretDraftState,
    actor: SecretDraftSeat,
    item: DraftItemId,
) -> ActionChoice {
    let mut choice = ActionChoice::leaf(
        commit_segment(item),
        format!("Commit {}", item.label()),
        format!("Commit {} as your hidden draft choice", item.label()),
    );
    choice.metadata = vec![
        metadata("action_family", "commit"),
        metadata("actor_seat", actor.as_str()),
        metadata("round_number", state.round_number.to_string()),
        metadata("priority_seat", state.priority_seat.as_str()),
        metadata("item_id", item.as_str()),
        metadata("item_label", item.label()),
        metadata("thread", item.thread().as_str()),
        metadata("value", item.value().to_string()),
        metadata("seat_0_score", state.scores[0].to_string()),
        metadata("seat_1_score", state.scores[1].to_string()),
        metadata("pending_warning", "another seat may already be pending"),
    ];
    choice.tags = vec!["hidden-commit".to_owned(), "draft-item".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
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
    use crate::{setup::SetupOptions, setup_match, state::TerminalOutcome};
    use engine_core::{ActionPath, FreshnessToken, RulesVersion, SeatId};

    fn standard_state() -> SecretDraftState {
        setup_match(
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn actor(seat: &str) -> Actor {
        Actor {
            seat_id: SeatId(seat.to_owned()),
        }
    }

    fn command(state: &SecretDraftState, seat: &str, segments: Vec<String>) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath { segments },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn uncommitted_actor_gets_visible_pool_choices() {
        let state = standard_state();
        let tree = legal_action_tree(&state, &actor("seat_0"));

        assert_eq!(tree.freshness_token, state.freshness_token);
        assert_eq!(tree.root.choices.len(), DraftItemId::ALL.len());
        assert_eq!(tree.root.choices[0].segment, "commit/ember_1");
        assert_eq!(tree.root.choices[11].segment, "commit/grove_4");
        assert!(tree.root.choices[0]
            .metadata
            .iter()
            .any(|entry| entry.key == "item_id" && entry.value == "ember_1"));
        assert!(tree.root.choices[0]
            .metadata
            .iter()
            .any(|entry| entry.key == "pending_warning"));
    }

    #[test]
    fn committed_actor_gets_empty_tree_and_pending_metadata() {
        let mut state = standard_state();
        state.set_commitment_for_internal(SecretDraftSeat::Seat0, DraftItemId::Ember4);

        let tree = legal_action_tree(&state, &actor("seat_0"));
        let metadata = legal_action_metadata(&state, &actor("seat_0"));

        assert!(tree.root.choices.is_empty());
        assert!(metadata
            .iter()
            .any(|entry| entry.key == "action_status" && entry.value == "pending"));
        assert!(!format!("{metadata:?}").contains("ember_4"));
    }

    #[test]
    fn terminal_actor_gets_empty_tree() {
        let mut state = standard_state();
        state.phase = Phase::Terminal;
        state.terminal_outcome = Some(TerminalOutcome::Draw);

        assert!(legal_action_tree(&state, &actor("seat_0"))
            .root
            .choices
            .is_empty());
        assert!(legal_action_metadata(&state, &actor("seat_0"))
            .iter()
            .any(|entry| entry.key == "action_status" && entry.value == "terminal"));
    }

    #[test]
    fn validation_accepts_current_visible_item() {
        let state = standard_state();
        let validated = validate_command(
            &state,
            &command(&state, "seat_1", vec!["commit/tide_3".to_owned()]),
        )
        .expect("valid command");

        assert_eq!(
            validated,
            ValidatedAction {
                actor: SecretDraftSeat::Seat1,
                item: DraftItemId::Tide3
            }
        );
    }

    #[test]
    fn validation_rejects_stale_wrong_actor_terminal_committed_malformed_extra_and_unavailable() {
        let mut state = standard_state();
        let mut stale = command(&state, "seat_0", vec!["commit/ember_1".to_owned()]);
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).unwrap_err().code,
            "stale_action"
        );

        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_9", vec!["commit/ember_1".to_owned()])
            )
            .unwrap_err()
            .code,
            "wrong_seat"
        );

        state.phase = Phase::Terminal;
        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["commit/ember_1".to_owned()])
            )
            .unwrap_err()
            .code,
            "terminal_state"
        );

        let mut state = standard_state();
        state.set_commitment_for_internal(SecretDraftSeat::Seat0, DraftItemId::Ember1);
        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["commit/ember_2".to_owned()])
            )
            .unwrap_err()
            .code,
            "already_committed"
        );

        let state = standard_state();
        assert_eq!(
            validate_command(
                &state,
                &command(&state, "seat_0", vec!["bad/ember_1".to_owned()])
            )
            .unwrap_err()
            .code,
            "malformed_action"
        );
        assert_eq!(
            validate_command(
                &state,
                &command(
                    &state,
                    "seat_0",
                    vec!["commit/ember_1".to_owned(), "extra".to_owned()]
                )
            )
            .unwrap_err()
            .code,
            "malformed_action"
        );

        let mut state = standard_state();
        state
            .visible_pool
            .retain(|item| *item != DraftItemId::Grove4);
        let diagnostic = validate_command(
            &state,
            &command(&state, "seat_0", vec!["commit/grove_4".to_owned()]),
        )
        .unwrap_err();
        assert_eq!(diagnostic.code, "item_unavailable");
        assert!(!diagnostic.message.contains("opponent"));
        assert!(!diagnostic.message.contains("already chose"));
        assert!(!diagnostic.message.contains("grove_4"));
    }
}
