use engine_core::{
    ActionPath, Actor, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, Viewer,
};
use plain_tricks::{
    apply_action, filter_effects_for_viewer, legal_action_tree, project_view, setup_effects,
    setup_match, CardView, PlainTricksSeat, PrivateView, SetupOptions, TrickCardId,
    ValidatedAction,
};

fn seat_id(id: &str) -> SeatId {
    SeatId(id.to_owned())
}

fn actor(id: &str) -> Actor {
    Actor {
        seat_id: seat_id(id),
    }
}

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

fn seat_viewer(id: &str) -> Viewer {
    Viewer {
        seat_id: Some(seat_id(id)),
    }
}

fn command(state_freshness: FreshnessToken, seat: &str, card: TrickCardId) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec!["play".to_owned(), card.as_str().to_owned()],
        },
        freshness_token: state_freshness,
        rules_version: RulesVersion(1),
    }
}

fn validated(actor: PlainTricksSeat, card: TrickCardId) -> ValidatedAction {
    ValidatedAction {
        actor,
        card,
        round_index: 0,
        trick_index: 0,
    }
}

fn own_hand(state: &plain_tricks::PlainTricksState, seat_id: &str) -> Vec<TrickCardId> {
    let view = project_view(state, &seat_viewer(seat_id));
    let PrivateView::Seat(private) = view.private_view else {
        panic!("seat viewer gets private view");
    };
    private
        .own_hand
        .iter()
        .map(card_from_view)
        .collect::<Vec<_>>()
}

fn card_from_view(card: &CardView) -> TrickCardId {
    TrickCardId::parse(&card.card_id).expect("view contains known card id")
}

