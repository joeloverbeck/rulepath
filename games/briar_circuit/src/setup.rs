use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};

use crate::{
    cards::{canonical_deck, CardId},
    ids::{BriarCircuitSeat, STANDARD_HAND_SIZE},
    state::PassDirection,
};
use crate::{ids::STANDARD_SEAT_COUNT, state::BriarCircuitState, variants::Variant};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::briar_circuit_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<BriarCircuitState, Diagnostic> {
    if seats.len() != STANDARD_SEAT_COUNT as usize {
        return Err(invalid_seat_count_diagnostic(seats.len()));
    }

    let mut rng = SeededRng::from_seed(seed);
    let dealer = BriarCircuitSeat::Seat0;
    let deal = deal_hand(&mut rng, dealer, 0)?;

    Ok(BriarCircuitState::new_after_deal(
        options.variant.clone(),
        [
            seats[0].clone(),
            seats[1].clone(),
            seats[2].clone(),
            seats[3].clone(),
        ],
        dealer,
        0,
        deal.hands,
        deal.pass_direction,
    ))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "BC_UNSUPPORTED_SEAT_COUNT".to_owned(),
        message: format!("briar_circuit requires exactly four seats; received {actual}"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandDeal {
    pub dealer: BriarCircuitSeat,
    pub hand_index: u32,
    pub pass_direction: PassDirection,
    pub hands: [Vec<CardId>; 4],
    pub deal_order: Vec<BriarCircuitSeat>,
}

pub fn deal_hand<R: DeterministicRng>(
    rng: &mut R,
    dealer: BriarCircuitSeat,
    hand_index: u32,
) -> Result<HandDeal, Diagnostic> {
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, rng);

    let deal_order = deal_order_after(dealer);
    let mut hands = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for card_index in 0..deck.len() {
        let seat = deal_order[card_index % deal_order.len()];
        hands[seat.index()].push(deck[card_index]);
    }

    if hands
        .iter()
        .any(|hand| hand.len() != STANDARD_HAND_SIZE as usize)
    {
        return Err(Diagnostic {
            code: "BC_DEAL_PARTITION_FAILED".to_owned(),
            message: "briar_circuit deal did not partition the full deck into four hands"
                .to_owned(),
        });
    }

    Ok(HandDeal {
        dealer,
        hand_index,
        pass_direction: PassDirection::for_hand_index(hand_index),
        hands,
        deal_order: deal_order.to_vec(),
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

pub fn deal_order_after(dealer: BriarCircuitSeat) -> [BriarCircuitSeat; 4] {
    let first = dealer.next_clockwise();
    [
        first,
        first.next_clockwise(),
        first.next_clockwise().next_clockwise(),
        first.next_clockwise().next_clockwise().next_clockwise(),
    ]
}

pub const fn next_dealer(dealer: BriarCircuitSeat) -> BriarCircuitSeat {
    dealer.next_clockwise()
}
