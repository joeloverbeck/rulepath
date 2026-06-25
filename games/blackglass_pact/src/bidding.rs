use engine_core::{ActionChoice, ActionNode, ActionPreview, ActionTree, Actor, Diagnostic};

use crate::{
    effects::BlackglassPactEffect,
    ids::BlackglassSeat,
    partnerships::team_for_seat,
    setup::{complete_blind_nil_and_deal, BLIND_NIL_DEFICIT_THRESHOLD},
    state::{Bid, BlackglassPactState, BlindNilChoice, Phase},
};

pub const ACTION_BLIND_NIL: &str = "blind_nil";
pub const ACTION_BLIND_DECLARE: &str = "declare";
pub const ACTION_BLIND_DECLINE: &str = "decline";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BlindNilAction {
    pub choice: BlindNilChoice,
}

pub fn legal_action_tree(state: &BlackglassPactState, actor: &Actor) -> ActionTree {
    let Some(actor_seat) = actor_seat(state, actor) else {
        return ActionTree::flat(state.freshness_token, Vec::new());
    };
    if state.active_blind_nil_seat() != Some(actor_seat) {
        return ActionTree::flat(state.freshness_token, Vec::new());
    }

    let mut blind_choice =
        ActionChoice::leaf(ACTION_BLIND_NIL, "Blind nil", "Choose blind nil commitment");
    blind_choice.tags = vec!["blind-nil".to_owned(), "pre-deal".to_owned()];
    blind_choice.preview = ActionPreview::Available;
    blind_choice.next = Some(Box::new(ActionNode {
        choices: vec![
            blind_choice_leaf(
                ACTION_BLIND_DECLARE,
                "Declare",
                "Declare blind nil before the deal",
            ),
            blind_choice_leaf(
                ACTION_BLIND_DECLINE,
                "Decline",
                "Decline blind nil before the deal",
            ),
        ],
    }));

    ActionTree {
        root: ActionNode {
            choices: vec![blind_choice],
        },
        freshness_token: state.freshness_token,
    }
}

pub fn parse_blind_nil_action_path(segments: &[String]) -> Result<BlindNilAction, Diagnostic> {
    match segments {
        [family, leaf] if family == ACTION_BLIND_NIL && leaf == ACTION_BLIND_DECLARE => {
            Ok(BlindNilAction {
                choice: BlindNilChoice::Declared,
            })
        }
        [family, leaf] if family == ACTION_BLIND_NIL && leaf == ACTION_BLIND_DECLINE => {
            Ok(BlindNilAction {
                choice: BlindNilChoice::Declined,
            })
        }
        _ => Err(invalid_blind_nil_action_diagnostic()),
    }
}

pub fn apply_blind_nil_action(
    state: &mut BlackglassPactState,
    actor: &Actor,
    action: BlindNilAction,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    let actor_seat = actor_seat(state, actor).ok_or_else(wrong_actor_diagnostic)?;
    apply_blind_nil_choice(state, actor_seat, action.choice)
}

pub fn apply_blind_nil_choice(
    state: &mut BlackglassPactState,
    seat: BlackglassSeat,
    choice: BlindNilChoice,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    if state.active_blind_nil_seat() != Some(seat) {
        return Err(wrong_blind_nil_seat_diagnostic());
    }
    if state.blind_nil_choice_for(seat).is_some() {
        return Err(blind_nil_already_resolved_diagnostic());
    }

    state.blind_nil_choices[seat.index()] = Some(choice);
    let mut effects = match choice {
        BlindNilChoice::Declared => {
            state.bids[seat.index()] = Some(Bid::BlindNil);
            vec![BlackglassPactEffect::BlindNilDeclared {
                seat,
                team: team_for_seat(seat),
            }]
        }
        BlindNilChoice::Declined => vec![BlackglassPactEffect::BlindNilDeclined { seat }],
    };

    advance_blind_nil_cursor(state)?;
    if state.active_blind_nil_seat().is_none() {
        complete_blind_nil_and_deal(state)?;
        effects.extend(deal_effects(state));
    } else {
        state.advance_freshness();
    }

    Ok(effects)
}

pub fn opening_blind_nil_effect(state: &BlackglassPactState) -> Option<BlackglassPactEffect> {
    match &state.phase {
        Phase::BlindNilCommitment { pending, .. } if !pending.is_empty() => {
            Some(BlackglassPactEffect::BlindNilWindowOpened {
                pending: pending.clone(),
                threshold: BLIND_NIL_DEFICIT_THRESHOLD,
            })
        }
        _ => None,
    }
}

pub fn actor_seat(state: &BlackglassPactState, actor: &Actor) -> Option<BlackglassSeat> {
    state
        .seats
        .iter()
        .position(|seat_id| seat_id == &actor.seat_id)
        .and_then(BlackglassSeat::from_index)
}

fn advance_blind_nil_cursor(state: &mut BlackglassPactState) -> Result<(), Diagnostic> {
    match &mut state.phase {
        Phase::BlindNilCommitment {
            pending,
            next_index,
        } => {
            if *next_index >= pending.len() {
                return Err(blind_nil_already_resolved_diagnostic());
            }
            *next_index += 1;
            Ok(())
        }
        _ => Err(wrong_phase_diagnostic()),
    }
}

fn deal_effects(state: &BlackglassPactState) -> Vec<BlackglassPactEffect> {
    let next_bidder = match state.phase {
        Phase::Bidding { next, .. } => next,
        _ => state.dealer.next_clockwise(),
    };
    let mut effects = vec![BlackglassPactEffect::DealCompleted {
        dealer: state.dealer,
        hand_index: state.hand_index,
        counts: state
            .private_hands
            .iter()
            .map(|(seat, hand)| (*seat, hand.len()))
            .collect(),
        next_bidder,
    }];
    effects.extend(state.private_hands.iter().map(|(seat, cards)| {
        BlackglassPactEffect::PrivateHandReceived {
            seat: *seat,
            cards: cards.clone(),
        }
    }));
    effects
}

fn blind_choice_leaf(segment: &str, label: &str, accessibility_label: &str) -> ActionChoice {
    let mut choice = ActionChoice::leaf(segment, label, accessibility_label);
    choice.tags = vec!["blind-nil".to_owned(), "pre-deal".to_owned()];
    choice.preview = ActionPreview::Available;
    choice
}

fn invalid_blind_nil_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_INVALID_BLIND_NIL_ACTION".to_owned(),
        message: "blind nil action must be blind_nil/declare or blind_nil/decline".to_owned(),
    }
}

fn wrong_actor_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_UNKNOWN_ACTOR".to_owned(),
        message: "actor does not map to a blackglass_pact seat".to_owned(),
    }
}

fn wrong_blind_nil_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_BLIND_NIL_SEAT".to_owned(),
        message: "only the active eligible seat may resolve blind nil".to_owned(),
    }
}

fn blind_nil_already_resolved_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_BLIND_NIL_ALREADY_RESOLVED".to_owned(),
        message: "blind nil commitment has already been resolved for this seat".to_owned(),
    }
}

fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_PHASE".to_owned(),
        message: "blind nil action is only legal during blind nil commitment".to_owned(),
    }
}
