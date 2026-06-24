use engine_core::{ActionChoice, ActionNode, ActionTree, Actor, CommandEnvelope, Diagnostic};

use crate::{
    bots::{legal_bot_actions, BriarCircuitBotAction},
    cards::CardId,
    effects::{BriarCircuitEffect, PassCommitmentStatus},
    ids::{
        BriarCircuitSeat, ACTION_PASS, ACTION_PASS_CONFIRM, ACTION_PASS_SELECT,
        ACTION_PASS_UNSELECT, ACTION_PLAY, RULES_VERSION_LABEL, STANDARD_PASS_SIZE,
    },
    rules::{apply_play_card, validate_play_card, PlayActionResult},
    state::BriarCircuitState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PassAction {
    Select(CardId),
    Unselect(CardId),
    Confirm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassActionResult {
    pub effects: Vec<BriarCircuitEffect>,
    pub exchange_completed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayAction {
    Play(CardId),
}

pub fn legal_action_tree(state: &BriarCircuitState, actor: &Actor) -> ActionTree {
    let Some(seat) = BriarCircuitSeat::parse(&actor.seat_id.0) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    let choices = legal_bot_actions(state, seat)
        .unwrap_or_default()
        .into_iter()
        .map(action_choice)
        .collect();
    ActionTree::flat(state.freshness_token, choices)
}

pub fn validate_pass_command(
    state: &BriarCircuitState,
    envelope: &CommandEnvelope,
) -> Result<(BriarCircuitSeat, PassAction), Diagnostic> {
    if envelope.rules_version.0 != 1 {
        return Err(diagnostic(
            "BC_WRONG_RULES_VERSION",
            "briar_circuit command used an unsupported rules version",
        ));
    }
    if envelope.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "BC_STALE_COMMAND",
            "briar_circuit command used a stale freshness token",
        ));
    }
    let seat = BriarCircuitSeat::parse(&envelope.actor.seat_id.0)
        .ok_or_else(|| diagnostic("BC_WRONG_SEAT", "actor is not a Briar Circuit seat"))?;
    let action = parse_pass_action_path(&envelope.action_path.segments)?;
    validate_pass_action(state, seat, action)?;
    Ok((seat, action))
}

pub fn validate_play_command(
    state: &BriarCircuitState,
    envelope: &CommandEnvelope,
) -> Result<(BriarCircuitSeat, PlayAction), Diagnostic> {
    if envelope.rules_version.0 != 1 {
        return Err(diagnostic(
            "BC_WRONG_RULES_VERSION",
            "briar_circuit command used an unsupported rules version",
        ));
    }
    if envelope.freshness_token != state.freshness_token {
        return Err(diagnostic(
            "BC_STALE_COMMAND",
            "briar_circuit command used a stale freshness token",
        ));
    }
    let seat = BriarCircuitSeat::parse(&envelope.actor.seat_id.0)
        .ok_or_else(|| diagnostic("BC_WRONG_SEAT", "actor is not a Briar Circuit seat"))?;
    let action = parse_play_action_path(&envelope.action_path.segments)?;
    let PlayAction::Play(card) = action;
    validate_play_card(state, seat, card)?;
    Ok((seat, action))
}

pub fn parse_pass_action_path(segments: &[String]) -> Result<PassAction, Diagnostic> {
    match segments {
        [family, action, card] if family == ACTION_PASS && action == ACTION_PASS_SELECT => {
            let card = CardId::parse(card)
                .ok_or_else(|| diagnostic("BC_UNKNOWN_CARD", "unknown pass card id"))?;
            Ok(PassAction::Select(card))
        }
        [family, action, card] if family == ACTION_PASS && action == ACTION_PASS_UNSELECT => {
            let card = CardId::parse(card)
                .ok_or_else(|| diagnostic("BC_UNKNOWN_CARD", "unknown pass card id"))?;
            Ok(PassAction::Unselect(card))
        }
        [family, action] if family == ACTION_PASS && action == ACTION_PASS_CONFIRM => {
            Ok(PassAction::Confirm)
        }
        _ => Err(diagnostic(
            "BC_WRONG_PHASE",
            "command is not a Briar Circuit pass action",
        )),
    }
}

pub fn parse_play_action_path(segments: &[String]) -> Result<PlayAction, Diagnostic> {
    match segments {
        [family, card] if family == ACTION_PLAY => {
            let card = CardId::parse(card)
                .ok_or_else(|| diagnostic("BC_UNKNOWN_CARD", "unknown play card id"))?;
            Ok(PlayAction::Play(card))
        }
        _ => Err(diagnostic(
            "BC_WRONG_PHASE",
            "command is not a Briar Circuit play action",
        )),
    }
}

