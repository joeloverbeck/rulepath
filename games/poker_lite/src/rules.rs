use engine_core::Diagnostic;

use crate::{
    actions::{self, PokerLiteAction, ValidatedAction},
    ids::{CrestCardId, PokerLiteSeat, STANDARD_MAX_CONTRIBUTION},
    state::{Phase, PledgeRoundState, PokerLiteState, ShowdownReveal, TerminalOutcome},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ShowdownStrength {
    pub pair_flag: bool,
    pub private_rank_value: u8,
}

pub fn apply_action(state: &mut PokerLiteState, action: ValidatedAction) -> Result<(), Diagnostic> {
    ensure_action_still_legal(state, action)?;

    match action.action {
        PokerLiteAction::Hold => apply_hold(state, action.actor),
        PokerLiteAction::Press => apply_press(state, action.actor),
        PokerLiteAction::Lift => apply_lift(state, action.actor),
        PokerLiteAction::Match => apply_match(state, action.actor),
        PokerLiteAction::Yield => apply_yield(state, action.actor),
    }

    debug_assert_accounting(state);
    state.freshness_token = state.freshness_token.next();
    Ok(())
}

pub fn showdown_strength(private_card: CrestCardId, center_card: CrestCardId) -> ShowdownStrength {
    ShowdownStrength {
        pair_flag: private_card.rank() == center_card.rank(),
        private_rank_value: private_card.rank().value(),
    }
}

pub fn compare_showdown(reveal: ShowdownReveal) -> Option<PokerLiteSeat> {
    let seat_0 = showdown_strength(reveal.seat_0_private, reveal.center);
    let seat_1 = showdown_strength(reveal.seat_1_private, reveal.center);

    if seat_0 > seat_1 {
        Some(PokerLiteSeat::Seat0)
    } else if seat_1 > seat_0 {
        Some(PokerLiteSeat::Seat1)
    } else {
        None
    }
}

fn ensure_action_still_legal(
    state: &PokerLiteState,
    action: ValidatedAction,
) -> Result<(), Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if state.active_seat != Some(action.actor) {
        return Err(actions::wrong_seat_diagnostic());
    }
    if action.action == PokerLiteAction::Lift
        && state.round.outstanding_actor == Some(action.actor)
        && state.round.lift_used
    {
        return Err(actions::lift_cap_diagnostic());
    }
    if !actions::legal_actions(state, action.actor).contains(&action.action) {
        return Err(actions::unavailable_action_diagnostic());
    }
    Ok(())
}

fn apply_hold(state: &mut PokerLiteState, actor: PokerLiteSeat) {
    state.round.consecutive_holds = state.round.consecutive_holds.saturating_add(1);
    if state.round.consecutive_holds >= 2 {
        close_current_round(state);
    } else {
        state.active_seat = Some(actor.other());
    }
}

fn apply_press(state: &mut PokerLiteState, actor: PokerLiteSeat) {
    add_to_pool(state, actor, state.round.unit);
    state.round.outstanding_actor = Some(actor.other());
    state.round.outstanding_amount = state.round.unit;
    state.round.consecutive_holds = 0;
    state.active_seat = Some(actor.other());
}

fn apply_lift(state: &mut PokerLiteState, actor: PokerLiteSeat) {
    let amount = state.round.outstanding_amount + state.round.unit;
    add_to_pool(state, actor, amount);
    state.round.outstanding_actor = Some(actor.other());
    state.round.outstanding_amount = state.round.unit;
    state.round.lift_used = true;
    state.round.consecutive_holds = 0;
    state.active_seat = Some(actor.other());
}

fn apply_match(state: &mut PokerLiteState, actor: PokerLiteSeat) {
    add_to_pool(state, actor, state.round.outstanding_amount);
    state.round.outstanding_actor = None;
    state.round.outstanding_amount = 0;
    state.round.consecutive_holds = 0;
    close_current_round(state);
}

fn apply_yield(state: &mut PokerLiteState, actor: PokerLiteSeat) {
    let winner = actor.other();
    state.phase = Phase::Terminal;
    state.active_seat = None;
    state.terminal_outcome = Some(TerminalOutcome::YieldWin {
        winner,
        loser: actor,
        shared_pool: state.shared_pool,
        contributions: state.contributions,
    });
}

fn close_current_round(state: &mut PokerLiteState) {
    match state.round.round_index {
        0 => {
            state.center_visible = true;
            state.phase = Phase::PledgeRound { round_index: 1 };
            state.active_seat = Some(PokerLiteSeat::Seat1);
            state.round = PledgeRoundState::for_round(1);
        }
        1 => resolve_showdown(state),
        _ => {
            state.phase = Phase::Terminal;
            state.active_seat = None;
        }
    }
}

fn resolve_showdown(state: &mut PokerLiteState) {
    let reveal = ShowdownReveal {
        seat_0_private: state.private_card_for_internal(PokerLiteSeat::Seat0),
        seat_1_private: state.private_card_for_internal(PokerLiteSeat::Seat1),
        center: state.center_card_internal(),
    };
    let outcome = match compare_showdown(reveal) {
        Some(winner) => TerminalOutcome::ShowdownWin {
            winner,
            shared_pool: state.shared_pool,
            contributions: state.contributions,
            reveal,
        },
        None => TerminalOutcome::Split {
            shared_pool: state.shared_pool,
            each: state.shared_pool / 2,
            contributions: state.contributions,
            reveal,
        },
    };
    state.phase = Phase::Terminal;
    state.active_seat = None;
    state.terminal_outcome = Some(outcome);
}

fn add_to_pool(state: &mut PokerLiteState, actor: PokerLiteSeat, amount: u8) {
    let index = actor.index();
    state.contributions[index] = state.contributions[index].saturating_add(amount);
    state.shared_pool = state.shared_pool.saturating_add(amount);
}

fn debug_assert_accounting(state: &PokerLiteState) {
    debug_assert_eq!(
        state.shared_pool,
        state.contributions[0] + state.contributions[1]
    );
    debug_assert!(state
        .contributions
        .iter()
        .all(|amount| *amount <= STANDARD_MAX_CONTRIBUTION));
    if let Some(TerminalOutcome::Split {
        shared_pool, each, ..
    }) = state.terminal_outcome
    {
        debug_assert_eq!(shared_pool % 2, 0);
        debug_assert_eq!(each * 2, shared_pool);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparator_pair_high_card_and_tie_cases() {
        let pair_reveal = ShowdownReveal {
            seat_0_private: CrestCardId::LowDawn,
            seat_1_private: CrestCardId::HighDawn,
            center: CrestCardId::LowDusk,
        };
        assert_eq!(compare_showdown(pair_reveal), Some(PokerLiteSeat::Seat0));

        let high_card_reveal = ShowdownReveal {
            seat_0_private: CrestCardId::MiddleDawn,
            seat_1_private: CrestCardId::HighDawn,
            center: CrestCardId::LowDawn,
        };
        assert_eq!(
            compare_showdown(high_card_reveal),
            Some(PokerLiteSeat::Seat1)
        );

        let tie_reveal = ShowdownReveal {
            seat_0_private: CrestCardId::MiddleDawn,
            seat_1_private: CrestCardId::MiddleDusk,
            center: CrestCardId::HighDawn,
        };
        assert_eq!(compare_showdown(tie_reveal), None);
    }
}
