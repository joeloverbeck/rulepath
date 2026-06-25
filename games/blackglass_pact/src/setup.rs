use engine_core::{DeterministicRng, Diagnostic, SeatId, Seed, SeededRng};
use game_stdlib::SeatCountRange;

use crate::{
    cards::{canonical_deck, CardId},
    ids::{
        BlackglassSeat, DATA_VERSION_LABEL, RULES_VERSION_LABEL, STANDARD_HAND_SIZE,
        STANDARD_SEAT_COUNT,
    },
    partnerships::team_for_seat,
    state::{BlackglassPactState, Phase},
    variants::Variant,
};

pub const BLIND_NIL_DEFICIT_THRESHOLD: i32 = 100;
pub const HAND_SEED_DERIVATION_V1: &str = "blackglass_pact_hand_seed_v1_match_hand_rules_data";
const HAND_SEED_MIXER: u64 = 0x9E37_79B9_7F4A_7C15;
const VERSION_MIX: u64 =
    stable_label_mix(RULES_VERSION_LABEL) ^ stable_label_mix(DATA_VERSION_LABEL);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::blackglass_pact_standard(),
        }
    }
}

pub fn setup_match(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<BlackglassPactState, Diagnostic> {
    setup_match_with_scores(seed, seats, options, [0, 0])
}

pub fn setup_match_with_scores(
    seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
    team_scores: [i32; 2],
) -> Result<BlackglassPactState, Diagnostic> {
    validate_standard_seat_count(seats.len())?;

    let mut state = BlackglassPactState::new_admitted_setup(
        options.variant.clone(),
        [
            seats[0].clone(),
            seats[1].clone(),
            seats[2].clone(),
            seats[3].clone(),
        ],
        BlackglassSeat::North,
        0,
        seed,
    );
    state.team_scores = team_scores;
    state.phase = initial_blind_nil_phase(state.dealer, state.team_scores);
    if state.active_blind_nil_seat().is_none() {
        complete_blind_nil_and_deal(&mut state)?;
    }

    Ok(state)
}

pub fn validate_standard_seat_count(actual: usize) -> Result<(), Diagnostic> {
    SeatCountRange::inclusive(STANDARD_SEAT_COUNT as usize, STANDARD_SEAT_COUNT as usize)
        .expect("standard Blackglass seat count range is valid")
        .validate(actual)
        .map(|_| ())
        .map_err(|_| invalid_seat_count_diagnostic(actual))
}

pub fn invalid_seat_count_diagnostic(actual: usize) -> Diagnostic {
    Diagnostic {
        code: "BP_UNSUPPORTED_SEAT_COUNT".to_owned(),
        message: format!("blackglass_pact requires exactly four seats; received {actual}"),
    }
}

pub fn initial_blind_nil_phase(dealer: BlackglassSeat, team_scores: [i32; 2]) -> Phase {
    Phase::BlindNilCommitment {
        pending: eligible_blind_nil_order(dealer, team_scores),
        next_index: 0,
    }
}

pub fn eligible_blind_nil_order(
    dealer: BlackglassSeat,
    team_scores: [i32; 2],
) -> Vec<BlackglassSeat> {
    let mut order = Vec::new();
    let mut seat = dealer.next_clockwise();
    for _ in 0..STANDARD_SEAT_COUNT {
        let team = team_for_seat(seat);
        let other_team_score = team_scores[1 - team.index()];
        let deficit = other_team_score - team_scores[team.index()];
        if deficit >= BLIND_NIL_DEFICIT_THRESHOLD {
            order.push(seat);
        }
        seat = seat.next_clockwise();
    }
    order
}

pub fn complete_blind_nil_and_deal(state: &mut BlackglassPactState) -> Result<(), Diagnostic> {
    let deal = deal_for_hand(state.seed, state.dealer, state.hand_index)?;
    state.private_hands = deal.hands;
    state.phase = Phase::Bidding {
        next: first_bidder_after_blind(state.dealer, &state.bids),
        accepted: state.bids,
    };
    state.advance_freshness();
    Ok(())
}

pub fn first_bidder_after_blind(
    dealer: BlackglassSeat,
    bids: &[Option<crate::state::Bid>; 4],
) -> BlackglassSeat {
    let mut seat = dealer.next_clockwise();
    for _ in 0..STANDARD_SEAT_COUNT {
        if bids[seat.index()].is_none() {
            return seat;
        }
        seat = seat.next_clockwise();
    }
    dealer.next_clockwise()
}

pub const fn seed_for_hand(match_seed: Seed, hand_index: u32) -> Seed {
    Seed(match_seed.0 ^ VERSION_MIX ^ (hand_index as u64).wrapping_mul(HAND_SEED_MIXER))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandDeal {
    pub dealer: BlackglassSeat,
    pub hand_index: u32,
    pub hands: Vec<(BlackglassSeat, Vec<CardId>)>,
    pub deal_order: Vec<BlackglassSeat>,
}

pub fn deal_for_hand(
    match_seed: Seed,
    dealer: BlackglassSeat,
    hand_index: u32,
) -> Result<HandDeal, Diagnostic> {
    let mut rng = SeededRng::from_seed(seed_for_hand(match_seed, hand_index));
    deal_hand(&mut rng, dealer, hand_index)
}

pub fn deal_hand<R: DeterministicRng>(
    rng: &mut R,
    dealer: BlackglassSeat,
    hand_index: u32,
) -> Result<HandDeal, Diagnostic> {
    let mut deck = canonical_deck();
    shuffle_deck(&mut deck, rng);

    let deal_order = deal_order_after(dealer);
    let mut hands = BlackglassSeat::ALL
        .into_iter()
        .map(|seat| (seat, Vec::with_capacity(STANDARD_HAND_SIZE as usize)))
        .collect::<Vec<_>>();

    let mut dealt = deck.into_iter();
    for _ in 0..STANDARD_HAND_SIZE {
        for seat in &deal_order {
            let card = dealt.next().ok_or_else(deal_deck_exhausted)?;
            hands[seat.index()].1.push(card);
        }
    }
    if dealt.next().is_some() {
        return Err(Diagnostic {
            code: "BP_DEAL_TAIL_REMAINED".to_owned(),
            message: "blackglass_pact deal must consume all 52 cards into four 13-card hands"
                .to_owned(),
        });
    }

    Ok(HandDeal {
        dealer,
        hand_index,
        hands,
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

pub fn deal_order_after(dealer: BlackglassSeat) -> Vec<BlackglassSeat> {
    let mut order = Vec::with_capacity(STANDARD_SEAT_COUNT as usize);
    let mut seat = dealer.next_clockwise();
    for _ in 0..STANDARD_SEAT_COUNT {
        order.push(seat);
        seat = seat.next_clockwise();
    }
    order
}

fn deal_deck_exhausted() -> Diagnostic {
    Diagnostic {
        code: "BP_DEAL_DECK_EXHAUSTED".to_owned(),
        message: "blackglass_pact setup deck exhausted during deal".to_owned(),
    }
}

const fn stable_label_mix(label: &str) -> u64 {
    let bytes = label.as_bytes();
    let mut index = 0;
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        index += 1;
    }
    hash
}
