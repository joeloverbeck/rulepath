use engine_core::{CommandEnvelope, Diagnostic, EffectEnvelope};

use crate::{
    actions::{active_commit_seat, actor_seat, parse_commit_segment},
    effects::{
        cards_revealed_effect, commit_face_down_effect, deal_private_card_effect,
        hand_count_changed_effect, own_commit_confirmed_effect, refill_started_effect,
        round_scored_effect, terminal_effect, HighCardDuelEffect,
    },
    ids::{CardId, HighCardDuelSeat},
    state::{HighCardDuelState, Phase, RevealedRound, Score, TerminalOutcome},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedAction {
    pub actor: HighCardDuelSeat,
    pub card: CardId,
}

pub fn validate_command(
    state: &HighCardDuelState,
    command: &CommandEnvelope,
) -> Result<ValidatedAction, Diagnostic> {
    if command.freshness_token != state.freshness_token {
        return Err(stale_action_diagnostic());
    }

    let actor = actor_seat(state, &command.actor).ok_or_else(wrong_seat_diagnostic)?;
    let Some(active_seat) = active_commit_seat(state) else {
        return Err(wrong_phase_diagnostic(state.phase));
    };
    if actor != active_seat {
        return Err(wrong_seat_diagnostic());
    }
    if state.commitment_for(actor).is_some() {
        return Err(commitment_conflict_diagnostic());
    }

    let [segment] = command.action_path.segments.as_slice() else {
        return Err(wrong_phase_diagnostic(state.phase));
    };
    let card = parse_commit_segment(segment).ok_or_else(invalid_private_card_diagnostic)?;
    if !state.hand_for(actor).contains(&card) {
        return Err(invalid_private_card_diagnostic());
    }

    Ok(ValidatedAction { actor, card })
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "wrong_seat".to_owned(),
        message: "this seat is not allowed to act at the current decision point".to_owned(),
    }
}

pub fn wrong_phase_diagnostic(phase: Phase) -> Diagnostic {
    Diagnostic {
        code: "wrong_phase".to_owned(),
        message: format!("commit actions are not available during {}", phase.as_str()),
    }
}

pub fn invalid_private_card_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "invalid_private_card".to_owned(),
        message: "the requested private card is redacted or unavailable".to_owned(),
    }
}

pub fn stale_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "stale_action".to_owned(),
        message: "the action was submitted for an older decision point".to_owned(),
    }
}

pub fn commitment_conflict_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "commitment_unavailable".to_owned(),
        message: "a face-down commitment is already present for this round".to_owned(),
    }
}

pub fn apply_action(
    state: &mut HighCardDuelState,
    action: ValidatedAction,
) -> Vec<EffectEnvelope<HighCardDuelEffect>> {
    let mut effects = Vec::new();
    let actor_index = action.actor.index();
    let card_index = state.hands[actor_index]
        .iter()
        .position(|card| *card == action.card)
        .expect("validated action card must be in actor hand");
    let card = state.hands[actor_index].remove(card_index);

    state.commitments[actor_index] = Some(card);
    effects.push(commit_face_down_effect(action.actor, state.round_number));
    effects.push(own_commit_confirmed_effect(
        action.actor,
        state.seats[actor_index].clone(),
        card,
        state.round_number,
    ));

    match state.phase {
        Phase::LeadCommit => {
            state.phase = Phase::ReplyCommit;
            state.freshness_token = state.freshness_token.next();
        }
        Phase::ReplyCommit => {
            resolve_round(state, &mut effects);
        }
        Phase::Revealed | Phase::Terminal => {
            panic!(
                "validated action cannot apply during {}",
                state.phase.as_str()
            );
        }
    }

    effects
}

fn resolve_round(
    state: &mut HighCardDuelState,
    effects: &mut Vec<EffectEnvelope<HighCardDuelEffect>>,
) {
    let seat_0_card = state.commitments[HighCardDuelSeat::Seat0.index()]
        .expect("reply validation implies seat_0 commitment exists");
    let seat_1_card = state.commitments[HighCardDuelSeat::Seat1.index()]
        .expect("reply validation implies seat_1 commitment exists");
    let round_number = state.round_number;
    let winner = round_winner(seat_0_card, seat_1_card);

    effects.push(cards_revealed_effect(
        round_number,
        seat_0_card,
        seat_1_card,
    ));
    score_round(&mut state.score, winner);
    effects.push(round_scored_effect(round_number, winner, state.score));
    state.revealed_history.push(RevealedRound {
        round_number,
        seat_0_card,
        seat_1_card,
        winner,
    });
    state.commitments = [None, None];

    if round_number >= state.variant.round_limit {
        state.phase = Phase::Terminal;
        state.terminal_outcome = Some(match state.score.winner() {
            Some(seat) => TerminalOutcome::Win { seat },
            None => TerminalOutcome::Draw,
        });
        effects.push(terminal_effect(state.score.winner(), state.score));
        state.freshness_token = state.freshness_token.next();
        return;
    }

    let next_round = round_number + 1;
    let next_lead = lead_for_round(next_round);
    effects.push(refill_started_effect(next_round, next_lead));
    refill_hands(state, next_lead, effects);
    effects.push(hand_count_changed_effect(
        state.hands[HighCardDuelSeat::Seat0.index()].len() as u8,
        state.hands[HighCardDuelSeat::Seat1.index()].len() as u8,
        state.deck.len() as u8,
    ));

    state.round_number = next_round;
    state.lead_seat = next_lead;
    state.phase = Phase::LeadCommit;
    state.freshness_token = state.freshness_token.next();
}

pub fn round_winner(seat_0_card: CardId, seat_1_card: CardId) -> Option<HighCardDuelSeat> {
    if seat_0_card.rank() > seat_1_card.rank() {
        Some(HighCardDuelSeat::Seat0)
    } else if seat_1_card.rank() > seat_0_card.rank() {
        Some(HighCardDuelSeat::Seat1)
    } else {
        None
    }
}

pub fn lead_for_round(round_number: u8) -> HighCardDuelSeat {
    if round_number % 2 == 1 {
        HighCardDuelSeat::Seat0
    } else {
        HighCardDuelSeat::Seat1
    }
}

fn score_round(score: &mut Score, winner: Option<HighCardDuelSeat>) {
    match winner {
        Some(HighCardDuelSeat::Seat0) => score.seat_0 = score.seat_0.saturating_add(1),
        Some(HighCardDuelSeat::Seat1) => score.seat_1 = score.seat_1.saturating_add(1),
        None => {}
    }
}

fn refill_hands(
    state: &mut HighCardDuelState,
    first_seat: HighCardDuelSeat,
    effects: &mut Vec<EffectEnvelope<HighCardDuelEffect>>,
) {
    let mut seat = first_seat;
    while !state.deck.is_empty()
        && (state.hands[HighCardDuelSeat::Seat0.index()].len() < state.variant.hand_size as usize
            || state.hands[HighCardDuelSeat::Seat1.index()].len()
                < state.variant.hand_size as usize)
    {
        let seat_index = seat.index();
        if state.hands[seat_index].len() < state.variant.hand_size as usize {
            let card = state
                .deck
                .pop()
                .expect("deck is checked non-empty before refill draw");
            state.hands[seat_index].push(card);
            effects.push(deal_private_card_effect(
                seat,
                state.seats[seat_index].clone(),
                card,
            ));
        }
        seat = seat.other();
    }
}
