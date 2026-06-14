use engine_core::Diagnostic;

use crate::{
    actions::{self, RiverLedgerAction, ValidatedAction},
    betting,
    ids::RiverLedgerSeat,
    state::{BettingRoundState, Phase, RiverLedgerState, SeatStatus, Street, TerminalOutcome},
};

pub fn apply_action(
    state: &mut RiverLedgerState,
    action: ValidatedAction,
) -> Result<(), Diagnostic> {
    ensure_action_still_legal(state, action)?;

    match action.action {
        RiverLedgerAction::Fold => apply_fold(state, action.actor),
        RiverLedgerAction::Check => apply_check(state, action.actor),
        RiverLedgerAction::Call => apply_call(state, action.actor, action.required_to_call),
        RiverLedgerAction::Bet => apply_bet(state, action.actor),
        RiverLedgerAction::Raise => apply_raise(state, action.actor, action.required_to_call),
    }

    debug_assert_ledger(state);
    state.freshness_token = state.freshness_token.next();
    Ok(())
}

fn ensure_action_still_legal(
    state: &RiverLedgerState,
    action: ValidatedAction,
) -> Result<(), Diagnostic> {
    if state.terminal_outcome.is_some() || !matches!(state.phase, Phase::Betting { .. }) {
        return Err(actions::terminal_or_non_betting_diagnostic());
    }
    if state.active_seat != Some(action.actor) {
        return Err(actions::wrong_seat_diagnostic());
    }
    if action.action == RiverLedgerAction::Raise && state.betting.raise_cap_reached() {
        return Err(actions::raise_cap_diagnostic());
    }
    if !actions::legal_actions(state, action.actor).contains(&action.action) {
        return Err(actions::unavailable_action_diagnostic());
    }
    Ok(())
}

fn apply_fold(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    state.ledger.seats[actor.index()].status = SeatStatus::Folded;
    betting::remove_pending_response(state, actor);

    let live = betting::live_seats(state);
    if live.len() == 1 {
        state.phase = Phase::Terminal;
        state.active_seat = None;
        state.terminal_outcome = Some(TerminalOutcome::LastLiveHand {
            winner: live[0],
            pot_total: state.ledger.pot_total,
        });
        return;
    }

    if betting::round_is_closed(state) {
        close_current_street(state);
    } else {
        state.active_seat = state.betting.actors_to_respond.first().copied();
    }
}

fn apply_check(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    betting::remove_pending_response(state, actor);
    if betting::round_is_closed(state) {
        close_current_street(state);
    } else {
        state.active_seat = state.betting.actors_to_respond.first().copied();
    }
}

fn apply_call(state: &mut RiverLedgerState, actor: RiverLedgerSeat, amount: u16) {
    add_contribution(state, actor, amount);
    betting::remove_pending_response(state, actor);
    if betting::round_is_closed(state) {
        close_current_street(state);
    } else {
        state.active_seat = state.betting.actors_to_respond.first().copied();
    }
}

fn apply_bet(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    let amount = u16::from(state.betting.street.unit());
    add_contribution(state, actor, amount);
    state.betting.current_to_call = state.ledger.seats[actor.index()].street_contribution;
    state.betting.raises_this_street = 0;
    state.betting.last_aggressor = Some(actor);
    state.betting.actors_to_respond = betting::response_order_after(state, actor);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn apply_raise(state: &mut RiverLedgerState, actor: RiverLedgerSeat, required_to_call: u16) {
    let amount = required_to_call + u16::from(state.betting.street.unit());
    add_contribution(state, actor, amount);
    state.betting.current_to_call = state.ledger.seats[actor.index()].street_contribution;
    state.betting.raises_this_street = state.betting.raises_this_street.saturating_add(1);
    state.betting.last_aggressor = Some(actor);
    state.betting.actors_to_respond = betting::response_order_after(state, actor);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn add_contribution(state: &mut RiverLedgerState, actor: RiverLedgerSeat, amount: u16) {
    let ledger = &mut state.ledger.seats[actor.index()];
    ledger.street_contribution = ledger.street_contribution.saturating_add(amount);
    ledger.total_contribution = ledger.total_contribution.saturating_add(amount);
    state.ledger.pot_total = state.ledger.pot_total.saturating_add(amount);
}

fn close_current_street(state: &mut RiverLedgerState) {
    match state.betting.street {
        Street::Preflop => advance_to_street(state, Street::Flop, 3),
        Street::Flop => advance_to_street(state, Street::Turn, 1),
        Street::Turn => advance_to_street(state, Street::River, 1),
        Street::River => {
            for seat in &mut state.ledger.seats {
                if seat.status == SeatStatus::Live {
                    seat.status = SeatStatus::ShowdownEligible;
                }
            }
            state.phase = Phase::Showdown;
            state.active_seat = None;
            state.betting.actors_to_respond.clear();
        }
    }
}

fn advance_to_street(state: &mut RiverLedgerState, street: Street, reveal_count: usize) {
    state.reveal_next_board_cards(reveal_count);
    for seat in &mut state.ledger.seats {
        seat.street_contribution = 0;
    }
    let first = betting::first_live_after(state, state.button);
    let actors_to_respond = first
        .map(|seat| response_order_beginning_with(state, seat))
        .unwrap_or_default();
    state.phase = Phase::Betting { street };
    state.active_seat = first;
    state.betting = BettingRoundState::for_street(street, actors_to_respond);
}

fn response_order_beginning_with(
    state: &RiverLedgerState,
    first: RiverLedgerSeat,
) -> Vec<RiverLedgerSeat> {
    let live = betting::live_seats(state);
    if live.is_empty() {
        return Vec::new();
    }

    let count = state.seats.len() as u8;
    let mut order = Vec::with_capacity(live.len());
    let mut current = first;
    for _ in 0..count {
        if state.ledger.seats[current.index()].status == SeatStatus::Live {
            order.push(current);
        }
        current = current
            .next_in_count(count)
            .expect("valid live response order");
        if current == first {
            break;
        }
    }
    order
}

fn debug_assert_ledger(state: &RiverLedgerState) {
    let total = state
        .ledger
        .seats
        .iter()
        .map(|seat| seat.total_contribution)
        .sum::<u16>();
    debug_assert_eq!(total, state.ledger.pot_total);
}
