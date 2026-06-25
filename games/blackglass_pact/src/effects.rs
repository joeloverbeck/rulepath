use crate::{
    cards::CardId,
    ids::{BlackglassSeat, TeamId},
    state::Bid,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlackglassPactEffect {
    BlindNilWindowOpened {
        pending: Vec<BlackglassSeat>,
        threshold: i32,
    },
    BlindNilDeclared {
        seat: BlackglassSeat,
        team: TeamId,
    },
    BlindNilDeclined {
        seat: BlackglassSeat,
    },
    DealCompleted {
        dealer: BlackglassSeat,
        hand_index: u32,
        counts: Vec<(BlackglassSeat, usize)>,
        next_bidder: BlackglassSeat,
    },
    PrivateHandReceived {
        seat: BlackglassSeat,
        cards: Vec<CardId>,
    },
    BidAccepted {
        seat: BlackglassSeat,
        team: TeamId,
        bid: Bid,
    },
}
