use engine_core::{FreshnessToken, SeatId, Seed};

use crate::{
    cards::{CardId, Suit},
    ids::{VowTideSeat, VARIANT_ID},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BiddingState {
    pub active_seat: VowTideSeat,
    pub bids: Vec<(VowTideSeat, Option<u8>)>,
}

impl BiddingState {
    pub fn new(seat_count: usize, active_seat: VowTideSeat) -> Self {
        Self {
            active_seat,
            bids: VowTideSeat::ALL
                .into_iter()
                .take(seat_count)
                .map(|seat| (seat, None))
                .collect(),
        }
    }

    pub fn bid_for(&self, seat: VowTideSeat) -> Option<u8> {
        self.bids
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .and_then(|(_, bid)| *bid)
    }

    pub fn bid_for_mut(&mut self, seat: VowTideSeat) -> Option<&mut Option<u8>> {
        self.bids
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, bid)| bid)
    }

    pub fn accepted_bid_total(&self) -> u8 {
        self.bids.iter().filter_map(|(_, bid)| *bid).sum()
    }

    pub fn all_bids_set(&self) -> bool {
        self.bids.iter().all(|(_, bid)| bid.is_some())
    }

    pub fn next_unset_after(&self, seat: VowTideSeat, seat_count: usize) -> Option<VowTideSeat> {
        let mut candidate = seat.next_clockwise(seat_count);
        for _ in 0..seat_count {
            if self.bid_for(candidate).is_none() {
                return Some(candidate);
            }
            candidate = candidate.next_clockwise(seat_count);
        }
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayingTrickState {
    pub trick_index: u8,
    pub leader: VowTideSeat,
    pub active_seat: VowTideSeat,
    pub current_trick: CurrentTrick,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TrickPlay {
    pub seat: VowTideSeat,
    pub card: CardId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentTrick {
    pub leader: VowTideSeat,
    pub plays: Vec<TrickPlay>,
}

impl CurrentTrick {
    pub fn new(leader: VowTideSeat) -> Self {
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
    pub winner: VowTideSeat,
    pub plays: Vec<TrickPlay>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalOutcome {
    pub winners: Vec<VowTideSeat>,
    pub final_scores: Vec<(VowTideSeat, i16)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    Bidding(BiddingState),
    PlayingTrick(PlayingTrickState),
    Terminal(TerminalOutcome),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VowTideState {
    pub variant: Variant,
    pub seats: Vec<SeatId>,
    pub seat_labels: Vec<String>,
    pub dealer: VowTideSeat,
    pub hand_index: u32,
    pub hand_schedule: Vec<u8>,
    pub cumulative_scores: Vec<(VowTideSeat, i16)>,
    pub public_bids: Vec<(VowTideSeat, Option<u8>)>,
    pub phase: Phase,
    pub private_hands: Vec<(VowTideSeat, Vec<CardId>)>,
    pub trick_counts: Vec<(VowTideSeat, u8)>,
    pub captured_tricks: Vec<CapturedTrick>,
    pub trump_indicator: CardId,
    pub hidden_stock: Vec<CardId>,
    pub deal_order: Vec<VowTideSeat>,
    pub freshness_token: FreshnessToken,
    pub seed: Seed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitialDealState {
    pub hand_index: u32,
    pub private_hands: Vec<(VowTideSeat, Vec<CardId>)>,
    pub trump_indicator: CardId,
    pub hidden_stock: Vec<CardId>,
    pub deal_order: Vec<VowTideSeat>,
}

impl VowTideState {
    pub fn new_after_deal(
        variant: Variant,
        seats: Vec<SeatId>,
        hand_schedule: Vec<u8>,
        dealer: VowTideSeat,
        deal: InitialDealState,
        seed: Seed,
    ) -> Self {
        let seat_count = seats.len();
        let seat_order: Vec<_> = VowTideSeat::ALL.into_iter().take(seat_count).collect();
        let active_seat = dealer.next_clockwise(seat_count);

        Self {
            variant,
            seats,
            seat_labels: seat_order
                .iter()
                .map(|seat| seat.fallback_label().to_owned())
                .collect(),
            dealer,
            hand_index: deal.hand_index,
            hand_schedule,
            cumulative_scores: seat_order.iter().map(|seat| (*seat, 0)).collect(),
            public_bids: seat_order.iter().map(|seat| (*seat, None)).collect(),
            phase: Phase::Bidding(BiddingState::new(seat_count, active_seat)),
            private_hands: deal.private_hands,
            trick_counts: seat_order.iter().map(|seat| (*seat, 0)).collect(),
            captured_tricks: Vec::new(),
            trump_indicator: deal.trump_indicator,
            hidden_stock: deal.hidden_stock,
            deal_order: deal.deal_order,
            freshness_token: FreshnessToken(0),
            seed,
        }
    }

    pub fn seat_count(&self) -> usize {
        self.seats.len()
    }

    pub fn current_hand_size(&self) -> Option<u8> {
        self.hand_schedule.get(self.hand_index as usize).copied()
    }

    pub fn active_seat(&self) -> Option<VowTideSeat> {
        match &self.phase {
            Phase::Bidding(bidding) => Some(bidding.active_seat),
            Phase::PlayingTrick(playing) => Some(playing.active_seat),
            Phase::Terminal(_) => None,
        }
    }

    pub fn bidding_state(&self) -> Option<&BiddingState> {
        match &self.phase {
            Phase::Bidding(bidding) => Some(bidding),
            _ => None,
        }
    }

    pub fn bidding_state_mut(&mut self) -> Option<&mut BiddingState> {
        match &mut self.phase {
            Phase::Bidding(bidding) => Some(bidding),
            _ => None,
        }
    }

    pub fn bid_for(&self, seat: VowTideSeat) -> Option<u8> {
        self.public_bids
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .and_then(|(_, bid)| *bid)
    }

    pub fn public_bid_for_mut(&mut self, seat: VowTideSeat) -> Option<&mut Option<u8>> {
        self.public_bids
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, bid)| bid)
    }

    pub fn hand_for_internal(&self, seat: VowTideSeat) -> &[CardId] {
        self.private_hands
            .iter()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand.as_slice())
            .unwrap_or(&[])
    }

    pub fn hand_for_internal_mut(&mut self, seat: VowTideSeat) -> Option<&mut Vec<CardId>> {
        self.private_hands
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
            .map(|(_, hand)| hand)
    }

    pub fn playing_state(&self) -> Option<&PlayingTrickState> {
        match &self.phase {
            Phase::PlayingTrick(playing) => Some(playing),
            _ => None,
        }
    }

    pub fn playing_state_mut(&mut self) -> Option<&mut PlayingTrickState> {
        match &mut self.phase {
            Phase::PlayingTrick(playing) => Some(playing),
            _ => None,
        }
    }

    pub fn increment_trick_count(&mut self, seat: VowTideSeat) {
        if let Some((_, count)) = self
            .trick_counts
            .iter_mut()
            .find(|(candidate, _)| *candidate == seat)
        {
            *count = count.saturating_add(1);
        }
    }

    pub fn trump_suit(&self) -> Suit {
        self.trump_indicator.card().suit
    }

    pub fn hidden_stock_internal(&self) -> &[CardId] {
        &self.hidden_stock
    }

    pub fn stable_internal_summary(&self) -> String {
        let phase = match &self.phase {
            Phase::Bidding(bidding) => format!("bidding:{}", bidding.active_seat.as_str()),
            Phase::PlayingTrick(playing) => {
                format!(
                    "playing:{}:{}",
                    playing.trick_index,
                    playing.active_seat.as_str()
                )
            }
            Phase::Terminal(_) => "terminal".to_owned(),
        };
        format!(
            "{}|variant={}|dealer={}|hand={}|schedule={:?}|scores={:?}|bids={:?}|phase={phase}|trump={}|stock={}|hands={}",
            VARIANT_ID,
            self.variant.id,
            self.dealer.as_str(),
            self.hand_index,
            self.hand_schedule,
            self.cumulative_scores,
            self.public_bids,
            self.trump_indicator.as_str(),
            self.hidden_stock
                .iter()
                .map(|card| card.as_str())
                .collect::<Vec<_>>()
                .join("/"),
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
