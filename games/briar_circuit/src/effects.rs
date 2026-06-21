use crate::{cards::CardId, ids::BriarCircuitSeat, state::PassDirection};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassCommitmentStatus {
    pub direction: PassDirection,
    pub committed_count: usize,
    pub pending_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BriarCircuitEffect {
    PassSelectionUpdated {
        seat: BriarCircuitSeat,
        selected_count: usize,
        selected_cards: Vec<CardId>,
    },
    PassCommitmentPublic(PassCommitmentStatus),
    PassExchangePublic {
        direction: PassDirection,
    },
    PassExchangePrivate {
        seat: BriarCircuitSeat,
        sent_cards: Vec<CardId>,
        received_cards: Vec<CardId>,
    },
    CardPlayed {
        seat: BriarCircuitSeat,
        card: CardId,
    },
    HeartsBroken {
        seat: BriarCircuitSeat,
    },
    TrickCaptured {
        trick_index: u8,
        winner: BriarCircuitSeat,
        cards: Vec<CardId>,
    },
}