pub fn apply_play_action(
    state: &mut BriarCircuitState,
    seat: BriarCircuitSeat,
    action: PlayAction,
) -> Result<PlayActionResult, Diagnostic> {
    match action {
        PlayAction::Play(card) => apply_play_card(state, seat, card),
    }
}

pub fn apply_pass_action(
    state: &mut BriarCircuitState,
    seat: BriarCircuitSeat,
    action: PassAction,
) -> Result<PassActionResult, Diagnostic> {
    validate_pass_action(state, seat, action)?;

    let mut effects = Vec::new();
    match action {
        PassAction::Select(card) => {
            let pass = state.pass_state_mut().expect("validated pass phase");
            pass.selection_for_mut(seat)
                .expect("selection row exists")
                .push(card);
            let selected_cards = pass.selection_for(seat).to_vec();
            effects.push(BriarCircuitEffect::PassSelectionUpdated {
                seat,
                selected_count: selected_cards.len(),
                selected_cards,
            });
            state.freshness_token.0 += 1;
            Ok(PassActionResult {
                effects,
                exchange_completed: false,
            })
        }
        PassAction::Unselect(card) => {
            let pass = state.pass_state_mut().expect("validated pass phase");
            let selection = pass.selection_for_mut(seat).expect("selection row exists");
            let index = selection
                .iter()
                .position(|selected| *selected == card)
                .ok_or_else(|| diagnostic("BC_CARD_NOT_SELECTED", "card is not selected"))?;
            selection.remove(index);
            let selected_cards = pass.selection_for(seat).to_vec();
            effects.push(BriarCircuitEffect::PassSelectionUpdated {
                seat,
                selected_count: selected_cards.len(),
                selected_cards,
            });
            state.freshness_token.0 += 1;
            Ok(PassActionResult {
                effects,
                exchange_completed: false,
            })
        }
        PassAction::Confirm => {
            {
                let pass = state.pass_state_mut().expect("validated pass phase");
                if !pass.committed.contains(&seat) {
                    pass.committed.push(seat);
                    pass.committed.sort();
                }
                effects.push(BriarCircuitEffect::PassCommitmentPublic(
                    PassCommitmentStatus {
                        direction: pass.direction,
                        committed_count: pass.committed_count(),
                        pending_count: pass.pending_count(),
                    },
                ));
            }
            state.freshness_token.0 += 1;

            if state
                .pass_state()
                .is_some_and(|pass| pass.committed_count() == BriarCircuitSeat::ALL.len())
            {
                let exchange_effects = apply_atomic_exchange(state)?;
                effects.extend(exchange_effects);
                state.enter_playing_with_two_clubs_leader();
                Ok(PassActionResult {
                    effects,
                    exchange_completed: true,
                })
            } else {
                Ok(PassActionResult {
                    effects,
                    exchange_completed: false,
                })
            }
        }
    }
}

fn action_choice(action: BriarCircuitBotAction) -> ActionChoice {
    let path = action_path(action);
    let label = action_label(action);
    let mut choice = ActionChoice::leaf(
        path.first().cloned().unwrap_or_default(),
        label.clone(),
        label,
    );
    choice.next = nested_action_node(&path[1..]);
    choice
}

fn nested_action_node(path: &[String]) -> Option<Box<ActionNode>> {
    let (segment, remaining) = path.split_first()?;
    let label = segment.replace('_', " ");
    let mut choice = ActionChoice::leaf(segment.clone(), label.clone(), label);
    choice.next = nested_action_node(remaining);
    Some(Box::new(ActionNode {
        choices: vec![choice],
    }))
}

fn action_path(action: BriarCircuitBotAction) -> Vec<String> {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            vec![
                ACTION_PASS.to_owned(),
                ACTION_PASS_SELECT.to_owned(),
                card.as_str(),
            ]
        }
        BriarCircuitBotAction::Pass(PassAction::Unselect(card)) => {
            vec![
                ACTION_PASS.to_owned(),
                ACTION_PASS_UNSELECT.to_owned(),
                card.as_str(),
            ]
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => {
            vec![ACTION_PASS.to_owned(), ACTION_PASS_CONFIRM.to_owned()]
        }
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            vec![ACTION_PLAY.to_owned(), card.as_str()]
        }
    }
}

