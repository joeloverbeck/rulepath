use engine_core::Diagnostic;

use crate::{
    effects::VowTideEffect,
    ids::VowTideSeat,
    setup::deal_for_hand,
    state::{
        BiddingState, HandScoreBreakdown, Phase, SeatHandResult, Standing, TerminalOutcome,
        VowTideState,
    },
};

pub fn resolve_completed_hand(
    state: &mut VowTideState,
    effects: &mut Vec<VowTideEffect>,
) -> Result<(), Diagnostic> {
    let breakdown = score_current_hand(state)?;
    let additions = breakdown
        .seats
        .iter()
        .map(|seat| (seat.seat, seat.addition))
        .collect::<Vec<_>>();
    let cumulative_scores = breakdown
        .seats
        .iter()
        .map(|seat| (seat.seat, seat.cumulative_after))
        .collect::<Vec<_>>();
    effects.push(VowTideEffect::HandScored {
        hand_index: breakdown.hand_index,
        additions,
        cumulative_scores,
    });
    state.completed_hands.push(breakdown);

    if state.hand_index as usize + 1 >= state.hand_schedule.len() {
        let terminal = terminal_outcome(state);
        effects.push(VowTideEffect::MatchCompleted {
            winners: terminal.winners.clone(),
        });
        state.terminal_outcome = Some(terminal.clone());
        state.phase = Phase::Terminal(terminal);
        return Ok(());
    }

    advance_to_next_hand(state, effects)
}

pub fn score_current_hand(state: &mut VowTideState) -> Result<HandScoreBreakdown, Diagnostic> {
    let hand_index = state.hand_index;
    let hand_size = state
        .current_hand_size()
        .ok_or_else(scoring_state_diagnostic)?;
    let dealer = state.dealer;
    let trump_indicator = state.trump_indicator;
    let mut seats = Vec::with_capacity(state.seat_count());

    for seat in VowTideSeat::ALL.into_iter().take(state.seat_count()) {
        let bid = state.bid_for(seat).ok_or_else(scoring_state_diagnostic)?;
        let tricks_taken = state.trick_count_for(seat);
        let cumulative_before = state.cumulative_score_for(seat);
        let exact = bid == tricks_taken;
        let addition = if exact { 10 + i16::from(bid) } else { 0 };
        let cumulative_after = cumulative_before + addition;
        *state
            .cumulative_score_for_mut(seat)
            .expect("seat has cumulative score row") = cumulative_after;
        seats.push(SeatHandResult {
            seat,
            bid,
            tricks_taken,
            exact,
            successful_zero: exact && bid == 0,
            addition,
            cumulative_before,
            cumulative_after,
        });
    }

    Ok(HandScoreBreakdown {
        hand_index,
        hand_size,
        dealer,
        trump_indicator,
        seats,
    })
}

pub fn terminal_outcome(state: &VowTideState) -> TerminalOutcome {
    let mut scores = state.cumulative_scores.clone();
    scores.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    let top_score = scores.first().map(|(_, score)| *score).unwrap_or_default();
    let winners = scores
        .iter()
        .filter_map(|(seat, score)| (*score == top_score).then_some(*seat))
        .collect::<Vec<_>>();

    let mut standings = Vec::with_capacity(scores.len());
    let mut previous_score: Option<i16> = None;
    let mut current_rank = 0;
    for (index, (seat, score)) in scores.into_iter().enumerate() {
        if previous_score != Some(score) {
            current_rank = (index + 1) as u8;
            previous_score = Some(score);
        }
        standings.push(Standing {
            seat,
            cumulative_score: score,
            rank: current_rank,
            is_winner: score == top_score,
        });
    }

    TerminalOutcome {
        winners,
        standings,
        hands_played: state.completed_hands.len() as u32,
    }
}

fn advance_to_next_hand(
    state: &mut VowTideState,
    effects: &mut Vec<VowTideEffect>,
) -> Result<(), Diagnostic> {
    let next_hand_index = state.hand_index + 1;
    let next_dealer = state.dealer.next_clockwise(state.seat_count());
    let hand_size = state.hand_schedule[next_hand_index as usize];
    let deal = deal_for_hand(
        state.seed,
        next_dealer,
        next_hand_index,
        state.seat_count(),
        hand_size,
    )?;

    state.dealer = next_dealer;
    state.hand_index = next_hand_index;
    state.private_hands = deal.hands;
    state.trump_indicator = deal.trump_indicator;
    state.hidden_stock = deal.hidden_stock;
    state.deal_order = deal.deal_order;
    state.public_bids = VowTideSeat::ALL
        .into_iter()
        .take(state.seat_count())
        .map(|seat| (seat, None))
        .collect();
    state.trick_counts = VowTideSeat::ALL
        .into_iter()
        .take(state.seat_count())
        .map(|seat| (seat, 0))
        .collect();
    state.captured_tricks.clear();
    state.phase = Phase::Bidding(BiddingState::new(
        state.seat_count(),
        next_dealer.next_clockwise(state.seat_count()),
    ));
    effects.push(VowTideEffect::HandAdvanced {
        hand_index: next_hand_index,
        dealer: next_dealer,
        hand_size,
        trump_indicator: state.trump_indicator,
    });
    Ok(())
}

fn scoring_state_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "VT_SCORING_STATE_INCOMPLETE".to_owned(),
        message: "vow_tide cannot score a hand before bids and trick counts are complete"
            .to_owned(),
    }
}
