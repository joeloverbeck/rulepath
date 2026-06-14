use std::cmp::Ordering;

use crate::{
    evaluator::{best_five_from_seven, compare_evaluations, HandEvaluation},
    ids::RiverLedgerSeat,
    pot::{allocate_single_pot, PotAllocation},
    state::{
        RiverLedgerState, SeatStatus, ShowdownReveal, ShowdownSeatExplanation, TerminalOutcome,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeatEvaluation {
    seat: RiverLedgerSeat,
    evaluation: HandEvaluation,
}

pub fn showdown_eligible_seats(state: &RiverLedgerState) -> Vec<RiverLedgerSeat> {
    state
        .ledger
        .seats
        .iter()
        .filter(|entry| {
            matches!(
                entry.status,
                SeatStatus::Live | SeatStatus::ShowdownEligible
            )
        })
        .map(|entry| entry.seat)
        .collect()
}

pub fn resolve_showdown(state: &RiverLedgerState) -> TerminalOutcome {
    let evaluations = evaluate_showdown_seats(state);
    let winners = winning_seats(&evaluations);
    let allocation = allocate_single_pot(
        state.ledger.pot_total,
        &winners,
        state.button,
        state.seats.len() as u8,
    );
    let explanations = explain_showdown(state, &evaluations, &allocation);

    TerminalOutcome::Showdown {
        winners: allocation.winners,
        pot_total: allocation.pot_total,
        allocations: allocation.shares,
        explanations,
    }
}

fn evaluate_showdown_seats(state: &RiverLedgerState) -> Vec<SeatEvaluation> {
    assert_eq!(
        state.board.len(),
        5,
        "showdown evaluation requires a complete public board"
    );

    showdown_eligible_seats(state)
        .into_iter()
        .map(|seat| {
            let hand = state
                .private_hand_for_internal(seat)
                .expect("eligible seat has private hand");
            let seven = [
                hand[0],
                hand[1],
                state.board[0],
                state.board[1],
                state.board[2],
                state.board[3],
                state.board[4],
            ];
            SeatEvaluation {
                seat,
                evaluation: best_five_from_seven(seven),
            }
        })
        .collect()
}

fn winning_seats(evaluations: &[SeatEvaluation]) -> Vec<RiverLedgerSeat> {
    let best = evaluations
        .iter()
        .map(|entry| &entry.evaluation)
        .max_by(|left, right| compare_evaluations(left, right))
        .expect("showdown requires at least one eligible seat");

    evaluations
        .iter()
        .filter(|entry| compare_evaluations(&entry.evaluation, best) == Ordering::Equal)
        .map(|entry| entry.seat)
        .collect()
}

fn explain_showdown(
    state: &RiverLedgerState,
    evaluations: &[SeatEvaluation],
    allocation: &PotAllocation,
) -> Vec<ShowdownSeatExplanation> {
    state
        .ledger
        .seats
        .iter()
        .map(|ledger| {
            if let Some(entry) = evaluations.iter().find(|entry| entry.seat == ledger.seat) {
                let share = allocation
                    .shares
                    .iter()
                    .find(|share| share.seat == ledger.seat)
                    .map(|share| share.amount)
                    .unwrap_or(0);
                ShowdownSeatExplanation {
                    seat: ledger.seat,
                    status: ledger.status,
                    revealed: Some(reveal_for(state, entry)),
                    summary: format!(
                        "{} reached showdown with {}; tie_break={:?}; allocated={}; total_contribution={}",
                        ledger.seat.as_str(),
                        entry.evaluation.category.as_str(),
                        entry.evaluation.tie_break_vector,
                        share,
                        ledger.total_contribution
                    ),
                }
            } else {
                ShowdownSeatExplanation {
                    seat: ledger.seat,
                    status: ledger.status,
                    revealed: None,
                    summary: format!(
                        "{} folded before showdown; allocated=0; total_contribution={}",
                        ledger.seat.as_str(),
                        ledger.total_contribution
                    ),
                }
            }
        })
        .collect()
}

fn reveal_for(state: &RiverLedgerState, entry: &SeatEvaluation) -> ShowdownReveal {
    ShowdownReveal {
        seat: entry.seat,
        hole_cards: state
            .private_hand_for_internal(entry.seat)
            .expect("showdown entry has a private hand"),
        best_five: entry.evaluation.used_cards,
        category: entry.evaluation.category.as_str().to_owned(),
        tie_break_vector: entry.evaluation.tie_break_vector.clone(),
    }
}
