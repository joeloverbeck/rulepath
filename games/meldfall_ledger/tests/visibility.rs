use engine_core::{Diagnostic, EffectEnvelope, FreshnessToken, SeatId, Seed, Viewer};
use meldfall_ledger::{
    actions::{draw_source_action_tree, table_action_tree, LayoffPosition, MeldfallAction},
    cards::{Card, CardId, Rank, Suit},
    effects::{effect_stable_string, private_stock_draw_effect, public_effect, MeldfallEffect},
    setup::{default_seats, setup_match, SetupOptions},
    state::{MatchState, MeldId},
    visibility::{
        contains_card_id_in_debug, project_action_tree_for_viewer, project_effects_for_viewer,
        project_view, redact_diagnostic_for_viewer, PrivateView,
    },
};

#[test]
fn public_observer_view_exposes_counts_and_public_cards_not_hidden_hands_or_stock() {
    let state = visibility_state();
    let observer = viewer(None);
    let view = project_view(&state, &observer);

    assert_eq!(view.stock_count, 2);
    assert_eq!(view.hand_counts, vec![2, 2, 1]);
    assert_eq!(view.discard, vec![card(Rank::Nine, Suit::Clubs).as_str()]);
    assert_eq!(view.private, PrivateView::Observer);

    let surface = format!("{view:?}|{}", view.stable_string());
    for hidden in hidden_hand_and_stock_cards(&state) {
        assert!(
            !surface.contains(&hidden.as_str()),
            "observer view leaked hidden card {} in {surface}",
            hidden.as_str()
        );
    }
}

#[test]
fn seat_private_view_exposes_only_the_viewer_hand() {
    let state = visibility_state();

    for viewer_seat in 0..state.seats.len() {
        let view = project_view(&state, &viewer(Some(viewer_seat)));
        let surface = format!("{view:?}|{}", view.stable_string());

        for card in &state.round.seats[viewer_seat].hand {
            assert!(
                surface.contains(&card.as_str()),
                "seat {viewer_seat} did not see own card {}",
                card.as_str()
            );
        }
        for (source_seat, seat) in state.round.seats.iter().enumerate() {
            if source_seat == viewer_seat {
                continue;
            }
            for hidden in &seat.hand {
                assert!(
                    !surface.contains(&hidden.as_str()),
                    "seat {viewer_seat} leaked seat {source_seat} card {}",
                    hidden.as_str()
                );
            }
        }
        for hidden_stock in &state.round.stock {
            assert!(
                !surface.contains(&hidden_stock.as_str()),
                "seat {viewer_seat} leaked stock card {}",
                hidden_stock.as_str()
            );
        }
    }
}

#[test]
fn action_tree_projection_keeps_active_hand_actions_private() {
    let mut state = visibility_state();
    state.round.active_seat_index = 0;
    let active_card = state.round.seats[0].hand[0];
    let other_card = state.round.seats[1].hand[0];
    let table_tree = table_action_tree(
        FreshnessToken(12),
        vec![MeldfallAction::MeldNew {
            cards: vec![
                active_card,
                state.round.seats[0].hand[1],
                card(Rank::Ten, Suit::Clubs),
            ],
        }],
        vec![MeldfallAction::LayOff {
            card: active_card,
            target_meld: MeldId(0),
            position: LayoffPosition::Append,
        }],
        vec![MeldfallAction::Discard { card: active_card }],
        false,
    );

    let active_surface = format!(
        "{:?}",
        project_action_tree_for_viewer(&table_tree, &state, &viewer(Some(0)))
    );
    assert!(active_surface.contains(&active_card.as_str()));

    for forbidden_viewer in [None, Some(1), Some(2)] {
        let projected =
            project_action_tree_for_viewer(&table_tree, &state, &viewer(forbidden_viewer));
        let surface = format!("{projected:?}");
        assert!(!surface.contains(&active_card.as_str()));
        assert!(!surface.contains(&other_card.as_str()));
        assert_eq!(
            projected
                .root
                .choices
                .iter()
                .map(|choice| choice.segment.as_str())
                .collect::<Vec<_>>(),
            vec!["finish-turn"]
        );
    }
}

