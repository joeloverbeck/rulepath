use engine_core::{CommandEnvelope, Diagnostic};

use crate::{
    actions::{self, ValidatedBid},
    effects::VowTideEffect,
    state::{Phase, PlayingTrickState, VowTideState},
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
        message: "bid commands are legal only during Vow Tide bidding".to_owned(),
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
