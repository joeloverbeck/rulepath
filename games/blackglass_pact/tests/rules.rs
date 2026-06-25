use blackglass_pact::{
    apply_bid_choice, apply_blind_nil_choice, canonical_seat_ids, eligible_blind_nil_order,
    legal_action_tree, parse_bid_action_path, partner_for, setup_match, setup_match_with_scores,
    team_for_seat, validate_standard_seat_count, Bid, BlackglassSeat, BlindNilChoice, Phase,
    SetupOptions, TeamId, ACTION_BID_NIL, STANDARD_HAND_SIZE,
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
