use crate::ids::{DraftItemId, SecretDraftSeat};
use crate::state::TerminalOutcome;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecretDraftEffect {
    CommitmentPlaced {
        seat: SecretDraftSeat,
        round: u8,
    },
    OwnCommitAccepted {
        seat: SecretDraftSeat,
        round: u8,
    },
    PendingSeatsChanged {
        round: u8,
        seat_0_committed: bool,
        seat_1_committed: bool,
    },
    RevealBatchStarted {
        round: u8,
        group_id: String,
    },
    ChoicesRevealed {
        round: u8,
        seat_0_item: DraftItemId,
        seat_1_item: DraftItemId,
    },
    DraftResolved {
        round: u8,
        seat_0_award: DraftItemId,
        seat_1_award: DraftItemId,
        removed_items: [DraftItemId; 2],
        conflict: Option<ConflictSummary>,
    },
    PoolChanged {
        remaining_count: u8,
    },
    ScoreChanged {
        scores: [u32; 2],
        tie_break_summary: TieBreakSummary,
    },
    RoundAdvanced {
        next_round: u8,
        priority_seat: SecretDraftSeat,
    },
    Terminal {
        outcome: TerminalOutcome,
        final_scores: [u32; 2],
        tie_break_summary: TieBreakSummary,
    },
    PublicDiagnostic {
        code: String,
        message: String,
    },
    PrivateDiagnostic {
        seat: SecretDraftSeat,
        code: String,
        message: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConflictSummary {
    pub contested_item: DraftItemId,
    pub priority_seat: SecretDraftSeat,
    pub fallback_item: DraftItemId,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TieBreakSummary {
    pub scores: [u32; 2],
    pub complete_sets: [u8; 2],
    pub highest_single_values: [u8; 2],
    pub distinct_threads: [u8; 2],
    pub priority_conflict_wins: [u8; 2],
}

pub fn public_effect(payload: SecretDraftEffect) -> engine_core::EffectEnvelope<SecretDraftEffect> {
    engine_core::EffectEnvelope {
        visibility: engine_core::VisibilityScope::Public,
        payload,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_reveal_effect_variants_do_not_carry_item_ids() {
        let effects = [
            SecretDraftEffect::CommitmentPlaced {
                seat: SecretDraftSeat::Seat0,
                round: 1,
            },
            SecretDraftEffect::OwnCommitAccepted {
                seat: SecretDraftSeat::Seat0,
                round: 1,
            },
            SecretDraftEffect::PendingSeatsChanged {
                round: 1,
                seat_0_committed: true,
                seat_1_committed: false,
            },
        ];

        let text = format!("{effects:?}");
        for item in DraftItemId::ALL {
            assert!(!text.contains(item.as_str()));
        }
    }
}
