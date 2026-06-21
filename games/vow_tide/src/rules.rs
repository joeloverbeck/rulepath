use engine_core::{CommandEnvelope, Diagnostic};
use game_stdlib::trick_taking::winning_play_index;

use crate::{
    actions::{self, ValidatedBid, ValidatedPlay},
    cards::CardId,
    effects::VowTideEffect,
    ids::VowTideSeat,
    scoring,
    state::{CapturedTrick, CurrentTrick, Phase, PlayingTrickState, TrickPlay, VowTideState},
};

pub fn validate_bid_command(
    state: &VowTideState,
    envelope: &CommandEnvelope,
) -> Result<ValidatedBid, Diagnostic> {
    if envelope.freshness_token != state.freshness_token {
        return Err(stale_command_diagnostic());
    }
    if envelope.rules_version.0 != 1 {
        return Err(wrong_rules_version_diagnostic());
    }

    let actor = actions::actor_seat(state, &envelope.actor).ok_or_else(wrong_seat_diagnostic)?;
    let bidding = state.bidding_state().ok_or_else(wrong_phase_diagnostic)?;
    if bidding.active_seat != actor {
        return Err(wrong_seat_diagnostic());
    }

    let action = actions::parse_bid_action_path(&envelope.action_path.segments)?;
    if bidding.bid_for(actor).is_some() {
        return Err(bid_already_set_diagnostic());
    }

    let hand_size = state
        .current_hand_size()
        .ok_or_else(wrong_phase_diagnostic)?;
    if action.value > hand_size {
        return Err(bid_out_of_range_diagnostic());
    }
    if Some(action.value) == actions::hook_forbidden_bid(state, bidding, actor) {
        return Err(bid_hook_forbidden_diagnostic(action.value));
    }

    Ok(ValidatedBid {
        actor,
        value: action.value,
        hand_index: state.hand_index,
        hand_size,
    })
}

pub fn apply_bid(
    state: &mut VowTideState,
    bid: ValidatedBid,
) -> Result<Vec<VowTideEffect>, Diagnostic> {
    validate_bid_still_legal(state, bid)?;

    let seat_count = state.seat_count();
    let dealer = state.dealer;
    let mut effects = Vec::new();
    let (public_total, all_bids_set, next_unset, hook_total) = {
        let bidding = state
            .bidding_state_mut()
            .expect("validated action requires bidding phase");
        *bidding
            .bid_for_mut(bid.actor)
            .expect("validated actor has bid row") = Some(bid.value);
        let public_total = bidding.accepted_bid_total();
        let all_bids_set = bidding.all_bids_set();
        let next_unset = if all_bids_set {
            None
        } else {
            Some(
                bidding
                    .next_unset_after(bid.actor, seat_count)
                    .expect("bidding remains incomplete"),
            )
        };
        (public_total, all_bids_set, next_unset, public_total)
    };
    *state
        .public_bid_for_mut(bid.actor)
        .expect("validated actor has public bid row") = Some(bid.value);
    effects.push(VowTideEffect::BidAccepted {
        seat: bid.actor,
        bid: bid.value,
        public_total,
    });

    if all_bids_set {
        let first_leader = dealer.next_clockwise(seat_count);
        state.phase = Phase::PlayingTrick(PlayingTrickState {
            trick_index: 0,
            leader: first_leader,
            active_seat: first_leader,
            current_trick: CurrentTrick::new(first_leader),
        });
        effects.push(VowTideEffect::BiddingCompleted { first_leader });
    } else {
        let next = next_unset.expect("incomplete bidding has next bidder");
        let bidding = state
            .bidding_state_mut()
            .expect("incomplete bid remains in bidding phase");
        bidding.active_seat = next;
        if next == dealer {
            let forbidden = if hook_total <= bid.hand_size {
                Some(bid.hand_size - hook_total)
            } else {
                None
            };
            if let Some(forbidden) = forbidden {
                effects.push(VowTideEffect::DealerHookConstrained {
                    dealer,
                    forbidden_bid: forbidden,
                    hand_size: bid.hand_size,
                    public_total_before_dealer: hook_total,
                });
            }
        }
    }

    state.freshness_token = state.freshness_token.next();
    Ok(effects)
}

