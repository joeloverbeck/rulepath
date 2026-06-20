use crate::{
    ids::RiverLedgerSeat,
    state::{RiverLedgerState, SeatStatus},
};

pub fn live_seats(state: &RiverLedgerState) -> Vec<RiverLedgerSeat> {
    state
        .ledger
        .seats
        .iter()
        .filter(|entry| entry.status == SeatStatus::Live)
        .map(|entry| entry.seat)
        .collect()
}

pub fn non_folded_seats(state: &RiverLedgerState) -> Vec<RiverLedgerSeat> {
    state
        .ledger
        .seats
        .iter()
        .filter(|entry| entry.status != SeatStatus::Folded)
        .map(|entry| entry.seat)
        .collect()
}

pub fn call_price(state: &RiverLedgerState, seat: RiverLedgerSeat) -> Option<u16> {
    let ledger = state.ledger.seats.get(seat.index())?;
    if ledger.status != SeatStatus::Live {
        return None;
    }
    Some(
        state
            .betting
            .current_to_call
            .saturating_sub(ledger.street_contribution),
    )
}

pub fn raise_right_open(state: &RiverLedgerState, seat: RiverLedgerSeat) -> bool {
    if state.betting.raise_cap_reached() {
        return false;
    }
    let Some(ledger) = state.ledger.seats.get(seat.index()) else {
        return false;
    };
    if ledger.status != SeatStatus::Live {
        return false;
    }

    match state
        .betting
        .last_completed_action_to_call
        .get(seat.index())
        .copied()
        .flatten()
    {
        None => true,
        Some(last_to_call) => {
            state.betting.current_to_call.saturating_sub(last_to_call)
                >= u16::from(state.betting.street.unit())
        }
    }
}

pub fn record_completed_action(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    ensure_reopen_tracking(state, actor);
    state.betting.last_completed_action_to_call[actor.index()] =
        Some(state.betting.current_to_call);
}

pub fn next_live_after(state: &RiverLedgerState, seat: RiverLedgerSeat) -> Option<RiverLedgerSeat> {
    let count = state.seats.len() as u8;
    let mut current = seat;
    for _ in 0..count {
        current = current.next_in_count(count)?;
        if state.ledger.seats[current.index()].status == SeatStatus::Live {
            return Some(current);
        }
    }
    None
}

pub fn first_live_after(
    state: &RiverLedgerState,
    seat: RiverLedgerSeat,
) -> Option<RiverLedgerSeat> {
    next_live_after(state, seat)
}

pub fn response_order_after(
    state: &RiverLedgerState,
    actor: RiverLedgerSeat,
) -> Vec<RiverLedgerSeat> {
    let count = state.seats.len() as u8;
    let mut order = Vec::new();
    let mut current = actor;
    for _ in 0..count.saturating_sub(1) {
        current = current
            .next_in_count(count)
            .expect("valid actor in valid seat count");
        if state.ledger.seats[current.index()].status == SeatStatus::Live {
            order.push(current);
        }
    }
    order
}

pub fn remove_pending_response(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    retain_actionable_responses(state);
    state
        .betting
        .actors_to_respond
        .retain(|seat| *seat != actor);
}

pub fn retain_actionable_responses(state: &mut RiverLedgerState) {
    let live = state
        .ledger
        .seats
        .iter()
        .filter(|entry| entry.status == SeatStatus::Live)
        .map(|entry| entry.seat)
        .collect::<Vec<_>>();
    state
        .betting
        .actors_to_respond
        .retain(|seat| live.contains(seat));
}

fn ensure_reopen_tracking(state: &mut RiverLedgerState, seat: RiverLedgerSeat) {
    if state.betting.last_completed_action_to_call.len() <= seat.index() {
        state
            .betting
            .last_completed_action_to_call
            .resize(seat.index() + 1, None);
    }
}

pub fn round_is_closed(state: &RiverLedgerState) -> bool {
    if !state.betting.actors_to_respond.is_empty() {
        return false;
    }

    state
        .ledger
        .seats
        .iter()
        .filter(|entry| entry.status == SeatStatus::Live)
        .all(|entry| entry.street_contribution == state.betting.current_to_call)
}
