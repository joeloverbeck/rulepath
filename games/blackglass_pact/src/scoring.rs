use engine_core::Diagnostic;

use crate::{
    effects::BlackglassPactEffect,
    ids::{BlackglassSeat, TeamId},
    partnerships::{members_for_team, team_for_seat},
    setup::{complete_blind_nil_and_deal, initial_blind_nil_phase},
    state::{
        Bid, BlackglassPactState, HandScoreBreakdown, MatchOutcome, NilResult, Phase,
        SeatScoreBreakdown, SeatStanding, TeamScoreBreakdown, TeamStanding,
    },
};

pub const TARGET_SCORE: i32 = 500;
pub const BAG_THRESHOLD: u8 = 10;
pub const BAG_PENALTY_POINTS: i32 = 100;

pub fn score_completed_hand(
    state: &mut BlackglassPactState,
) -> Result<Vec<BlackglassPactEffect>, Diagnostic> {
    let Phase::HandScoring { completed_tricks } = state.phase else {
        return Err(wrong_phase_diagnostic());
    };
    if completed_tricks != crate::ids::STANDARD_HAND_SIZE {
        return Err(incomplete_hand_diagnostic());
    }

    let breakdown = score_hand(state)?;
    apply_breakdown(state, &breakdown);

    let mut effects = vec![BlackglassPactEffect::HandScored {
        breakdown: breakdown.clone(),
    }];
    for team_breakdown in &breakdown.team_breakdowns {
        if team_breakdown.bag_penalty_count > 0 {
            effects.push(BlackglassPactEffect::BagPenaltyApplied {
                team: team_breakdown.team,
                penalty_count: team_breakdown.bag_penalty_count,
                points_deducted: i32::from(team_breakdown.bag_penalty_count) * BAG_PENALTY_POINTS,
                next_bags: team_breakdown.next_bags,
            });
        }
    }

    state.last_hand_score = Some(breakdown.clone());
    if let Some(winning_team) = terminal_winner(state.team_scores) {
        let outcome = match_outcome(state, winning_team, breakdown);
        state.outcome = Some(outcome.clone());
        state.phase = Phase::Terminal { winning_team };
        effects.push(BlackglassPactEffect::MatchCompleted { outcome });
    } else {
        advance_to_next_hand(state)?;
        effects.push(BlackglassPactEffect::DealerAdvanced {
            dealer: state.dealer,
            hand_index: state.hand_index,
        });
    }

    state.advance_freshness();
    Ok(effects)
}

pub fn score_hand(state: &BlackglassPactState) -> Result<HandScoreBreakdown, Diagnostic> {
    Ok(HandScoreBreakdown {
        hand_index: state.hand_index,
        team_breakdowns: [
            score_team(state, TeamId::NorthSouth)?,
            score_team(state, TeamId::EastWest)?,
        ],
        seat_breakdowns: [
            score_seat(state, BlackglassSeat::North),
            score_seat(state, BlackglassSeat::East),
            score_seat(state, BlackglassSeat::South),
            score_seat(state, BlackglassSeat::West),
        ],
    })
}

pub fn terminal_winner(team_scores: [i32; 2]) -> Option<TeamId> {
    let any_at_target = team_scores.iter().any(|score| *score >= TARGET_SCORE);
    if !any_at_target || team_scores[0] == team_scores[1] {
        return None;
    }
    if team_scores[0] > team_scores[1] {
        Some(TeamId::NorthSouth)
    } else {
        Some(TeamId::EastWest)
    }
}

pub fn match_outcome(
    state: &BlackglassPactState,
    winning_team: TeamId,
    final_hand: HandScoreBreakdown,
) -> MatchOutcome {
    let team_ranks = competition_ranks(state.team_scores);
    MatchOutcome {
        winning_team_ids: vec![winning_team],
        standings_by_team: [
            team_standing(state, TeamId::NorthSouth, winning_team, team_ranks[0]),
            team_standing(state, TeamId::EastWest, winning_team, team_ranks[1]),
        ],
        standings_by_seat: [
            seat_standing(state, BlackglassSeat::North, team_ranks),
            seat_standing(state, BlackglassSeat::East, team_ranks),
            seat_standing(state, BlackglassSeat::South, team_ranks),
            seat_standing(state, BlackglassSeat::West, team_ranks),
        ],
        final_hand,
    }
}

