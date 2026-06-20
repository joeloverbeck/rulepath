use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    cards::CardId,
    effects::{BriarCircuitEffect, PassCommitmentStatus},
    ids::{
        BriarCircuitSeat, ACTION_PASS, ACTION_PASS_CONFIRM, ACTION_PASS_SELECT,
        ACTION_PASS_UNSELECT, RULES_VERSION_LABEL, STANDARD_PASS_SIZE,
    },
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
