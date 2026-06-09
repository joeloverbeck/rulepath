use engine_core::{
    ActionChoice, ActionMetadata, ActionNode, ActionPreview, ActionTree, Actor, CommandEnvelope,
    Diagnostic,
};

use crate::{
    ids::{PlainTricksSeat, TrickCardId, ACTION_PLAY},
    state::{Phase, PlainTricksState},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlainTricksAction {
    pub card: TrickCardId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: PlainTricksSeat,
    pub card: TrickCardId,
    pub round_index: u8,
    pub trick_index: u8,
}

pub fn legal_action_tree(state: &PlainTricksState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor_seat)
    {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let legal_cards = legal_cards(state, actor_seat);
    if legal_cards.is_empty() {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut play_choice = ActionChoice::leaf(ACTION_PLAY, "Play", "Play a card");
    play_choice.metadata = public_trick_metadata(state, actor_seat);
    play_choice.tags = vec!["play".to_owned(), "card-choice".to_owned()];
    play_choice.preview = ActionPreview::Available;
    play_choice.next = Some(Box::new(ActionNode {
        choices: legal_cards
            .into_iter()
            .map(|card| card_choice(state, actor_seat, card))
            .collect(),
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![play_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn legal_cards(state: &PlainTricksState, actor: PlainTricksSeat) -> Vec<TrickCardId> {
    if state.phase == Phase::Terminal
        || state.terminal_outcome.is_some()
        || state.active_seat != Some(actor)
    {
        return Vec::new();
    }

    let hand = state.hand_for_internal(actor);
    if state.current_trick.plays.is_empty() {
        return hand.to_vec();
    }

    let Some(led_suit) = state.current_trick.led_suit else {
        return Vec::new();
    };
    let suited = hand
        .iter()
        .copied()
        .filter(|card| card.suit() == led_suit)
        .collect::<Vec<_>>();
    if suited.is_empty() {
        hand.to_vec()
    } else {
        suited
    }
}

pub fn actor_seat(state: &PlainTricksState, actor: &Actor) -> Option<PlainTricksSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(PlainTricksSeat::from_index)
}

pub fn parse_action_path(segments: &[String]) -> Option<PlainTricksAction> {
    let [family, card] = segments else {
        return None;
    };
    if family != ACTION_PLAY {
        return None;
    }
    TrickCardId::parse(card).map(|card| PlainTricksAction { card })
}

pub fn validate_command(
    state: &PlainTricksState,
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

    let action =
        parse_action_path(&command.action_path.segments).ok_or_else(malformed_action_diagnostic)?;

    if !state.hand_for_internal(actor).contains(&action.card) {
        return Err(not_in_hand_diagnostic(action.card));
    }

    if must_follow_suit(state, actor) && action.card.suit() != state.current_trick.led_suit.unwrap()
    {
        return Err(must_follow_suit_diagnostic());
    }

    if !legal_cards(state, actor).contains(&action.card) {
        return Err(unavailable_action_diagnostic());
    }

    Ok(ValidatedAction {
        actor,
        card: action.card,
        round_index: state.round_index,
        trick_index: state.trick_index,
    })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "only the active seated actor may play a Plain Tricks card".to_owned(),
    }
}

pub fn terminal_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "terminal_state".to_owned(),
        message: "cards cannot be played after the match is complete".to_owned(),
    }
}

pub fn malformed_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "malformed_action".to_owned(),
        message: "Plain Tricks actions require a play segment and one recognized card segment"
            .to_owned(),
    }
}

pub fn not_in_hand_diagnostic(card: TrickCardId) -> Diagnostic {
    Diagnostic {
        code: "card_not_in_hand".to_owned(),
        message: format!(
            "the submitted card `{}` is not in the actor's hand",
            card.as_str()
        ),
    }
}

pub fn must_follow_suit_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "must_follow_suit".to_owned(),
        message: "a card of the led suit must be played".to_owned(),
    }
}

pub fn unavailable_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "action_unavailable".to_owned(),
        message: "the requested card play is not available at this decision point".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

fn must_follow_suit(state: &PlainTricksState, actor: PlainTricksSeat) -> bool {
    let Some(led_suit) = state.current_trick.led_suit else {
        return false;
    };
    !state.current_trick.plays.is_empty()
        && state
            .hand_for_internal(actor)
            .iter()
            .any(|card| card.suit() == led_suit)
}

