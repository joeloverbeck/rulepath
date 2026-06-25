use engine_core::{Actor, Diagnostic};
use game_stdlib::trick_taking::{follow_suit_indices, winning_play_index};

use crate::{
    bidding::actor_seat,
    cards::{CardId, Suit},
    effects::BlackglassPactEffect,
    ids::{BlackglassSeat, STANDARD_HAND_SIZE},
    state::{BlackglassPactState, Phase, PlayedCard},
};

pub const ACTION_PLAY: &str = "play";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayAction {
    pub card: CardId,
}

pub fn parse_play_action_path(segments: &[String]) -> Result<PlayAction, Diagnostic> {
    match segments {
        [family, card] if family == ACTION_PLAY => {
            let card = CardId::parse(card).ok_or_else(malformed_play_action_diagnostic)?;
            Ok(PlayAction { card })
        }
        _ => Err(malformed_play_action_diagnostic()),
    }
}

pub fn apply_play_action(
    state: &mut BlackglassPactState,
    actor: &Actor,
    action: PlayAction,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    let actor_seat = actor_seat(state, actor).ok_or_else(wrong_play_seat_diagnostic)?;
    apply_play_choice(state, actor_seat, action.card)
}

pub fn apply_play_choice(
    state: &mut BlackglassPactState,
    seat: BlackglassSeat,
    card: CardId,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    validate_play(state, seat, card)?;

    let removed = {
        let hand = state
            .hand_for_internal_mut(seat)
            .expect("validated seat has a private hand");
        let card_index = hand
            .iter()
            .position(|candidate| *candidate == card)
            .expect("validated card is owned");
        hand.remove(card_index)
    };

    let (trick_index, breaks_spades) = {
        let Phase::PlayingTrick {
            plays, trick_index, ..
        } = &state.phase
        else {
            return Err(wrong_phase_diagnostic());
        };
        (
            *trick_index,
            play_breaks_spades(state.spades_broken, plays, removed),
        )
    };

    let mut effects = vec![BlackglassPactEffect::CardPlayed {
        seat,
        card: removed,
        trick_index,
    }];
    if breaks_spades {
        state.spades_broken = true;
        effects.push(BlackglassPactEffect::SpadesBroken {
            seat,
            card: removed,
            trick_index,
        });
    }

    {
        let Phase::PlayingTrick { plays, .. } = &mut state.phase else {
            return Err(wrong_phase_diagnostic());
        };
        plays.push(PlayedCard {
            seat,
            card: removed,
        });
    }

    if current_trick_len(state) == crate::ids::STANDARD_SEAT_COUNT as usize {
        resolve_current_trick(state, &mut effects)?;
    } else {
        let Phase::PlayingTrick { next, .. } = &mut state.phase else {
            return Err(wrong_phase_diagnostic());
        };
        *next = seat.next_clockwise();
    }

    state.advance_freshness();
    Ok(effects)
}

pub fn legal_play_cards(state: &BlackglassPactState, seat: BlackglassSeat) -> Vec<CardId> {
    let Phase::PlayingTrick { next, plays, .. } = &state.phase else {
        return Vec::new();
    };
    if *next != seat {
        return Vec::new();
    }

    let hand = state.hand_for_internal(seat);
    if plays.is_empty() {
        return legal_leads(state.spades_broken, hand);
    }

    let led_suit = plays[0].card.card().suit;
    follow_suit_indices(hand, led_suit, |card| card.card().suit)
        .into_iter()
        .map(|index| hand[index])
        .collect()
}

pub fn validate_play(
    state: &BlackglassPactState,
    seat: BlackglassSeat,
    card: CardId,
) -> Result<(), Diagnostic> {
    let Phase::PlayingTrick { next, plays, .. } = &state.phase else {
        return Err(wrong_phase_diagnostic());
    };
    if *next != seat {
        return Err(wrong_play_seat_diagnostic());
    }
    if !state.hand_for_internal(seat).contains(&card) {
        return Err(card_not_owned_diagnostic());
    }
    if plays.is_empty() && !lead_is_legal(state.spades_broken, state.hand_for_internal(seat), card)
    {
        return Err(spades_not_broken_diagnostic());
    }
    if !plays.is_empty() && !legal_play_cards(state, seat).contains(&card) {
        return Err(must_follow_suit_diagnostic());
    }
    Ok(())
}

