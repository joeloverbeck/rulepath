use std::collections::BTreeSet;

use blackglass_pact::{
    apply_bid_choice, apply_blind_nil_choice, canonical_deck, canonical_seat_ids,
    legal_action_tree, parse_bid_action_path, public_team_contracts, setup_match,
    setup_match_with_scores, Bid, BlackglassSeat, BlindNilChoice, Card, CardId, Rank, SetupOptions,
    Suit, TeamId, STANDARD_CARD_COUNT, STANDARD_HAND_SIZE,
};
use engine_core::{Actor, SeatId, Seed};

#[test]
fn canonical_deck_contains_fifty_two_unique_cards_in_stable_order() {
    let deck = canonical_deck();

    assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(
        deck.iter().copied().collect::<BTreeSet<_>>().len(),
        STANDARD_CARD_COUNT as usize
    );
    assert_eq!(deck[0], Card::new(Rank::Two, Suit::Clubs).id());
    assert_eq!(deck[12], Card::new(Rank::Ace, Suit::Clubs).id());
    assert_eq!(deck[13], Card::new(Rank::Two, Suit::Diamonds).id());
    assert_eq!(deck[26], Card::new(Rank::Two, Suit::Hearts).id());
    assert_eq!(deck[39], Card::new(Rank::Two, Suit::Spades).id());
    assert_eq!(deck[51], Card::new(Rank::Ace, Suit::Spades).id());
}

#[test]
fn card_ids_round_trip_to_cards_and_strings() {
    for card_id in canonical_deck() {
        let card = card_id.card();

        assert_eq!(card.id(), card_id);
        assert_eq!(CardId::parse(&card.id_str()), Some(card_id));
    }
}

#[test]
fn admitted_setup_has_no_private_deal_before_blind_commitment() {
    let state = setup_match_with_scores(
        Seed(1803),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("setup succeeds");

    for seat in BlackglassSeat::ALL {
        assert!(state.hand_for_internal(seat).is_empty());
    }
    assert_eq!(state.stable_setup_summary(), "game=blackglass_pact;variant=blackglass_pact_standard;seats=seat_0,seat_1,seat_2,seat_3;dealer=seat_0;hand_index=0;teams=team_0:seat_0+seat_2|team_1:seat_1+seat_3");
}

#[test]
fn deterministic_deal_conserves_all_cards_as_four_thirteen_card_hands() {
    let mut state = setup_match_with_scores(
        Seed(1805),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("setup succeeds");

    apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declined)
        .expect("first blind decision succeeds");
    apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declared)
        .expect("second blind decision succeeds");

    let mut dealt = BTreeSet::new();
    for seat in BlackglassSeat::ALL {
        let hand = state.hand_for_internal(seat);
        assert_eq!(hand.len(), STANDARD_HAND_SIZE as usize);
        dealt.extend(hand.iter().copied());
    }

    assert_eq!(dealt.len(), STANDARD_CARD_COUNT as usize);
    assert_eq!(dealt, canonical_deck().into_iter().collect::<BTreeSet<_>>());
}

