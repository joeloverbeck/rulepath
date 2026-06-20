use engine_core::{Diagnostic, EffectEnvelope};

use crate::{
    actions::{self, RiverLedgerAction, ValidatedAction},
    betting,
    effects::{public_effect, RiverLedgerEffect},
    ids::RiverLedgerSeat,
    showdown,
    state::{BettingRoundState, Phase, RiverLedgerState, SeatStatus, Street, TerminalOutcome},
};

pub fn apply_action(
    state: &mut RiverLedgerState,
    action: ValidatedAction,
) -> Result<Vec<EffectEnvelope<RiverLedgerEffect>>, Diagnostic> {
    ensure_action_still_legal(state, action)?;

    let before_pot_total = state.ledger.pot_total;
    let before_street = state.betting.street;
    let before_board_len = state.board.len();
    let before_terminal = state.terminal_outcome.clone();
    let actor = action.actor;

    match action.action {
        RiverLedgerAction::Fold => apply_fold(state, action.actor),
        RiverLedgerAction::Check => apply_check(state, action.actor),
        RiverLedgerAction::Call => apply_call(state, action.actor, action.adds_to_pot),
        RiverLedgerAction::Bet => apply_bet(state, action.actor, action.adds_to_pot),
        RiverLedgerAction::Raise => apply_raise(
            state,
            action.actor,
            action.adds_to_pot,
            action.is_full_raise,
        ),
    }

    resolve_automatic_progress(state);
    debug_assert_ledger(state);
    state.freshness_token = state.freshness_token.next();
    Ok(effects_for_transition(
        state,
        actor,
        before_pot_total,
        before_street,
        before_board_len,
        before_terminal.as_ref(),
    ))
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
    betting::record_completed_action(state, actor);
    state.ledger.seats[actor.index()].status = SeatStatus::Folded;
    betting::remove_pending_response(state, actor);

    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn apply_check(state: &mut RiverLedgerState, actor: RiverLedgerSeat) {
    betting::record_completed_action(state, actor);
    betting::remove_pending_response(state, actor);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn apply_call(state: &mut RiverLedgerState, actor: RiverLedgerSeat, amount: u16) {
    add_contribution(state, actor, amount);
    betting::record_completed_action(state, actor);
    betting::remove_pending_response(state, actor);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn apply_bet(state: &mut RiverLedgerState, actor: RiverLedgerSeat, amount: u16) {
    add_contribution(state, actor, amount);
    state.betting.current_to_call = state.ledger.seats[actor.index()].street_contribution;
    state.betting.raises_this_street = 0;
    state.betting.last_aggressor = Some(actor);
    betting::record_completed_action(state, actor);
    state.betting.actors_to_respond = betting::response_order_after(state, actor);
    betting::retain_actionable_responses(state);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn apply_raise(
    state: &mut RiverLedgerState,
    actor: RiverLedgerSeat,
    amount: u16,
    is_full_raise: bool,
) {
    add_contribution(state, actor, amount);
    state.betting.current_to_call = state.ledger.seats[actor.index()].street_contribution;
    if is_full_raise {
        state.betting.raises_this_street = state.betting.raises_this_street.saturating_add(1);
    }
    state.betting.last_aggressor = Some(actor);
    betting::record_completed_action(state, actor);
    state.betting.actors_to_respond = betting::response_order_after(state, actor);
    betting::retain_actionable_responses(state);
    state.active_seat = state.betting.actors_to_respond.first().copied();
}

fn add_contribution(state: &mut RiverLedgerState, actor: RiverLedgerSeat, amount: u16) {
    let ledger = &mut state.ledger.seats[actor.index()];
    ledger.remaining_stack = ledger
        .remaining_stack
        .checked_sub(amount)
        .expect("validated River Ledger action cannot exceed remaining stack");
    ledger.street_contribution = ledger
        .street_contribution
        .checked_add(amount)
        .expect("validated River Ledger street contribution fits u16");
    ledger.total_contribution = ledger
        .total_contribution
        .checked_add(amount)
        .expect("validated River Ledger total contribution fits u16");
    if ledger.remaining_stack == 0 && ledger.status == SeatStatus::Live {
        ledger.status = SeatStatus::AllIn;
    }
    state.ledger.pot_total = state
        .ledger
        .pot_total
        .checked_add(amount)
        .expect("validated River Ledger pot total fits u16");
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
            state.terminal_outcome = Some(showdown::resolve_showdown(state));
            state.phase = Phase::Terminal;
            state.active_seat = None;
            state.betting.actors_to_respond.clear();
        }
    }
}

fn resolve_automatic_progress(state: &mut RiverLedgerState) {
    loop {
        if state.terminal_outcome.is_some() || !matches!(state.phase, Phase::Betting { .. }) {
            return;
        }

        betting::retain_actionable_responses(state);
        let non_folded = betting::non_folded_seats(state);
        if non_folded.len() == 1 {
            state.phase = Phase::Terminal;
            state.active_seat = None;
            state.betting.actors_to_respond.clear();
            state.terminal_outcome = Some(TerminalOutcome::LastLiveHand {
                winner: non_folded[0],
                pot_total: state.ledger.pot_total,
            });
            return;
        }

        let live = betting::live_seats(state);
        if live.is_empty() {
            runout_to_showdown(state);
            return;
        }

        if live.len() == 1
            && state.betting.actors_to_respond.is_empty()
            && betting::call_price(state, live[0]).unwrap_or(0) == 0
        {
            return_unmatched_excess(state, live[0]);
            runout_to_showdown(state);
            return;
        }

        if state.betting.actors_to_respond.is_empty() && betting::round_is_closed(state) {
            close_current_street(state);
            continue;
        }

        state.active_seat = state.betting.actors_to_respond.first().copied();
        return;
    }
}

fn runout_to_showdown(state: &mut RiverLedgerState) {
    let missing_board_cards = 5usize.saturating_sub(state.board.len());
    if missing_board_cards > 0 {
        state.reveal_next_board_cards(missing_board_cards);
    }
    for seat in &mut state.ledger.seats {
        if matches!(seat.status, SeatStatus::Live | SeatStatus::AllIn) {
            seat.status = SeatStatus::ShowdownEligible;
        }
    }
    state.terminal_outcome = Some(showdown::resolve_showdown(state));
    state.phase = Phase::Terminal;
    state.active_seat = None;
    state.betting.actors_to_respond.clear();
}

fn return_unmatched_excess(state: &mut RiverLedgerState, live_seat: RiverLedgerSeat) {
    let max_other_contribution = state
        .ledger
        .seats
        .iter()
        .filter(|seat| seat.seat != live_seat && seat.status != SeatStatus::Folded)
        .map(|seat| seat.total_contribution)
        .max()
        .unwrap_or(0);
    let live_index = live_seat.index();
    let live_total = state.ledger.seats[live_index].total_contribution;
    let return_amount = live_total.saturating_sub(max_other_contribution);
    if return_amount == 0 {
        return;
    }

    let ledger = &mut state.ledger.seats[live_index];
    ledger.total_contribution = ledger
        .total_contribution
        .checked_sub(return_amount)
        .expect("unmatched excess cannot exceed live contribution");
    ledger.street_contribution = ledger.street_contribution.saturating_sub(return_amount);
    ledger.remaining_stack = ledger
        .remaining_stack
        .checked_add(return_amount)
        .expect("returned unmatched excess fits remaining stack");
    state.ledger.pot_total = state
        .ledger
        .pot_total
        .checked_sub(return_amount)
        .expect("unmatched excess cannot exceed pot total");
    state.betting.current_to_call = state
        .ledger
        .seats
        .iter()
        .filter(|seat| seat.status != SeatStatus::Folded)
        .map(|seat| seat.street_contribution)
        .max()
        .unwrap_or(0);
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
    let conserved = state
        .ledger
        .seats
        .iter()
        .all(|seat| seat.remaining_stack + seat.total_contribution == seat.starting_stack);
    debug_assert!(conserved);
}

fn effects_for_transition(
    state: &RiverLedgerState,
    actor: RiverLedgerSeat,
    before_pot_total: u16,
    before_street: Street,
    before_board_len: usize,
    before_terminal: Option<&TerminalOutcome>,
) -> Vec<EffectEnvelope<RiverLedgerEffect>> {
    let mut effects = Vec::new();

    let amount_added = state.ledger.pot_total.saturating_sub(before_pot_total);
    if amount_added > 0 {
        effects.push(public_effect(RiverLedgerEffect::ContributionChanged {
            seat: actor,
            amount_added,
            pot_total: state.ledger.pot_total,
        }));
    }

    if state.board.len() > before_board_len || state.betting.street != before_street {
        effects.push(public_effect(RiverLedgerEffect::StreetAdvanced {
            street: state.betting.street,
            public_board: state.board.clone(),
        }));
    }

    if before_terminal.is_none() {
        if let Some(outcome) = &state.terminal_outcome {
            effects.push(public_effect(RiverLedgerEffect::ShowdownResolved {
                outcome: outcome.clone(),
            }));
        }
    }

    effects
}
