//! Match, round, hand, discard, stock, and public tableau state for Meldfall Ledger.
//!
//! The state meanings are game-local and are not engine-core vocabulary.

use engine_core::SeatId;

use crate::{
    cards::{CardId, Rank, Suit},
    setup::InitialSetup,
    variants::Variant,
};

pub type SeatIndex = usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MeldId(pub u32);

impl MeldId {
    pub fn as_string(self) -> String {
        format!("meld_{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TurnOrdinal(pub u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchState {
    pub variant: Variant,
    pub seats: Vec<SeatId>,
    pub cumulative_scores: Vec<i32>,
    pub dealer_index: SeatIndex,
    pub round: RoundState,
    pub terminal: Option<MatchOutcome>,
}

impl MatchState {
    pub fn from_initial_setup(setup: InitialSetup) -> Self {
        let seat_count = setup.seats.len();
        let seats = setup.seats.clone();
        let variant = setup.variant.clone();
        let dealer_index = setup.dealer_index;
        let round = RoundState::from_initial_setup(setup);
        Self {
            variant,
            seats,
            cumulative_scores: vec![0; seat_count],
            dealer_index,
            round,
            terminal: None,
        }
    }

    pub fn stable_internal_summary(&self) -> String {
        let seats = self
            .seats
            .iter()
            .map(|seat| seat.0.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let scores = self
            .cumulative_scores
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let terminal = self
            .terminal
            .as_ref()
            .map(MatchOutcome::stable_string)
            .unwrap_or_else(|| "none".to_owned());
        format!(
            "match|variant={}|seats=[{}]|scores=[{}]|dealer={}|round={}|terminal={}",
            self.variant.id,
            seats,
            scores,
            self.dealer_index,
            self.round.stable_internal_summary(),
            terminal
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundState {
    pub active_seat_index: SeatIndex,
    pub phase: TurnPhase,
    pub stock: Vec<CardId>,
    pub discard: Vec<CardId>,
    pub tableau: MeldTableau,
    pub pending_pickup: Option<DiscardPickupCommitment>,
    pub round_played_scores: Vec<i32>,
    pub seats: Vec<SeatState>,
}

impl RoundState {
    pub fn from_initial_setup(setup: InitialSetup) -> Self {
        let seat_count = setup.seats.len();
        Self {
            active_seat_index: setup.active_seat_index,
            phase: TurnPhase::Draw,
            stock: setup.stock,
            discard: vec![setup.initial_discard],
            tableau: MeldTableau::default(),
            pending_pickup: None,
            round_played_scores: vec![0; seat_count],
            seats: setup
                .private_hands
                .into_iter()
                .map(|hand| SeatState { hand })
                .collect(),
        }
    }

    pub fn stable_internal_summary(&self) -> String {
        let stock = card_list(&self.stock);
        let discard = card_list(&self.discard);
        let scores = self
            .round_played_scores
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let hands = self
            .seats
            .iter()
            .enumerate()
            .map(|(index, seat)| format!("{index}:{}", card_list(&seat.hand)))
            .collect::<Vec<_>>()
            .join(";");
        let pending = self
            .pending_pickup
            .as_ref()
            .map(DiscardPickupCommitment::stable_string)
            .unwrap_or_else(|| "none".to_owned());
        format!(
            "round|active={}|phase={}|stock=[{}]|discard=[{}]|tableau=[{}]|pending={}|played_scores=[{}]|hands=[{}]",
            self.active_seat_index,
            self.phase.as_str(),
            stock,
            discard,
            self.tableau.stable_string(),
            pending,
            scores,
            hands
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TurnPhase {
    Draw,
    Table,
    Discard,
    RoundSettled,
    MatchComplete,
}

impl TurnPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draw => "draw",
            Self::Table => "table",
            Self::Discard => "discard",
            Self::RoundSettled => "round_settled",
            Self::MatchComplete => "match_complete",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatState {
    pub hand: Vec<CardId>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MeldTableau {
    pub groups: Vec<MeldGroup>,
}

impl MeldTableau {
    pub fn stable_string(&self) -> String {
        self.groups
            .iter()
            .map(MeldGroup::stable_string)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeldGroup {
    pub id: MeldId,
    pub kind: MeldKind,
    pub origin_seat: SeatIndex,
    pub cards: Vec<TableCard>,
}

impl MeldGroup {
    pub fn stable_string(&self) -> String {
        let cards = self
            .cards
            .iter()
            .map(TableCard::stable_string)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}:{}:origin={}:cards=[{}]",
            self.id.as_string(),
            self.kind.stable_string(),
            self.origin_seat,
            cards
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeldKind {
    Set { rank: Rank },
    Run { suit: Suit },
    Unknown,
}

impl MeldKind {
    pub fn stable_string(&self) -> String {
        match self {
            Self::Set { rank } => format!("set:{}", rank.as_str()),
            Self::Run { suit } => format!("run:{}", suit.as_str()),
            Self::Unknown => "unknown".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableCard {
    pub card: CardId,
    pub played_by: SeatIndex,
    pub score_credit_owner: SeatIndex,
    pub play_turn: TurnOrdinal,
}

impl TableCard {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:played_by={}:credit={}:turn={}",
            self.card.as_str(),
            self.played_by,
            self.score_credit_owner,
            self.play_turn.0
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscardPickupCommitment {
    pub selected_card: CardId,
    pub source_discard_index: usize,
    pub required_by_seat: SeatIndex,
}

impl DiscardPickupCommitment {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:discard_index={}:seat={}",
            self.selected_card.as_str(),
            self.source_discard_index,
            self.required_by_seat
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchOutcome {
    pub standings: Vec<SeatStanding>,
    pub winner: Option<SeatIndex>,
}

impl MatchOutcome {
    pub fn stable_string(&self) -> String {
        let standings = self
            .standings
            .iter()
            .map(SeatStanding::stable_string)
            .collect::<Vec<_>>()
            .join(",");
        let winner = self
            .winner
            .map(|seat| seat.to_string())
            .unwrap_or_else(|| "none".to_owned());
        format!("winner={winner}:standings=[{standings}]")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeatStanding {
    pub seat_index: SeatIndex,
    pub cumulative_score: i32,
    pub latest_round_delta: i32,
    pub rank: usize,
    pub winner: bool,
}

impl SeatStanding {
    pub fn stable_string(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.seat_index, self.cumulative_score, self.latest_round_delta, self.rank, self.winner
        )
    }
}

pub fn card_list(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| card.as_str())
        .collect::<Vec<_>>()
        .join(",")
}
