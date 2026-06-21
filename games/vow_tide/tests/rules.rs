use engine_core::{SeatId, Seed};
use vow_tide::{
    ids::{
        canonical_seat_ids, hand_schedule_for_seats, max_hand_size_for_seats, VowTideSeat,
        STANDARD_MAX_SEATS, STANDARD_MIN_SEATS,
    },
    setup::{setup_match, SetupOptions},
    state::Phase,
};

#[test]
fn setup_accepts_three_to_seven_ordered_seats() {
    let options = SetupOptions::default();

    for seat_count in STANDARD_MIN_SEATS as usize..=STANDARD_MAX_SEATS as usize {
        let seats = canonical_seat_ids(seat_count);
        let state = setup_match(Seed(11), &seats, &options).expect("setup succeeds");

        assert_eq!(state.seats, seats);
        assert_eq!(state.dealer, VowTideSeat::Seat0);
        assert_eq!(state.hand_index, 0);
        assert_eq!(state.cumulative_scores.len(), seat_count);
        assert_eq!(state.private_hands.len(), seat_count);
        assert_eq!(
            state.current_hand_size(),
            max_hand_size_for_seats(seat_count)
        );
        assert_eq!(
            state.seat_labels,
            (1..=seat_count)
                .map(|index| format!("Tide {index}"))
                .collect::<Vec<_>>()
        );
        assert!(matches!(state.phase, Phase::Bidding(_)));
        assert_eq!(state.active_seat(), Some(VowTideSeat::Seat1));
    }
}

#[test]
fn setup_rejects_unsupported_seat_counts_with_stable_diagnostic() {
    let options = SetupOptions::default();

    for seat_count in [0, 1, 2, 8] {
        let seats = (0..seat_count)
            .map(|index| SeatId(format!("seat_{index}")))
            .collect::<Vec<_>>();
        let diagnostic =
            setup_match(Seed(0), &seats, &options).expect_err("unsupported count is rejected");

        assert_eq!(diagnostic.code, "VT_INVALID_SEAT_COUNT");
        assert!(diagnostic.message.contains(&seat_count.to_string()));
    }
}

#[test]
fn schedule_is_max_down_to_one_up_to_max_for_every_supported_count() {
    let cases = [
        (3, 10, 19),
        (4, 10, 19),
        (5, 10, 19),
        (6, 8, 15),
        (7, 7, 13),
    ];

    for (seat_count, max_hand_size, total_hands) in cases {
        let schedule = hand_schedule_for_seats(seat_count).expect("supported count has schedule");

        assert_eq!(max_hand_size_for_seats(seat_count), Some(max_hand_size));
        assert_eq!(schedule.len(), total_hands);
        assert_eq!(schedule.first(), Some(&max_hand_size));
        assert_eq!(schedule.last(), Some(&max_hand_size));
        assert_eq!(
            schedule.iter().filter(|hand_size| **hand_size == 1).count(),
            1
        );

        for window in schedule[..max_hand_size as usize].windows(2) {
            assert_eq!(window[0], window[1] + 1);
        }
        for window in schedule[max_hand_size as usize - 1..].windows(2) {
            assert_eq!(window[0] + 1, window[1]);
        }
    }
}
