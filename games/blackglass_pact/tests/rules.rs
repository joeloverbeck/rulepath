use blackglass_pact::{
    apply_bid_choice, apply_blind_nil_choice, canonical_seat_ids, eligible_blind_nil_order,
    legal_action_tree, legal_play_cards, parse_bid_action_path, partner_for, setup_match,
    setup_match_with_scores, team_for_seat, validate_standard_seat_count, Bid, BlackglassSeat,
    BlindNilChoice, Card, CardId, Phase, Rank, SetupOptions, Suit, TeamId, ACTION_BID_NIL,
    STANDARD_HAND_SIZE,
};
use engine_core::{Actor, SeatId, Seed};

#[test]
fn setup_accepts_exactly_four_seats() {
    let state = setup_match(Seed(1800), &canonical_seat_ids(), &SetupOptions::default())
        .expect("four-seat setup succeeds");

    assert_eq!(state.dealer, BlackglassSeat::North);
    assert_eq!(state.seats, canonical_seat_ids());
    assert_eq!(state.team_scores, [0, 0]);
    assert_eq!(state.team_bags, [0, 0]);
    assert_eq!(
        state.phase,
        Phase::Bidding {
            next: BlackglassSeat::East,
            accepted: [None, None, None, None],
        }
    );
    for seat in BlackglassSeat::ALL {
        assert_eq!(
            state.hand_for_internal(seat).len(),
            STANDARD_HAND_SIZE as usize
        );
    }
}

#[test]
fn setup_rejects_all_non_four_counts_with_stable_code() {
    for count in [0usize, 1, 2, 3, 5, 6, 7, 8] {
        let seats: Vec<SeatId> = (0..count)
            .map(|index| SeatId::from_zero_based_index(index as u32))
            .collect();
        let diagnostic = setup_match(Seed(1801), &seats, &SetupOptions::default())
            .expect_err("unsupported seat count rejected");

        assert_eq!(diagnostic.code, "BP_UNSUPPORTED_SEAT_COUNT");
        assert!(
            diagnostic.message.contains("exactly four seats"),
            "{}",
            diagnostic.message
        );
        assert!(validate_standard_seat_count(count).is_err());
    }
}

#[test]
fn fixed_partnership_mapping_is_stable() {
    assert_eq!(team_for_seat(BlackglassSeat::North), TeamId::NorthSouth);
    assert_eq!(team_for_seat(BlackglassSeat::South), TeamId::NorthSouth);
    assert_eq!(team_for_seat(BlackglassSeat::East), TeamId::EastWest);
    assert_eq!(team_for_seat(BlackglassSeat::West), TeamId::EastWest);

    assert_eq!(partner_for(BlackglassSeat::North), BlackglassSeat::South);
    assert_eq!(partner_for(BlackglassSeat::East), BlackglassSeat::West);
}

#[test]
fn blind_nil_eligibility_boundary_is_99_vs_100() {
    assert_eq!(
        eligible_blind_nil_order(BlackglassSeat::North, [1, 100]),
        Vec::new()
    );
    assert_eq!(
        eligible_blind_nil_order(BlackglassSeat::North, [0, 100]),
        vec![BlackglassSeat::South, BlackglassSeat::North]
    );
}

#[test]
fn blind_nil_order_skips_ineligible_seats_clockwise() {
    let mut state = setup_match_with_scores(
        Seed(1804),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("deficit setup succeeds");

    assert_eq!(
        state.phase,
        Phase::BlindNilCommitment {
            pending: vec![BlackglassSeat::South, BlackglassSeat::North],
            next_index: 0,
        }
    );
    assert_eq!(state.active_blind_nil_seat(), Some(BlackglassSeat::South));

    apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declined)
        .expect("south may resolve first");
    assert_eq!(state.active_blind_nil_seat(), Some(BlackglassSeat::North));

    let diagnostic =
        apply_blind_nil_choice(&mut state, BlackglassSeat::East, BlindNilChoice::Declared)
            .expect_err("ineligible east is skipped");
    assert_eq!(diagnostic.code, "BP_WRONG_BLIND_NIL_SEAT");

    apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declared)
        .expect("north may resolve second");
    assert_eq!(
        state.phase,
        Phase::Bidding {
            next: BlackglassSeat::East,
            accepted: [Some(Bid::BlindNil), None, None, None],
        }
    );
}

