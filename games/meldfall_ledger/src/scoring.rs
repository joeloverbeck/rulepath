//! Round and match scoring for Meldfall Ledger.
//!
//! Scores are game-local behavior: tabled cards score to their per-card
//! score-credit owner, hand cards subtract from their holder, and terminal
//! outcome is checked only after a settled round.

use crate::{
    cards::CardId,
    ids::STANDARD_TARGET_SCORE,
    state::{MatchOutcome, MatchState, SeatStanding},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundSettlement {
    pub seats: Vec<SeatSettlement>,
    pub terminal: Option<MatchOutcome>,
}

impl RoundSettlement {
    pub fn stable_public_string(&self) -> String {
        let seats = self
            .seats
            .iter()
            .map(SeatSettlement::stable_public_string)
            .collect::<Vec<_>>()
            .join(";");
        let terminal = self
            .terminal
            .as_ref()
            .map(MatchOutcome::stable_string)
            .unwrap_or_else(|| "none".to_owned());
        format!("settlement|seats=[{seats}]|terminal={terminal}")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatSettlement {
    pub seat_index: usize,
    pub tabled_positive: i32,
    pub in_hand_penalty: i32,
    pub remaining_hand_count: usize,
    pub round_delta: i32,
    pub cumulative_score: i32,
    pub rank: usize,
    pub winner: bool,
}

impl SeatSettlement {
    pub fn stable_public_string(&self) -> String {
        format!(
            "{}:tabled={}:penalty={}:remaining={}:delta={}:cumulative={}:rank={}:winner={}",
            self.seat_index,
            self.tabled_positive,
            self.in_hand_penalty,
            self.remaining_hand_count,
            self.round_delta,
            self.cumulative_score,
            self.rank,
            self.winner
        )
    }
}

pub fn settle_round(state: &mut MatchState) -> RoundSettlement {
    let tabled = tabled_positive_totals(state);
    let penalties = in_hand_penalties(state);
    let deltas = tabled
        .iter()
        .zip(&penalties)
        .map(|(positive, penalty)| positive - penalty)
        .collect::<Vec<_>>();
    apply_round_deltas(state, &deltas, &tabled, &penalties)
}

pub fn apply_round_deltas(
    state: &mut MatchState,
    deltas: &[i32],
    tabled_positive: &[i32],
    in_hand_penalties: &[i32],
) -> RoundSettlement {
    assert_eq!(state.cumulative_scores.len(), deltas.len());
    let terminal = terminal_outcome_for_scores_after_deltas(&state.cumulative_scores, deltas);
    for (score, delta) in state.cumulative_scores.iter_mut().zip(deltas) {
        *score += *delta;
    }
    state.terminal = terminal.clone();

    let seats = state
        .cumulative_scores
        .iter()
        .enumerate()
        .map(|(seat_index, cumulative_score)| {
            let rank = rank_for_score(*cumulative_score, &state.cumulative_scores);
            let winner = terminal.as_ref().and_then(|outcome| outcome.winner) == Some(seat_index);
            SeatSettlement {
                seat_index,
                tabled_positive: tabled_positive[seat_index],
                in_hand_penalty: in_hand_penalties[seat_index],
                remaining_hand_count: state.round.seats[seat_index].hand.len(),
                round_delta: deltas[seat_index],
                cumulative_score: *cumulative_score,
                rank,
                winner,
            }
        })
        .collect();
    RoundSettlement { seats, terminal }
}

pub fn terminal_outcome_for_scores_after_deltas(
    current_scores: &[i32],
    deltas: &[i32],
) -> Option<MatchOutcome> {
    let cumulative = current_scores
        .iter()
        .zip(deltas)
        .map(|(score, delta)| score + delta)
        .collect::<Vec<_>>();
    terminal_outcome_for_scores(&cumulative, deltas)
}

pub fn terminal_outcome_for_scores(
    cumulative_scores: &[i32],
    latest_round_deltas: &[i32],
) -> Option<MatchOutcome> {
    let highest = cumulative_scores.iter().copied().max()?;
    if highest < STANDARD_TARGET_SCORE {
        return None;
    }
    let leaders = cumulative_scores
        .iter()
        .enumerate()
        .filter_map(|(seat, score)| (*score == highest).then_some(seat))
        .collect::<Vec<_>>();
    if leaders.len() != 1 {
        return None;
    }
    let winner = leaders[0];
    Some(MatchOutcome {
        winner: Some(winner),
        standings: cumulative_scores
            .iter()
            .enumerate()
            .map(|(seat_index, cumulative_score)| SeatStanding {
                seat_index,
                cumulative_score: *cumulative_score,
                latest_round_delta: latest_round_deltas[seat_index],
                rank: rank_for_score(*cumulative_score, cumulative_scores),
                winner: seat_index == winner,
            })
            .collect(),
    })
}

fn tabled_positive_totals(state: &MatchState) -> Vec<i32> {
    let mut totals = vec![0; state.seats.len()];
    for group in &state.round.tableau.groups {
        for table_card in &group.cards {
            totals[table_card.score_credit_owner] += card_score(table_card.card);
        }
    }
    totals
}

fn in_hand_penalties(state: &MatchState) -> Vec<i32> {
    state
        .round
        .seats
        .iter()
        .map(|seat| seat.hand.iter().copied().map(card_score).sum())
        .collect()
}

pub fn card_score(card: CardId) -> i32 {
    i32::from(card.card().rank.score_value())
}

fn rank_for_score(score: i32, scores: &[i32]) -> usize {
    1 + scores.iter().filter(|other| **other > score).count()
}
