use blackglass_pact::{
    canonical_seat_ids, partner_for, setup_match, team_for_seat, validate_standard_seat_count,
    BlackglassSeat, Phase, SetupOptions, TeamId,
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
        Phase::BlindNilCommitment {
            pending: Vec::new(),
            next_index: 0,
        }
    );
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
