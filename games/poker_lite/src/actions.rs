use engine_core::{
    ActionChoice, ActionMetadata, ActionPreview, ActionTree, Actor, CommandEnvelope, Diagnostic,
};

use crate::{
    ids::{PokerLiteSeat, ACTION_HOLD, ACTION_LIFT, ACTION_MATCH, ACTION_PRESS, ACTION_YIELD},
    state::{Phase, PokerLiteState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PokerLiteAction {
    Hold,
    Press,
    Lift,
    Match,
    Yield,
}

impl PokerLiteAction {
    pub const fn segment(self) -> &'static str {
        match self {
            Self::Hold => ACTION_HOLD,
            Self::Press => ACTION_PRESS,
            Self::Lift => ACTION_LIFT,
            Self::Match => ACTION_MATCH,
            Self::Yield => ACTION_YIELD,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Hold => "Hold",
            Self::Press => "Press",
            Self::Lift => "Lift",
            Self::Match => "Match",
            Self::Yield => "Yield",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: PokerLiteSeat,
    pub action: PokerLiteAction,
    pub round_index: u8,
    pub round_unit: u8,
    pub required_to_match: u8,
    pub adds_to_pool: u8,
}

pub fn legal_action_tree(state: &PokerLiteState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor_seat)
    {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    ActionTree::flat(
        state.freshness_token,
        legal_actions(state, actor_seat)
            .into_iter()
            .map(|action| action_choice(state, actor_seat, action))
            .collect(),
    )
}

pub fn legal_actions(state: &PokerLiteState, actor: PokerLiteSeat) -> Vec<PokerLiteAction> {
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor)
    {
        return Vec::new();
    }

    if state.round.outstanding_actor == Some(actor) {
        let mut actions = Vec::with_capacity(3);
        if !state.round.lift_used {
            actions.push(PokerLiteAction::Lift);
        }
        actions.push(PokerLiteAction::Match);
        actions.push(PokerLiteAction::Yield);
        actions
    } else if state.round.outstanding_actor.is_none() {
        vec![PokerLiteAction::Hold, PokerLiteAction::Press]
    } else {
        Vec::new()
    }
}

pub fn actor_seat(state: &PokerLiteState, actor: &Actor) -> Option<PokerLiteSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(PokerLiteSeat::from_index)
}

pub fn parse_action_segment(segment: &str) -> Option<PokerLiteAction> {
    match segment {
        ACTION_HOLD => Some(PokerLiteAction::Hold),
        ACTION_PRESS => Some(PokerLiteAction::Press),
        ACTION_LIFT => Some(PokerLiteAction::Lift),
        ACTION_MATCH => Some(PokerLiteAction::Match),
        ACTION_YIELD => Some(PokerLiteAction::Yield),
        _ => None,
    }
}

pub fn validate_command(
    state: &PokerLiteState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(terminal_diagnostic());
    }
    if state.active_seat != Some(actor) {
        return Err(wrong_seat_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(malformed_action_diagnostic());
    };
    let action = parse_action_segment(segment).ok_or_else(malformed_action_diagnostic)?;

    if action == PokerLiteAction::Lift
        && state.round.outstanding_actor == Some(actor)
        && state.round.lift_used
    {
        return Err(lift_cap_diagnostic());
    }

    if !legal_actions(state, actor).contains(&action) {
        return Err(unavailable_action_diagnostic());
    }

    let required_to_match = match action {
        PokerLiteAction::Hold | PokerLiteAction::Press => 0,
        PokerLiteAction::Lift | PokerLiteAction::Match | PokerLiteAction::Yield => {
            state.round.outstanding_amount
        }
    };
    let adds_to_pool = match action {
        PokerLiteAction::Hold | PokerLiteAction::Yield => 0,
        PokerLiteAction::Press => state.round.unit,
        PokerLiteAction::Lift => state.round.outstanding_amount + state.round.unit,
        PokerLiteAction::Match => state.round.outstanding_amount,
    };

    Ok(ValidatedAction {
        actor,
        action,
        round_index: state.round.round_index,
        round_unit: state.round.unit,
        required_to_match,
        adds_to_pool,
    })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "only the active seated actor may choose a Crest Ledger action".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "pledge actions are not available after the match is complete".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "Crest Ledger actions require exactly one recognized action segment".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "the requested pledge action is not available at this decision point".to_owned(),
    }
}

