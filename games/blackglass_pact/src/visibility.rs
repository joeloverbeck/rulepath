use crate::{
    ids::{BlackglassSeat, TeamId},
    partnerships::team_for_seat,
    state::{Bid, BlackglassPactState},
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

pub fn public_bidding_projection(state: &BlackglassPactState) -> PublicBiddingProjection {
    PublicBiddingProjection {
        bids: public_bid_rows(state),
        team_contracts: public_team_contracts(state),
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