fn setup_with_void_follow() -> (plain_tricks::PlainTricksState, TrickCardId, TrickCardId) {
    for seed in 0..500 {
        let state = setup_match(
            Seed(seed),
            &[seat_id("seat_0"), seat_id("seat_1")],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        let seat_0 = own_hand(&state, "seat_0");
        let seat_1 = own_hand(&state, "seat_1");
        for lead in &seat_0 {
            if !seat_1.iter().any(|card| card.suit() == lead.suit()) {
                let follow = seat_1[0];
                return (state, *lead, follow);
            }
        }
    }
    panic!("expected a seed with a void follow case");
}

fn assert_absent(text: &str, card: TrickCardId) {
    let debug_name = format!("{card:?}");
    assert!(
        !text.contains(card.as_str()),
        "unexpected hidden id {} in {text}",
        card.as_str()
    );
    assert!(
        !text.contains(&card.label()),
        "unexpected hidden label {} in {text}",
        card.label()
    );
    assert!(
        !text.contains(&debug_name),
        "unexpected hidden debug name {debug_name} in {text}"
    );
}

fn first_legal_card(state: &plain_tricks::PlainTricksState, seat_id: &str) -> TrickCardId {
    let tree = legal_action_tree(state, &actor(seat_id));
    let play = tree
        .root
        .choices
        .first()
        .expect("play family exists for active actor");
    let card_segment = play
        .next
        .as_ref()
        .and_then(|node| node.choices.first())
        .map(|choice| choice.segment.as_str())
        .expect("legal card choice exists");
    TrickCardId::parse(card_segment).expect("legal choice is a known card")
}

#[test]
fn observer_view_action_tree_effects_and_diagnostics_do_not_leak_setup_cards() {
    let state = setup_match(
        Seed(41),
        &[seat_id("seat_0"), seat_id("seat_1")],
        &SetupOptions::default(),
    )
    .expect("setup succeeds");

    let observer_view_text = format!("{:?}", project_view(&state, &observer()));
    let non_actor_tree_text = format!("{:?}", legal_action_tree(&state, &actor("seat_1")));
    let observer_effect_text = format!(
        "{:?}",
        filter_effects_for_viewer(&setup_effects(&state), &observer())
    );
    let diagnostic_text = format!(
        "{:?}",
        plain_tricks::validate_command(
            &state,
            &command(state.freshness_token, "seat_1", TrickCardId::Gale1),
        )
        .expect_err("non-actor command rejects")
    );

    for card in TrickCardId::ALL {
        assert_absent(&observer_view_text, card);
        assert_absent(&non_actor_tree_text, card);
        assert_absent(&observer_effect_text, card);
        assert_absent(&diagnostic_text, card);
    }
}

#[test]
fn seat_private_view_contains_only_own_hand_and_never_tail_or_opponent_hand() {
    let state = setup_match(
        Seed(53),
        &[seat_id("seat_0"), seat_id("seat_1")],
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat_0_view = project_view(&state, &seat_viewer("seat_0"));
    let seat_1_view = project_view(&state, &seat_viewer("seat_1"));

    let plain_tricks::PrivateView::Seat(seat_0_private) = &seat_0_view.private_view else {
        panic!("seat_0 gets private view");
    };
    let plain_tricks::PrivateView::Seat(seat_1_private) = &seat_1_view.private_view else {
        panic!("seat_1 gets private view");
    };

    let seat_0_text = format!("{seat_0_view:?}");
    let seat_1_text = format!("{seat_1_view:?}");
    let seat_0_ids = seat_0_private
        .own_hand
        .iter()
        .map(|card| card.card_id.as_str())
        .collect::<Vec<_>>();
    let seat_1_ids = seat_1_private
        .own_hand
        .iter()
        .map(|card| card.card_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(seat_0_ids.len(), 6);
    assert_eq!(seat_1_ids.len(), 6);

    for card in TrickCardId::ALL {
        if seat_0_ids.contains(&card.as_str()) {
            assert!(seat_0_text.contains(card.as_str()));
        } else {
            assert_absent(&seat_0_text, card);
        }
        if seat_1_ids.contains(&card.as_str()) {
            assert!(seat_1_text.contains(card.as_str()));
        } else {
            assert_absent(&seat_1_text, card);
        }
    }
}

#[test]
fn played_cards_become_public_and_void_has_no_explicit_flag() {
    let (mut state, lead_card, follow_card) = setup_with_void_follow();

    let _lead_effects = apply_action(&mut state, validated(PlainTricksSeat::Seat0, lead_card))
        .expect("lead applies");
    let follow_effects = apply_action(&mut state, validated(PlainTricksSeat::Seat1, follow_card))
        .expect("void follow applies");

    let observer_view = project_view(&state, &observer());
    let view_text = format!("{observer_view:?}");
    let public_effect_text = format!(
        "{:?}",
        filter_effects_for_viewer(&follow_effects, &observer())
    );

    assert!(view_text.contains(lead_card.as_str()));
    assert!(view_text.contains(follow_card.as_str()));
    assert!(
        public_effect_text.contains(follow_card.as_str())
            || public_effect_text.contains(&format!("{follow_card:?}"))
    );
    assert!(!view_text.to_ascii_lowercase().contains("void"));
    assert!(!public_effect_text.to_ascii_lowercase().contains("void"));
    for hidden in TrickCardId::ALL {
        if hidden != lead_card && hidden != follow_card {
            assert_absent(&view_text, hidden);
            assert_absent(&public_effect_text, hidden);
        }
    }
}

#[test]
fn terminal_view_still_does_not_reveal_tail() {
    let mut state = setup_match(
        Seed(67),
        &[seat_id("seat_0"), seat_id("seat_1")],
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut publicly_played = Vec::new();
    while state.terminal_outcome.is_none() {
        let actor = state.active_seat.expect("active seat before terminal");
        let seat_name = actor.as_str();
        let card = first_legal_card(&state, seat_name);
        publicly_played.push(card);
        apply_action(&mut state, validated(actor, card)).expect("legal action applies");
    }

    let text = format!("{:?}", project_view(&state, &observer()));
    for card in TrickCardId::ALL {
        if publicly_played.contains(&card) {
            assert!(text.contains(card.as_str()));
        } else {
            assert_absent(&text, card);
        }
    }
}

#[test]
fn non_actor_tree_is_empty_even_when_follow_suit_would_depend_on_private_hand() {
    let (mut state, lead_card, _) = setup_with_void_follow();
    apply_action(&mut state, validated(PlainTricksSeat::Seat0, lead_card)).expect("lead applies");

    let tree = legal_action_tree(&state, &actor("seat_0"));
    assert!(tree.root.choices.is_empty());
    let tree_text = format!("{tree:?}");
    for card in TrickCardId::ALL {
        assert_absent(&tree_text, card);
    }
}
