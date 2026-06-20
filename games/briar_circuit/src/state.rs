use engine_core::{FreshnessToken, SeatId};

use crate::{
    cards::CardId,
    ids::{BriarCircuitSeat, STANDARD_PASS_SIZE},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PassDirection {
    Left,
    Right,
    Across,
    Hold,
}

impl PassDirection {
    pub const fn for_hand_index(hand_index: u32) -> Self {
        match hand_index % 4 {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Across,
            _ => Self::Hold,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Across => "across",
            Self::Hold => "hold",
        }
    }

    pub const fn target_for(self, seat: BriarCircuitSeat) -> BriarCircuitSeat {
        match self {
            Self::Left => seat.pass_left_target(),
            Self::Right => seat.pass_right_target(),
            Self::Across => seat.pass_across_target(),
            Self::Hold => seat,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PassState {
    pub direction: PassDirection,
    pub selections: Vec<(BriarCircuitSeat, Vec<CardId>)>,
    pub committed: Vec<BriarCircuitSeat>,
}

impl PassState {
    pub fn new(direction: PassDirection) -> Self {
        Self {
            direction,
            selections: BriarCircuitSeat::ALL
                .into_iter()
                .map(|seat| (seat, Vec::with_capacity(STANDARD_PASS_SIZE as usize)))
                .collect(),
            committed: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TrickPlay {
    pub seat: BriarCircuitSeat,
    pub card: CardId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentTrick {
    pub leader: BriarCircuitSeat,
    pub plays: Vec<TrickPlay>,
}

impl CurrentTrick {
    pub fn new(leader: BriarCircuitSeat) -> Self {
        Self {
            leader,
            plays: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapturedTrick {
    pub hand_index: u32,
    pub trick_index: u8,
    pub winner: BriarCircuitSeat,
    pub plays: Vec<TrickPlay>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayingTrickState {
    pub hearts_broken: bool,
    pub trick_index: u8,
    pub leader: BriarCircuitSeat,
    pub active_seat: BriarCircuitSeat,
    pub current_trick: CurrentTrick,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandScoreBreakdown {
    pub raw_points: [u8; 4],
    pub hand_additions: [u8; 4],
    pub moon_shooter: Option<BriarCircuitSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalOutcome {
    UniqueLowScoreWin {
        winner: BriarCircuitSeat,
        cumulative_scores: [u16; 4],
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    Passing(PassState),
    PlayingTrick(PlayingTrickState),
    ScoringHand(HandScoreBreakdown),
    Terminal(TerminalOutcome),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BriarCircuitState {
    pub variant: Variant,
    pub seats: [SeatId; 4],
    pub dealer: BriarCircuitSeat,
    pub hand_index: u32,
    pub cumulative_scores: [u16; 4],
    pub phase: Phase,
    pub private_hands: Vec<(BriarCircuitSeat, Vec<CardId>)>,
    pub captured_tricks: Vec<CapturedTrick>,
    pub freshness_token: FreshnessToken,
}

impl BriarCircuitState {
    pub fn new_empty_hand(variant: Variant, seats: [SeatId; 4]) -> Self {
        let dealer = BriarCircuitSeat::Seat0;
        let hand_index = 0;
        let pass_direction = PassDirection::for_hand_index(hand_index);
        Self {
            variant,
            seats,
            dealer,
            hand_index,
            cumulative_scores: [0; 4],
            phase: match pass_direction {
                PassDirection::Hold => Phase::PlayingTrick(PlayingTrickState {
                    hearts_broken: false,
                    trick_index: 0,
                    leader: BriarCircuitSeat::Seat0,
                    active_seat: BriarCircuitSeat::Seat0,
                    current_trick: CurrentTrick::new(BriarCircuitSeat::Seat0),
                }),
                direction => Phase::Passing(PassState::new(direction)),
            },
            private_hands: BriarCircuitSeat::ALL
                .into_iter()
                .map(|seat| (seat, Vec::new()))
                .collect(),
            captured_tricks: Vec::new(),
            freshness_token: FreshnessToken(0),
        }
    }

    pub fn hand_for_internal(&self, seat: BriarCircuitSeat) -> &[CardId] {
        self.private_hands
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand.as_slice())
            .unwrap_or(&[])
    }

    pub fn stable_internal_summary(&self) -> String {
        let phase = match &self.phase {
            Phase::Passing(pass) => format!("passing:{}", pass.direction.as_str()),
            Phase::PlayingTrick(play) => {
                format!("playing:{}:{}", play.trick_index, play.active_seat.as_str())
            }
            Phase::ScoringHand(_) => "scoring".to_owned(),
            Phase::Terminal(_) => "terminal".to_owned(),
        };
        format!(
            "{}|dealer={}|hand={}|scores={:?}|phase={phase}|hands={}",
            self.variant.id,
            self.dealer.as_str(),
            self.hand_index,
            self.cumulative_scores,
            self.private_hands
                .iter()
                .map(|(seat, hand)| format!("{}:{}", seat.as_str(), hand.len()))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
