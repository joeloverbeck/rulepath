use engine_core::{Actor, SeatId, Seed, StableSerialize, Viewer};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use vow_tide::{
    actions::legal_action_tree,
    cards::{Card, Rank, Suit},
    effects::VowTideEffect,
    ids::{canonical_seat_ids, VowTideSeat},
    rules,
    setup::{setup_match, SetupOptions},
    state::{CurrentTrick, Phase, PlayingTrickState},
    visibility::{filter_effects_for_viewer, project_view, PrivateView},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixViewer {
    Observer,
    Seat(VowTideSeat),
}

impl MatrixViewer {
    fn as_viewer(self) -> Viewer {
        match self {
            Self::Observer => Viewer { seat_id: None },
            Self::Seat(seat) => Viewer {
                seat_id: Some(SeatId(seat.as_str().to_owned())),
            },
        }
    }

    const fn seat(self) -> Option<VowTideSeat> {
        match self {
            Self::Observer => None,
            Self::Seat(seat) => Some(seat),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum HandStockSurface {
    View,
    ActionTree,
    Diagnostic,
    Effect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum HandStockCanary {
    Hand,
    Stock,
}

fn matrix_viewers(seat_count: usize) -> Vec<MatrixViewer> {
    let mut viewers = vec![MatrixViewer::Observer];
    viewers.extend(
        VowTideSeat::ALL
            .into_iter()
            .take(seat_count)
            .map(MatrixViewer::Seat),
    );
    viewers
}

fn hand_stock_surfaces() -> Vec<HandStockSurface> {
    vec![
        HandStockSurface::View,
        HandStockSurface::ActionTree,
        HandStockSurface::Diagnostic,
        HandStockSurface::Effect,
    ]
}

fn hand_stock_matrix_state(
    seat_count: usize,
    source: VowTideSeat,
) -> vow_tide::state::VowTideState {
    let mut state = setup_match(
        Seed(20260621),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    install_canary_hands_and_stock(&mut state);
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        trick_index: 0,
        leader: source,
        active_seat: source,
        current_trick: CurrentTrick::new(source),
    });
    state
}

fn hand_stock_snapshot(
    state: &vow_tide::state::VowTideState,
    viewer: &MatrixViewer,
    surface: &HandStockSurface,
) -> String {
    match surface {
        HandStockSurface::View => project_view(state, &viewer.as_viewer()).stable_summary(),
        HandStockSurface::ActionTree => viewer
            .seat()
            .map(|seat| {
                format!(
                    "{:?}",
                    legal_action_tree(
                        state,
                        &Actor {
                            seat_id: SeatId(seat.as_str().to_owned()),
                        },
                    )
                )
            })
            .unwrap_or_default(),
        HandStockSurface::Diagnostic => format!("{:?}", rules::wrong_phase_diagnostic()),
        HandStockSurface::Effect => {
            let effects = vec![
                VowTideEffect::BidAccepted {
                    seat: VowTideSeat::Seat0,
                    bid: 1,
                    public_total: 1,
                },
                VowTideEffect::BiddingCompleted {
                    first_leader: VowTideSeat::Seat0,
                },
            ];
            format!(
                "{:?}",
                filter_effects_for_viewer(&effects, &viewer.as_viewer())
            )
        }
    }
}

fn hand_stock_expectation(
    source: &VowTideSeat,
    viewer: &MatrixViewer,
    surface: &HandStockSurface,
    canary: &HandStockCanary,
) -> ExposureExpectation {
    match canary {
        HandStockCanary::Hand
            if viewer.seat() == Some(*source)
                && matches!(
                    surface,
                    HandStockSurface::View | HandStockSurface::ActionTree
                ) =>
        {
            ExposureExpectation::MustBePresent
        }
        _ => ExposureExpectation::MustBeAbsent,
    }
}

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
fn shared_geometry_hand_and_stock_no_leak_for_three_to_seven_players() {
    for seat_count in 3..=7 {
        for source in VowTideSeat::ALL.into_iter().take(seat_count) {
            let state = hand_stock_matrix_state(seat_count, source);
            let hand_canary = state.hand_for_internal(source)[0].as_str();
            let stock_canary = state.hidden_stock[0].as_str();
            assert_pairwise_no_leak(
                matrix_viewers(seat_count),
                hand_stock_surfaces(),
                [
                    LeakProbe {
                        source_seat: source,
                        canary_id: HandStockCanary::Hand,
                        canary: hand_canary,
                    },
                    LeakProbe {
                        source_seat: source,
                        canary_id: HandStockCanary::Stock,
                        canary: stock_canary,
                    },
                ],
                |viewer, surface| hand_stock_snapshot(&state, viewer, surface),
                hand_stock_expectation,
                |snapshot, canary| snapshot.contains(canary),
            )
            .unwrap_or_else(|failures| {
                panic!(
                    "Vow Tide hand/stock no-leak matrix count {seat_count} source {source:?}: {failures}"
                )
            });
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
