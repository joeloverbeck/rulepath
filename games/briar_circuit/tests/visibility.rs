use briar_circuit::{
    apply_pass_action, effect_envelopes, filter_effects_for_viewer, project_action_previews,
    project_pass_view, project_view, setup_match, BriarCircuitEffect, BriarCircuitSeat, Card,
    CurrentTrick, PassAction, PassDirection, PassState, Phase, PlayingTrickState, Rank,
    SetupOptions, Suit, TrickPlay,
};
use engine_core::{SeatId, Seed, Viewer, VisibilityScope};

fn viewer(seat: Option<BriarCircuitSeat>) -> Viewer {
    Viewer {
        seat_id: seat.map(|seat| SeatId(seat.as_str().to_owned())),
    }
}

fn card(rank: Rank, suit: Suit) -> briar_circuit::CardId {
    Card::new(rank, suit).id()
}

#[test]
fn pass_view_shows_only_viewers_own_selection() {
    let mut state = setup_match(
        Seed(1608),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let card = state.hand_for_internal(seat)[0];

    apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select succeeds");

    let owner = project_pass_view(&state, &viewer(Some(seat))).expect("pass view");
    assert_eq!(owner.direction, "left");
    assert_eq!(owner.committed_count, 0);
    assert_eq!(owner.pending_count, 4);
    assert_eq!(owner.own_selection, vec![card]);
    assert!(!owner.own_committed);

    let other =
        project_pass_view(&state, &viewer(Some(BriarCircuitSeat::Seat1))).expect("other pass view");
    assert!(other.own_selection.is_empty());
    assert!(!other.own_committed);

    let observer = project_pass_view(&state, &viewer(None)).expect("observer pass view");
    assert!(observer.own_selection.is_empty());
    assert!(!observer.own_committed);
}

#[test]
fn pass_commitment_public_effect_carries_only_counts_and_direction() {
    let mut state = setup_match(
        Seed(1609),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat = BriarCircuitSeat::Seat0;
    let cards = state.hand_for_internal(seat)[..3].to_vec();

    for card in cards {
        apply_pass_action(&mut state, seat, PassAction::Select(card)).expect("select succeeds");
    }
    let result =
        apply_pass_action(&mut state, seat, PassAction::Confirm).expect("confirm succeeds");

    assert!(result.effects.iter().any(|effect| {
        matches!(
            effect,
            BriarCircuitEffect::PassCommitmentPublic(status)
                if status.direction == PassDirection::Left
                    && status.committed_count == 1
                    && status.pending_count == 3
        )
    }));

    let owner = project_pass_view(&state, &viewer(Some(seat))).expect("pass view");
    assert!(owner.own_committed);
}

#[test]
fn play_diagnostics_do_not_expose_hidden_alternatives() {
    let led = Card::new(Rank::King, Suit::Spades).id();
    let owned_spade = Card::new(Rank::Two, Suit::Spades).id();
    let illegal_club = Card::new(Rank::Ace, Suit::Clubs).id();
    let mut state = setup_match(
        Seed(1611),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    state.private_hands = vec![
        (BriarCircuitSeat::Seat0, vec![]),
        (BriarCircuitSeat::Seat1, vec![owned_spade, illegal_club]),
        (BriarCircuitSeat::Seat2, vec![]),
        (BriarCircuitSeat::Seat3, vec![]),
    ];
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 2,
        leader: BriarCircuitSeat::Seat0,
        active_seat: BriarCircuitSeat::Seat1,
        current_trick: CurrentTrick {
            leader: BriarCircuitSeat::Seat0,
            plays: vec![TrickPlay {
                seat: BriarCircuitSeat::Seat0,
                card: led,
            }],
        },
    });

    let err = briar_circuit::validate_play_card(&state, BriarCircuitSeat::Seat1, illegal_club)
        .expect_err("must-follow violation rejects");

    assert_eq!(err.code, "BC_MUST_FOLLOW_SUIT");
    assert!(!err.message.contains(&owned_spade.as_str()));
    assert!(!err.message.contains(BriarCircuitSeat::Seat0.as_str()));
}

#[test]
fn pairwise_view_projection_hides_other_hands_and_pass_selections() {
    let mut state = setup_match(
        Seed(1612),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let hidden_by_seat = [
        (BriarCircuitSeat::Seat0, card(Rank::Ace, Suit::Clubs)),
        (BriarCircuitSeat::Seat1, card(Rank::Ace, Suit::Diamonds)),
        (BriarCircuitSeat::Seat2, card(Rank::Ace, Suit::Hearts)),
        (BriarCircuitSeat::Seat3, card(Rank::Ace, Suit::Spades)),
    ];
    state.private_hands = hidden_by_seat
        .iter()
        .map(|(seat, card)| (*seat, vec![*card]))
        .collect();
    let mut pass = PassState::new(PassDirection::Left);
    for (seat, card) in hidden_by_seat {
        pass.selection_for_mut(seat).expect("selection").push(card);
    }
    state.phase = Phase::Passing(pass);

    for (source, hidden_card) in hidden_by_seat {
        let hidden = format!("{hidden_card:?}");
        let owner_payload = format!("{:?}", project_view(&state, &viewer(Some(source))));
        assert!(owner_payload.contains(&hidden));

        for unauthorized in BriarCircuitSeat::ALL {
            if unauthorized == source {
                continue;
            }
            let payload = format!("{:?}", project_view(&state, &viewer(Some(unauthorized))));
            assert!(
                !payload.contains(&hidden),
                "viewer {unauthorized:?} leaked {source:?} card {hidden}"
            );
        }

        let observer_payload = format!("{:?}", project_view(&state, &viewer(None)));
        assert!(!observer_payload.contains(&hidden));
    }
}

#[test]
fn action_previews_are_only_available_to_active_owner() {
    let source = BriarCircuitSeat::Seat2;
    let hidden_card = card(Rank::Nine, Suit::Diamonds);
    let mut state = setup_match(
        Seed(1613),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    state.private_hands = BriarCircuitSeat::ALL
        .into_iter()
        .map(|seat| {
            if seat == source {
                (seat, vec![hidden_card])
            } else {
                (seat, Vec::new())
            }
        })
        .collect();
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 4,
        leader: source,
        active_seat: source,
        current_trick: CurrentTrick::new(source),
    });
    let hidden = format!("{hidden_card:?}");

    let owner_previews = format!(
        "{:?}",
        project_action_previews(&state, &viewer(Some(source)))
    );
    assert!(owner_previews.contains(&hidden));

    for unauthorized in BriarCircuitSeat::ALL {
        if unauthorized == source {
            continue;
        }
        let payload = format!(
            "{:?}",
            project_action_previews(&state, &viewer(Some(unauthorized)))
        );
        assert!(!payload.contains(&hidden));
    }
    assert!(!format!("{:?}", project_action_previews(&state, &viewer(None))).contains(&hidden));
}

#[test]
fn private_effects_are_filtered_independently_from_views() {
    let source = BriarCircuitSeat::Seat1;
    let selected = card(Rank::King, Suit::Hearts);
    let received = card(Rank::Two, Suit::Clubs);
    let envelopes: Vec<_> = [
        BriarCircuitEffect::PassSelectionUpdated {
            seat: source,
            selected_count: 1,
            selected_cards: vec![selected],
        },
        BriarCircuitEffect::PassExchangePrivate {
            seat: source,
            sent_cards: vec![selected],
            received_cards: vec![received],
        },
        BriarCircuitEffect::PassCommitmentPublic(briar_circuit::PassCommitmentStatus {
            direction: PassDirection::Left,
            committed_count: 1,
            pending_count: 3,
        }),
    ]
    .into_iter()
    .flat_map(effect_envelopes)
    .collect();
    let selected_canary = format!("{selected:?}");
    let received_canary = format!("{received:?}");

    let owner_effects = format!(
        "{:?}",
        filter_effects_for_viewer(&envelopes, &viewer(Some(source)))
    );
    assert!(owner_effects.contains(&selected_canary));
    assert!(owner_effects.contains(&received_canary));

    for unauthorized in BriarCircuitSeat::ALL {
        if unauthorized == source {
            continue;
        }
        let payload = format!(
            "{:?}",
            filter_effects_for_viewer(&envelopes, &viewer(Some(unauthorized)))
        );
        assert!(!payload.contains(&selected_canary));
        assert!(!payload.contains(&received_canary));
        assert!(payload.contains("PassCommitmentPublic"));
    }

    let observer_payload = format!("{:?}", filter_effects_for_viewer(&envelopes, &viewer(None)));
    assert!(!observer_payload.contains(&selected_canary));
    assert!(!observer_payload.contains(&received_canary));
    assert!(observer_payload.contains("PassCommitmentPublic"));
}

#[test]
fn played_card_identity_is_public_without_pass_provenance() {
    let played = card(Rank::Seven, Suit::Clubs);
    let public_effects = effect_envelopes(BriarCircuitEffect::CardPlayed {
        seat: BriarCircuitSeat::Seat3,
        card: played,
    });
    assert_eq!(public_effects.len(), 1);
    assert!(matches!(
        public_effects[0].visibility,
        VisibilityScope::Public
    ));
    assert_eq!(
        public_effects[0].payload,
        BriarCircuitEffect::CardPlayed {
            seat: BriarCircuitSeat::Seat3,
            card: played,
        }
    );
    let payload = format!(
        "{:?}",
        filter_effects_for_viewer(&public_effects, &viewer(None))
    );

    assert!(payload.contains(&format!("{played:?}")));
    assert!(!payload.contains("PassExchangePrivate"));
    assert!(!payload.contains("PassSelectionUpdated"));
}

/// Commits a seat's three-card pass using the first three owned cards.
fn commit_seat_pass(state: &mut briar_circuit::BriarCircuitState, seat: BriarCircuitSeat) {
    for _ in 0..3 {
        let card = *state
            .hand_for_internal(seat)
            .iter()
            .find(|card| !pass_selection(state, seat).contains(card))
            .expect("a selectable card exists");
        apply_pass_action(state, seat, PassAction::Select(card)).expect("select succeeds");
    }
    apply_pass_action(state, seat, PassAction::Confirm).expect("confirm succeeds");
}

fn pass_selection(
    state: &briar_circuit::BriarCircuitState,
    seat: BriarCircuitSeat,
) -> Vec<briar_circuit::CardId> {
    match &state.phase {
        Phase::Passing(pass) => pass.selection_for(seat).to_vec(),
        _ => Vec::new(),
    }
}

#[test]
fn passing_phase_reports_next_uncommitted_seat_as_active() {
    // Hand 0 is a left-pass hand, so the match opens in the pass phase. The simultaneous
    // pass is driven one commitment at a time, so the public view must surface the next
    // seat that still owes a commitment so the turn machinery can advance every seat.
    let mut state = setup_match(
        Seed(1608),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    assert!(matches!(state.phase, Phase::Passing(_)));

    let active =
        |state: &briar_circuit::BriarCircuitState| project_view(state, &viewer(None)).active_seat;

    assert_eq!(active(&state), Some(BriarCircuitSeat::Seat0));
    commit_seat_pass(&mut state, BriarCircuitSeat::Seat0);
    assert_eq!(active(&state), Some(BriarCircuitSeat::Seat1));
    commit_seat_pass(&mut state, BriarCircuitSeat::Seat1);
    assert_eq!(active(&state), Some(BriarCircuitSeat::Seat2));
    commit_seat_pass(&mut state, BriarCircuitSeat::Seat2);
    assert_eq!(active(&state), Some(BriarCircuitSeat::Seat3));

    // The fourth commitment triggers the atomic exchange and enters trick play, so the
    // active seat becomes the two-of-clubs opener rather than a passing seat.
    commit_seat_pass(&mut state, BriarCircuitSeat::Seat3);
    assert!(matches!(state.phase, Phase::PlayingTrick(_)));
    assert_eq!(active(&state), Some(state.two_clubs_leader()));
}
