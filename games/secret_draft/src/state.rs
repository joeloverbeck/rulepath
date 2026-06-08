use engine_core::{FreshnessToken, SeatId};

use crate::{
    ids::{DraftItemId, DraftThread, SecretDraftSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Commit,
    Terminal,
}

impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Commit => "commit",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DraftItemSpec {
    pub id: DraftItemId,
    pub label: &'static str,
    pub thread: DraftThread,
    pub value: u8,
}

pub const STANDARD_ITEMS: [DraftItemSpec; 12] = [
    item(DraftItemId::Ember1),
    item(DraftItemId::Ember2),
    item(DraftItemId::Ember3),
    item(DraftItemId::Ember4),
    item(DraftItemId::Tide1),
    item(DraftItemId::Tide2),
    item(DraftItemId::Tide3),
    item(DraftItemId::Tide4),
    item(DraftItemId::Grove1),
    item(DraftItemId::Grove2),
    item(DraftItemId::Grove3),
    item(DraftItemId::Grove4),
];

const fn item(id: DraftItemId) -> DraftItemSpec {
    DraftItemSpec {
        id,
        label: id.label(),
        thread: id.thread(),
        value: id.value(),
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ScoreSummary {
    pub base_value: u32,
    pub complete_sets: u8,
    pub high_thread_bonus_count: u8,
    pub conflict_discipline_bonus: u8,
    pub total: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RevealedRound {
    pub round_number: u8,
    pub seat_0_choice: DraftItemId,
    pub seat_1_choice: DraftItemId,
    pub seat_0_award: DraftItemId,
    pub seat_1_award: DraftItemId,
    pub priority_seat: SecretDraftSeat,
    pub contested: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    Win { seat: SecretDraftSeat },
    Draw,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecretDraftState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub round_number: u8,
    pub phase: Phase,
    pub visible_pool: Vec<DraftItemId>,
    pub drafted: [Vec<DraftItemId>; 2],
    commitments: [Option<DraftItemId>; 2],
    pub fallback_awards: [u8; 2],
    pub priority_conflict_wins: [u8; 2],
    pub scores: [ScoreSummary; 2],
    pub revealed_history: Vec<RevealedRound>,
    pub priority_seat: SecretDraftSeat,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl SecretDraftState {
    pub fn drafted_for(&self, seat: SecretDraftSeat) -> &[DraftItemId] {
        &self.drafted[seat.index()]
    }

    pub fn commitment_for_internal(&self, seat: SecretDraftSeat) -> Option<DraftItemId> {
        self.commitments[seat.index()]
    }

    pub fn seat_committed(&self, seat: SecretDraftSeat) -> bool {
        self.commitments[seat.index()].is_some()
    }

    pub fn empty_commitments(&self) -> bool {
        self.commitments == [None, None]
    }

    pub(crate) fn new_with_empty_commitments(
        variant: Variant,
        seats: [SeatId; 2],
        visible_pool: Vec<DraftItemId>,
    ) -> Self {
        Self {
            variant,
            seats,
            round_number: 1,
            phase: Phase::Commit,
            visible_pool,
            drafted: [Vec::new(), Vec::new()],
            commitments: [None, None],
            fallback_awards: [0, 0],
            priority_conflict_wins: [0, 0],
            scores: [ScoreSummary::default(), ScoreSummary::default()],
            revealed_history: Vec::new(),
            priority_seat: SecretDraftSeat::Seat0,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }
}