fn card_choice(
    state: &PlainTricksState,
    actor: PlainTricksSeat,
    card: TrickCardId,
) -> ActionChoice {
    let label = card.label();
    let accessibility_label = format!("Play {label}");
    let mut choice = ActionChoice::leaf(card.as_str(), label, accessibility_label.clone());
    choice.metadata = vec![
        metadata("action_family", ACTION_PLAY),
        metadata("round_index", state.round_index.to_string()),
        metadata("trick_index", state.trick_index.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("card_id", card.as_str()),
        metadata("card_suit", card.suit().as_str()),
        metadata("card_rank", card.rank().as_str()),
        metadata("card_label", card.label()),
        metadata("accessibility_copy", accessibility_label),
    ];
    choice.tags = vec![
        "play".to_owned(),
        "card".to_owned(),
        card.suit().as_str().to_owned(),
    ];
    choice.preview = ActionPreview::Available;
    choice
}

fn public_trick_metadata(state: &PlainTricksState, actor: PlainTricksSeat) -> Vec<ActionMetadata> {
    let mut metadata_values = vec![
        metadata("action_family", ACTION_PLAY),
        metadata("round_index", state.round_index.to_string()),
        metadata("trick_index", state.trick_index.to_string()),
        metadata("actor_seat", actor.as_str()),
        metadata("current_leader", state.current_leader.as_str()),
    ];
    if let Some(led_suit) = state.current_trick.led_suit {
        metadata_values.push(metadata("led_suit", led_suit.as_str()));
    }
    if let Some(lead_play) = state.current_trick.plays.first() {
        metadata_values.push(metadata("led_card", lead_play.card.as_str()));
    }
    metadata_values
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
        ids::TrickSuit,
        setup::{setup_match, SetupOptions},
        state::{CurrentTrick, TerminalOutcome, TrickCounts, TrickPlay},
    };
    use engine_core::{ActionPath, FreshnessToken, RulesVersion, SeatId, Seed};

    fn actor(seat: &str) -> Actor {
        Actor {
            seat_id: SeatId(seat.to_owned()),
        }
    }

    fn command(state: &PlainTricksState, seat: &str, segments: Vec<&str>) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn custom_state(
        active: PlainTricksSeat,
        hands: [Vec<TrickCardId>; 2],
        current_trick: CurrentTrick,
    ) -> PlainTricksState {
        let mut state = PlainTricksState::new_after_deal(
            crate::Variant::plain_tricks_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            0,
            PlainTricksSeat::Seat0,
            hands,
            Vec::new(),
        );
        state.active_seat = Some(active);
        state.current_leader = PlainTricksSeat::Seat0;
        state.current_trick = current_trick;
        state
    }

    fn standard_state() -> PlainTricksState {
        setup_match(
            Seed(1),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds")
    }

    fn leaf_segments(tree: &ActionTree) -> Vec<String> {
        tree.root
            .choices
            .first()
            .and_then(|choice| choice.next.as_ref())
            .map(|node| {
                node.choices
                    .iter()
                    .map(|choice| choice.segment.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    #[test]
    fn leader_can_play_every_card_in_hand() {
        let state = custom_state(
            PlainTricksSeat::Seat0,
            [
                vec![TrickCardId::Gale1, TrickCardId::River2, TrickCardId::Ember3],
                vec![TrickCardId::Gale4],
            ],
            CurrentTrick::default(),
        );

        assert_eq!(
            leaf_segments(&legal_action_tree(&state, &actor("seat_0"))),
            vec!["gale_1", "river_2", "ember_3"]
        );
    }

    #[test]
    fn follower_must_follow_suit_when_holding_one_or_more_led_suit_cards() {
        let current_trick = CurrentTrick {
            led_suit: Some(TrickSuit::Gale),
            plays: vec![TrickPlay {
                seat: PlainTricksSeat::Seat0,
                card: TrickCardId::Gale6,
            }],
        };
        let state = custom_state(
            PlainTricksSeat::Seat1,
            [
                vec![TrickCardId::Gale6],
                vec![TrickCardId::River2, TrickCardId::Gale1, TrickCardId::Gale3],
            ],
            current_trick,
        );

        assert_eq!(
            leaf_segments(&legal_action_tree(&state, &actor("seat_1"))),
            vec!["gale_1", "gale_3"]
        );
    }

    #[test]
    fn void_follower_can_play_any_card() {
        let current_trick = CurrentTrick {
            led_suit: Some(TrickSuit::Gale),
            plays: vec![TrickPlay {
                seat: PlainTricksSeat::Seat0,
                card: TrickCardId::Gale6,
            }],
        };
        let state = custom_state(
            PlainTricksSeat::Seat1,
            [
                vec![TrickCardId::Gale6],
                vec![TrickCardId::River2, TrickCardId::Ember1],
            ],
            current_trick,
        );

        assert_eq!(
            leaf_segments(&legal_action_tree(&state, &actor("seat_1"))),
            vec!["river_2", "ember_1"]
        );
    }

    #[test]
    fn non_actor_viewer_receives_empty_tree() {
        let state = standard_state();

        assert!(legal_action_tree(&state, &actor("seat_1"))
            .root
            .choices
            .is_empty());
        assert!(legal_action_tree(&state, &actor("seat_x"))
            .root
            .choices
            .is_empty());
    }

    #[test]
    fn validate_accepts_legal_play_path() {
        let state = custom_state(
            PlainTricksSeat::Seat0,
            [
                vec![TrickCardId::Gale1, TrickCardId::River2],
                vec![TrickCardId::Ember3],
            ],
            CurrentTrick::default(),
        );

        let validated =
            validate_command(&state, &command(&state, "seat_0", vec!["play", "river_2"]))
                .expect("play validates");

        assert_eq!(validated.actor, PlainTricksSeat::Seat0);
        assert_eq!(validated.card, TrickCardId::River2);
        assert_eq!(validated.round_index, 0);
        assert_eq!(validated.trick_index, 0);
    }

    #[test]
    fn validate_rejects_not_in_hand_and_must_follow_without_leaking_other_cards() {
        let current_trick = CurrentTrick {
            led_suit: Some(TrickSuit::Gale),
            plays: vec![TrickPlay {
                seat: PlainTricksSeat::Seat0,
                card: TrickCardId::Gale6,
            }],
        };
        let state = custom_state(
            PlainTricksSeat::Seat1,
            [
                vec![TrickCardId::Gale6],
                vec![TrickCardId::River2, TrickCardId::Gale1],
            ],
            current_trick,
        );

        let not_in_hand =
            validate_command(&state, &command(&state, "seat_1", vec!["play", "ember_1"]))
                .expect_err("card is not in hand");
        assert_eq!(not_in_hand.code, "card_not_in_hand");
        assert!(not_in_hand.message.contains("ember_1"));
        assert!(!not_in_hand.message.contains("gale_1"));

        let must_follow =
            validate_command(&state, &command(&state, "seat_1", vec!["play", "river_2"]))
                .expect_err("must follow suit");
        assert_eq!(must_follow.code, "must_follow_suit");
        assert_eq!(must_follow.message, "a card of the led suit must be played");
        assert!(!must_follow.message.contains("gale_1"));
    }

    #[test]
    fn validation_diagnostics_cover_stale_wrong_terminal_and_malformed() {
        let mut state = standard_state();

        let mut stale = command(&state, "seat_0", vec!["play", "gale_1"]);
        stale.freshness_token = FreshnessToken(99);
        assert_eq!(
            validate_command(&state, &stale).expect_err("stale").code,
            "stale_action"
        );

        assert_eq!(
            validate_command(&state, &command(&state, "seat_1", vec!["play", "gale_1"]))
                .expect_err("wrong seat")
                .code,
            "wrong_seat"
        );

        assert_eq!(
            validate_command(&state, &command(&state, "seat_0", vec!["bad"]))
                .expect_err("malformed")
                .code,
            "malformed_action"
        );

        state.phase = Phase::Terminal;
        state.terminal_outcome = Some(TerminalOutcome::Split {
            each: 6,
            totals: TrickCounts {
                seat_0: 6,
                seat_1: 6,
            },
        });
        assert_eq!(
            validate_command(&state, &command(&state, "seat_0", vec!["play", "gale_1"]))
                .expect_err("terminal")
                .code,
            "terminal_state"
        );
    }
}
