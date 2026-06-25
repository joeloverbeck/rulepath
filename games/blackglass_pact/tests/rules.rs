use blackglass_pact::{
    apply_blind_nil_choice, canonical_seat_ids, eligible_blind_nil_order, partner_for, setup_match,
    setup_match_with_scores, team_for_seat, validate_standard_seat_count, Bid, BlackglassSeat,
    BlindNilChoice, Phase, SetupOptions, TeamId, STANDARD_HAND_SIZE,
};
use engine_core::{SeatId, Seed};

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