pub fn validate_play_command(
    state: &VowTideState,
    envelope: &CommandEnvelope,
) -> Result<ValidatedPlay, Diagnostic> {
    if envelope.freshness_token != state.freshness_token {
        return Err(stale_command_diagnostic());
    }
    if envelope.rules_version.0 != 1 {
        return Err(wrong_rules_version_diagnostic());
    }

    let actor = actions::actor_seat(state, &envelope.actor).ok_or_else(wrong_seat_diagnostic)?;
    let playing = state.playing_state().ok_or_else(wrong_phase_diagnostic)?;
    if playing.active_seat != actor {
        return Err(wrong_seat_diagnostic());
    }
    let action = actions::parse_play_action_path(&envelope.action_path.segments)?;
    if !state.hand_for_internal(actor).contains(&action.card) {
        return Err(card_not_owned_diagnostic(action.card));
    }
    if !actions::legal_cards(state, actor).contains(&action.card) {
        return Err(must_follow_suit_diagnostic());
    }

    Ok(ValidatedPlay {
        actor,
        card: action.card,
        hand_index: state.hand_index,
        trick_index: playing.trick_index,
    })
}

pub fn apply_play(
    state: &mut VowTideState,
    play: ValidatedPlay,
) -> Result<Vec<VowTideEffect>, Diagnostic> {
    validate_play_still_legal(state, play)?;
    let mut effects = Vec::new();

    let hand = state
        .hand_for_internal_mut(play.actor)
        .expect("validated actor has hand");
    let card_index = hand
        .iter()
        .position(|card| *card == play.card)
        .expect("validated card must be in hand");
    let card = hand.remove(card_index);

    {
        let playing = state
            .playing_state_mut()
            .expect("validated play requires playing phase");
        playing.current_trick.plays.push(TrickPlay {
            seat: play.actor,
            card,
        });
    }
    effects.push(VowTideEffect::CardPlayed {
        seat: play.actor,
        card,
        trick_index: play.trick_index,
    });

    let seat_count = state.seat_count();
    let current_len = state
        .playing_state()
        .expect("playing phase")
        .current_trick
        .plays
        .len();
    if current_len == seat_count {
        resolve_current_trick(state, &mut effects)?;
    } else {
        let next = play.actor.next_clockwise(seat_count);
        state
            .playing_state_mut()
            .expect("playing phase")
            .active_seat = next;
    }

    state.freshness_token = state.freshness_token.next();
    Ok(effects)
}

pub fn trick_winner(
    plays: &[TrickPlay],
    trump: crate::cards::Suit,
) -> Result<VowTideSeat, Diagnostic> {
    let led_suit = plays
        .first()
        .map(|play| play.card.card().suit)
        .ok_or_else(malformed_trick_state_diagnostic)?;
    let winner_index = winning_play_index(
        plays,
        led_suit,
        Some(trump),
        |play| play.card.card().suit,
        |play| play.card.card().rank,
    )
    .ok_or_else(malformed_trick_state_diagnostic)?;
    Ok(plays[winner_index].seat)
}

fn validate_play_still_legal(state: &VowTideState, play: ValidatedPlay) -> Result<(), Diagnostic> {
    let playing = state.playing_state().ok_or_else(wrong_phase_diagnostic)?;
    if playing.active_seat != play.actor {
        return Err(wrong_seat_diagnostic());
    }
    if state.hand_index != play.hand_index || playing.trick_index != play.trick_index {
        return Err(stale_command_diagnostic());
    }
    if !state.hand_for_internal(play.actor).contains(&play.card) {
        return Err(card_not_owned_diagnostic(play.card));
    }
    if !actions::legal_cards(state, play.actor).contains(&play.card) {
        return Err(must_follow_suit_diagnostic());
    }
    Ok(())
}

