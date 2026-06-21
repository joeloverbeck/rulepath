use crate::ids::VowTideSeat;

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
}
