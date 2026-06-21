use engine_core::Diagnostic;
use game_stdlib::trick_taking::{follow_suit_indices, winning_play_index};

use crate::{
    cards::{Card, CardId, Rank, Suit},
    effects::BriarCircuitEffect,
    ids::{BriarCircuitSeat, STANDARD_TRICKS_PER_HAND},
    scoring::{score_completed_hand, terminal_outcome_for},
    setup::{deal_for_hand, next_dealer},
    state::{
        BriarCircuitState, CapturedTrick, CurrentTrick, PassDirection, PassState, Phase,
        PlayingTrickState, TrickPlay,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayLegalityReason {
    Legal,
    WrongPhase,
    WrongSeat,
    CardNotOwned,
    TwoClubsMustOpen,
    MustFollowSuit,
    FirstTrickPointForbidden,
    HeartsNotBroken,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayActionResult {
    pub effects: Vec<BriarCircuitEffect>,
    pub trick_completed: bool,
    pub hand_completed: bool,
}

pub fn legal_play_cards(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
) -> Result<Vec<CardId>, Diagnostic> {
    let play = playing_state_for_actor(state, seat)?;
    Ok(legal_cards_for_playing_state(
        state.hand_for_internal(seat),
        play,
    ))
}

pub fn validate_play_card(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    card: CardId,
) -> Result<(), Diagnostic> {
    match play_legality_reason(state, seat, card) {
        PlayLegalityReason::Legal => Ok(()),
        PlayLegalityReason::WrongPhase => Err(diagnostic(
            "BC_WRONG_PHASE",
            "briar_circuit play action is only legal during trick play",
        )),
        PlayLegalityReason::WrongSeat => Err(diagnostic(
            "BC_WRONG_SEAT",
            "only the active Briar Circuit seat may play",
        )),
        PlayLegalityReason::CardNotOwned => {
            Err(diagnostic("BC_CARD_NOT_OWNED", "played card is not owned"))
        }
        PlayLegalityReason::TwoClubsMustOpen => Err(diagnostic(
            "BC_TWO_CLUBS_MUST_OPEN",
            "the first play of the hand must be the two of clubs",
        )),
        PlayLegalityReason::MustFollowSuit => Err(diagnostic(
            "BC_MUST_FOLLOW_SUIT",
            "actor must follow the led suit",
        )),
        PlayLegalityReason::FirstTrickPointForbidden => Err(diagnostic(
            "BC_FIRST_TRICK_POINT_FORBIDDEN",
            "point cards are forbidden on the first trick while a non-point card is available",
        )),
        PlayLegalityReason::HeartsNotBroken => Err(diagnostic(
            "BC_HEARTS_NOT_BROKEN",
            "hearts cannot be led until broken while a non-heart remains",
        )),
    }
}

pub fn play_legality_reason(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    card: CardId,
) -> PlayLegalityReason {
    let play = match &state.phase {
        Phase::PlayingTrick(play) => play,
        _ => return PlayLegalityReason::WrongPhase,
    };
    if play.active_seat != seat {
        return PlayLegalityReason::WrongSeat;
    }
    let hand = state.hand_for_internal(seat);
    if !hand.contains(&card) {
        return PlayLegalityReason::CardNotOwned;
    }

    let legal = legal_cards_for_playing_state(hand, play);
    if legal.contains(&card) {
        return PlayLegalityReason::Legal;
    }

    if play.trick_index == 0 && play.current_trick.plays.is_empty() {
        return PlayLegalityReason::TwoClubsMustOpen;
    }
    if let Some(led_suit) = led_suit(play) {
        if hand.iter().any(|held| held.card().suit == led_suit) && card.card().suit != led_suit {
            return PlayLegalityReason::MustFollowSuit;
        }
    }
    if play.trick_index == 0 && is_point_card(card.card()) {
        return PlayLegalityReason::FirstTrickPointForbidden;
    }
    if play.current_trick.plays.is_empty()
        && !play.hearts_broken
        && card.card().is_heart()
        && hand.iter().any(|held| !held.card().is_heart())
    {
        return PlayLegalityReason::HeartsNotBroken;
    }

    PlayLegalityReason::CardNotOwned
}

pub fn apply_play_card(
    state: &mut BriarCircuitState,
    seat: BriarCircuitSeat,
    card: CardId,
) -> Result<PlayActionResult, Diagnostic> {
    validate_play_card(state, seat, card)?;
    remove_card_from_hand(state, seat, card)?;

    let mut effects = vec![BriarCircuitEffect::CardPlayed { seat, card }];
    let mut trick_completed = false;
    let mut hand_completed = false;

    let heart_played = card.card().is_heart();
    let mut completed_trick = None;
    {
        let play = state
            .playing_state_mut()
            .expect("validated trick play phase");
        if heart_played && !play.hearts_broken {
            play.hearts_broken = true;
            effects.push(BriarCircuitEffect::HeartsBroken { seat });
        }
        play.current_trick.plays.push(TrickPlay { seat, card });
        if play.current_trick.plays.len() == BriarCircuitSeat::ALL.len() {
            completed_trick = Some((
                play.trick_index,
                play.current_trick.plays.clone(),
                play.hearts_broken,
            ));
        } else {
            play.active_seat = play.active_seat.next_clockwise();
        }
    }
    state.freshness_token = state.freshness_token.next();

    if let Some((trick_index, plays, hearts_broken)) = completed_trick {
        trick_completed = true;
        let winner = trick_winner(&plays).expect("four-play trick has winner");
        state.captured_tricks.push(CapturedTrick {
            hand_index: state.hand_index,
            trick_index,
            winner: winner.seat,
            plays: plays.clone(),
        });
        effects.push(BriarCircuitEffect::TrickCaptured {
            trick_index,
            winner: winner.seat,
            cards: plays.iter().map(|play| play.card).collect(),
        });

        if trick_index + 1 >= STANDARD_TRICKS_PER_HAND {
            hand_completed = true;
            let scoring = score_completed_hand(&state.captured_tricks, state.cumulative_scores);
            state.cumulative_scores = scoring.cumulative_after;
            // Retain the just-completed hand's public scoring so the browser can show a
            // between-hands summary while (for non-terminal hands) the next hand is dealt.
            state.last_hand_summary = Some(scoring.clone());
            match terminal_outcome_for(&scoring.outcome) {
                Some(outcome) => state.phase = Phase::Terminal(outcome),
                // The match threshold is never reached in a single hand, so any
                // non-terminal completed hand (in progress or a tied-low score)
                // continues by dealing the next hand per BC-MATCH-001/003.
                None => begin_next_hand(state)?,
            }
        } else {
            let next_index = trick_index + 1;
            state.phase = Phase::PlayingTrick(PlayingTrickState {
                hearts_broken,
                trick_index: next_index,
                leader: winner.seat,
                active_seat: winner.seat,
                current_trick: CurrentTrick::new(winner.seat),
            });
        }
        state.freshness_token = state.freshness_token.next();
    }

    Ok(PlayActionResult {
        effects,
        trick_completed,
        hand_completed,
    })
}

/// Rotate the dealer, advance the hand index, and deterministically deal the
/// next hand, entering its pass selection (or the opening lead on a hold hand).
/// Captured tricks reset so the new hand scores only its own points.
fn begin_next_hand(state: &mut BriarCircuitState) -> Result<(), Diagnostic> {
    let dealer = next_dealer(state.dealer);
    let hand_index = state.hand_index + 1;
    let deal = deal_for_hand(state.seed, dealer, hand_index)?;

    state.dealer = dealer;
    state.hand_index = hand_index;
    state.captured_tricks.clear();
    state.private_hands = BriarCircuitSeat::ALL.into_iter().zip(deal.hands).collect();
    state.phase = match deal.pass_direction {
        PassDirection::Hold => {
            let leader = state.two_clubs_leader();
            Phase::PlayingTrick(PlayingTrickState {
                hearts_broken: false,
                trick_index: 0,
                leader,
                active_seat: leader,
                current_trick: CurrentTrick::new(leader),
            })
        }
        direction => Phase::Passing(PassState::new(direction)),
    };
    Ok(())
}

pub fn legal_cards_for_playing_state(hand: &[CardId], play: &PlayingTrickState) -> Vec<CardId> {
    if hand.is_empty() {
        return Vec::new();
    }

    if play.trick_index == 0 && play.current_trick.plays.is_empty() {
        return hand
            .iter()
            .copied()
            .filter(|card| card.card().is_two_of_clubs())
            .collect();
    }

    if let Some(led_suit) = led_suit(play) {
        let followed = follow_suit_indices(hand, led_suit, |card| card.card().suit)
            .into_iter()
            .map(|index| hand[index])
            .collect::<Vec<_>>();
        if followed
            .first()
            .is_some_and(|card| card.card().suit == led_suit)
        {
            return followed;
        }
    }

    if play.trick_index == 0 {
        let non_points: Vec<_> = hand
            .iter()
            .copied()
            .filter(|card| !is_point_card(card.card()))
            .collect();
        if !non_points.is_empty() {
            return non_points;
        }
        return hand.to_vec();
    }

    if play.current_trick.plays.is_empty() && !play.hearts_broken {
        let non_hearts: Vec<_> = hand
            .iter()
            .copied()
            .filter(|card| !card.card().is_heart())
            .collect();
        if !non_hearts.is_empty() {
            return non_hearts;
        }
    }

    hand.to_vec()
}

pub fn trick_winner(plays: &[TrickPlay]) -> Option<TrickPlay> {
    let led_suit = plays.first()?.card.card().suit;
    winning_play_index(
        plays,
        led_suit,
        None,
        |play| play.card.card().suit,
        |play| play.card.card().rank.value(),
    )
    .map(|index| plays[index])
}

pub const fn is_point_card(card: Card) -> bool {
    card.is_heart() || matches!((card.rank, card.suit), (Rank::Queen, Suit::Spades))
}

fn playing_state_for_actor(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
) -> Result<&PlayingTrickState, Diagnostic> {
    let play = state.playing_state().ok_or_else(|| {
        diagnostic(
            "BC_WRONG_PHASE",
            "briar_circuit play action is only legal during trick play",
        )
    })?;
    if play.active_seat != seat {
        return Err(diagnostic(
            "BC_WRONG_SEAT",
            "only the active Briar Circuit seat may play",
        ));
    }
    Ok(play)
}

fn led_suit(play: &PlayingTrickState) -> Option<Suit> {
    play.current_trick
        .plays
        .first()
        .map(|played| played.card.card().suit)
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
        .ok_or_else(|| diagnostic("BC_CARD_NOT_OWNED", "played card is not owned"))?;
    hand.remove(index);
    Ok(())
}

fn diagnostic(code: &str, message: &str) -> Diagnostic {
    Diagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