#[test]
fn public_draw_action_tree_exposes_discard_indices_not_stock_identity() {
    let state = visibility_state();
    let stock_top = *state.round.stock.last().expect("stock has hidden top");
    let tree = draw_source_action_tree(FreshnessToken(8), &state.round);

    for viewer_seat in [None, Some(0), Some(1), Some(2)] {
        let projected = project_action_tree_for_viewer(&tree, &state, &viewer(viewer_seat));
        let surface = format!("{projected:?}");
        assert!(surface.contains("draw-stock"));
        assert!(surface.contains("draw-discard-0"));
        assert!(!surface.contains(&stock_top.as_str()));
    }
}

#[test]
fn effect_filtering_keeps_private_stock_draw_visible_only_to_owner() {
    let state = visibility_state();
    let hidden_drawn = card(Rank::King, Suit::Spades);
    let effects = vec![
        public_effect(MeldfallEffect::Draw {
            seat: 0,
            source: meldfall_ledger::effects::DrawSource::Stock,
            cards_moved: 1,
            stock_count_after: 1,
            discard_count_after: 1,
        }),
        private_stock_draw_effect(state.seats[0].clone(), 0, hidden_drawn, 1),
    ];

    let owner_surface = filtered_effect_surface(&effects, &viewer(Some(0)));
    assert!(owner_surface.contains(&hidden_drawn.as_str()));

    for forbidden_viewer in [None, Some(1), Some(2)] {
        let surface = filtered_effect_surface(&effects, &viewer(forbidden_viewer));
        assert!(surface.contains("Draw"));
        assert!(!surface.contains(&hidden_drawn.as_str()));
    }
}

#[test]
fn diagnostics_can_be_redacted_before_reaching_forbidden_viewers() {
    let hidden_card = card(Rank::Queen, Suit::Diamonds);
    let diagnostic = Diagnostic {
        code: "ML_TEST".to_owned(),
        message: format!("test diagnostic mentions {}", hidden_card.as_str()),
    };

    let redacted = redact_diagnostic_for_viewer(&diagnostic, false);
    assert_eq!(redacted.code, diagnostic.code);
    assert!(!redacted.message.contains(&hidden_card.as_str()));
    assert!(redacted.message.contains("hidden_card"));

    let authorized = redact_diagnostic_for_viewer(&diagnostic, true);
    assert!(authorized.message.contains(&hidden_card.as_str()));
}

fn visibility_state() -> meldfall_ledger::state::MatchState {
    let seats = default_seats(3).expect("seat count supported");
    let setup = setup_match(Seed(1912), &seats, &SetupOptions::default()).expect("setup succeeds");
    let mut state = MatchState::from_initial_setup(setup);
    state.round.stock = vec![
        card(Rank::Ace, Suit::Spades),
        card(Rank::King, Suit::Spades),
    ];
    state.round.discard = vec![card(Rank::Nine, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::Two, Suit::Clubs), card(Rank::Three, Suit::Clubs)];
    state.round.seats[1].hand = vec![
        card(Rank::Four, Suit::Diamonds),
        card(Rank::Five, Suit::Diamonds),
    ];
    state.round.seats[2].hand = vec![card(Rank::Six, Suit::Hearts)];
    state
}

fn viewer(seat_index: Option<usize>) -> Viewer {
    Viewer {
        seat_id: seat_index.map(|index| SeatId(format!("seat_{index}"))),
    }
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn hidden_hand_and_stock_cards(state: &meldfall_ledger::state::MatchState) -> Vec<CardId> {
    state
        .round
        .seats
        .iter()
        .flat_map(|seat| seat.hand.iter().copied())
        .chain(state.round.stock.iter().copied())
        .collect()
}

fn filtered_effect_surface(effects: &[EffectEnvelope<MeldfallEffect>], viewer: &Viewer) -> String {
    project_effects_for_viewer(effects, viewer)
        .iter()
        .map(effect_stable_string)
        .collect::<Vec<_>>()
        .join("|")
}

#[test]
fn contains_card_id_helper_detects_debug_surfaces() {
    let hidden = card(Rank::Jack, Suit::Hearts);
    assert!(contains_card_id_in_debug(&vec![hidden.as_str()], hidden));
}
