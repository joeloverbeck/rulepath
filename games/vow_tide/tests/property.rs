use std::collections::BTreeSet;

use engine_core::Seed;
use vow_tide::{
    actions::{legal_action_tree, legal_bids, validate_bid_command},
    cards::{canonical_deck, CardId},
    ids::{
        canonical_seat_ids, hand_schedule_for_seats, VowTideSeat, STANDARD_MAX_SEATS,
        STANDARD_MIN_SEATS,
    },
    rules::apply_bid,
    setup::{deal_for_hand, seed_for_hand, setup_match, SetupOptions},
};

#[test]
fn same_seed_and_hand_index_reproduce_identical_deal() {
    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let hand_size = hand_schedule_for_seats(seat_count).expect("schedule exists")[0];
        let first = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("first deal succeeds");
        let second = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("second deal succeeds");

        assert_eq!(first, second);
    }
}

#[test]
fn different_seed_changes_deal_for_supported_counts() {
    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let hand_size = hand_schedule_for_seats(seat_count).expect("schedule exists")[0];
        let first = deal_for_hand(Seed(42), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("first deal succeeds");
        let second = deal_for_hand(Seed(43), VowTideSeat::Seat0, 0, seat_count, hand_size)
            .expect("second deal succeeds");

        assert_ne!(first.hands, second.hands);
        assert_ne!(first.trump_indicator, second.trump_indicator);
    }
}

#[test]
fn hand_seed_derivation_is_partitioned_by_hand_index() {
    assert_eq!(seed_for_hand(Seed(77), 0), Seed(77));
    assert_ne!(seed_for_hand(Seed(77), 1), Seed(77));
    assert_ne!(seed_for_hand(Seed(77), 1), seed_for_hand(Seed(77), 2));
}

#[test]
fn setup_deals_clockwise_from_left_of_dealer_and_conserves_cards() {
    let options = SetupOptions::default();
    let canonical = canonical_deck().into_iter().collect::<BTreeSet<_>>();

    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let seats = canonical_seat_ids(seat_count);
        let state = setup_match(Seed(91), &seats, &options).expect("setup succeeds");
        let hand_size = state.current_hand_size().expect("current hand size");

        assert_eq!(state.deal_order[0], VowTideSeat::Seat1);
        assert_eq!(state.deal_order.last(), Some(&VowTideSeat::Seat0));
        for (_, hand) in &state.private_hands {
            assert_eq!(hand.len(), hand_size as usize);
        }

        let all_cards = all_internal_cards(
            &state.private_hands,
            state.trump_indicator,
            &state.hidden_stock,
        );
        assert_eq!(all_cards.len(), canonical.len());
        assert_eq!(all_cards, canonical);
        assert!(!state.hidden_stock.contains(&state.trump_indicator));
        assert!(state
            .private_hands
            .iter()
            .all(|(_, hand)| !hand.contains(&state.trump_indicator)));
        assert_eq!(state.trump_suit(), state.trump_indicator.card().suit);
    }
}

#[test]
fn dealer_hook_never_empties_legal_bids_for_reachable_prefixes() {
    for hand_size in 1..=10 {
        for prefix_total in 0..=(hand_size * 3) {
            let legal = dealer_legal_bids_for_prefix(hand_size, prefix_total);
            assert!(!legal.is_empty());
            if prefix_total <= hand_size {
                assert!(!legal.contains(&(hand_size - prefix_total)));
                assert_eq!(legal.len(), hand_size as usize);
            } else {
                assert_eq!(legal, (0..=hand_size).collect::<Vec<_>>());
            }
        }
    }
}

#[test]
fn legal_tree_and_validator_agree_for_bidding_states() {
    for prefix in [
        Vec::new(),
        vec![0],
        vec![2],
        vec![3, 3, 3],
        vec![10, 10, 10],
    ] {
        let mut state = setup_state(4);
        for (seat, bid) in [VowTideSeat::Seat1, VowTideSeat::Seat2, VowTideSeat::Seat3]
            .into_iter()
            .zip(prefix)
        {
            apply_bid_value(&mut state, seat, bid);
        }

        let Some(active) = state.active_seat() else {
            continue;
        };
        let tree_bids = bid_leaf_segments(&legal_action_tree(&state, &actor(active)))
            .into_iter()
            .map(|segment| segment.parse::<u8>().expect("u8 bid"))
            .collect::<Vec<_>>();
        let helper_bids = legal_bids(&state, active);
        assert_eq!(tree_bids, helper_bids);

        for bid in 0..=state.current_hand_size().expect("hand size") {
            let result = validate_bid_command(&state, &command(&state, active, bid));
            assert_eq!(result.is_ok(), helper_bids.contains(&bid));
        }
    }
}

fn all_internal_cards(
    hands: &[(VowTideSeat, Vec<CardId>)],
    trump_indicator: CardId,
    hidden_stock: &[CardId],
) -> BTreeSet<CardId> {
    let mut cards = BTreeSet::new();
    for (_, hand) in hands {
        for card in hand {
            assert!(
                cards.insert(*card),
                "duplicate dealt card {}",
                card.as_str()
            );
        }
    }
    assert!(
        cards.insert(trump_indicator),
        "duplicate trump {}",
        trump_indicator.as_str()
    );
    for card in hidden_stock {
        assert!(
            cards.insert(*card),
            "duplicate stock card {}",
            card.as_str()
        );
    }
    cards
}

fn dealer_legal_bids_for_prefix(hand_size: u8, prefix_total: u8) -> Vec<u8> {
    let forbidden = if prefix_total <= hand_size {
        Some(hand_size - prefix_total)
    } else {
        None
    };
    (0..=hand_size)
        .filter(|bid| Some(*bid) != forbidden)
        .collect()
}

fn setup_state(seat_count: usize) -> vow_tide::state::VowTideState {
    setup_match(
        Seed(29),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn apply_bid_value(state: &mut vow_tide::state::VowTideState, seat: VowTideSeat, value: u8) {
    let bid = validate_bid_command(state, &command(state, seat, value)).expect("bid validates");
    apply_bid(state, bid).expect("bid applies");
}

fn command(
    state: &vow_tide::state::VowTideState,
    seat: VowTideSeat,
    value: u8,
) -> engine_core::CommandEnvelope {
    engine_core::CommandEnvelope {
        actor: actor(seat),
        action_path: engine_core::ActionPath {
            segments: vec!["bid".to_owned(), value.to_string()],
        },
        freshness_token: state.freshness_token,
        rules_version: engine_core::RulesVersion(1),
    }
}

fn actor(seat: VowTideSeat) -> engine_core::Actor {
    engine_core::Actor {
        seat_id: engine_core::SeatId(seat.as_str().to_owned()),
    }
}

fn bid_leaf_segments(tree: &engine_core::ActionTree) -> Vec<String> {
    tree.root
        .choices
        .first()
        .and_then(|choice| choice.next.as_ref())
        .map(|node| {
            node.choices
                .iter()
                .map(|choice| choice.segment.clone())
                .collect()
        })
        .unwrap_or_default()
}
