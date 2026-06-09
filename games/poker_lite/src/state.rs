use engine_core::{FreshnessToken, SeatId};

use crate::{
    ids::{CrestCardId, PokerLiteSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    PledgeRound { round_index: u8 },
    Terminal,
}

impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PledgeRound { round_index: 0 } => "pledge_round_1",
            Self::PledgeRound { round_index: 1 } => "pledge_round_2",
            Self::PledgeRound { .. } => "pledge_round_unknown",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PledgeRoundState {
    pub round_index: u8,
    pub unit: u8,
    pub outstanding_actor: Option<PokerLiteSeat>,
    pub outstanding_amount: u8,
    pub lift_used: bool,
    pub consecutive_holds: u8,
}

impl PledgeRoundState {
    pub const fn initial() -> Self {
        Self::for_round(0)
    }

    pub const fn for_round(round_index: u8) -> Self {
        Self {
            round_index,
            unit: match round_index {
                0 => 1,
                1 => 2,
                _ => 0,
            },
            outstanding_actor: None,
            outstanding_amount: 0,
            lift_used: false,
            consecutive_holds: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ShowdownReveal {
    pub seat_0_private: CrestCardId,
    pub seat_1_private: CrestCardId,
    pub center: CrestCardId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalReveal {
    Showdown(ShowdownReveal),
    NoShowdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    YieldWin {
        winner: PokerLiteSeat,
        loser: PokerLiteSeat,
        shared_pool: u8,
        contributions: [u8; 2],
    },
    ShowdownWin {
        winner: PokerLiteSeat,
        shared_pool: u8,
        contributions: [u8; 2],
        reveal: ShowdownReveal,
    },
    Split {
        shared_pool: u8,
        each: u8,
        contributions: [u8; 2],
        reveal: ShowdownReveal,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PokerLiteState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub phase: Phase,
    pub active_seat: Option<PokerLiteSeat>,
    private_cards: [CrestCardId; 2],
    center_card: CrestCardId,
    pub center_visible: bool,
    deck_tail: Vec<CrestCardId>,
    pub contributions: [u8; 2],
    pub shared_pool: u8,
    pub round: PledgeRoundState,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl PokerLiteState {
    pub fn private_card_for_internal(&self, seat: PokerLiteSeat) -> CrestCardId {
        self.private_cards[seat.index()]
    }

    pub fn private_cards_internal(&self) -> [CrestCardId; 2] {
        self.private_cards
    }

    pub fn center_card_internal(&self) -> CrestCardId {
        self.center_card
    }

    pub fn deck_tail_internal(&self) -> &[CrestCardId] {
        &self.deck_tail
    }

    pub fn stable_internal_summary(&self) -> String {
        format!(
            "variant={};phase={};active={};private={};center={};center_visible={};deck_tail={};contributions={},{};shared_pool={};round={}:unit{}:outstanding={}:amount{}:lift_used{}:holds{};terminal={};freshness={}",
            self.variant.id,
            self.phase.as_str(),
            self.active_seat
                .map(PokerLiteSeat::as_str)
                .unwrap_or("none"),
            stable_cards(&self.private_cards),
            self.center_card.as_str(),
            self.center_visible,
            stable_cards(&self.deck_tail),
            self.contributions[0],
            self.contributions[1],
            self.shared_pool,
            self.round.round_index,
            self.round.unit,
            self.round
                .outstanding_actor
                .map(PokerLiteSeat::as_str)
                .unwrap_or("none"),
            self.round.outstanding_amount,
            self.round.lift_used,
            self.round.consecutive_holds,
            stable_terminal(self.terminal_outcome),
            self.freshness_token.0,
        )
    }

    pub(crate) fn new_after_deal(
        variant: Variant,
        seats: [SeatId; 2],
        private_cards: [CrestCardId; 2],
        center_card: CrestCardId,
        deck_tail: Vec<CrestCardId>,
    ) -> Self {
        Self {
            variant,
            seats,
            phase: Phase::PledgeRound { round_index: 0 },
            active_seat: Some(PokerLiteSeat::Seat0),
            private_cards,
            center_card,
            center_visible: false,
            deck_tail,
            contributions: [1, 1],
            shared_pool: 2,
            round: PledgeRoundState::initial(),
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }
}

fn stable_cards(cards: &[CrestCardId]) -> String {
    if cards.is_empty() {
        return "none".to_owned();
    }
    cards
        .iter()
        .map(|card| card.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_terminal(terminal: Option<TerminalOutcome>) -> String {
    match terminal {
        Some(TerminalOutcome::YieldWin { winner, loser, .. }) => {
            format!("yield_win:{}:{}", winner.as_str(), loser.as_str())
        }
        Some(TerminalOutcome::ShowdownWin { winner, .. }) => {
            format!("showdown_win:{}", winner.as_str())
        }
        Some(TerminalOutcome::Split { each, .. }) => format!("split:{each}"),
        None => "none".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_summary_keeps_hidden_fields_internal() {
        let state = PokerLiteState::new_after_deal(
            Variant::poker_lite_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            [CrestCardId::LowDawn, CrestCardId::MiddleDusk],
            CrestCardId::HighDawn,
            vec![
                CrestCardId::LowDusk,
                CrestCardId::MiddleDawn,
                CrestCardId::HighDusk,
            ],
        );

        assert_eq!(
            state.private_card_for_internal(PokerLiteSeat::Seat0),
            CrestCardId::LowDawn
        );
        assert_eq!(state.center_card_internal(), CrestCardId::HighDawn);
        assert_eq!(state.deck_tail_internal().len(), 3);
        assert_eq!(
            state.stable_internal_summary(),
            "variant=poker_lite_standard;phase=pledge_round_1;active=seat_0;private=low_dawn,middle_dusk;center=high_dawn;center_visible=false;deck_tail=low_dusk,middle_dawn,high_dusk;contributions=1,1;shared_pool=2;round=0:unit1:outstanding=none:amount0:lift_usedfalse:holds0;terminal=none;freshness=0"
        );
    }
}
