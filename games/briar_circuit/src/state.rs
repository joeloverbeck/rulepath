use engine_core::{FreshnessToken, SeatId};

use crate::{
    cards::CardId,
    ids::{BriarCircuitSeat, STANDARD_HAND_SIZE, STANDARD_PASS_SIZE},
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

    pub fn selection_for(&self, seat: BriarCircuitSeat) -> &[CardId] {
        self.selections
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, selection)| selection.as_slice())
            .unwrap_or(&[])
    }

    pub fn selection_for_mut(&mut self, seat: BriarCircuitSeat) -> Option<&mut Vec<CardId>> {
        self.selections
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, selection)| selection)
    }

    pub fn is_committed(&self, seat: BriarCircuitSeat) -> bool {
        self.committed.contains(&seat)
    }

    pub fn committed_count(&self) -> usize {
        self.committed.len()
    }

    pub fn pending_count(&self) -> usize {
        BriarCircuitSeat::ALL.len() - self.committed.len()
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

    pub fn new_after_deal(
        variant: Variant,
        seats: [SeatId; 4],
        dealer: BriarCircuitSeat,
        hand_index: u32,
        hands: [Vec<CardId>; 4],
        pass_direction: PassDirection,
    ) -> Self {
        debug_assert!(hands
            .iter()
            .all(|hand| hand.len() == STANDARD_HAND_SIZE as usize));

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
                    leader: opening_leader_from_hands(&hands),
                    active_seat: opening_leader_from_hands(&hands),
                    current_trick: CurrentTrick::new(opening_leader_from_hands(&hands)),
                }),
                direction => Phase::Passing(PassState::new(direction)),
            },
            private_hands: BriarCircuitSeat::ALL.into_iter().zip(hands).collect(),
            captured_tricks: Vec::new(),
            freshness_token: FreshnessToken(0),
        }
    }

    pub fn pass_direction(&self) -> PassDirection {
        PassDirection::for_hand_index(self.hand_index)
    }

    pub fn hand_for_internal(&self, seat: BriarCircuitSeat) -> &[CardId] {
        self.private_hands
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand.as_slice())
            .unwrap_or(&[])
    }

    pub fn hand_for_internal_mut(&mut self, seat: BriarCircuitSeat) -> Option<&mut Vec<CardId>> {
        self.private_hands
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand)
    }

    pub fn pass_state(&self) -> Option<&PassState> {
        match &self.phase {
            Phase::Passing(pass) => Some(pass),
            _ => None,
        }
    }

    pub fn pass_state_mut(&mut self) -> Option<&mut PassState> {
        match &mut self.phase {
            Phase::Passing(pass) => Some(pass),
            _ => None,
        }
    }

    pub fn playing_state(&self) -> Option<&PlayingTrickState> {
        match &self.phase {
            Phase::PlayingTrick(play) => Some(play),
            _ => None,
        }
    }

    pub fn playing_state_mut(&mut self) -> Option<&mut PlayingTrickState> {
        match &mut self.phase {
            Phase::PlayingTrick(play) => Some(play),
            _ => None,
        }
    }

    pub fn enter_playing_with_two_clubs_leader(&mut self) {
        let leader = opening_leader_from_private_hands(&self.private_hands);
        self.phase = Phase::PlayingTrick(PlayingTrickState {
            hearts_broken: false,
            trick_index: 0,
            leader,
            active_seat: leader,
            current_trick: CurrentTrick::new(leader),
        });
        self.freshness_token = FreshnessToken(self.freshness_token.0 + 1);
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
                .map(|(seat, hand)| format!(
                    "{}:{}",
                    seat.as_str(),
                    hand.iter()
                        .map(|card| card.as_str())
                        .collect::<Vec<_>>()
                        .join("/")
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

fn opening_leader_from_hands(hands: &[Vec<CardId>; 4]) -> BriarCircuitSeat {
    for seat in BriarCircuitSeat::ALL {
        if hands[seat.index()]
            .iter()
            .any(|card| card.card().is_two_of_clubs())
        {
            return seat;
        }
    }
    BriarCircuitSeat::Seat0
}

fn opening_leader_from_private_hands(
    hands: &[(BriarCircuitSeat, Vec<CardId>)],
) -> BriarCircuitSeat {
    for (seat, hand) in hands {
        if hand.iter().any(|card| card.card().is_two_of_clubs()) {
            return *seat;
        }
    }
    BriarCircuitSeat::Seat0
}
