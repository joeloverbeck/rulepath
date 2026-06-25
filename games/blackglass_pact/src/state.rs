use engine_core::{FreshnessToken, SeatId, Seed};

use crate::{
    cards::CardId,
    ids::{BlackglassSeat, TeamId},
    variants::Variant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlindNilChoice {
    Declared,
    Declined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Bid {
    Tricks(u8),
    Nil,
    BlindNil,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayedCard {
    pub seat: BlackglassSeat,
    pub card: CardId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    BlindNilCommitment {
        pending: Vec<BlackglassSeat>,
        next_index: usize,
    },
    Bidding {
        next: BlackglassSeat,
        accepted: [Option<Bid>; 4],
    },
    PlayingTrick {
        leader: BlackglassSeat,
        next: BlackglassSeat,
        plays: Vec<PlayedCard>,
        trick_index: u8,
    },
    Terminal {
        winning_team: TeamId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlackglassPactState {
    pub variant: Variant,
    pub seats: [SeatId; 4],
    pub dealer: BlackglassSeat,
    pub hand_index: u32,
    pub phase: Phase,
    pub spades_broken: bool,
    pub bids: [Option<Bid>; 4],
    pub tricks_won: [u8; 4],
    pub team_scores: [i32; 2],
    pub team_bags: [u8; 2],
    pub private_hands: Vec<(BlackglassSeat, Vec<CardId>)>,
    pub freshness_token: FreshnessToken,
    pub seed: Seed,
}

impl BlackglassPactState {
    pub fn new_admitted_setup(
        variant: Variant,
        seats: [SeatId; 4],
        dealer: BlackglassSeat,
        hand_index: u32,
        seed: Seed,
    ) -> Self {
        Self {
            variant,
            seats,
            dealer,
            hand_index,
            phase: Phase::BlindNilCommitment {
                pending: Vec::new(),
                next_index: 0,
            },
            spades_broken: false,
            bids: [None, None, None, None],
            tricks_won: [0, 0, 0, 0],
            team_scores: [0, 0],
            team_bags: [0, 0],
            private_hands: Vec::new(),
            freshness_token: FreshnessToken(0),
            seed,
        }
    }

    pub fn hand_for_internal(&self, seat: BlackglassSeat) -> &[CardId] {
        self.private_hands
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand.as_slice())
            .unwrap_or(&[])
    }

    pub fn stable_setup_summary(&self) -> String {
        let seats = self
            .seats
            .iter()
            .map(|seat| seat.0.as_str())
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "game=blackglass_pact;variant={};seats={};dealer={};hand_index={};teams=team_0:seat_0+seat_2|team_1:seat_1+seat_3",
            self.variant.id,
            seats,
            self.dealer.as_str(),
            self.hand_index
        )
    }
}
