use crate::{
    cards::{Rank, Suit},
    ids::{BriarCircuitSeat, STANDARD_RAW_POINTS_PER_HAND},
    state::{
        CapturedTrick, HandScoreBreakdown, MoonStatus, OutcomeBreakdown, OutcomeStatus,
        SeatOutcomeBreakdown, TerminalOutcome,
    },
};

pub const MATCH_THRESHOLD: u16 = crate::ids::STANDARD_MATCH_THRESHOLD;

pub fn score_completed_hand(
    captured: &[CapturedTrick],
    cumulative_before: [u16; 4],
) -> HandScoreBreakdown {
    let raw_hearts = raw_hearts_by_seat(captured);
    let queen_spades = queen_spades_by_seat(captured);
    let raw_points = raw_points_by_seat(raw_hearts, queen_spades);
    let moon_shooter = moon_shooter(raw_points);
    let hand_additions = adjusted_additions(raw_points, moon_shooter);
    let cumulative_after = add_scores(cumulative_before, hand_additions);
    let outcome = outcome_breakdown(
        raw_hearts,
        queen_spades,
        raw_points,
        moon_shooter,
        hand_additions,
        cumulative_before,
        cumulative_after,
    );

    HandScoreBreakdown {
        raw_points,
        hand_additions,
        moon_shooter,
        cumulative_before,
        cumulative_after,
        outcome,
    }
}

pub fn terminal_outcome_for(breakdown: &OutcomeBreakdown) -> Option<TerminalOutcome> {
    match &breakdown.status {
        OutcomeStatus::Terminal { winner, .. } => {
            let mut cumulative_scores = [0; 4];
            for seat in &breakdown.seats {
                cumulative_scores[seat.seat.index()] = seat.cumulative_after;
            }
            Some(TerminalOutcome::UniqueLowScoreWin {
                winner: *winner,
                cumulative_scores,
                breakdown: breakdown.clone(),
            })
        }
        _ => None,
    }
}

pub fn raw_points_total(raw_points: [u8; 4]) -> u8 {
    raw_points.iter().copied().sum()
}

fn raw_hearts_by_seat(captured: &[CapturedTrick]) -> [u8; 4] {
    let mut raw_hearts = [0; 4];
    for trick in captured {
        for play in &trick.plays {
            if play.card.card().is_heart() {
                raw_hearts[trick.winner.index()] += 1;
            }
        }
    }
    raw_hearts
}

fn queen_spades_by_seat(captured: &[CapturedTrick]) -> [bool; 4] {
    let mut queen_spades = [false; 4];
    for trick in captured {
        for play in &trick.plays {
            let card = play.card.card();
            if matches!((card.rank, card.suit), (Rank::Queen, Suit::Spades)) {
                queen_spades[trick.winner.index()] = true;
            }
        }
    }
    queen_spades
}

fn raw_points_by_seat(raw_hearts: [u8; 4], queen_spades: [bool; 4]) -> [u8; 4] {
    let mut raw_points = raw_hearts;
    for seat in BriarCircuitSeat::ALL {
        if queen_spades[seat.index()] {
            raw_points[seat.index()] += 13;
        }
    }
    raw_points
}

fn moon_shooter(raw_points: [u8; 4]) -> Option<BriarCircuitSeat> {
    BriarCircuitSeat::ALL
        .into_iter()
        .find(|seat| raw_points[seat.index()] == STANDARD_RAW_POINTS_PER_HAND)
}

fn adjusted_additions(raw_points: [u8; 4], moon_shooter: Option<BriarCircuitSeat>) -> [u8; 4] {
    match moon_shooter {
        Some(shooter) => {
            let mut additions = [26; 4];
            additions[shooter.index()] = 0;
            additions
        }
        None => raw_points,
    }
}

fn add_scores(cumulative_before: [u16; 4], hand_additions: [u8; 4]) -> [u16; 4] {
    let mut cumulative_after = cumulative_before;
    for seat in BriarCircuitSeat::ALL {
        cumulative_after[seat.index()] += u16::from(hand_additions[seat.index()]);
    }
    cumulative_after
}

fn outcome_breakdown(
    raw_hearts: [u8; 4],
    queen_spades: [bool; 4],
    raw_points: [u8; 4],
    moon_shooter: Option<BriarCircuitSeat>,
    hand_additions: [u8; 4],
    cumulative_before: [u16; 4],
    cumulative_after: [u16; 4],
) -> OutcomeBreakdown {
    let threshold_reached = cumulative_after
        .iter()
        .any(|score| *score >= MATCH_THRESHOLD);
    let ranks = ranks_for(cumulative_after);
    let seats = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| SeatOutcomeBreakdown {
            seat,
            raw_hearts_count: raw_hearts[seat.index()],
            captured_queen_spades: queen_spades[seat.index()],
            raw_hand_points: raw_points[seat.index()],
            moon_status: moon_status_for(seat, moon_shooter),
            adjusted_hand_addition: hand_additions[seat.index()],
            cumulative_before: cumulative_before[seat.index()],
            cumulative_after: cumulative_after[seat.index()],
            rank: ranks[seat.index()],
            threshold_reached,
        })
        .collect();
    let status = outcome_status(threshold_reached, cumulative_after);

    OutcomeBreakdown {
        seats,
        threshold_reached,
        status,
    }
}

fn moon_status_for(seat: BriarCircuitSeat, moon_shooter: Option<BriarCircuitSeat>) -> MoonStatus {
    match moon_shooter {
        Some(shooter) if shooter == seat => MoonStatus::Shooter,
        Some(_) => MoonStatus::OpponentAdjusted,
        None => MoonStatus::None,
    }
}

fn ranks_for(cumulative_after: [u16; 4]) -> [u8; 4] {
    let mut ranks = [0; 4];
    for seat in BriarCircuitSeat::ALL {
        let score = cumulative_after[seat.index()];
        ranks[seat.index()] = 1 + cumulative_after
            .iter()
            .filter(|other_score| **other_score < score)
            .count() as u8;
    }
    ranks
}

fn outcome_status(threshold_reached: bool, cumulative_after: [u16; 4]) -> OutcomeStatus {
    if !threshold_reached {
        return OutcomeStatus::InProgress;
    }

    let low_score = *cumulative_after
        .iter()
        .min()
        .expect("four cumulative scores exist");
    let tied_seats: Vec<_> = BriarCircuitSeat::ALL
        .into_iter()
        .filter(|seat| cumulative_after[seat.index()] == low_score)
        .collect();

    match tied_seats.as_slice() {
        [winner] => OutcomeStatus::Terminal {
            winner: *winner,
            losers: BriarCircuitSeat::ALL
                .into_iter()
                .filter(|seat| seat != winner)
                .collect(),
        },
        _ => OutcomeStatus::TiedLowContinuation {
            tied_low_score: low_score,
            tied_seats,
        },
    }
}
