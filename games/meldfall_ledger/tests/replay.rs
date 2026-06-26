use engine_core::{FreshnessToken, SeatId, Seed, Viewer};
use meldfall_ledger::{
    actions::draw_source_action_tree,
    cards::{Card, CardId, Rank, Suit},
    effects::{private_stock_draw_effect, public_effect, DrawSource, MeldfallEffect},
    replay_support::{export_viewer_snapshot, import_viewer_export},
    setup::{default_seats, setup_match, SetupOptions},
    state::MatchState,
};

#[test]
fn viewer_scoped_exports_round_trip_for_public_and_all_six_seats() {
    let state = export_state();
    let action_tree = draw_source_action_tree(FreshnessToken(34), &state.round);
    let effects = export_effects(&state);

    for viewer in all_export_viewers() {
        let export = export_viewer_snapshot(&state, &action_tree, &effects, &viewer);
        let imported = import_viewer_export(&export, &viewer).expect("same viewer imports");

        assert_eq!(imported.viewer, export.viewer);
        assert_eq!(imported.steps, export.steps);
        assert_eq!(
            export.stable_string(),
            export_viewer_snapshot(&state, &action_tree, &effects, &viewer).stable_string()
        );
        assert_eq!(
            export.stable_hash(),
            export_viewer_snapshot(&state, &action_tree, &effects, &viewer).stable_hash()
        );
    }
}

#[test]
fn viewer_scoped_import_rejects_privilege_elevation_and_cross_seat_replay() {
    let state = export_state();
    let action_tree = draw_source_action_tree(FreshnessToken(34), &state.round);
    let effects = export_effects(&state);

    let public_export = export_viewer_snapshot(&state, &action_tree, &effects, &viewer(None));
    assert!(import_viewer_export(&public_export, &viewer(Some(0))).is_err());

    let seat_zero_export = export_viewer_snapshot(&state, &action_tree, &effects, &viewer(Some(0)));
    assert!(import_viewer_export(&seat_zero_export, &viewer(None)).is_err());
    assert!(import_viewer_export(&seat_zero_export, &viewer(Some(1))).is_err());
}

#[test]
fn viewer_scoped_exports_do_not_leak_hidden_hands_or_stock_order() {
    let state = export_state();
    let action_tree = draw_source_action_tree(FreshnessToken(34), &state.round);
    let effects = export_effects(&state);

    for viewer_index in [None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)] {
        let export = export_viewer_snapshot(&state, &action_tree, &effects, &viewer(viewer_index));
        let surface = format!("{}|{}", export.stable_string(), export.to_json());

        for stock_card in &state.round.stock {
            assert!(
                !surface.contains(&stock_card.as_str()),
                "viewer {viewer_index:?} export leaked stock card {}",
                stock_card.as_str()
            );
        }
        for (source_index, seat) in state.round.seats.iter().enumerate() {
            for hidden in &seat.hand {
                if viewer_index == Some(source_index) {
                    assert!(
                        surface.contains(&hidden.as_str()),
                        "seat {source_index} export omitted own card {}",
                        hidden.as_str()
                    );
                } else {
                    assert!(
                        !surface.contains(&hidden.as_str()),
                        "viewer {viewer_index:?} export leaked seat {source_index} card {}",
                        hidden.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn no_leak_trace_inventory_documents_viewer_export_scope() {
    for trace in [
        include_str!("golden_traces/public-observer-no-leak-6p.trace.json"),
        include_str!("golden_traces/seat-private-export-round-trip-all-viewers.trace.json"),
        include_str!("golden_traces/viewer-export-no-privilege-elevation.trace.json"),
    ] {
        assert!(trace.contains("\"game_id\": \"meldfall_ledger\""));
        assert!(trace.contains("\"public_no_leak\": true"));
    }
}

fn export_state() -> MatchState {
    let seats = default_seats(6).expect("seat count supported");
    let setup = setup_match(Seed(1913), &seats, &SetupOptions::default()).expect("setup succeeds");
    let mut state = MatchState::from_initial_setup(setup);
    state.round.stock = vec![
        card(Rank::Ace, Suit::Spades),
        card(Rank::King, Suit::Spades),
    ];
    state.round.discard = vec![card(Rank::Nine, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::Two, Suit::Clubs)];
    state.round.seats[1].hand = vec![card(Rank::Three, Suit::Diamonds)];
    state.round.seats[2].hand = vec![card(Rank::Four, Suit::Hearts)];
    state.round.seats[3].hand = vec![card(Rank::Five, Suit::Spades)];
    state.round.seats[4].hand = vec![card(Rank::Six, Suit::Clubs)];
    state.round.seats[5].hand = vec![card(Rank::Seven, Suit::Diamonds)];
    state
}

fn export_effects(state: &MatchState) -> Vec<engine_core::EffectEnvelope<MeldfallEffect>> {
    vec![
        public_effect(MeldfallEffect::Draw {
            seat: 0,
            source: DrawSource::Stock,
            cards_moved: 1,
            stock_count_after: state.round.stock.len(),
            discard_count_after: state.round.discard.len(),
        }),
        private_stock_draw_effect(
            state.seats[0].clone(),
            0,
            card(Rank::Queen, Suit::Hearts),
            1,
        ),
    ]
}

fn all_export_viewers() -> Vec<Viewer> {
    let mut viewers = vec![viewer(None)];
    viewers.extend((0..6).map(|index| viewer(Some(index))));
    viewers
}

fn viewer(seat_index: Option<usize>) -> Viewer {
    Viewer {
        seat_id: seat_index.map(|index| SeatId(format!("seat_{index}"))),
    }
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}