fn action_label(action: BriarCircuitBotAction) -> String {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            format!("Select {}", card.as_str().replace('_', " "))
        }
        BriarCircuitBotAction::Pass(PassAction::Unselect(card)) => {
            format!("Unselect {}", card.as_str().replace('_', " "))
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => "Confirm pass".to_owned(),
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            format!("Play {}", card.as_str().replace('_', " "))
        }
    }
}

fn validate_pass_action(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    action: PassAction,
) -> Result<(), Diagnostic> {
    let pass = state.pass_state().ok_or_else(|| {
        diagnostic(
            "BC_WRONG_PHASE",
            "briar_circuit pass action is only legal during the pass phase",
        )
    })?;
    if pass.is_committed(seat) {
        return Err(diagnostic(
            "BC_PASS_ALREADY_COMMITTED",
            "pass selection has already been committed",
        ));
    }

    match action {
        PassAction::Select(card) => {
            if !state.hand_for_internal(seat).contains(&card) {
                return Err(diagnostic(
                    "BC_CARD_NOT_OWNED",
                    "selected pass card is not owned by the actor",
                ));
            }
            let selection = pass.selection_for(seat);
            if selection.contains(&card) {
                return Err(diagnostic(
                    "BC_PASS_DUPLICATE_CARD",
                    "pass selection cannot contain duplicate cards",
                ));
            }
            if selection.len() >= STANDARD_PASS_SIZE as usize {
                return Err(diagnostic(
                    "BC_PASS_REQUIRES_THREE",
                    "pass selection already contains three cards",
                ));
            }
        }
        PassAction::Unselect(card) => {
            if !pass.selection_for(seat).contains(&card) {
                return Err(diagnostic("BC_CARD_NOT_SELECTED", "card is not selected"));
            }
        }
        PassAction::Confirm => {
            if pass.selection_for(seat).len() != STANDARD_PASS_SIZE as usize {
                return Err(diagnostic(
                    "BC_PASS_REQUIRES_THREE",
                    "pass confirm requires exactly three selected cards",
                ));
            }
        }
    }
    Ok(())
}

fn apply_atomic_exchange(
    state: &mut BriarCircuitState,
) -> Result<Vec<BriarCircuitEffect>, Diagnostic> {
    let pass = state
        .pass_state()
        .cloned()
        .ok_or_else(|| diagnostic("BC_WRONG_PHASE", "pass exchange requires pass phase"))?;
    let mut received: Vec<(BriarCircuitSeat, Vec<CardId>)> = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| (seat, Vec::new()))
        .collect();

    for (source, cards) in &pass.selections {
        let target = pass.direction.target_for(*source);
        for card in cards {
            remove_card_from_hand(state, *source, *card)?;
            received
                .iter_mut()
                .find(|(seat, _)| *seat == target)
                .expect("target receive row exists")
                .1
                .push(*card);
        }
    }

    for (target, cards) in &received {
        state
            .hand_for_internal_mut(*target)
            .expect("target hand exists")
            .extend(cards.iter().copied());
        state
            .hand_for_internal_mut(*target)
            .expect("target hand exists")
            .sort();
    }

    let mut effects = vec![BriarCircuitEffect::PassExchangePublic {
        direction: pass.direction,
    }];
    for (seat, sent_cards) in pass.selections {
        let received_cards = received
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, cards)| cards.clone())
            .unwrap_or_default();
        effects.push(BriarCircuitEffect::PassExchangePrivate {
            seat,
            sent_cards,
            received_cards,
        });
    }
    Ok(effects)
}