#[test]
fn bidding_runs_left_of_dealer_through_dealer() {
    let mut state = setup_match(Seed(1808), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup deals and enters bidding");

    assert_eq!(
        active_bid_leaf_segments(&state, BlackglassSeat::East).len(),
        14
    );
    assert_eq!(
        active_bid_leaf_segments(&state, BlackglassSeat::East)[0],
        ACTION_BID_NIL
    );

    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(4)).expect("east bids first");
    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::South));
    apply_bid_choice(&mut state, BlackglassSeat::South, Bid::Nil).expect("south bids second");
    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::West));
    apply_bid_choice(&mut state, BlackglassSeat::West, Bid::Tricks(3)).expect("west bids third");
    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::North));
    apply_bid_choice(&mut state, BlackglassSeat::North, Bid::Tricks(5)).expect("dealer bids last");

    assert_eq!(
        state.phase,
        Phase::PlayingTrick {
            leader: BlackglassSeat::East,
            next: BlackglassSeat::East,
            plays: Vec::new(),
            trick_index: 0,
        }
    );
}

#[test]
fn blind_nil_declarer_is_skipped_in_bidding_and_bid_is_locked() {
    let mut state = setup_match_with_scores(
        Seed(1809),
        &canonical_seat_ids(),
        &SetupOptions::default(),
        [0, 100],
    )
    .expect("blind setup succeeds");

    apply_blind_nil_choice(&mut state, BlackglassSeat::South, BlindNilChoice::Declined)
        .expect("south declines");
    apply_blind_nil_choice(&mut state, BlackglassSeat::North, BlindNilChoice::Declared)
        .expect("north declares");

    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::East));
    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(4)).expect("east bids");
    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::South));
    apply_bid_choice(&mut state, BlackglassSeat::South, Bid::Tricks(2)).expect("south bids");
    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::West));
    apply_bid_choice(&mut state, BlackglassSeat::West, Bid::Nil).expect("west bids");

    let diagnostic =
        apply_bid_choice(&mut state, BlackglassSeat::North, Bid::Tricks(1)).expect_err("locked");
    assert_eq!(diagnostic.code, "BP_BID_LOCKED");
    assert_eq!(
        state.phase,
        Phase::PlayingTrick {
            leader: BlackglassSeat::East,
            next: BlackglassSeat::East,
            plays: Vec::new(),
            trick_index: 0,
        }
    );
}

#[test]
fn invalid_and_immutable_bid_diagnostics_are_stable() {
    let mut state = setup_match(Seed(1810), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");

    for path in [
        vec!["bid".to_owned(), "0".to_owned()],
        vec!["bid".to_owned(), "14".to_owned()],
        vec!["bid".to_owned(), "pass".to_owned()],
    ] {
        let diagnostic = parse_bid_action_path(&path).expect_err("invalid bid path rejected");
        assert_eq!(diagnostic.code, "BP_BID_OUT_OF_RANGE");
    }

    let diagnostic = apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(0))
        .expect_err("numeric zero rejected");
    assert_eq!(diagnostic.code, "BP_BID_OUT_OF_RANGE");

    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(1)).expect("east bid accepts");
    let diagnostic = apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(2))
        .expect_err("rebid rejected");
    assert_eq!(diagnostic.code, "BP_BID_LOCKED");
}

#[test]
fn dealer_last_bid_has_no_total_thirteen_hook() {
    let mut state = setup_match(Seed(1811), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");

    apply_bid_choice(&mut state, BlackglassSeat::East, Bid::Tricks(6)).expect("east bids");
    apply_bid_choice(&mut state, BlackglassSeat::South, Bid::Tricks(6)).expect("south bids");
    apply_bid_choice(&mut state, BlackglassSeat::West, Bid::Nil).expect("west bids");

    assert_eq!(state.phase_active_bidder(), Some(BlackglassSeat::North));
    let leaves = active_bid_leaf_segments(&state, BlackglassSeat::North);
    assert_eq!(leaves.len(), 14);
    assert!(leaves.contains(&"1".to_owned()));
    assert!(leaves.contains(&"13".to_owned()));
}

#[test]
fn spade_lead_is_blocked_before_broken_when_non_spade_is_held() {
    let mut state = playing_state([
        vec![card(Rank::Ace, Suit::Spades), card(Rank::Two, Suit::Clubs)],
        vec![card(Rank::Three, Suit::Clubs)],
        vec![card(Rank::Four, Suit::Clubs)],
        vec![card(Rank::Five, Suit::Clubs)],
    ]);

    assert_eq!(
        legal_play_cards(&state, BlackglassSeat::East),
        vec![card(Rank::Two, Suit::Clubs)]
    );
    let diagnostic = blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Ace, Suit::Spades),
    )
    .expect_err("spade lead blocked");
    assert_eq!(diagnostic.code, "BP_SPADES_NOT_BROKEN");
}