pub fn trick_winner(plays: &[PlayedCard]) -> Result<BlackglassSeat, Diagnostic> {
    let led_suit = plays
        .first()
        .map(|play| play.card.card().suit)
        .ok_or_else(malformed_trick_state_diagnostic)?;
    let winner_index = winning_play_index(
        plays,
        led_suit,
        Some(Suit::Spades),
        |play| play.card.card().suit,
        |play| play.card.card().rank.value(),
    )
    .ok_or_else(malformed_trick_state_diagnostic)?;

    Ok(plays[winner_index].seat)
}

pub fn legal_leads(spades_broken: bool, hand: &[CardId]) -> Vec<CardId> {
    if spades_broken || hand.iter().all(|card| card.card().suit == Suit::Spades) {
        return hand.to_vec();
    }
    hand.iter()
        .copied()
        .filter(|card| card.card().suit != Suit::Spades)
        .collect()
}

pub fn lead_is_legal(spades_broken: bool, hand: &[CardId], card: CardId) -> bool {
    legal_leads(spades_broken, hand).contains(&card)
}

fn resolve_current_trick(
    state: &mut BlackglassPactState,
    effects: &mut Vec<BlackglassPactEffect>,
) -> Result<(), Diagnostic> {
    let (plays, trick_index) = match &state.phase {
        Phase::PlayingTrick {
            plays, trick_index, ..
        } => (plays.clone(), *trick_index),
        _ => return Err(wrong_phase_diagnostic()),
    };
    let winner = trick_winner(&plays)?;
    state.tricks_won[winner.index()] = state.tricks_won[winner.index()].saturating_add(1);
    effects.push(BlackglassPactEffect::TrickCaptured {
        winner,
        trick_index,
        plays,
    });

    let completed_tricks = trick_index.saturating_add(1);
    if completed_tricks >= STANDARD_HAND_SIZE {
        state.phase = Phase::HandScoring { completed_tricks };
    } else {
        state.phase = Phase::PlayingTrick {
            leader: winner,
            next: winner,
            plays: Vec::new(),
            trick_index: completed_tricks,
        };
    }
    Ok(())
}

fn current_trick_len(state: &BlackglassPactState) -> usize {
    match &state.phase {
        Phase::PlayingTrick { plays, .. } => plays.len(),
        _ => 0,
    }
}

fn play_breaks_spades(spades_broken: bool, plays: &[PlayedCard], card: CardId) -> bool {
    !spades_broken
        && card.card().suit == Suit::Spades
        && (plays.is_empty() || plays[0].card.card().suit != Suit::Spades)
}

fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_PHASE".to_owned(),
        message: "card play is only legal while a trick is active".to_owned(),
    }
}

fn wrong_play_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_PLAY_SEAT".to_owned(),
        message: "only the active playing seat may play a card".to_owned(),
    }
}

fn card_not_owned_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_CARD_NOT_OWNED".to_owned(),
        message: "the submitted card is not in the actor's hand".to_owned(),
    }
}

fn spades_not_broken_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_SPADES_NOT_BROKEN".to_owned(),
        message: "spades cannot be led before they are broken while a non-spade is held".to_owned(),
    }
}

fn must_follow_suit_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_MUST_FOLLOW_SUIT".to_owned(),
        message: "a card of the led suit must be played when held".to_owned(),
    }
}

fn malformed_play_action_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_MALFORMED_PLAY_ACTION".to_owned(),
        message: "play actions must be play/<card-id>".to_owned(),
    }
}

fn malformed_trick_state_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_MALFORMED_TRICK_STATE".to_owned(),
        message: "trick resolution requires a non-empty trick with a winner".to_owned(),
    }
}