fn remove_card_from_hand(
    state: &mut BriarCircuitState,
    seat: BriarCircuitSeat,
    card: CardId,
) -> Result<(), Diagnostic> {
    let hand = state
        .hand_for_internal_mut(seat)
        .ok_or_else(|| diagnostic("BC_WRONG_SEAT", "unknown seat hand"))?;
    let index = hand
        .iter()
        .position(|candidate| *candidate == card)
        .ok_or_else(|| diagnostic("BC_CARD_NOT_OWNED", "pass card is not owned by source seat"))?;
    hand.remove(index);
    Ok(())
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[allow(dead_code)]
fn _rules_version_label() -> &'static str {
    RULES_VERSION_LABEL
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{SeatId, Seed};

    use crate::{
        ids::canonical_seat_ids,
        rules::legal_play_cards,
        setup::{setup_match, SetupOptions},
    };

    #[derive(Debug, Eq, PartialEq)]
    struct BrowserChoiceParity {
        root_label: String,
        path: Vec<String>,
    }

    fn standard_state() -> BriarCircuitState {
        setup_match(Seed(1600), &canonical_seat_ids(), &SetupOptions::default()).expect("setup")
    }

    fn actor(seat: BriarCircuitSeat) -> Actor {
        Actor {
            seat_id: SeatId(seat.as_str().to_owned()),
        }
    }

    fn browser_choices(tree: &ActionTree) -> Vec<BrowserChoiceParity> {
        let mut choices = Vec::new();
        for choice in &tree.root.choices {
            collect_browser_choices(choice, choice.label.clone(), Vec::new(), &mut choices, true);
        }
        choices
    }

    fn collect_browser_choices(
        choice: &ActionChoice,
        root_label: String,
        mut path: Vec<String>,
        choices: &mut Vec<BrowserChoiceParity>,
        is_root: bool,
    ) {
        path.push(choice.segment.clone());
        if let Some(next) = &choice.next {
            if !is_root {
                assert_eq!(
                    choice.label,
                    choice.segment.replace('_', " "),
                    "nested choice labels must stay browser-compatible"
                );
            }
            for next_choice in &next.choices {
                collect_browser_choices(
                    next_choice,
                    root_label.clone(),
                    path.clone(),
                    choices,
                    false,
                );
            }
        } else {
            choices.push(BrowserChoiceParity { root_label, path });
        }
    }

    fn card_label(prefix: &str, card: CardId) -> String {
        format!("{prefix} {}", card.as_str().replace('_', " "))
    }

    fn commit_standard_pass(state: &mut BriarCircuitState, seat: BriarCircuitSeat) {
        let cards = state.hand_for_internal(seat)[..STANDARD_PASS_SIZE as usize].to_vec();
        for card in cards {
            apply_pass_action(state, seat, PassAction::Select(card)).expect("select pass card");
        }
        apply_pass_action(state, seat, PassAction::Confirm).expect("confirm pass");
    }

    #[test]
    fn legal_action_tree_matches_browser_pass_select_choices() {
        let state = standard_state();
        let seat = BriarCircuitSeat::Seat0;
        let expected = state
            .hand_for_internal(seat)
            .iter()
            .copied()
            .map(|card| BrowserChoiceParity {
                root_label: card_label("Select", card),
                path: vec![
                    ACTION_PASS.to_owned(),
                    ACTION_PASS_SELECT.to_owned(),
                    card.as_str(),
                ],
            })
            .collect::<Vec<_>>();

        let tree = legal_action_tree(&state, &actor(seat));

        assert_eq!(tree.freshness_token, state.freshness_token);
        assert_eq!(browser_choices(&tree), expected);
    }

    #[test]
    fn legal_action_tree_matches_browser_pass_confirm_choice() {
        let mut state = standard_state();
        let seat = BriarCircuitSeat::Seat0;
        let cards = state.hand_for_internal(seat)[..STANDARD_PASS_SIZE as usize].to_vec();
        for card in cards {
            apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select card");
        }

        let tree = legal_action_tree(&state, &actor(seat));

        assert_eq!(
            browser_choices(&tree),
            vec![BrowserChoiceParity {
                root_label: "Confirm pass".to_owned(),
                path: vec![ACTION_PASS.to_owned(), ACTION_PASS_CONFIRM.to_owned()],
            }]
        );
        assert!(!browser_choices(&tree).iter().any(|choice| choice
            .path
            .iter()
            .any(|segment| segment == ACTION_PASS_UNSELECT)));
    }

    #[test]
    fn legal_action_tree_matches_browser_play_choices() {
        let mut state = standard_state();
        for seat in BriarCircuitSeat::ALL {
            commit_standard_pass(&mut state, seat);
        }
        let active = state.playing_state().expect("playing phase").active_seat;
        let expected = legal_play_cards(&state, active)
            .expect("legal play cards")
            .into_iter()
            .map(|card| BrowserChoiceParity {
                root_label: card_label("Play", card),
                path: vec![ACTION_PLAY.to_owned(), card.as_str()],
            })
            .collect::<Vec<_>>();

        let tree = legal_action_tree(&state, &actor(active));

        assert_eq!(browser_choices(&tree), expected);
    }

    #[test]
    fn legal_action_tree_rejects_unknown_actor_without_choices() {
        let state = standard_state();
        let actor = Actor {
            seat_id: SeatId("seat_99".to_owned()),
        };

        let tree = legal_action_tree(&state, &actor);

        assert_eq!(tree.freshness_token, state.freshness_token);
        assert!(tree.root.choices.is_empty());
    }
}
