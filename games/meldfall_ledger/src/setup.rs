use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};

use crate::{
    cards::{canonical_deck, CardId},
    ids::{
        canonical_seat_ids, hand_size_for_seats, next_clockwise_index, supported_seat_count,
        STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    },
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
    pub dealer_index: usize,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::classic_500_single_deck_v1(),
            dealer_index: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitialSetup {
    pub variant: Variant,
    pub seats: Vec<SeatId>,
    pub dealer_index: usize,
    pub active_seat_index: usize,
    pub deal_order: Vec<usize>,
    pub private_hands: Vec<Vec<CardId>>,
    pub initial_discard: CardId,
    pub stock: Vec<CardId>,
    pub seed: Seed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicSetupView {
    pub dealer: SeatId,
    pub active_seat: SeatId,
    pub hand_counts: Vec<(SeatId, usize)>,
    pub discard_count: usize,
    pub stock_count: usize,
}

impl InitialSetup {
    pub fn public_view(&self) -> PublicSetupView {
        PublicSetupView {
            dealer: self.seats[self.dealer_index].clone(),
            active_seat: self.seats[self.active_seat_index].clone(),
            hand_counts: self
                .seats
                .iter()
                .cloned()
                .zip(self.private_hands.iter().map(Vec::len))
                .collect(),
            discard_count: 1,
            stock_count: self.stock.len(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<InitialSetup, Diagnostic> {
    validate_seat_count(seats.len())?;
    let dealer_index = options.dealer_index % seats.len();
    let hand_size = hand_size_for_seats(seats.len()).expect("validated seat count has hand size");
    let deal = deal_for_round(seed, dealer_index, seats.len(), hand_size)?;
    let active_seat_index = next_clockwise_index(dealer_index, seats.len())
        .expect("validated dealer index has next clockwise seat");

    Ok(InitialSetup {
        variant: options.variant.clone(),
        seats: seats.to_vec(),
        dealer_index,
        active_seat_index,
        deal_order: deal.deal_order,
        private_hands: deal.private_hands,
        initial_discard: deal.initial_discard,
        stock: deal.stock,
        seed,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundDeal {
    pub dealer_index: usize,
    pub hand_size: u8,
    pub deal_order: Vec<usize>,
    pub private_hands: Vec<Vec<CardId>>,
    pub initial_discard: CardId,
    pub stock: Vec<CardId>,
}

pub fn deal_for_round(
    seed: Seed,
    dealer_index: usize,
    seat_count: usize,
    hand_size: u8,
) -> Result<RoundDeal, Diagnostic> {
    validate_seat_count(seat_count)?;
    if dealer_index >= seat_count {
        return Err(Diagnostic {
            code: "ML_INVALID_DEALER_INDEX".to_owned(),
            message: format!(
                "meldfall_ledger dealer index {dealer_index} is outside {seat_count} seats"
            ),
        });
    }
    let expected_hand_size = hand_size_for_seats(seat_count).expect("validated seat count");
    if hand_size != expected_hand_size {
        return Err(Diagnostic {
            code: "ML_INVALID_HAND_SIZE".to_owned(),
            message: format!(
                "meldfall_ledger deals {expected_hand_size} cards per seat for {seat_count} seats"
            ),
        });
    }

    let mut rng = SeededRng::from_seed(seed);
    deal_round(&mut rng, dealer_index, seat_count, hand_size)
}

pub fn deal_round<R: DeterministicRng>(
    rng: &mut R,
    dealer_index: usize,
    seat_count: usize,
    hand_size: u8,
) -> Result<RoundDeal, Diagnostic> {
    validate_seat_count(seat_count)?;
    if dealer_index >= seat_count {
        return Err(Diagnostic {
            code: "ML_INVALID_DEALER_INDEX".to_owned(),
            message: format!(
                "meldfall_ledger dealer index {dealer_index} is outside {seat_count} seats"
            ),
        });
    }

    let needed_for_hands = seat_count * hand_size as usize;
    if needed_for_hands + 1 > crate::ids::STANDARD_CARD_COUNT as usize {
        return Err(Diagnostic {
            code: "ML_DEAL_CAPACITY_EXCEEDED".to_owned(),
            message: format!(
                "meldfall_ledger cannot deal {hand_size} cards to {seat_count} seats and start a discard pile"
            ),
        });
    }

    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, rng);

    let deal_order = deal_order_after(dealer_index, seat_count);
    let mut private_hands = vec![Vec::with_capacity(hand_size as usize); seat_count];
    let mut dealt = deck.into_iter();
    for _ in 0..hand_size {
        for seat_index in &deal_order {
            let card = dealt.next().ok_or_else(deal_deck_exhausted)?;
            private_hands[*seat_index].push(card);
        }
    }

    let initial_discard = dealt.next().ok_or_else(deal_deck_exhausted)?;
    let stock = dealt.collect::<Vec<_>>();
    if stock.is_empty() {
        return Err(Diagnostic {
            code: "ML_DEAL_STOCK_EMPTY".to_owned(),
            message: "meldfall_ledger setup must leave a hidden stock after the initial discard"
                .to_owned(),
        });
    }

    Ok(RoundDeal {
        dealer_index,
        hand_size,
        deal_order,
        private_hands,
        initial_discard,
        stock,
    })
}

pub fn validate_seat_count(count: usize) -> Result<(), Diagnostic> {
    if supported_seat_count(count) {
        Ok(())
    } else {
        Err(invalid_seat_count_diagnostic(count))
    }
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "ML_INVALID_SEAT_COUNT".to_owned(),
        message: format!(
            "meldfall_ledger supports {STANDARD_MIN_SEATS} to {STANDARD_MAX_SEATS} seats; received {actual}"
        ),
    }
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [CardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index = rng
            .next_index_unbiased_v1(index + 1)
            .expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

pub fn deal_order_after(dealer_index: usize, seat_count: usize) -> Vec<usize> {
    let mut order = Vec::with_capacity(seat_count);
    let mut index = next_clockwise_index(dealer_index, seat_count)
        .expect("validated dealer index has next clockwise seat");
    for _ in 0..seat_count {
        order.push(index);
        index = next_clockwise_index(index, seat_count)
            .expect("validated ring index remains inside seat count");
    }
    order
}

pub fn default_seats(seat_count: usize) -> Result<Vec<SeatId>, Diagnostic> {
    validate_seat_count(seat_count)?;
    Ok(canonical_seat_ids(seat_count))
}

fn deal_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "ML_DEAL_DECK_EXHAUSTED".to_owned(),
        message: "meldfall_ledger setup deck exhausted during deal".to_owned(),
    }
}