pub fn lift_cap_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "lift_cap_exceeded".to_owned(),
        message: "the current pledge round has already used its lift".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

fn action_choice(
    state: &PokerLiteState,
    actor: PokerLiteSeat,
    action: PokerLiteAction,
) -> ActionChoice {
    let required_to_match = match action {
        PokerLiteAction::Hold | PokerLiteAction::Press => 0,
        PokerLiteAction::Lift | PokerLiteAction::Match | PokerLiteAction::Yield => {
            state.round.outstanding_amount
        }
    };
    let adds_to_pool = match action {
        PokerLiteAction::Hold | PokerLiteAction::Yield => 0,
        PokerLiteAction::Press => state.round.unit,
        PokerLiteAction::Lift => state.round.outstanding_amount + state.round.unit,
        PokerLiteAction::Match => state.round.outstanding_amount,
    };
    let accessibility_copy = accessibility_copy(action, adds_to_pool, required_to_match);
    let mut choice = ActionChoice::leaf(action.segment(), action.label(), &accessibility_copy);
    choice.metadata = vec![
        metadata("action_family", action.segment()),
        metadata("round_index", state.round.round_index.to_string()),
        metadata("round_unit", state.round.unit.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("required_to_match", required_to_match.to_string()),
        metadata("adds_to_pool", adds_to_pool.to_string()),
        metadata(
            "shared_pool_after",
            state.shared_pool.saturating_add(adds_to_pool).to_string(),
        ),
        metadata(
            "lift_cap_remaining",
            if state.round.lift_used { "0" } else { "1" },
        ),
        metadata("center_visible", state.center_visible.to_string()),
        metadata("accessibility_copy", accessibility_copy),
    ];
    choice.tags = vec!["pledge".to_owned(), action.segment().to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn accessibility_copy(action: PokerLiteAction, adds_to_pool: u8, required_to_match: u8) -> String {
    match action {
        PokerLiteAction::Hold => "Hold without adding markers".to_owned(),
        PokerLiteAction::Press => format!("Press by adding {adds_to_pool} marker"),
        PokerLiteAction::Lift => format!(
            "Lift by matching {required_to_match} and adding {} marker",
            adds_to_pool.saturating_sub(required_to_match)
        ),
        PokerLiteAction::Match => format!("Match by adding {required_to_match} marker"),
        PokerLiteAction::Yield => "Yield the current shared pool".to_owned(),
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
    use crate::{setup::SetupOptions, setup_match};
    use engine_core::{ActionPath, FreshnessToken, RulesVersion, SeatId, Seed};

    fn standard_state() -> PokerLiteState {
        setup_match(
            Seed(1),
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

    fn command(state: &PokerLiteState, seat: &str, segment: &str) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath {
                segments: vec![segment.to_owned()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    #[test]
    fn no_outstanding_pledge_offers_hold_and_press_only() {
        let state = standard_state();
        let tree = legal_action_tree(&state, &actor("seat_0"));
        let segments = tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>();

        assert_eq!(segments, vec!["hold", "press"]);
        assert!(legal_action_tree(&state, &actor("seat_1"))
            .root
            .choices
            .is_empty());
    }

    #[test]
    fn facing_pledge_offers_lift_match_yield_until_lift_cap_used() {
        let mut state = standard_state();
        state.active_seat = Some(PokerLiteSeat::Seat1);
        state.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
        state.round.outstanding_amount = 1;

        let segments = legal_action_tree(&state, &actor("seat_1"))
            .root
            .choices
            .into_iter()
            .map(|choice| choice.segment)
            .collect::<Vec<_>>();
        assert_eq!(segments, vec!["lift", "match", "yield"]);

        state.round.lift_used = true;
        let segments = legal_action_tree(&state, &actor("seat_1"))
            .root
            .choices
            .into_iter()
            .map(|choice| choice.segment)
            .collect::<Vec<_>>();
        assert_eq!(segments, vec!["match", "yield"]);
    }

    #[test]
    fn validation_accepts_current_legal_actions_with_public_amounts() {
        let state = standard_state();
        let validated =
            validate_command(&state, &command(&state, "seat_0", "press")).expect("press is valid");
        assert_eq!(
            validated,
            ValidatedAction {
                actor: PokerLiteSeat::Seat0,
                action: PokerLiteAction::Press,
                round_index: 0,
                round_unit: 1,
                required_to_match: 0,
                adds_to_pool: 1,
            }
        );
    }

    #[test]
    fn action_metadata_is_public_allow_listed() {
        let state = standard_state();
        let tree = legal_action_tree(&state, &actor("seat_0"));
        let allowed = [
            "action_family",
            "round_index",
            "round_unit",
            "actor_seat",
            "required_to_match",
            "adds_to_pool",
            "shared_pool_after",
            "lift_cap_remaining",
            "center_visible",
            "accessibility_copy",
        ];

        for choice in &tree.root.choices {
            for entry in &choice.metadata {
                assert!(
                    allowed.contains(&entry.key.as_str()),
                    "unexpected metadata key {}",
                    entry.key
                );
            }
            let serialized = format!("{choice:?}");
            for forbidden in ["card", "rank", "deck", "hidden", "strength", "private"] {
                assert!(
                    !serialized.contains(forbidden),
                    "metadata leaked forbidden word {forbidden}: {serialized}"
                );
            }
        }
    }

    #[test]
    fn validation_rejects_stale_wrong_terminal_malformed_unavailable_and_lift_cap() {
        let state = standard_state();
        let mut stale = command(&state, "seat_0", "hold");
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).unwrap_err().code,
            "stale_action"
        );

        assert_eq!(
            validate_command(&state, &command(&state, "seat_9", "hold"))
                .unwrap_err()
                .code,
            "wrong_seat"
        );
        assert_eq!(
            validate_command(&state, &command(&state, "seat_1", "hold"))
                .unwrap_err()
                .code,
            "wrong_seat"
        );

        let mut terminal = standard_state();
        terminal.phase = Phase::Terminal;
        assert_eq!(
            validate_command(&terminal, &command(&terminal, "seat_0", "hold"))
                .unwrap_err()
                .code,
            "terminal_state"
        );

        let malformed = CommandEnvelope {
            actor: actor("seat_0"),
            action_path: ActionPath {
                segments: vec!["hold".to_owned(), "extra".to_owned()],
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        assert_eq!(
            validate_command(&state, &malformed).unwrap_err().code,
            "malformed_action"
        );
        assert_eq!(
            validate_command(&state, &command(&state, "seat_0", "bad"))
                .unwrap_err()
                .code,
            "malformed_action"
        );

        assert_eq!(
            validate_command(&state, &command(&state, "seat_0", "match"))
                .unwrap_err()
                .code,
            "action_unavailable"
        );

        let mut facing = standard_state();
        facing.active_seat = Some(PokerLiteSeat::Seat1);
        facing.round.outstanding_actor = Some(PokerLiteSeat::Seat1);
        facing.round.outstanding_amount = 1;
        facing.round.lift_used = true;
        let lift_cap = validate_command(&facing, &command(&facing, "seat_1", "lift")).unwrap_err();
        assert_eq!(lift_cap.code, "lift_cap_exceeded");
        let diagnostic_text = format!("{lift_cap:?}");
        for forbidden in ["low_", "middle_", "high_", "Sprout", "Current", "Crown"] {
            assert!(!diagnostic_text.contains(forbidden));
        }
    }
}
