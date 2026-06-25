use std::collections::BTreeSet;

use blackglass_pact::{
    apply_bid_choice, apply_blind_nil_choice, canonical_deck, canonical_seat_ids,
    legal_action_tree, parse_bid_action_path, public_team_contracts, score_completed_hand,
    setup_match, setup_match_with_scores, terminal_winner, trick_winner, Bid, BlackglassSeat,
    BlindNilChoice, Card, CardId, Phase, PlayedCard, Rank, SetupOptions, Suit, TeamId,
    STANDARD_CARD_COUNT, STANDARD_HAND_SIZE,
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

#[test]
fn spades_trump_helper_agrees_with_independent_oracle() {
    let seats = [
        BlackglassSeat::East,
        BlackglassSeat::South,
        BlackglassSeat::West,
        BlackglassSeat::North,
    ];
    for led_suit in Suit::ALL {
        for second_suit in Suit::ALL {
            for third_suit in Suit::ALL {
                for fourth_suit in Suit::ALL {
                    let suits = [led_suit, second_suit, third_suit, fourth_suit];
                    let ranks = [Rank::Two, Rank::Seven, Rank::Ace, Rank::King];
                    let plays = suits
                        .into_iter()
                        .zip(ranks)
                        .zip(seats)
                        .map(|((suit, rank), seat)| PlayedCard {
                            seat,
                            card: Card::new(rank, suit).id(),
                        })
                        .collect::<Vec<_>>();

                    assert_eq!(trick_winner(&plays).expect("winner"), oracle_winner(&plays));
                }
            }
        }
    }
}

#[test]
fn four_plays_on_thirteenth_trick_complete_the_hand() {
    let mut state = setup_match(Seed(1817), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.private_hands = vec![
        (BlackglassSeat::North, vec![card(Rank::Five, Suit::Clubs)]),
        (BlackglassSeat::East, vec![card(Rank::Two, Suit::Clubs)]),
        (BlackglassSeat::South, vec![card(Rank::Three, Suit::Clubs)]),
        (BlackglassSeat::West, vec![card(Rank::Four, Suit::Clubs)]),
    ];
    state.phase = Phase::PlayingTrick {
        leader: BlackglassSeat::East,
        next: BlackglassSeat::East,
        plays: Vec::new(),
        trick_index: 12,
    };

    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Two, Suit::Clubs),
    )
    .expect("east plays");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::South,
        card(Rank::Three, Suit::Clubs),
    )
    .expect("south plays");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::West,
        card(Rank::Four, Suit::Clubs),
    )
    .expect("west plays");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::North,
        card(Rank::Five, Suit::Clubs),
    )
    .expect("north plays");

    assert_eq!(
        state.phase,
        Phase::HandScoring {
            completed_tricks: 13
        }
    );
    assert_eq!(state.tricks_won[BlackglassSeat::North.index()], 1);
}

#[test]
fn exact_tie_at_or_above_target_continues_to_next_hand() {
    let mut state = scoring_state([500, 500], [0, 0], [None, None, None, None], [0, 0, 0, 0]);

    let effects = score_completed_hand(&mut state).expect("tie hand scores");

    assert!(state.outcome.is_none());
    assert!(!matches!(state.phase, Phase::Terminal { .. }));
    assert_eq!(state.dealer, BlackglassSeat::East);
    assert_eq!(state.hand_index, 1);
    assert!(effects.iter().any(|effect| matches!(
        effect,
        blackglass_pact::BlackglassPactEffect::DealerAdvanced { .. }
    )));
}

#[test]
fn unique_higher_team_at_target_terminal_has_stable_outcome_arrays() {
    let mut state = scoring_state(
        [490, 470],
        [0, 0],
        [Some(Bid::Tricks(1)), None, Some(Bid::Tricks(1)), None],
        [2, 0, 0, 0],
    );

    score_completed_hand(&mut state).expect("terminal hand scores");

    assert_eq!(
        state.phase,
        Phase::Terminal {
            winning_team: TeamId::NorthSouth
        }
    );
    let outcome = state.outcome.as_ref().expect("terminal outcome");
    assert_eq!(outcome.winning_team_ids, vec![TeamId::NorthSouth]);
    assert_eq!(outcome.standings_by_team[0].team_id, TeamId::NorthSouth);
    assert_eq!(outcome.standings_by_team[0].competition_rank, 1);
    assert!(outcome.standings_by_team[0].is_winner);
    assert_eq!(
        outcome.standings_by_seat[0].seat_id,
        SeatId::from_zero_based_index(0)
    );
    assert_eq!(outcome.standings_by_seat[0].team_rank, 1);
}

#[test]
fn terminal_winner_requires_target_and_non_tie() {
    assert_eq!(terminal_winner([499, 300]), None);
    assert_eq!(terminal_winner([500, 500]), None);
    assert_eq!(terminal_winner([501, 500]), Some(TeamId::NorthSouth));
    assert_eq!(terminal_winner([500, 520]), Some(TeamId::EastWest));
}

#[test]
fn score_overflow_is_rejected_with_stable_diagnostic() {
    let state = scoring_state(
        [i32::MAX, 0],
        [0, 0],
        [Some(Bid::Tricks(13)), None, None, None],
        [13, 0, 0, 0],
    );

    let diagnostic = blackglass_pact::score_hand(&state).expect_err("overflow rejected");
    assert_eq!(diagnostic.code, "BP_SCORE_OVERFLOW");
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

fn oracle_winner(plays: &[PlayedCard]) -> BlackglassSeat {
    let winning_suit = if plays
        .iter()
        .any(|play| play.card.card().suit == Suit::Spades)
    {
        Suit::Spades
    } else {
        plays[0].card.card().suit
    };
    plays
        .iter()
        .filter(|play| play.card.card().suit == winning_suit)
        .max_by_key(|play| play.card.card().rank.value())
        .expect("non-empty plays")
        .seat
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn scoring_state(
    team_scores: [i32; 2],
    team_bags: [u8; 2],
    bids: [Option<Bid>; 4],
    tricks_won: [u8; 4],
) -> blackglass_pact::BlackglassPactState {
    let mut state = setup_match(Seed(1819), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    state.team_scores = team_scores;
    state.team_bags = team_bags;
    state.bids = bids;
    state.tricks_won = tricks_won;
    state.phase = Phase::HandScoring {
        completed_tricks: 13,
    };
    state
}
