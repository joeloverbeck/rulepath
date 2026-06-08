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
    pub scores: [u32; 2],
    pub revealed_history: Vec<RevealedRound>,
    pub priority_seat: SecretDraftSeat,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl SecretDraftState {
    pub fn drafted_for(&self, seat: SecretDraftSeat) -> &[DraftItemId] {
        &self.drafted[seat.index()]
    }

    pub fn score_for(&self, seat: SecretDraftSeat) -> u32 {
        self.scores[seat.index()]
    }

    pub fn commitment_for_internal(&self, seat: SecretDraftSeat) -> Option<DraftItemId> {
        self.commitments[seat.index()]
    }

    pub(crate) fn set_commitment_for_internal(&mut self, seat: SecretDraftSeat, item: DraftItemId) {
        self.commitments[seat.index()] = Some(item);
    }

    pub(crate) fn clear_commitments_internal(&mut self) {
        self.commitments = [None, None];
    }

    pub fn seat_committed(&self, seat: SecretDraftSeat) -> bool {
        self.commitments[seat.index()].is_some()
    }

    pub fn empty_commitments(&self) -> bool {
        self.commitments == [None, None]
    }

    pub fn stable_summary(&self) -> String {
        format!(
            "variant={};round={};phase={};priority={};pool={};drafted_0={};drafted_1={};commitment_slots={};fallback_awards={},{};priority_conflict_wins={},{};scores={},{};revealed={};terminal={};freshness={}",
            self.variant.id,
            self.round_number,
            self.phase.as_str(),
            self.priority_seat.as_str(),
            stable_items(&self.visible_pool),
            stable_items(&self.drafted[0]),
            stable_items(&self.drafted[1]),
            stable_commitment_slots(self.commitments),
            self.fallback_awards[0],
            self.fallback_awards[1],
            self.priority_conflict_wins[0],
            self.priority_conflict_wins[1],
            self.scores[0],
            self.scores[1],
            self.revealed_history.len(),
            match self.terminal_outcome {
                Some(TerminalOutcome::Win { seat }) => format!("win:{}", seat.as_str()),
                Some(TerminalOutcome::Draw) => "draw".to_owned(),
                None => "none".to_owned(),
            },
            self.freshness_token.0,
        )
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
            scores: [0, 0],
            revealed_history: Vec::new(),
            priority_seat: SecretDraftSeat::Seat0,
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }
}

fn stable_items(items: &[DraftItemId]) -> String {
    if items.is_empty() {
        return "none".to_owned();
    }
    items
        .iter()
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_commitment_slots(commitments: [Option<DraftItemId>; 2]) -> String {
    commitments
        .iter()
        .map(|commitment| commitment.map(|item| item.as_str()).unwrap_or("empty"))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variants::Variant;

    #[test]
    fn initial_state_stable_summary_is_explicit() {
        let state = SecretDraftState::new_with_empty_commitments(
            Variant::secret_draft_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            DraftItemId::ALL.to_vec(),
        );

        assert_eq!(
            state.stable_summary(),
            "variant=secret_draft_standard;round=1;phase=commit;priority=seat_0;pool=ember_1,ember_2,ember_3,ember_4,tide_1,tide_2,tide_3,tide_4,grove_1,grove_2,grove_3,grove_4;drafted_0=none;drafted_1=none;commitment_slots=empty,empty;fallback_awards=0,0;priority_conflict_wins=0,0;scores=0,0;revealed=0;terminal=none;freshness=0"
        );
    }
}