fn resolve_current_trick(
    state: &mut VowTideState,
    effects: &mut Vec<VowTideEffect>,
) -> Result<(), Diagnostic> {
    let (trick_index, plays) = {
        let playing = state.playing_state().expect("playing phase");
        (playing.trick_index, playing.current_trick.plays.clone())
    };
    let winner = trick_winner(&plays, state.trump_suit())?;
    state.increment_trick_count(winner);
    state.captured_tricks.push(CapturedTrick {
        hand_index: state.hand_index,
        trick_index,
        winner,
        plays: plays.clone(),
    });
    effects.push(VowTideEffect::TrickCaptured {
        trick_index,
        winner,
        cards: plays.iter().map(|play| play.card).collect(),
    });

    let hand_size = state
        .current_hand_size()
        .ok_or_else(wrong_phase_diagnostic)?;
    let playing = state.playing_state_mut().expect("playing phase");
    if trick_index + 1 < hand_size {
        let next_trick_index = trick_index + 1;
        playing.trick_index = next_trick_index;
        playing.leader = winner;
        playing.active_seat = winner;
        playing.current_trick = CurrentTrick::new(winner);
    } else {
        scoring::resolve_completed_hand(state, effects)?;
    }
    Ok(())
}

fn validate_bid_still_legal(state: &VowTideState, bid: ValidatedBid) -> Result<(), Diagnostic> {
    let bidding = state.bidding_state().ok_or_else(wrong_phase_diagnostic)?;
    if bidding.active_seat != bid.actor {
        return Err(wrong_seat_diagnostic());
    }
    if bidding.bid_for(bid.actor).is_some() {
        return Err(bid_already_set_diagnostic());
    }
    if state.current_hand_size() != Some(bid.hand_size) || state.hand_index != bid.hand_index {
        return Err(stale_command_diagnostic());
    }
    if bid.value > bid.hand_size {
        return Err(bid_out_of_range_diagnostic());
    }
    if Some(bid.value) == actions::hook_forbidden_bid(state, bidding, bid.actor) {
        return Err(bid_hook_forbidden_diagnostic(bid.value));
    }
    Ok(())
}

pub fn wrong_rules_version_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_WRONG_RULES_VERSION".to_owned(),
        message: "vow_tide command used an unsupported rules version".to_owned(),
    }
}

pub fn stale_command_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_STALE_COMMAND".to_owned(),
        message: "vow_tide command used a stale freshness token".to_owned(),
    }
}

pub fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_WRONG_PHASE".to_owned(),
        message: "command is not legal during the current Vow Tide phase".to_owned(),
    }
}

pub fn unknown_card_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_UNKNOWN_CARD".to_owned(),
        message: "play commands require a recognized Vow Tide card id".to_owned(),
    }
}

pub fn card_not_owned_diagnostic(card: CardId) -> Diagnostic {
    Diagnostic {
        code: "VT_CARD_NOT_OWNED".to_owned(),
        message: format!(
            "the submitted card `{}` is not owned by the actor",
            card.as_str()
        ),
    }
}

pub fn must_follow_suit_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_MUST_FOLLOW_SUIT".to_owned(),
        message: "a Vow Tide follower holding the led suit must play that suit".to_owned(),
    }
}

fn malformed_trick_state_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_MALFORMED_TRICK_STATE".to_owned(),
        message: "vow_tide trick resolution requires at least one eligible play".to_owned(),
    }
}

pub fn wrong_seat_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_WRONG_SEAT".to_owned(),
        message: "only the active seated actor may bid".to_owned(),
    }
}

pub fn bid_out_of_range_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_BID_OUT_OF_RANGE".to_owned(),
        message: "bid commands require a decimal value in the current hand range".to_owned(),
    }
}

pub fn bid_hook_forbidden_diagnostic(value: u8) -> Diagnostic {
    Diagnostic {
        code: "VT_BID_HOOK_FORBIDDEN".to_owned(),
        message: format!("dealer bid {value} would make total bids equal the hand size"),
    }
}

pub fn bid_already_set_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_BID_ALREADY_SET".to_owned(),
        message: "accepted Vow Tide bids are public and immutable".to_owned(),
    }
}
