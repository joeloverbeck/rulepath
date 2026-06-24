use briar_circuit::{
    apply_pass_action, apply_play_action, effect_envelopes, filter_effects_for_viewer,
    legal_bot_actions, project_action_previews, project_pass_view, project_view,
    replay_support::{export_viewer_timeline, ViewerExportClass},
    setup_match, BriarCircuitEffect, BriarCircuitL1Bot, BriarCircuitSeat, Card, CurrentTrick,
    PassAction, PassDirection, PassState, Phase, PlayAction, PlayingTrickState, Rank, SetupOptions,
    Suit, TrickPlay,
};
use engine_core::{SeatId, Seed, Viewer, VisibilityScope};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};

fn viewer(seat: Option<BriarCircuitSeat>) -> Viewer {
    Viewer {
        seat_id: seat.map(|seat| SeatId(seat.as_str().to_owned())),
    }
}

fn card(rank: Rank, suit: Suit) -> briar_circuit::CardId {
    Card::new(rank, suit).id()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PassMatrixViewer {
    Observer,
    Seat(BriarCircuitSeat),
}

impl PassMatrixViewer {
    fn as_viewer(self) -> Viewer {
        match self {
            Self::Observer => viewer(None),
            Self::Seat(seat) => viewer(Some(seat)),
        }
    }

    const fn seat(self) -> Option<BriarCircuitSeat> {
        match self {
            Self::Observer => None,
            Self::Seat(seat) => Some(seat),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PassMatrixCase {
    Selected,
    Committed,
    Exchanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PassMatrixSurface {
    View,
    PassView,
    FilteredEffects,
    ActionPreviews,
    ViewerExport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PlayMatrixCase {
    PrivatePrePlay,
    PublicAfterPlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PlayMatrixSurface {
    View,
    FilteredEffects,
    ActionPreviews,
    ViewerExport,
    Diagnostic,
    BotLegalActions,
    BotExplanation,
}

fn pass_matrix_viewers() -> Vec<PassMatrixViewer> {
    let mut viewers = vec![PassMatrixViewer::Observer];
    viewers.extend(BriarCircuitSeat::ALL.map(PassMatrixViewer::Seat));
    viewers
}

fn pass_matrix_surfaces(case: PassMatrixCase) -> Vec<PassMatrixSurface> {
    match case {
        PassMatrixCase::Selected | PassMatrixCase::Committed => vec![
            PassMatrixSurface::View,
            PassMatrixSurface::PassView,
            PassMatrixSurface::FilteredEffects,
            PassMatrixSurface::ActionPreviews,
            PassMatrixSurface::ViewerExport,
        ],
        PassMatrixCase::Exchanged => vec![
            PassMatrixSurface::View,
            PassMatrixSurface::FilteredEffects,
            PassMatrixSurface::ViewerExport,
        ],
    }
}

fn play_matrix_surfaces() -> Vec<PlayMatrixSurface> {
    vec![
        PlayMatrixSurface::View,
        PlayMatrixSurface::FilteredEffects,
        PlayMatrixSurface::ActionPreviews,
        PlayMatrixSurface::ViewerExport,
        PlayMatrixSurface::Diagnostic,
        PlayMatrixSurface::BotLegalActions,
        PlayMatrixSurface::BotExplanation,
    ]
}

fn viewer_export_class(viewer: PassMatrixViewer) -> ViewerExportClass {
    match viewer {
        PassMatrixViewer::Observer => ViewerExportClass::Public,
        PassMatrixViewer::Seat(seat) => ViewerExportClass::SeatPrivate(seat),
    }
}

fn pass_matrix_state(
    source: BriarCircuitSeat,
    case: PassMatrixCase,
) -> (
    briar_circuit::BriarCircuitState,
    Vec<BriarCircuitEffect>,
    briar_circuit::CardId,
) {
    let mut state = setup_match(
        Seed(1608),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    let canary = state.hand_for_internal(source)[0];
    let mut effects = Vec::new();

    match case {
        PassMatrixCase::Selected => {
            effects.extend(
                apply_pass_action(&mut state, source, PassAction::Select(canary))
                    .expect("select succeeds")
                    .effects,
            );
        }
        PassMatrixCase::Committed => {
            let cards = state.hand_for_internal(source)[..3].to_vec();
            for card in cards {
                effects.extend(
                    apply_pass_action(&mut state, source, PassAction::Select(card))
                        .expect("select succeeds")
                        .effects,
                );
            }
            effects.extend(
                apply_pass_action(&mut state, source, PassAction::Confirm)
                    .expect("confirm succeeds")
                    .effects,
            );
        }
        PassMatrixCase::Exchanged => {
            for seat in BriarCircuitSeat::ALL {
                let mut cards = state.hand_for_internal(seat)[..3].to_vec();
                if seat == source && !cards.contains(&canary) {
                    cards[0] = canary;
                }
                for card in cards {
                    effects.extend(
                        apply_pass_action(&mut state, seat, PassAction::Select(card))
                            .expect("select succeeds")
                            .effects,
                    );
                }
                effects.extend(
                    apply_pass_action(&mut state, seat, PassAction::Confirm)
                        .expect("confirm succeeds")
                        .effects,
                );
            }
        }
    }

    (state, effects, canary)
}

fn play_matrix_state(
    source: BriarCircuitSeat,
    case: PlayMatrixCase,
) -> (
    briar_circuit::BriarCircuitState,
    Vec<BriarCircuitEffect>,
    briar_circuit::CardId,
) {
    let canaries = [
        card(Rank::Ace, Suit::Clubs),
        card(Rank::Ace, Suit::Diamonds),
        card(Rank::Ace, Suit::Hearts),
        card(Rank::Ace, Suit::Spades),
    ];
    let mut state = setup_match(
        Seed(1614),
        &briar_circuit::canonical_seat_ids(),
        &SetupOptions::default(),
    )
    .expect("setup succeeds");
    state.private_hands = BriarCircuitSeat::ALL
        .into_iter()
        .enumerate()
        .map(|(index, seat)| (seat, vec![canaries[index]]))
        .collect();
    state.phase = Phase::PlayingTrick(PlayingTrickState {
        hearts_broken: true,
        trick_index: 4,
        leader: source,
        active_seat: source,
        current_trick: CurrentTrick::new(source),
    });

    let canary = canaries[source.index()];
    let effects = match case {
        PlayMatrixCase::PrivatePrePlay => Vec::new(),
        PlayMatrixCase::PublicAfterPlay => {
            apply_play_action(&mut state, source, PlayAction::Play(canary))
                .expect("play succeeds")
                .effects
        }
    };

    (state, effects, canary)
}

fn pass_matrix_snapshot(
    state: &briar_circuit::BriarCircuitState,
    effects: &[BriarCircuitEffect],
    matrix_viewer: &PassMatrixViewer,
    surface: &PassMatrixSurface,
) -> String {
    match surface {
        PassMatrixSurface::View => format!("{:?}", project_view(state, &matrix_viewer.as_viewer())),
        PassMatrixSurface::PassView => {
            format!("{:?}", project_pass_view(state, &matrix_viewer.as_viewer()))
        }
        PassMatrixSurface::FilteredEffects => {
            let envelopes = effects
                .iter()
                .cloned()
                .flat_map(effect_envelopes)
                .collect::<Vec<_>>();
            format!(
                "{:?}",
                filter_effects_for_viewer(&envelopes, &matrix_viewer.as_viewer())
            )
        }
        PassMatrixSurface::ActionPreviews => {
            format!(
                "{:?}",
                project_action_previews(state, &matrix_viewer.as_viewer())
            )
        }
        PassMatrixSurface::ViewerExport => {
            format!(
                "{:?}",
                export_viewer_timeline(state, viewer_export_class(*matrix_viewer))
            )
        }
    }
}

fn play_matrix_snapshot(
    state: &briar_circuit::BriarCircuitState,
    effects: &[BriarCircuitEffect],
    matrix_viewer: &PassMatrixViewer,
    surface: &PlayMatrixSurface,
) -> String {
    match surface {
        PlayMatrixSurface::View => format!("{:?}", project_view(state, &matrix_viewer.as_viewer())),
        PlayMatrixSurface::FilteredEffects => {
            let envelopes = effects
                .iter()
                .cloned()
                .flat_map(effect_envelopes)
                .collect::<Vec<_>>();
            format!(
                "{:?}",
                filter_effects_for_viewer(&envelopes, &matrix_viewer.as_viewer())
            )
        }
        PlayMatrixSurface::ActionPreviews => {
            format!(
                "{:?}",
                project_action_previews(state, &matrix_viewer.as_viewer())
            )
        }
        PlayMatrixSurface::ViewerExport => {
            format!(
                "{:?}",
                export_viewer_timeline(state, viewer_export_class(*matrix_viewer))
            )
        }
        PlayMatrixSurface::Diagnostic => {
            let seat = matrix_viewer.seat().unwrap_or(BriarCircuitSeat::Seat0);
            let wrong_card = card(Rank::Two, Suit::Clubs);
            format!(
                "{:?}",
                briar_circuit::validate_play_card(state, seat, wrong_card)
            )
        }
        PlayMatrixSurface::BotLegalActions => matrix_viewer
            .seat()
            .map(|seat| format!("{:?}", legal_bot_actions(state, seat)))
            .unwrap_or_default(),
        PlayMatrixSurface::BotExplanation => matrix_viewer
            .seat()
            .and_then(|seat| {
                BriarCircuitL1Bot::new(Seed(1614))
                    .select_decision(state, seat)
                    .ok()
                    .map(|decision| decision.explanation)
            })
            .unwrap_or_default(),
    }
}

fn pass_matrix_expectation(
    case: PassMatrixCase,
    source: BriarCircuitSeat,
    viewer: &PassMatrixViewer,
    surface: &PassMatrixSurface,
) -> ExposureExpectation {
    let target = source.pass_left_target();
    match case {
        PassMatrixCase::Selected | PassMatrixCase::Committed => {
            if viewer.seat() == Some(source) && *surface != PassMatrixSurface::ActionPreviews {
                ExposureExpectation::MustBePresent
            } else {
                ExposureExpectation::MustBeAbsent
            }
        }
        PassMatrixCase::Exchanged => match surface {
            PassMatrixSurface::FilteredEffects if viewer.seat() == Some(source) => {
                ExposureExpectation::MustBePresent
            }
            PassMatrixSurface::FilteredEffects if viewer.seat() == Some(target) => {
                ExposureExpectation::MustBePresent
            }
            PassMatrixSurface::View | PassMatrixSurface::ViewerExport
                if viewer.seat() == Some(target) =>
            {
                ExposureExpectation::MustBePresent
            }
            _ => ExposureExpectation::MustBeAbsent,
        },
    }
}

fn play_matrix_expectation(
    case: PlayMatrixCase,
    source: BriarCircuitSeat,
    viewer: &PassMatrixViewer,
    surface: &PlayMatrixSurface,
) -> ExposureExpectation {
    match case {
        PlayMatrixCase::PrivatePrePlay => match surface {
            PlayMatrixSurface::View
            | PlayMatrixSurface::ActionPreviews
            | PlayMatrixSurface::ViewerExport
            | PlayMatrixSurface::BotLegalActions
                if viewer.seat() == Some(source) =>
            {
                ExposureExpectation::MustBePresent
            }
            _ => ExposureExpectation::MustBeAbsent,
        },
        PlayMatrixCase::PublicAfterPlay => match surface {
            PlayMatrixSurface::View
            | PlayMatrixSurface::FilteredEffects
            | PlayMatrixSurface::ViewerExport => ExposureExpectation::MustBePresent,
            _ => ExposureExpectation::MustBeAbsent,
        },
    }
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
fn pass_phase_pairwise_no_leak_matrix_covers_selection_commit_and_exchange() {
    for source in BriarCircuitSeat::ALL {
        for case in [
            PassMatrixCase::Selected,
            PassMatrixCase::Committed,
            PassMatrixCase::Exchanged,
        ] {
            let (state, effects, canary) = pass_matrix_state(source, case);
            let canary = format!("{canary:?}");
            assert_pairwise_no_leak(
                pass_matrix_viewers(),
                pass_matrix_surfaces(case),
                [LeakProbe {
                    source_seat: source,
                    canary_id: case,
                    canary,
                }],
                |matrix_viewer, surface| {
                    pass_matrix_snapshot(&state, &effects, matrix_viewer, surface)
                },
                |probe_source, matrix_viewer, surface, _case| {
                    pass_matrix_expectation(case, *probe_source, matrix_viewer, surface)
                },
                |snapshot, canary| snapshot.contains(canary),
            )
            .unwrap_or_else(|failures| {
                panic!("Briar pass-phase no-leak matrix {source:?} {case:?}: {failures}")
            });
        }
    }
}

#[test]
fn play_export_and_bot_pairwise_no_leak_matrix_covers_private_and_public_cards() {
    for source in BriarCircuitSeat::ALL {
        for case in [
            PlayMatrixCase::PrivatePrePlay,
            PlayMatrixCase::PublicAfterPlay,
        ] {
            let (state, effects, canary) = play_matrix_state(source, case);
            let canary = format!("{canary:?}");
            assert_pairwise_no_leak(
                pass_matrix_viewers(),
                play_matrix_surfaces(),
                [LeakProbe {
                    source_seat: source,
                    canary_id: case,
                    canary,
                }],
                |matrix_viewer, surface| {
                    play_matrix_snapshot(&state, &effects, matrix_viewer, surface)
                },
                |probe_source, matrix_viewer, surface, _case| {
                    play_matrix_expectation(case, *probe_source, matrix_viewer, surface)
                },
                |snapshot, canary| snapshot.contains(canary),
            )
            .unwrap_or_else(|failures| {
                panic!("Briar play/export/bot no-leak matrix {source:?} {case:?}: {failures}")
            });
        }
    }
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
    let source_seat_id = SeatId(source.as_str().to_owned());
    let private_scopes = envelopes
        .iter()
        .filter_map(|effect| match &effect.visibility {
            VisibilityScope::PrivateToSeat(seat_id) => Some(seat_id),
            VisibilityScope::Public => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(private_scopes, vec![&source_seat_id, &source_seat_id]);
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