fn score_team(state: &BlackglassPactState, team: TeamId) -> Result<TeamScoreBreakdown, Diagnostic> {
    let contract = state.ordinary_team_contract(team);
    let ordinary_tricks = members_for_team(team)
        .into_iter()
        .filter(|seat| matches!(state.bid_for(*seat), Some(Bid::Tricks(_))))
        .map(|seat| state.tricks_won[seat.index()])
        .sum::<u8>();
    let ordinary_made = ordinary_tricks >= contract;
    let ordinary_base = if ordinary_made {
        10 * i32::from(contract)
    } else {
        -10 * i32::from(contract)
    };
    let ordinary_overtricks = if ordinary_made {
        ordinary_tricks.saturating_sub(contract)
    } else {
        0
    };

    let nil_delta = members_for_team(team)
        .into_iter()
        .map(|seat| nil_delta(state.bid_for(seat), state.tricks_won[seat.index()]))
        .sum::<i32>();
    let failed_nil_bags = members_for_team(team)
        .into_iter()
        .filter(|seat| is_failed_nil(state.bid_for(*seat), state.tricks_won[seat.index()]))
        .map(|seat| state.tricks_won[seat.index()])
        .sum::<u8>();

    let new_bags = ordinary_overtricks
        .checked_add(failed_nil_bags)
        .ok_or_else(score_overflow_diagnostic)?;
    let prior_bags = state.team_bags[team.index()];
    let raw_bags = prior_bags
        .checked_add(new_bags)
        .ok_or_else(score_overflow_diagnostic)?;
    let bag_penalty_count = raw_bags / BAG_THRESHOLD;
    let next_bags = raw_bags % BAG_THRESHOLD;
    let hand_delta = ordinary_base + i32::from(new_bags) + nil_delta
        - i32::from(bag_penalty_count) * BAG_PENALTY_POINTS;
    let prior_score = state.team_scores[team.index()];
    let next_score = prior_score
        .checked_add(hand_delta)
        .ok_or_else(score_overflow_diagnostic)?;

    Ok(TeamScoreBreakdown {
        team,
        contract,
        ordinary_tricks,
        ordinary_made,
        ordinary_base,
        ordinary_overtricks,
        failed_nil_bags,
        new_bags,
        prior_bags,
        raw_bags,
        bag_penalty_count,
        next_bags,
        nil_delta,
        hand_delta,
        prior_score,
        next_score,
    })
}

fn score_seat(state: &BlackglassPactState, seat: BlackglassSeat) -> SeatScoreBreakdown {
    let bid = state.bid_for(seat);
    let tricks = state.tricks_won[seat.index()];
    SeatScoreBreakdown {
        seat,
        team: team_for_seat(seat),
        bid,
        tricks,
        nil_result: nil_result(bid, tricks),
    }
}

fn apply_breakdown(state: &mut BlackglassPactState, breakdown: &HandScoreBreakdown) {
    for team_breakdown in &breakdown.team_breakdowns {
        let index = team_breakdown.team.index();
        state.team_scores[index] = team_breakdown.next_score;
        state.team_bags[index] = team_breakdown.next_bags;
    }
}

fn advance_to_next_hand(state: &mut BlackglassPactState) -> Result<(), Diagnostic> {
    state.dealer = state.dealer.next_clockwise();
    state.hand_index = state.hand_index.saturating_add(1);
    state.spades_broken = false;
    state.bids = [None, None, None, None];
    state.blind_nil_choices = [None, None, None, None];
    state.tricks_won = [0, 0, 0, 0];
    state.private_hands.clear();
    state.phase = initial_blind_nil_phase(state.dealer, state.team_scores);
    if state.active_blind_nil_seat().is_none() {
        complete_blind_nil_and_deal(state)?;
    }
    Ok(())
}

fn nil_delta(bid: Option<Bid>, tricks: u8) -> i32 {
    match bid {
        Some(Bid::Nil) if tricks == 0 => 100,
        Some(Bid::Nil) => -100,
        Some(Bid::BlindNil) if tricks == 0 => 200,
        Some(Bid::BlindNil) => -200,
        Some(Bid::Tricks(_)) | None => 0,
    }
}

fn nil_result(bid: Option<Bid>, tricks: u8) -> Option<NilResult> {
    match bid {
        Some(Bid::Nil | Bid::BlindNil) if tricks == 0 => Some(NilResult::Made),
        Some(Bid::Nil | Bid::BlindNil) => Some(NilResult::Failed),
        Some(Bid::Tricks(_)) | None => None,
    }
}

fn is_failed_nil(bid: Option<Bid>, tricks: u8) -> bool {
    matches!(bid, Some(Bid::Nil | Bid::BlindNil)) && tricks > 0
}

fn competition_ranks(scores: [i32; 2]) -> [u8; 2] {
    if scores[0] == scores[1] {
        [1, 1]
    } else if scores[0] > scores[1] {
        [1, 2]
    } else {
        [2, 1]
    }
}

fn team_standing(
    state: &BlackglassPactState,
    team: TeamId,
    winning_team: TeamId,
    competition_rank: u8,
) -> TeamStanding {
    let members = members_for_team(team);
    TeamStanding {
        team_id: team,
        member_seat_ids: [
            state.seats[members[0].index()].clone(),
            state.seats[members[1].index()].clone(),
        ],
        score: state.team_scores[team.index()],
        bags: state.team_bags[team.index()],
        competition_rank,
        is_winner: team == winning_team,
    }
}

fn seat_standing(
    state: &BlackglassPactState,
    seat: BlackglassSeat,
    team_ranks: [u8; 2],
) -> SeatStanding {
    let team = team_for_seat(seat);
    SeatStanding {
        seat_id: state.seats[seat.index()].clone(),
        team_id: team,
        final_bid: state.bid_for(seat).unwrap_or(Bid::Nil),
        final_hand_tricks: state.tricks_won[seat.index()],
        nil_result: nil_result(state.bid_for(seat), state.tricks_won[seat.index()]),
        team_rank: team_ranks[team.index()],
    }
}

fn wrong_phase_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_WRONG_PHASE".to_owned(),
        message: "hand scoring is only legal after the thirteenth trick".to_owned(),
    }
}

fn incomplete_hand_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_INCOMPLETE_HAND".to_owned(),
        message: "hand scoring requires exactly thirteen completed tricks".to_owned(),
    }
}

fn score_overflow_diagnostic() -> Diagnostic {
    Diagnostic {
        code: "BP_SCORE_OVERFLOW".to_owned(),
        message: "score arithmetic exceeded supported evidence bounds".to_owned(),
    }
}