#[test]
fn blind_nil_choices_do_not_perturb_deal_bytes_for_same_seed() {
    let mut declared = setup_match_with_scores(
        Seed(1806),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("declared setup succeeds");
    let mut declined = setup_match_with_scores(
        Seed(1806),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("declined setup succeeds");

    apply_blind_nil_choice(
        &mut declared,
        BlackglassSeat::South,
        BlindNilChoice::Declared,
    )
    .expect("south declare succeeds");
    apply_blind_nil_choice(
        &mut declared,
        BlackglassSeat::North,
        BlindNilChoice::Declared,
    )
    .expect("north declare succeeds");
    apply_blind_nil_choice(
        &mut declined,
        BlackglassSeat::South,
        BlindNilChoice::Declined,
    )
    .expect("south decline succeeds");
    apply_blind_nil_choice(
        &mut declined,
        BlackglassSeat::North,
        BlindNilChoice::Declined,
    )
    .expect("north decline succeeds");

    let declared_hands = BlackglassSeat::ALL
        .into_iter()
        .map(|seat| state_hand_bytes(&declared, seat))
        .collect::<Vec<_>>();
    let declined_hands = BlackglassSeat::ALL
        .into_iter()
        .map(|seat| state_hand_bytes(&declined, seat))
        .collect::<Vec<_>>();

    assert_eq!(declared_hands, declined_hands);
}

#[test]
fn every_emitted_bid_leaf_validates_for_active_bidder() {
    let state = setup_match(Seed(1812), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup enters bidding");
    let tree = legal_action_tree(&state, &actor_for(BlackglassSeat::East));

    let paths = two_segment_leaf_paths(&tree);
    assert_eq!(paths.len(), 14);
    for path in paths {
        let action = parse_bid_action_path(&path).expect("emitted path parses");
        let mut copy = state.clone();
        apply_bid_choice(&mut copy, BlackglassSeat::East, action.bid)
            .expect("emitted bid validates");
    }
}

#[test]
fn every_accepted_bid_action_was_emitted_for_active_bidder() {
    let state = setup_match(Seed(1813), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup enters bidding");
    let emitted =
        two_segment_leaf_paths(&legal_action_tree(&state, &actor_for(BlackglassSeat::East)))
            .into_iter()
            .collect::<BTreeSet<_>>();

    let accepted = std::iter::once(Bid::Nil)
        .chain((1..=13).map(Bid::Tricks))
        .map(|bid| vec!["bid".to_owned(), bid_path_segment(bid)])
        .collect::<BTreeSet<_>>();

    assert_eq!(emitted, accepted);
}

#[test]
fn ordinary_team_contract_sums_only_positive_numeric_partner_bids() {
    let mut state = setup_match(Seed(1814), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup enters bidding");

    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(4)).expect("east bids");
    apply_bid_choice(&mut state, BlackglassSeat::South, Bid::Nil).expect("south bids nil");
    apply_bid_choice(&mut state, BlackglassSeat::West, Bid::Tricks(3)).expect("west bids");
    apply_bid_choice(&mut state, BlackglassSeat::North, Bid::Tricks(5)).expect("north bids");

    assert_eq!(state.ordinary_team_contract(TeamId::NorthSouth), 5);
    assert_eq!(state.ordinary_team_contract(TeamId::EastWest), 7);
    assert_eq!(
        public_team_contracts(&state)
            .into_iter()
            .map(|row| (row.team, row.ordinary_contract))
            .collect::<Vec<_>>(),
        vec![(TeamId::NorthSouth, 5), (TeamId::EastWest, 7)]
    );
}

#[test]
fn nil_and_blind_nil_contribute_zero_to_ordinary_contract() {
    let mut state = setup_match_with_scores(
        Seed(1815),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("blind setup succeeds");

    apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declared)
        .expect("south declares blind nil");
    apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declined)
        .expect("north declines");

    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Nil).expect("east bids nil");
    apply_bid_choice(&mut state, BlackglassSeat::West, Bid::Tricks(6)).expect("west bids");
    apply_bid_choice(&mut state, BlackglassSeat::North, Bid::Tricks(2)).expect("north bids");

    assert_eq!(state.ordinary_team_contract(TeamId::NorthSouth), 2);
    assert_eq!(state.ordinary_team_contract(TeamId::EastWest), 6);
}

fn state_hand_bytes(state: &blackglass_pact::BlackglassPactState, seat: BlackglassSeat) -> Vec<u8> {
    state
        .hand_for_internal(seat)
        .iter()
        .map(|card| card.index())
        .collect()
}

fn actor_for(seat: BlackglassSeat) -> Actor {
    Actor {
        seat_id: SeatId::from_zero_based_index(seat.index() as u32),
    }
}

fn two_segment_leaf_paths(tree: &engine_core::ActionTree) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    for choice in &tree.root.choices {
        if let Some(next) = &choice.next {
            for leaf in &next.choices {
                paths.push(vec![choice.segment.clone(), leaf.segment.clone()]);
            }
        }
    }
    paths
}

fn bid_path_segment(bid: Bid) -> String {
    match bid {
        Bid::Nil => "nil".to_owned(),
        Bid::Tricks(value) => value.to_string(),
        Bid::BlindNil => "blind_nil".to_owned(),
    }
}
