use engine_core::{FreshnessToken, SeatId};

use crate::{
    cards::Card,
    ids::{RiverLedgerSeat, MAX_RAISES_PER_STREET, STANDARD_BIG_BET_UNIT, STANDARD_SMALL_BET_UNIT},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl Street {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preflop => "preflop",
            Self::Flop => "flop",
            Self::Turn => "turn",
            Self::River => "river",
        }
    }

    pub const fn unit(self) -> u8 {
        match self {
            Self::Preflop | Self::Flop => STANDARD_SMALL_BET_UNIT,
            Self::Turn | Self::River => STANDARD_BIG_BET_UNIT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Setup,
    Betting { street: Street },
    Showdown,
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SeatStatus {
    Live,
    Folded,
    ShowdownEligible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatLedger {
    pub seat: RiverLedgerSeat,
    pub status: SeatStatus,
    pub street_contribution: u16,
    pub total_contribution: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionLedger {
    pub seats: Vec<SeatLedger>,
    pub pot_total: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BettingRoundState {
    pub street: Street,
    pub current_to_call: u16,
    pub raises_this_street: u8,
    pub last_aggressor: Option<RiverLedgerSeat>,
}

impl BettingRoundState {
    pub const fn for_street(street: Street) -> Self {
        Self {
            street,
            current_to_call: 0,
            raises_this_street: 0,
            last_aggressor: None,
        }
    }

    pub const fn raise_cap_reached(self) -> bool {
        self.raises_this_street >= MAX_RAISES_PER_STREET
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownReveal {
    pub seat: RiverLedgerSeat,
    pub hole_cards: [Card; 2],
    pub best_five: [Card; 5],
    pub category: String,
    pub tie_break_vector: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShowdownSeatExplanation {
    pub seat: RiverLedgerSeat,
    pub status: SeatStatus,
    pub revealed: Option<ShowdownReveal>,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalOutcome {
    LastLiveHand {
        winner: RiverLedgerSeat,
        pot_total: u16,
    },
    Showdown {
        winners: Vec<RiverLedgerSeat>,
        pot_total: u16,
        explanations: Vec<ShowdownSeatExplanation>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RiverLedgerState {
    pub variant: Variant,
    pub seats: Vec<SeatId>,
    pub phase: Phase,
    pub button: RiverLedgerSeat,
    pub small_blind: RiverLedgerSeat,
    pub big_blind: RiverLedgerSeat,
    pub active_seat: Option<RiverLedgerSeat>,
    pub board: Vec<Card>,
    pub ledger: ContributionLedger,
    pub betting: BettingRoundState,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}
