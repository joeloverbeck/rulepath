use engine_core::{FreshnessToken, SeatId};

use crate::{
    ids::{CardId, HighCardDuelSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    LeadCommit,
    ReplyCommit,
    Revealed,
    Terminal,
}

impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeadCommit => "lead_commit",
            Self::ReplyCommit => "reply_commit",
            Self::Revealed => "revealed",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Score {
    pub seat_0: u8,
    pub seat_1: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RevealedRound {
    pub round_number: u8,
    pub seat_0_card: CardId,
    pub seat_1_card: CardId,
    pub winner: Option<HighCardDuelSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighCardDuelState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub round_number: u8,
    pub phase: Phase,
    pub lead_seat: HighCardDuelSeat,
    pub score: Score,
    pub hands: [Vec<CardId>; 2],
    pub commitments: [Option<CardId>; 2],
    pub revealed_history: Vec<RevealedRound>,
    pub deck: Vec<CardId>,
    pub freshness_token: FreshnessToken,
}

impl HighCardDuelState {
    pub fn hand_for(&self, seat: HighCardDuelSeat) -> &[CardId] {
        &self.hands[seat.index()]
    }

    pub fn commitment_for(&self, seat: HighCardDuelSeat) -> Option<CardId> {
        self.commitments[seat.index()]
    }

    pub fn internal_card_order(&self) -> Vec<String> {
        let mut cards = Vec::with_capacity(
            self.hands[0].len()
                + self.hands[1].len()
                + self.deck.len()
                + self
                    .commitments
                    .iter()
                    .filter(|card| card.is_some())
                    .count()
                + self.revealed_history.len() * 2,
        );

        for hand in &self.hands {
            cards.extend(hand.iter().map(|card| card.stable_id()));
        }
        cards.extend(
            self.commitments
                .iter()
                .flatten()
                .map(|card| card.stable_id()),
        );
        for round in &self.revealed_history {
            cards.push(round.seat_0_card.stable_id());
            cards.push(round.seat_1_card.stable_id());
        }
        cards.extend(self.deck.iter().map(|card| card.stable_id()));
        cards
    }
}
