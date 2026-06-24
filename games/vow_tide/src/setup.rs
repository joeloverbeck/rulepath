use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};
use game_stdlib::SeatCount;

use crate::{
    cards::{canonical_deck, CardId},
    ids::{hand_schedule_for_seats, supported_seat_count, VowTideSeat},
    state::{InitialDealState, VowTideState},
    variants::Variant,
};

pub const HAND_SEED_DERIVATION_V1: &str = "vow_tide_hand_seed_v1_xor_golden_ratio";
const HAND_SEED_MIXER: u64 = 0x9E37_79B9_7F4A_7C15;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::vow_tide_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<VowTideState, Diagnostic> {
    if !supported_seat_count(seats.len()) {
        return Err(invalid_seat_count_diagnostic(seats.len()));
    }

    let schedule = hand_schedule_for_seats(seats.len())
        .expect("validated Vow Tide seat counts always have a schedule");
    let dealer = VowTideSeat::Seat0;
    let hand_index = 0;
    let hand_size = schedule[hand_index as usize];
    let deal = deal_for_hand(seed, dealer, hand_index, seats.len(), hand_size)?;

    Ok(VowTideState::new_after_deal(
        options.variant.clone(),
        seats.to_vec(),
        schedule,
        dealer,
        InitialDealState {
            hand_index,
            private_hands: deal.hands,
            trump_indicator: deal.trump_indicator,
            hidden_stock: deal.hidden_stock,
            deal_order: deal.deal_order,
        },
        seed,
    ))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "VT_INVALID_SEAT_COUNT".to_owned(),
        message: format!("vow_tide supports 3 to 7 seats; received {actual}"),
    }
}

/// Derive the per-hand deal seed from the match seed. Hand 0 reuses the match
/// seed unchanged so initial setup remains directly reproducible from the
/// supplied seed; later hands mix the hand index with a fixed v1 constant.
pub const fn seed_for_hand(match_seed: Seed, hand_index: u32) -> Seed {
    Seed(match_seed.0 ^ (hand_index as u64).wrapping_mul(HAND_SEED_MIXER))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandDeal {
    pub dealer: VowTideSeat,
    pub hand_index: u32,
    pub hand_size: u8,
    pub hands: Vec<(VowTideSeat, Vec<CardId>)>,
    pub trump_indicator: CardId,
    pub hidden_stock: Vec<CardId>,
    pub deal_order: Vec<VowTideSeat>,
}

pub fn deal_for_hand(
    match_seed: Seed,
    dealer: VowTideSeat,
    hand_index: u32,
    seat_count: usize,
    hand_size: u8,
) -> Result<HandDeal, Diagnostic> {
    if !supported_seat_count(seat_count) {
        return Err(invalid_seat_count_diagnostic(seat_count));
    }

    let mut rng = SeededRng::from_seed(seed_for_hand(match_seed, hand_index));
    deal_hand(&mut rng, dealer, hand_index, seat_count, hand_size)
}

pub fn deal_hand<R: DeterministicRng>(
    rng: &mut R,
    dealer: VowTideSeat,
    hand_index: u32,
    seat_count: usize,
    hand_size: u8,
) -> Result<HandDeal, Diagnostic> {
    if !supported_seat_count(seat_count) {
        return Err(invalid_seat_count_diagnostic(seat_count));
    }

    let needed_for_hands = seat_count * hand_size as usize;
    if needed_for_hands + 1 > crate::ids::STANDARD_CARD_COUNT as usize {
        return Err(Diagnostic {
            code: "VT_DEAL_CAPACITY_EXCEEDED".to_owned(),
            message: format!(
                "vow_tide cannot deal {hand_size} cards to {seat_count} seats and reveal trump"
            ),
        });
    }

    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, rng);

    let deal_order = deal_order_after(dealer, seat_count);
    let mut hands = VowTideSeat::ALL
        .into_iter()
        .take(seat_count)
        .map(|seat| (seat, Vec::with_capacity(hand_size as usize)))
        .collect::<Vec<_>>();

    let mut dealt = deck.into_iter();
    for _ in 0..hand_size {
        for seat in &deal_order {
            let card = dealt.next().ok_or_else(deal_deck_exhausted)?;
            hands[seat.index()].1.push(card);
        }
    }

    let trump_indicator = dealt.next().ok_or_else(deal_deck_exhausted)?;
    let hidden_stock = dealt.collect::<Vec<_>>();
    if hidden_stock.is_empty() {
        return Err(Diagnostic {
            code: "VT_DEAL_STOCK_EMPTY".to_owned(),
            message: "vow_tide deal must leave at least one hidden stock card after trump"
                .to_owned(),
        });
    }

    Ok(HandDeal {
        dealer,
        hand_index,
        hand_size,
        hands,
        trump_indicator,
        hidden_stock,
        deal_order,
    })
}

pub fn shuffle_deck<R: DeterministicRng>(deck: &mut [CardId], rng: &mut R) {
    for index in (1..deck.len()).rev() {
        let swap_index = rng
            .next_index(index + 1)
            .expect("shuffle upper bound is nonzero");
        deck.swap(index, swap_index);
    }
}

pub fn deal_order_after(dealer: VowTideSeat, seat_count: usize) -> Vec<VowTideSeat> {
    let count = SeatCount::new(seat_count).expect("validated seat count is nonzero");
    let mut order = Vec::with_capacity(seat_count);
    let mut index = count
        .next_ring_index(dealer.index())
        .expect("validated dealer is in range");
    for _ in 0..seat_count {
        let seat = VowTideSeat::from_index(index).expect("validated ring index maps to a seat");
        order.push(seat);
        index = count
            .next_ring_index(index)
            .expect("validated ring index remains in range");
    }
    order
}

fn deal_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "VT_DEAL_DECK_EXHAUSTED".to_owned(),
        message: "vow_tide setup deck exhausted during deal".to_owned(),
    }
}
