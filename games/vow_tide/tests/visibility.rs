use engine_core::{SeatId, Seed, StableSerialize, Viewer};
use vow_tide::{
    cards::{Card, Rank, Suit},
    effects::VowTideEffect,
    ids::{canonical_seat_ids, VowTideSeat},
    setup::{setup_match, SetupOptions},
    visibility::{filter_effects_for_viewer, project_view, PrivateView},
};

#[test]
fn exhaustive_seat_pair_no_leak_for_three_to_seven_players() {
    for seat_count in 3..=7 {
        let mut state = setup_match(
            Seed(20260621),
            &canonical_seat_ids(seat_count),
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        install_canary_hands_and_stock(&mut state);
        let stock_canary = state.hidden_stock[0].as_str();

        let observer = project_view(&state, &Viewer { seat_id: None });
        let observer_summary = observer.stable_summary();
        assert!(!observer_summary.contains(&stock_canary));
        for seat in VowTideSeat::ALL.into_iter().take(seat_count) {
            for card in state.hand_for_internal(seat) {
                assert!(
                    !observer_summary.contains(&card.as_str()),
                    "observer leaked {} from {}",
                    card.as_str(),
                    seat.as_str()
                );
            }
        }

        for viewer_seat in VowTideSeat::ALL.into_iter().take(seat_count) {
            let viewer = Viewer {
                seat_id: Some(SeatId(viewer_seat.as_str().to_owned())),
            };
            let view = project_view(&state, &viewer);
            let summary = view.stable_summary();
            assert!(!summary.contains(&stock_canary));
            assert!(matches!(view.private_view, PrivateView::Seat(_)));

            for source_seat in VowTideSeat::ALL.into_iter().take(seat_count) {
                for card in state.hand_for_internal(source_seat) {
                    let contains = summary.contains(&card.as_str());
                    assert_eq!(
                        contains,
                        source_seat == viewer_seat,
                        "viewer {} leak check for {} from {}",
                        viewer_seat.as_str(),
                        card.as_str(),
                        source_seat.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn projected_view_stable_bytes_are_deterministic_and_viewer_distinct() {
    let mut state = setup_match(Seed(9), &canonical_seat_ids(4), &SetupOptions::default())
        .expect("setup succeeds");
    install_canary_hands_and_stock(&mut state);

    let seat_0 = Viewer {
        seat_id: Some(SeatId("seat_0".to_owned())),
    };
    let seat_1 = Viewer {
        seat_id: Some(SeatId("seat_1".to_owned())),
    };
    let first = project_view(&state, &seat_0).stable_bytes();
    let second = project_view(&state, &seat_0).stable_bytes();
    let other = project_view(&state, &seat_1).stable_bytes();

    assert_eq!(first, second);
    assert_ne!(first, other);
}

#[test]
fn public_effect_filter_does_not_expose_hidden_stock_or_unplayed_hands() {
    let mut state = setup_match(Seed(13), &canonical_seat_ids(4), &SetupOptions::default())
        .expect("setup succeeds");
    install_canary_hands_and_stock(&mut state);
    let forbidden = state
        .private_hands
        .iter()
        .flat_map(|(_, hand)| hand.iter())
        .chain(state.hidden_stock.iter())
        .map(|card| card.as_str())
        .collect::<Vec<_>>();
    let effects = vec![
        VowTideEffect::BidAccepted {
            seat: VowTideSeat::Seat0,
            bid: 1,
            public_total: 1,
        },
        VowTideEffect::HandAdvanced {
            hand_index: 1,
            dealer: VowTideSeat::Seat1,
            hand_size: 9,
            trump_indicator: Card::new(Rank::Ace, Suit::Spades).id(),
        },
    ];
    let visible = filter_effects_for_viewer(&effects, &Viewer { seat_id: None });
    let effect_text = format!("{visible:?}");

    for token in forbidden {
        assert!(!effect_text.contains(&token), "effect leaked {token}");
    }
}

fn install_canary_hands_and_stock(state: &mut vow_tide::state::VowTideState) {
    let canaries = [
        Card::new(Rank::Two, Suit::Clubs).id(),
        Card::new(Rank::Three, Suit::Diamonds).id(),
        Card::new(Rank::Four, Suit::Hearts).id(),
        Card::new(Rank::Five, Suit::Spades).id(),
        Card::new(Rank::Six, Suit::Clubs).id(),
        Card::new(Rank::Seven, Suit::Diamonds).id(),
        Card::new(Rank::Eight, Suit::Hearts).id(),
    ];
    for (seat, card) in VowTideSeat::ALL
        .into_iter()
        .take(state.seat_count())
        .zip(canaries)
    {
        *state.hand_for_internal_mut(seat).expect("hand exists") = vec![card];
    }
    state.trump_indicator = Card::new(Rank::Ace, Suit::Spades).id();
    state.hidden_stock = vec![Card::new(Rank::King, Suit::Clubs).id()];
}
