use std::collections::BTreeSet;

use engine_core::{SeatId, Seed};
use starbridge_crossing::{
    home_spaces, setup_match, SetupOptions, StarPoint, StarZone, STANDARD_PEGS_PER_SEAT,
};

fn seats(count: usize) -> Vec<SeatId> {
    (0..count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

#[test]
fn setup_supports_exact_official_seat_counts() {
    for count in [2, 3, 4, 6] {
        let state =
            setup_match(Seed(7), &seats(count), &SetupOptions::default()).expect("setup succeeds");

        assert_eq!(state.seats.len(), count);
        assert_eq!(state.active_seat_index, 0);
        assert_eq!(
            state.pegs.len(),
            count * usize::from(STANDARD_PEGS_PER_SEAT)
        );
        assert_eq!(state.finish_ranks, Vec::new());
        assert_eq!(state.terminal_status, None);
        assert_eq!(state.ply_count, 0);
        assert_eq!(state.command_count, 0);
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_stable_diagnostics() {
    for count in [1, 5, 7] {
        let diagnostic =
            setup_match(Seed(7), &seats(count), &SetupOptions::default()).expect_err("rejects");

        assert_eq!(diagnostic.code, "invalid_seat_count");
        assert_eq!(
            diagnostic.message,
            format!("starbridge_crossing supports exactly 2, 3, 4, or 6 seats; got {count}")
        );
    }
}

#[test]
fn setup_assigns_homes_and_targets_by_supported_seat_count() {
    let expected = [
        (2, vec![StarPoint::North, StarPoint::South]),
        (
            3,
            vec![StarPoint::North, StarPoint::SouthEast, StarPoint::SouthWest],
        ),
        (
            4,
            vec![
                StarPoint::North,
                StarPoint::NorthEast,
                StarPoint::South,
                StarPoint::SouthWest,
            ],
        ),
        (6, StarPoint::ALL.to_vec()),
    ];

    for (count, homes) in expected {
        let state = setup_match(Seed(7), &seats(count), &SetupOptions::default()).unwrap();
        let actual_homes = state.seats.iter().map(|seat| seat.home).collect::<Vec<_>>();
        let actual_targets = state
            .seats
            .iter()
            .map(|seat| seat.target)
            .collect::<Vec<_>>();

        assert_eq!(actual_homes, homes);
        assert_eq!(
            actual_targets,
            homes
                .iter()
                .copied()
                .map(StarPoint::opposite)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn setup_places_ten_public_pegs_in_each_home() {
    let state = setup_match(Seed(7), &seats(6), &SetupOptions::default()).unwrap();
    let occupied_spaces = state
        .pegs
        .iter()
        .map(|peg| peg.space)
        .collect::<BTreeSet<_>>();

    assert_eq!(occupied_spaces.len(), state.pegs.len());
    for seat in &state.seats {
        let home = home_spaces(seat.home)
            .map(|space| space.id)
            .collect::<BTreeSet<_>>();
        let seat_pegs = state.pegs_for_seat(seat.seat_index).collect::<Vec<_>>();

        assert_eq!(home.len(), usize::from(STANDARD_PEGS_PER_SEAT));
        assert_eq!(seat_pegs.len(), usize::from(STANDARD_PEGS_PER_SEAT));
        for peg in seat_pegs {
            assert!(home.contains(&peg.space));
            assert_eq!(state.occupancy(peg.space), Some(peg.id));
        }
    }
}

#[test]
fn setup_leaves_non_home_spaces_empty() {
    let state = setup_match(Seed(7), &seats(2), &SetupOptions::default()).unwrap();

    for space in starbridge_crossing::spaces() {
        if !matches!(
            space.zone,
            StarZone::Home(StarPoint::North) | StarZone::Home(StarPoint::South)
        ) {
            assert_eq!(state.occupancy(space.id), None);
        }
    }
}
