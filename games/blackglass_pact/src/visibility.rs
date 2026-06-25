use crate::{
    cards::CardId,
    ids::{BlackglassSeat, TeamId},
    partnerships::team_for_seat,
    state::{Bid, BlackglassPactState, HandScoreBreakdown, MatchOutcome, Phase, PlayedCard},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicBidRow {
    pub seat: BlackglassSeat,
    pub team: TeamId,
    pub bid: Option<Bid>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicTeamContract {
    pub team: TeamId,
    pub ordinary_contract: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBiddingProjection {
    pub bids: Vec<PublicBidRow>,
    pub team_contracts: Vec<PublicTeamContract>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlackglassViewer {
    Observer,
    Seat(BlackglassSeat),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicHandCount {
    pub seat: BlackglassSeat,
    pub count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicView {
    pub viewer: BlackglassViewer,
    pub dealer: BlackglassSeat,
    pub hand_index: u32,
    pub phase: PublicPhase,
    pub spades_broken: bool,
    pub hand_counts: Vec<PublicHandCount>,
    pub bids: Vec<PublicBidRow>,
    pub team_contracts: Vec<PublicTeamContract>,
    pub team_scores: Vec<(TeamId, i32)>,
    pub team_bags: Vec<(TeamId, u8)>,
    pub current_trick: Vec<PlayedCard>,
    pub last_hand_score: Option<HandScoreBreakdown>,
    pub outcome: Option<MatchOutcome>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatView {
    pub public: PublicView,
    pub seat: BlackglassSeat,
    pub own_hand: Vec<CardId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewerView {
    Public(PublicView),
    Seat(SeatView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicPhase {
    BlindNilCommitment {
        active: Option<BlackglassSeat>,
        pending_count: usize,
    },
    Bidding {
        next: BlackglassSeat,
    },
    PlayingTrick {
        leader: BlackglassSeat,
        next: BlackglassSeat,
        trick_index: u8,
    },
    HandScoring {
        completed_tricks: u8,
    },
    Terminal {
        winning_team: TeamId,
    },
}

pub fn observer_view(state: &BlackglassPactState) -> PublicView {
    public_view_for(state, BlackglassViewer::Observer)
}

pub fn seat_view(state: &BlackglassPactState, seat: BlackglassSeat) -> SeatView {
    SeatView {
        public: public_view_for(state, BlackglassViewer::Seat(seat)),
        seat,
        own_hand: state.hand_for_internal(seat).to_vec(),
    }
}

pub fn viewer_view(state: &BlackglassPactState, viewer: BlackglassViewer) -> ViewerView {
    match viewer {
        BlackglassViewer::Observer => ViewerView::Public(observer_view(state)),
        BlackglassViewer::Seat(seat) => ViewerView::Seat(seat_view(state, seat)),
    }
}

pub fn public_bidding_projection(state: &BlackglassPactState) -> PublicBiddingProjection {
    PublicBiddingProjection {
        bids: public_bid_rows(state),
        team_contracts: public_team_contracts(state),
    }
}

fn public_view_for(state: &BlackglassPactState, viewer: BlackglassViewer) -> PublicView {
    PublicView {
        viewer,
        dealer: state.dealer,
        hand_index: state.hand_index,
        phase: public_phase(&state.phase),
        spades_broken: state.spades_broken,
        hand_counts: BlackglassSeat::ALL
            .into_iter()
            .map(|seat| PublicHandCount {
                seat,
                count: state.hand_for_internal(seat).len(),
            })
            .collect(),
        bids: public_bid_rows(state),
        team_contracts: public_team_contracts(state),
        team_scores: TeamId::ALL
            .into_iter()
            .map(|team| (team, state.team_scores[team.index()]))
            .collect(),
        team_bags: TeamId::ALL
            .into_iter()
            .map(|team| (team, state.team_bags[team.index()]))
            .collect(),
        current_trick: match &state.phase {
            Phase::PlayingTrick { plays, .. } => plays.clone(),
            _ => Vec::new(),
        },
        last_hand_score: state.last_hand_score.clone(),
        outcome: state.outcome.clone(),
    }
}

fn public_phase(phase: &Phase) -> PublicPhase {
    match phase {
        Phase::BlindNilCommitment {
            pending,
            next_index,
        } => PublicPhase::BlindNilCommitment {
            active: pending.get(*next_index).copied(),
            pending_count: pending.len().saturating_sub(*next_index),
        },
        Phase::Bidding { next, .. } => PublicPhase::Bidding { next: *next },
        Phase::PlayingTrick {
            leader,
            next,
            trick_index,
            ..
        } => PublicPhase::PlayingTrick {
            leader: *leader,
            next: *next,
            trick_index: *trick_index,
        },
        Phase::HandScoring { completed_tricks } => PublicPhase::HandScoring {
            completed_tricks: *completed_tricks,
        },
        Phase::Terminal { winning_team } => PublicPhase::Terminal {
            winning_team: *winning_team,
        },
    }
}

pub fn public_bid_rows(state: &BlackglassPactState) -> Vec<PublicBidRow> {
    BlackglassSeat::ALL
        .into_iter()
        .map(|seat| PublicBidRow {
            seat,
            team: team_for_seat(seat),
            bid: state.bid_for(seat),
        })
        .collect()
}

pub fn public_team_contracts(state: &BlackglassPactState) -> Vec<PublicTeamContract> {
    TeamId::ALL
        .into_iter()
        .map(|team| PublicTeamContract {
            team,
            ordinary_contract: state.ordinary_team_contract(team),
        })
        .collect()
}
