use crate::{cards::CardId, ids::VowTideSeat};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VowTideEffect {
    BidAccepted {
        seat: VowTideSeat,
        bid: u8,
        public_total: u8,
    },
    DealerHookConstrained {
        dealer: VowTideSeat,
        forbidden_bid: u8,
        hand_size: u8,
        public_total_before_dealer: u8,
    },
    BiddingCompleted {
        first_leader: VowTideSeat,
    },
    CardPlayed {
        seat: VowTideSeat,
        card: CardId,
        trick_index: u8,
    },
    TrickCaptured {
        trick_index: u8,
        winner: VowTideSeat,
        cards: Vec<CardId>,
    },
    HandScored {
        hand_index: u32,
        additions: Vec<(VowTideSeat, i16)>,
        cumulative_scores: Vec<(VowTideSeat, i16)>,
    },
    HandAdvanced {
        hand_index: u32,
        dealer: VowTideSeat,
        hand_size: u8,
        trump_indicator: CardId,
    },
    MatchCompleted {
        winners: Vec<VowTideSeat>,
    },
}