#[test]
fn only_spades_lead_exception_breaks_spades() {
    let mut state = playing_state([
        vec![card(Rank::Ace, Suit::Spades)],
        vec![card(Rank::Three, Suit::Clubs)],
        vec![card(Rank::Four, Suit::Clubs)],
        vec![card(Rank::Five, Suit::Clubs)],
    ]);

    let effects = blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Ace, Suit::Spades),
    )
    .expect("only spade lead allowed");

    assert!(state.spades_broken);
    assert!(effects.iter().any(|effect| matches!(
        effect,
        blackglass_pact::BlackglassPactEffect::SpadesBroken { .. }
    )));
}

#[test]
fn follower_must_follow_suit_when_holding_led_suit() {
    let mut state = playing_state([
        vec![card(Rank::Two, Suit::Clubs)],
        vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::Three, Suit::Clubs),
        ],
        vec![card(Rank::Four, Suit::Clubs)],
        vec![card(Rank::Five, Suit::Clubs)],
    ]);

    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Two, Suit::Clubs),
    )
    .expect("east leads clubs");
    assert_eq!(
        legal_play_cards(&state, BlackglassSeat::South),
        vec![card(Rank::Three, Suit::Clubs)]
    );
    let diagnostic = blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::South,
        card(Rank::Ace, Suit::Spades),
    )
    .expect_err("must follow clubs");
    assert_eq!(diagnostic.code, "BP_MUST_FOLLOW_SUIT");
}

#[test]
fn void_follower_may_play_spade_off_suit_and_break_spades() {
    let mut state = playing_state([
        vec![card(Rank::Two, Suit::Clubs)],
        vec![card(Rank::Ace, Suit::Spades)],
        vec![card(Rank::Four, Suit::Clubs)],
        vec![card(Rank::Five, Suit::Clubs)],
    ]);

    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Two, Suit::Clubs),
    )
    .expect("east leads clubs");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::South,
        card(Rank::Ace, Suit::Spades),
    )
    .expect("void south may trump");

    assert!(state.spades_broken);
}

#[test]
fn highest_spade_wins_and_winner_leads_next() {
    let mut state = playing_state([
        vec![card(Rank::Two, Suit::Clubs)],
        vec![card(Rank::Ace, Suit::Spades)],
        vec![card(Rank::King, Suit::Clubs)],
        vec![card(Rank::Three, Suit::Clubs)],
    ]);

    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::East,
        card(Rank::Two, Suit::Clubs),
    )
    .expect("east leads");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::South,
        card(Rank::Ace, Suit::Spades),
    )
    .expect("south trumps");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::West,
        card(Rank::King, Suit::Clubs),
    )
    .expect("west follows");
    blackglass_pact::apply_play_choice(
        &mut state,
        BlackglassSeat::North,
        card(Rank::Three, Suit::Clubs),
    )
    .expect("north follows");

    assert_eq!(state.tricks_won[BlackglassSeat::South.index()], 1);
    assert_eq!(
        state.phase,
        Phase::PlayingTrick {
            leader: BlackglassSeat::South,
            next: BlackglassSeat::South,
            plays: Vec::new(),
            trick_index: 1,
        }
    );
}

trait BiddingTestExt {
    fn phase_active_bidder(&self) -> Option<BlackglassSeat>;
}

impl BiddingTestExt for blackglass_pact::BlackglassPactState {
    fn phase_active_bidder(&self) -> Option<BlackglassSeat> {
        match self.phase {
            Phase::Bidding { next, .. } => Some(next),
            _ => None,
        }
    }
}

fn active_bid_leaf_segments(
    state: &blackglass_pact::BlackglassPactState,
    seat: BlackglassSeat,
) -> Vec<String> {
    let tree = legal_action_tree(state, &actor_for(seat));
    let Some(root) = tree.root.choices.first() else {
        return Vec::new();
    };
    root.next
        .as_ref()
        .map(|node| {
            node.choices
                .iter()
                .map(|choice| choice.segment.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn actor_for(seat: BlackglassSeat) -> Actor {
    Actor {
        seat_id: SeatId::from_zero_based_index(seat.index() as u32),
    }
}

fn playing_state(hands_in_play_order: [Vec<CardId>; 4]) -> blackglass_pact::BlackglassPactState {
    let mut state = setup_match(Seed(1816), &canonical_seat_ids(), &SetupOptions::default())
        .expect("setup succeeds");
    let [east, south, west, north] = hands_in_play_order;
    state.private_hands = vec![
        (BlackglassSeat::North, north),
        (BlackglassSeat::East, east),
        (BlackglassSeat::South, south),
        (BlackglassSeat::West, west),
    ];
    state.phase = Phase::PlayingTrick {
        leader: BlackglassSeat::East,
        next: BlackglassSeat::East,
        plays: Vec::new(),
        trick_index: 0,
    };
    state.spades_broken = false;
    state
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}
