use engine_core::{EffectEnvelope, SeatId, VisibilityScope};

use crate::{
    ids::{CardId, HighCardDuelSeat},
    state::Score,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HighCardDuelEffect {
    DealPrivateCard {
        owner: HighCardDuelSeat,
        card: CardId,
    },
    HandCountChanged {
        seat_0_count: u8,
        seat_1_count: u8,
        deck_count: u8,
    },
    CommitFaceDown {
        seat: HighCardDuelSeat,
        round_number: u8,
    },
    OwnCommitConfirmed {
        owner: HighCardDuelSeat,
        card: CardId,
        round_number: u8,
    },
    CardsRevealed {
        round_number: u8,
        seat_0_card: CardId,
        seat_1_card: CardId,
    },
    RoundScored {
        round_number: u8,
        winner: Option<HighCardDuelSeat>,
        score: Score,
    },
    RefillStarted {
        next_round_number: u8,
        next_lead_seat: HighCardDuelSeat,
    },
    Terminal {
        winner: Option<HighCardDuelSeat>,
        score: Score,
    },
    PrivateDiagnostic {
        owner: HighCardDuelSeat,
        code: String,
        private_message: String,
    },
    PublicDiagnostic {
        code: String,
        public_message: String,
    },
}

impl HighCardDuelEffect {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::DealPrivateCard { .. } => "hcd_deal_private_card",
            Self::HandCountChanged { .. } => "hcd_hand_count_changed",
            Self::CommitFaceDown { .. } => "hcd_commit_face_down",
            Self::OwnCommitConfirmed { .. } => "hcd_own_commit_confirmed",
            Self::CardsRevealed { .. } => "hcd_cards_revealed",
            Self::RoundScored { .. } => "hcd_round_scored",
            Self::RefillStarted { .. } => "hcd_refill_started",
            Self::Terminal { .. } => "hcd_terminal",
            Self::PrivateDiagnostic { .. } => "hcd_private_diagnostic",
            Self::PublicDiagnostic { .. } => "hcd_public_diagnostic",
        }
    }

    pub fn public_payload_text(&self) -> String {
        match self {
            Self::DealPrivateCard { .. }
            | Self::OwnCommitConfirmed { .. }
            | Self::PrivateDiagnostic { .. } => "private".to_owned(),
            Self::HandCountChanged {
                seat_0_count,
                seat_1_count,
                deck_count,
            } => format!(
                "{}:seat_0_count={seat_0_count};seat_1_count={seat_1_count};deck_count={deck_count}",
                self.kind()
            ),
            Self::CommitFaceDown { seat, round_number } => {
                format!("{}:seat={};round={round_number}", self.kind(), seat.as_str())
            }
            Self::CardsRevealed {
                round_number,
                seat_0_card,
                seat_1_card,
            } => format!(
                "{}:round={round_number};seat_0_card={};seat_1_card={}",
                self.kind(),
                seat_0_card.stable_id(),
                seat_1_card.stable_id()
            ),
            Self::RoundScored {
                round_number,
                winner,
                score,
            } => format!(
                "{}:round={round_number};winner={};score={}-{}",
                self.kind(),
                seat_summary(*winner),
                score.seat_0,
                score.seat_1
            ),
            Self::RefillStarted {
                next_round_number,
                next_lead_seat,
            } => format!(
                "{}:next_round={next_round_number};next_lead={}",
                self.kind(),
                next_lead_seat.as_str()
            ),
            Self::Terminal { winner, score } => format!(
                "{}:winner={};score={}-{}",
                self.kind(),
                seat_summary(*winner),
                score.seat_0,
                score.seat_1
            ),
            Self::PublicDiagnostic {
                code,
                public_message,
            } => format!("{}:code={code};message={public_message}", self.kind()),
        }
    }
}

pub fn public_effect(payload: HighCardDuelEffect) -> EffectEnvelope<HighCardDuelEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::Public,
        payload,
    }
}

pub fn private_effect(
    owner_seat_id: SeatId,
    payload: HighCardDuelEffect,
) -> EffectEnvelope<HighCardDuelEffect> {
    EffectEnvelope {
        visibility: VisibilityScope::PrivateToSeat(owner_seat_id),
        payload,
    }
}

pub fn deal_private_card_effect(
    owner: HighCardDuelSeat,
    owner_seat_id: SeatId,
    card: CardId,
) -> EffectEnvelope<HighCardDuelEffect> {
    private_effect(
        owner_seat_id,
        HighCardDuelEffect::DealPrivateCard { owner, card },
    )
}

pub fn hand_count_changed_effect(
    seat_0_count: u8,
    seat_1_count: u8,
    deck_count: u8,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::HandCountChanged {
        seat_0_count,
        seat_1_count,
        deck_count,
    })
}

pub fn commit_face_down_effect(
    seat: HighCardDuelSeat,
    round_number: u8,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::CommitFaceDown { seat, round_number })
}

pub fn own_commit_confirmed_effect(
    owner: HighCardDuelSeat,
    owner_seat_id: SeatId,
    card: CardId,
    round_number: u8,
) -> EffectEnvelope<HighCardDuelEffect> {
    private_effect(
        owner_seat_id,
        HighCardDuelEffect::OwnCommitConfirmed {
            owner,
            card,
            round_number,
        },
    )
}

pub fn cards_revealed_effect(
    round_number: u8,
    seat_0_card: CardId,
    seat_1_card: CardId,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::CardsRevealed {
        round_number,
        seat_0_card,
        seat_1_card,
    })
}

pub fn round_scored_effect(
    round_number: u8,
    winner: Option<HighCardDuelSeat>,
    score: Score,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::RoundScored {
        round_number,
        winner,
        score,
    })
}

pub fn refill_started_effect(
    next_round_number: u8,
    next_lead_seat: HighCardDuelSeat,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::RefillStarted {
        next_round_number,
        next_lead_seat,
    })
}

pub fn terminal_effect(
    winner: Option<HighCardDuelSeat>,
    score: Score,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::Terminal { winner, score })
}

pub fn private_diagnostic_effect(
    owner: HighCardDuelSeat,
    owner_seat_id: SeatId,
    code: impl Into<String>,
    private_message: impl Into<String>,
) -> EffectEnvelope<HighCardDuelEffect> {
    private_effect(
        owner_seat_id,
        HighCardDuelEffect::PrivateDiagnostic {
            owner,
            code: code.into(),
            private_message: private_message.into(),
        },
    )
}

pub fn public_diagnostic_effect(
    code: impl Into<String>,
    public_message: impl Into<String>,
) -> EffectEnvelope<HighCardDuelEffect> {
    public_effect(HighCardDuelEffect::PublicDiagnostic {
        code: code.into(),
        public_message: public_message.into(),
    })
}

fn seat_summary(seat: Option<HighCardDuelSeat>) -> &'static str {
    seat.map_or("draw", HighCardDuelSeat::as_str)
}
