use engine_core::{FreshnessToken, SeatId, SeededRng};

use crate::{
    ids::{Grade, MaskTileId, MaskedClaimsSeat},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Phase {
    Claim {
        turn_index: u8,
    },
    Reaction {
        turn_index: u8,
        responder: MaskedClaimsSeat,
    },
    Terminal,
}

impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Claim { turn_index: 0 } => "turn_1_claim",
            Self::Claim { turn_index: 1 } => "turn_2_claim",
            Self::Claim { turn_index: 2 } => "turn_3_claim",
            Self::Claim { turn_index: 3 } => "turn_4_claim",
            Self::Claim { turn_index: 4 } => "turn_5_claim",
            Self::Claim { turn_index: 5 } => "turn_6_claim",
            Self::Claim { turn_index: 6 } => "turn_7_claim",
            Self::Claim { turn_index: 7 } => "turn_8_claim",
            Self::Claim { .. } => "turn_unknown_claim",
            Self::Reaction {
                turn_index: 0,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_1_reaction_seat_0",
            Self::Reaction {
                turn_index: 0,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_1_reaction_seat_1",
            Self::Reaction {
                turn_index: 1,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_2_reaction_seat_0",
            Self::Reaction {
                turn_index: 1,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_2_reaction_seat_1",
            Self::Reaction {
                turn_index: 2,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_3_reaction_seat_0",
            Self::Reaction {
                turn_index: 2,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_3_reaction_seat_1",
            Self::Reaction {
                turn_index: 3,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_4_reaction_seat_0",
            Self::Reaction {
                turn_index: 3,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_4_reaction_seat_1",
            Self::Reaction {
                turn_index: 4,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_5_reaction_seat_0",
            Self::Reaction {
                turn_index: 4,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_5_reaction_seat_1",
            Self::Reaction {
                turn_index: 5,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_6_reaction_seat_0",
            Self::Reaction {
                turn_index: 5,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_6_reaction_seat_1",
            Self::Reaction {
                turn_index: 6,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_7_reaction_seat_0",
            Self::Reaction {
                turn_index: 6,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_7_reaction_seat_1",
            Self::Reaction {
                turn_index: 7,
                responder: MaskedClaimsSeat::Seat0,
            } => "turn_8_reaction_seat_0",
            Self::Reaction {
                turn_index: 7,
                responder: MaskedClaimsSeat::Seat1,
            } => "turn_8_reaction_seat_1",
            Self::Reaction { .. } => "turn_unknown_reaction",
            Self::Terminal => "terminal",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PendingClaim {
    pub claimant: MaskedClaimsSeat,
    pub tile: MaskTileId,
    pub declared: Grade,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct VeiledClaim {
    pub declared: Grade,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ExposedMask {
    pub tile: MaskTileId,
    pub declared: Grade,
    pub claimant: MaskedClaimsSeat,
    pub challenger: MaskedClaimsSeat,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ChallengeCounters {
    pub exposed_lies: u8,
    pub successful_challenges: u8,
    pub challenges_declared: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TerminalOutcome {
    ScoreWin {
        winner: MaskedClaimsSeat,
        scores: [u8; 2],
    },
    TiebreakWin {
        winner: MaskedClaimsSeat,
        scores: [u8; 2],
        tiebreak: &'static str,
    },
    Draw {
        scores: [u8; 2],
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskedClaimsState {
    pub variant: Variant,
    pub seats: [SeatId; 2],
    pub phase: Phase,
    pub active_seat: Option<MaskedClaimsSeat>,
    pub turn_index: u8,
    pub claimant: MaskedClaimsSeat,
    pub(crate) hands: [Vec<MaskTileId>; 2],
    pub(crate) reserve: Vec<MaskTileId>,
    pub(crate) rng: SeededRng,
    pub pedestal: Option<PendingClaim>,
    pub veiled_gallery: [Vec<VeiledClaim>; 2],
    pub exposed_row: [Vec<ExposedMask>; 2],
    pub scores: [u8; 2],
    pub counters: [ChallengeCounters; 2],
    pub(crate) effect_history: Vec<String>,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}

impl MaskedClaimsState {
    #[allow(dead_code)]
    pub(crate) fn hand_for_internal(&self, seat: MaskedClaimsSeat) -> &[MaskTileId] {
        &self.hands[seat.index()]
    }

    #[allow(dead_code)]
    pub(crate) fn hands_internal(&self) -> &[Vec<MaskTileId>; 2] {
        &self.hands
    }

    #[allow(dead_code)]
    pub(crate) fn reserve_internal(&self) -> &[MaskTileId] {
        &self.reserve
    }

    #[allow(dead_code)]
    pub(crate) fn effect_history_internal(&self) -> &[String] {
        &self.effect_history
    }

    pub fn stable_internal_summary(&self) -> String {
        format!(
            "variant={};phase={};active={};turn_index={};claimant={};hands={}|{};reserve={};pedestal={};veiled={}|{};exposed={}|{};scores={},{};counters={}|{};effects={};terminal={};freshness={}",
            self.variant.id,
            self.phase.as_str(),
            self.active_seat
                .map(MaskedClaimsSeat::as_str)
                .unwrap_or("none"),
            self.turn_index,
            self.claimant.as_str(),
            stable_masks(&self.hands[0]),
            stable_masks(&self.hands[1]),
            stable_masks(&self.reserve),
            stable_pedestal(self.pedestal),
            stable_veiled(&self.veiled_gallery[0]),
            stable_veiled(&self.veiled_gallery[1]),
            stable_exposed(&self.exposed_row[0]),
            stable_exposed(&self.exposed_row[1]),
            self.scores[0],
            self.scores[1],
            stable_counters(self.counters[0]),
            stable_counters(self.counters[1]),
            stable_effect_history(&self.effect_history),
            stable_terminal(self.terminal_outcome),
            self.freshness_token.0,
        )
    }

    pub(crate) fn new_after_deal(
        variant: Variant,
        seats: [SeatId; 2],
        hands: [Vec<MaskTileId>; 2],
        reserve: Vec<MaskTileId>,
        rng: SeededRng,
    ) -> Self {
        Self {
            variant,
            seats,
            phase: Phase::Claim { turn_index: 0 },
            active_seat: Some(MaskedClaimsSeat::Seat0),
            turn_index: 0,
            claimant: MaskedClaimsSeat::Seat0,
            hands,
            reserve,
            rng,
            pedestal: None,
            veiled_gallery: [Vec::new(), Vec::new()],
            exposed_row: [Vec::new(), Vec::new()],
            scores: [0, 0],
            counters: [ChallengeCounters::default(), ChallengeCounters::default()],
            effect_history: Vec::new(),
            terminal_outcome: None,
            freshness_token: FreshnessToken(0),
        }
    }
}

fn stable_masks(masks: &[MaskTileId]) -> String {
    if masks.is_empty() {
        return "none".to_owned();
    }
    masks
        .iter()
        .map(|mask| mask.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_pedestal(pedestal: Option<PendingClaim>) -> String {
    match pedestal {
        Some(claim) => format!(
            "{}:{}:{}",
            claim.claimant.as_str(),
            claim.tile.as_str(),
            claim.declared.as_str()
        ),
        None => "none".to_owned(),
    }
}

fn stable_veiled(gallery: &[VeiledClaim]) -> String {
    if gallery.is_empty() {
        return "none".to_owned();
    }
    gallery
        .iter()
        .map(|claim| claim.declared.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_exposed(row: &[ExposedMask]) -> String {
    if row.is_empty() {
        return "none".to_owned();
    }
    row.iter()
        .map(|mask| {
            format!(
                "{}:{}:{}:{}",
                mask.tile.as_str(),
                mask.declared.as_str(),
                mask.claimant.as_str(),
                mask.challenger.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn stable_counters(counters: ChallengeCounters) -> String {
    format!(
        "lies={},successes={},declared={}",
        counters.exposed_lies, counters.successful_challenges, counters.challenges_declared
    )
}

fn stable_effect_history(effects: &[String]) -> String {
    if effects.is_empty() {
        return "none".to_owned();
    }
    effects.join("|")
}

fn stable_terminal(terminal: Option<TerminalOutcome>) -> String {
    match terminal {
        Some(TerminalOutcome::ScoreWin { winner, scores }) => {
            format!("score_win:{}:{}-{}", winner.as_str(), scores[0], scores[1])
        }
        Some(TerminalOutcome::TiebreakWin {
            winner,
            scores,
            tiebreak,
        }) => format!(
            "tiebreak_win:{}:{}:{}-{}",
            tiebreak,
            winner.as_str(),
            scores[0],
            scores[1]
        ),
        Some(TerminalOutcome::Draw { scores }) => {
            format!("draw:{}-{}", scores[0], scores[1])
        }
        None => "none".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Seed;

    #[test]
    fn initial_state_summary_keeps_hidden_fields_internal() {
        let state = MaskedClaimsState::new_after_deal(
            Variant::masked_claims_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            [
                vec![MaskTileId::MaskG1A, MaskTileId::MaskG3B],
                vec![MaskTileId::MaskG5C],
            ],
            vec![MaskTileId::MaskG2A],
            SeededRng::from_seed(Seed(0)),
        );

        assert_eq!(
            state.hand_for_internal(MaskedClaimsSeat::Seat0),
            &[MaskTileId::MaskG1A, MaskTileId::MaskG3B]
        );
        assert_eq!(state.reserve_internal(), &[MaskTileId::MaskG2A]);
        assert!(state.effect_history_internal().is_empty());
        assert_eq!(
            state.stable_internal_summary(),
            "variant=masked_claims_standard;phase=turn_1_claim;active=seat_0;turn_index=0;claimant=seat_0;hands=mask_g1_a,mask_g3_b|mask_g5_c;reserve=mask_g2_a;pedestal=none;veiled=none|none;exposed=none|none;scores=0,0;counters=lies=0,successes=0,declared=0|lies=0,successes=0,declared=0;effects=none;terminal=none;freshness=0"
        );
    }
}
