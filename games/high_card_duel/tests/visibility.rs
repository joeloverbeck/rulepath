use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectCursor, EffectLog, FreshnessToken, RulesVersion,
    SeatId, Seed, Viewer, VisibilityScope,
};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use high_card_duel::{
    action_tree_v1_bytes, apply_action, cards_revealed_effect, commit_face_down_effect,
    commit_segment, deal_private_card_effect, export_public_observer_replay,
    generate_internal_full_trace, hand_count_changed_effect, legal_action_tree,
    own_commit_confirmed_effect, private_diagnostic_effect, project_view, public_diagnostic_effect,
    refill_started_effect, round_scored_effect, setup_match, terminal_effect, validate_command,
    CardId, HighCardDuelEffect, HighCardDuelRandomBot, HighCardDuelSeat, OutcomeRationaleView,
    Phase, PrivateView, RoundOutcomeBreakdownView, Score, SetupOptions, Sigil, TerminalOutcome,
    TerminalView,
};

fn seat_id(index: u8) -> SeatId {
    SeatId(format!("seat-{index}"))
}

fn card(rank: u8, sigil: Sigil) -> CardId {
    CardId::new(rank, sigil).expect("test card is valid")
}

fn seats() -> Vec<SeatId> {
    vec![seat_id(0), seat_id(1)]
}

fn viewer(index: Option<u8>) -> Viewer {
    Viewer {
        seat_id: index.map(seat_id),
    }
}

fn command(actor_index: u8, card: CardId, freshness_token: FreshnessToken) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: seat_id(actor_index),
        },
        action_path: ActionPath {
            segments: vec![commit_segment(card)],
        },
        freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn apply_card(state: &mut high_card_duel::HighCardDuelState, actor_index: u8, card: CardId) {
    let command = command(actor_index, card, state.freshness_token);
    let action = validate_command(state, &command).expect("command validates");
    apply_action(state, action);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixViewer {
    Observer,
    Seat0,
    Seat1,
}

impl MatrixViewer {
    fn as_viewer(self) -> Viewer {
        match self {
            MatrixViewer::Observer => viewer(None),
            MatrixViewer::Seat0 => viewer(Some(0)),
            MatrixViewer::Seat1 => viewer(Some(1)),
        }
    }

    fn seat(self) -> Option<HighCardDuelSeat> {
        match self {
            MatrixViewer::Observer => None,
            MatrixViewer::Seat0 => Some(HighCardDuelSeat::Seat0),
            MatrixViewer::Seat1 => Some(HighCardDuelSeat::Seat1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixSurface {
    View,
    ActionTree,
    Diagnostic,
    Effect,
    ReplayExport,
    BotInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum CanaryId {
    Hand,
    ExportHiddenTail,
}

fn matrix_viewers() -> Vec<MatrixViewer> {
    vec![
        MatrixViewer::Observer,
        MatrixViewer::Seat0,
        MatrixViewer::Seat1,
    ]
}

fn matrix_surfaces() -> Vec<MatrixSurface> {
    vec![
        MatrixSurface::View,
        MatrixSurface::ActionTree,
        MatrixSurface::Diagnostic,
        MatrixSurface::Effect,
        MatrixSurface::ReplayExport,
        MatrixSurface::BotInput,
    ]
}

fn source_index(source: HighCardDuelSeat) -> u8 {
    match source {
        HighCardDuelSeat::Seat0 => 0,
        HighCardDuelSeat::Seat1 => 1,
    }
}

fn no_leak_probes() -> Vec<LeakProbe<HighCardDuelSeat, CanaryId, String>> {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let trace = generate_internal_full_trace(7);
    let replay = high_card_duel::replay_internal_full_trace(&trace);
    let hidden_tail = replay
        .final_state
        .deck
        .first()
        .copied()
        .expect("full trace leaves an unrevealed deck tail card");

    vec![
        LeakProbe {
            source_seat: HighCardDuelSeat::Seat0,
            canary_id: CanaryId::Hand,
            canary: state.hand_for(HighCardDuelSeat::Seat0)[0].stable_id(),
        },
        LeakProbe {
            source_seat: HighCardDuelSeat::Seat1,
            canary_id: CanaryId::Hand,
            canary: state.hand_for(HighCardDuelSeat::Seat1)[0].stable_id(),
        },
        LeakProbe {
            source_seat: HighCardDuelSeat::Seat0,
            canary_id: CanaryId::ExportHiddenTail,
            canary: hidden_tail.stable_id(),
        },
    ]
}

fn no_leak_snapshot(viewer: &MatrixViewer, surface: &MatrixSurface) -> String {
    match surface {
        MatrixSurface::View => {
            let state =
                setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
            project_view(&state, &viewer.as_viewer()).stable_summary()
        }
        MatrixSurface::ActionTree => {
            let mut state =
                setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
            if let Some(seat) = viewer.seat() {
                state.lead_seat = seat;
            }
            let actor = Actor {
                seat_id: viewer.seat().map_or_else(
                    || SeatId("observer".to_owned()),
                    |seat| seat_id(source_index(seat)),
                ),
            };
            format!("{:?}", legal_action_tree(&state, &actor))
        }
        MatrixSurface::Diagnostic => {
            let state =
                setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
            let bad_command = CommandEnvelope {
                actor: Actor {
                    seat_id: seat_id(1),
                },
                action_path: ActionPath {
                    segments: vec![commit_segment(state.hand_for(HighCardDuelSeat::Seat1)[0])],
                },
                freshness_token: state.freshness_token,
                rules_version: RulesVersion(1),
            };
            format!("{:?}", validate_command(&state, &bad_command))
        }
        MatrixSurface::Effect => {
            let state =
                setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
            let mut log = EffectLog::new();
            log.push(own_commit_confirmed_effect(
                HighCardDuelSeat::Seat0,
                seat_id(0),
                state.hand_for(HighCardDuelSeat::Seat0)[0],
                1,
            ));
            log.push(own_commit_confirmed_effect(
                HighCardDuelSeat::Seat1,
                seat_id(1),
                state.hand_for(HighCardDuelSeat::Seat1)[0],
                1,
            ));
            log.since(EffectCursor(0), &viewer.as_viewer())
                .iter()
                .filter_map(|entry| match &entry.envelope.payload {
                    HighCardDuelEffect::DealPrivateCard { card, .. }
                    | HighCardDuelEffect::OwnCommitConfirmed { card, .. } => Some(card.stable_id()),
                    HighCardDuelEffect::CardsRevealed {
                        seat_0_card,
                        seat_1_card,
                        ..
                    } => Some(format!(
                        "{}:{}",
                        seat_0_card.stable_id(),
                        seat_1_card.stable_id()
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("|")
        }
        MatrixSurface::ReplayExport => {
            let trace = generate_internal_full_trace(7);
            export_public_observer_replay(&trace).to_json()
        }
        MatrixSurface::BotInput => match viewer.seat() {
            Some(seat) => {
                let state = setup_match(Seed(7), &seats(), &SetupOptions::default())
                    .expect("setup succeeds");
                HighCardDuelRandomBot::input_for(&state, seat).stable_summary()
            }
            None => String::new(),
        },
    }
}

fn no_leak_expectation(
    source: &HighCardDuelSeat,
    viewer: &MatrixViewer,
    surface: &MatrixSurface,
    canary_id: &CanaryId,
) -> ExposureExpectation {
    match (surface, canary_id) {
        (MatrixSurface::ReplayExport, CanaryId::ExportHiddenTail) => {
            ExposureExpectation::MustBeAbsent
        }
        (MatrixSurface::ReplayExport, CanaryId::Hand) => ExposureExpectation::NotApplicable,
        (_, CanaryId::ExportHiddenTail) => ExposureExpectation::NotApplicable,
        (MatrixSurface::Diagnostic, CanaryId::Hand) => ExposureExpectation::MustBeAbsent,
        (MatrixSurface::View, CanaryId::Hand)
        | (MatrixSurface::ActionTree, CanaryId::Hand)
        | (MatrixSurface::Effect, CanaryId::Hand)
        | (MatrixSurface::BotInput, CanaryId::Hand) => {
            if viewer.seat() == Some(*source) {
                ExposureExpectation::MustBePresent
            } else {
                ExposureExpectation::MustBeAbsent
            }
        }
    }
}

#[test]
fn no_leak_harness_covers_public_seat_replay_effect_and_bot_surfaces() {
    assert_pairwise_no_leak(
        matrix_viewers(),
        matrix_surfaces(),
        no_leak_probes(),
        no_leak_snapshot,
        no_leak_expectation,
        |snapshot, canary| snapshot.contains(canary),
    )
    .expect("high card duel pairwise no-leak matrix passes");
}

#[test]
fn residual_profile_tree_count_effect_and_rng_surfaces_keep_lead_commit_hidden() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_hand = state.hand_for(HighCardDuelSeat::Seat0).to_vec();
    let reply_hand = state.hand_for(HighCardDuelSeat::Seat1).to_vec();
    let lead_card = lead_hand[0];
    let lead_id = lead_card.stable_id();
    let effects = {
        let command = command(0, lead_card, state.freshness_token);
        let action = validate_command(&state, &command).expect("lead commit validates");
        apply_action(&mut state, action)
    };

    let observer_view = project_view(&state, &viewer(None));
    let owner_view = project_view(&state, &viewer(Some(0)));
    let opponent_view = project_view(&state, &viewer(Some(1)));
    assert_eq!(observer_view.hand_counts.seat_0, 2);
    assert_eq!(observer_view.hand_counts.seat_1, 3);
    assert_eq!(observer_view.deck_count, 18);
    assert!(!observer_view.stable_summary().contains(&lead_id));
    assert!(owner_view.stable_summary().contains(&lead_id));
    assert!(!opponent_view.stable_summary().contains(&lead_id));

    let reply_actor = Actor {
        seat_id: seat_id(1),
    };
    let reply_tree = legal_action_tree(&state, &reply_actor);
    let reply_tree_debug = format!("{reply_tree:?}");
    let reply_tree_v1 = String::from_utf8_lossy(&action_tree_v1_bytes(&reply_tree)).into_owned();
    for hidden in lead_hand.iter().chain(state.deck.iter()) {
        let id = hidden.stable_id();
        assert!(!reply_tree_debug.contains(&id));
        assert!(!reply_tree_v1.contains(&id));
    }
    for card in &reply_hand {
        let id = card.stable_id();
        assert!(reply_tree_debug.contains(&id));
        assert!(reply_tree_v1.contains(&id));
    }

    let mut log = EffectLog::new();
    for effect in effects {
        log.push(effect);
    }
    let observer_effects = filtered_private_card_ids(&log, &viewer(None));
    let owner_effects = filtered_private_card_ids(&log, &viewer(Some(0)));
    let opponent_effects = filtered_private_card_ids(&log, &viewer(Some(1)));
    assert!(!observer_effects.contains(&lead_id));
    assert!(owner_effects.contains(&lead_id));
    assert!(!opponent_effects.contains(&lead_id));

    let reply_input = HighCardDuelRandomBot::input_for(&state, HighCardDuelSeat::Seat1);
    let reply_summary = reply_input.stable_summary();
    assert!(!reply_summary.contains(&lead_id));
    for card in &state.deck {
        assert!(!reply_summary.contains(&card.stable_id()));
    }
    let decision = HighCardDuelRandomBot::new(Seed(99))
        .select_decision(&state, HighCardDuelSeat::Seat1)
        .expect("reply bot chooses legal private card");
    let decision_debug = format!("{decision:?}");
    assert!(!decision_debug.contains(&lead_id));
    for card in &reply_hand {
        assert!(reply_summary.contains(&card.stable_id()));
    }

    let trace = generate_internal_full_trace(7);
    let replay = high_card_duel::replay_internal_full_trace(&trace);
    let public_export = export_public_observer_replay(&trace);
    let public_export_json = public_export.to_json();
    assert_eq!(public_export.export_class, "public_observer_projection_v1");
    assert_eq!(public_export.viewer, "observer");
    assert!(!public_export_json.contains("\"seed\""));
    assert!(!public_export_json.contains("commit/hcd:r"));
    for hidden_card in replay.final_state.deck.iter() {
        assert!(!public_export_json.contains(&hidden_card.stable_id()));
    }
}

fn filtered_private_card_ids(log: &EffectLog<HighCardDuelEffect>, viewer: &Viewer) -> String {
    log.since(EffectCursor(0), viewer)
        .iter()
        .filter_map(|entry| match &entry.envelope.payload {
            HighCardDuelEffect::DealPrivateCard { card, .. }
            | HighCardDuelEffect::OwnCommitConfirmed { card, .. } => Some(card.stable_id()),
            HighCardDuelEffect::CardsRevealed {
                seat_0_card,
                seat_1_card,
                ..
            } => Some(format!(
                "{}:{}",
                seat_0_card.stable_id(),
                seat_1_card.stable_id()
            )),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("|")
}

#[test]
fn effect_visibility_scopes_match_spec() {
    let private_owner = seat_id(0);
    let private_card = card(7, Sigil::A);
    let public_card = card(8, Sigil::B);
    let score = Score {
        seat_0: 1,
        seat_1: 0,
    };

    let effects = vec![
        (
            "hcd_deal_private_card",
            deal_private_card_effect(HighCardDuelSeat::Seat0, private_owner.clone(), private_card),
            VisibilityScope::PrivateToSeat(private_owner.clone()),
        ),
        (
            "hcd_hand_count_changed",
            hand_count_changed_effect(3, 3, 18),
            VisibilityScope::Public,
        ),
        (
            "hcd_commit_face_down",
            commit_face_down_effect(HighCardDuelSeat::Seat0, 1),
            VisibilityScope::Public,
        ),
        (
            "hcd_own_commit_confirmed",
            own_commit_confirmed_effect(
                HighCardDuelSeat::Seat0,
                private_owner.clone(),
                private_card,
                1,
            ),
            VisibilityScope::PrivateToSeat(private_owner.clone()),
        ),
        (
            "hcd_cards_revealed",
            cards_revealed_effect(1, private_card, public_card),
            VisibilityScope::Public,
        ),
        (
            "hcd_round_scored",
            round_scored_effect(1, Some(HighCardDuelSeat::Seat1), score),
            VisibilityScope::Public,
        ),
        (
            "hcd_refill_started",
            refill_started_effect(2, HighCardDuelSeat::Seat1),
            VisibilityScope::Public,
        ),
        (
            "hcd_terminal",
            terminal_effect(Some(HighCardDuelSeat::Seat0), score),
            VisibilityScope::Public,
        ),
        (
            "hcd_private_diagnostic",
            private_diagnostic_effect(
                HighCardDuelSeat::Seat0,
                private_owner.clone(),
                "invalid_private_card",
                "that card is not in your hand",
            ),
            VisibilityScope::PrivateToSeat(private_owner),
        ),
        (
            "hcd_public_diagnostic",
            public_diagnostic_effect("wrong_seat", "it is not this seat's turn"),
            VisibilityScope::Public,
        ),
    ];

    for (expected_kind, effect, expected_visibility) in effects {
        assert_eq!(effect.payload.kind(), expected_kind);
        assert_eq!(effect.visibility, expected_visibility);
    }
}

#[test]
fn effect_public_effects_contain_no_private_card_identity() {
    let pre_reveal_public_effects = [
        hand_count_changed_effect(3, 3, 18),
        commit_face_down_effect(HighCardDuelSeat::Seat0, 1),
        round_scored_effect(
            1,
            None,
            Score {
                seat_0: 0,
                seat_1: 0,
            },
        ),
        refill_started_effect(2, HighCardDuelSeat::Seat1),
        terminal_effect(
            None,
            Score {
                seat_0: 3,
                seat_1: 3,
            },
        ),
        public_diagnostic_effect("invalid_private_card", "private card redacted"),
    ];

    for effect in pre_reveal_public_effects {
        assert_eq!(effect.visibility, VisibilityScope::Public);
        assert!(!effect.payload.public_payload_text().contains("hcd:r"));
    }

    let reveal = cards_revealed_effect(1, card(3, Sigil::A), card(9, Sigil::B));
    assert_eq!(reveal.visibility, VisibilityScope::Public);
    assert_eq!(reveal.payload.kind(), "hcd_cards_revealed");
    assert!(reveal.payload.public_payload_text().contains("hcd:r03:a"));
    assert!(reveal.payload.public_payload_text().contains("hcd:r09:b"));
}

#[test]
fn effect_private_card_identity_is_private_to_owner() {
    let owner = seat_id(1);
    let effect = own_commit_confirmed_effect(
        HighCardDuelSeat::Seat1,
        owner.clone(),
        card(12, Sigil::B),
        1,
    );

    assert_eq!(effect.visibility, VisibilityScope::PrivateToSeat(owner));
    let HighCardDuelEffect::OwnCommitConfirmed { card, .. } = effect.payload else {
        panic!("expected own commit confirmation");
    };
    assert_eq!(card.stable_id(), "hcd:r12:b");
}

#[test]
fn observer_view_has_no_private_hand_or_deck_or_facedown_identity() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let view = project_view(&state, &viewer(None));

    assert!(matches!(view.private_view, PrivateView::Observer));
    assert_eq!(view.deck_count, 18);
    assert!(view.commitments.seat_0.card.is_none());
    assert!(view.commitments.seat_1.card.is_none());
    assert!(!view.stable_summary().contains("hcd:r"));
}

#[test]
fn seat_view_contains_only_own_hand() {
    let state = setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let view = project_view(&state, &viewer(Some(0)));
    let PrivateView::Seat { hand, .. } = view.private_view else {
        panic!("seat viewer gets private seat view");
    };

    assert_eq!(hand.len(), 3);
    for card in state.hand_for(HighCardDuelSeat::Seat0) {
        assert!(hand
            .iter()
            .any(|view_card| view_card.card_id == card.stable_id()));
    }
    for card in state.hand_for(HighCardDuelSeat::Seat1) {
        assert!(!hand
            .iter()
            .any(|view_card| view_card.card_id == card.stable_id()));
    }
}

#[test]
fn reply_actor_view_lacks_lead_card_before_reveal() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_card = state.hand_for(HighCardDuelSeat::Seat0)[0];
    apply_card(&mut state, 0, lead_card);

    let reply_view = project_view(&state, &viewer(Some(1)));

    assert_eq!(reply_view.phase, Phase::ReplyCommit);
    assert_eq!(reply_view.commitments.seat_0.status, "face_down");
    assert!(reply_view.commitments.seat_0.card.is_none());
    assert!(!reply_view.stable_summary().contains(&lead_card.stable_id()));
}

#[test]
fn terminal_public_view_still_hides_unused_deck_tail() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let hidden_tail = card(12, Sigil::B);
    state.phase = Phase::Terminal;
    state.terminal_outcome = Some(TerminalOutcome::Draw);
    state.deck = vec![hidden_tail];
    state.revealed_history.push(high_card_duel::RevealedRound {
        round_number: 6,
        seat_0_card: card(4, Sigil::A),
        seat_1_card: card(4, Sigil::B),
        winner: None,
    });

    let public_view = project_view(&state, &viewer(None));

    assert_eq!(public_view.deck_count, 1);
    assert!(!public_view
        .stable_summary()
        .contains(&hidden_tail.stable_id()));
    assert!(public_view.stable_summary().contains("hcd:r04:a"));
    assert!(public_view.stable_summary().contains("hcd:r04:b"));
    assert_eq!(
        public_view.terminal,
        TerminalView::Draw {
            rationale: OutcomeRationaleView {
                result_kind: "draw".to_owned(),
                decisive_cause: "final_score_after_round_limit".to_owned(),
                template_key: "high_card_duel.final_score_draw".to_owned(),
                decisive_rule_ids: vec![
                    "HCD-ROUND-006".to_owned(),
                    "HCD-END-001".to_owned(),
                    "HCD-END-003".to_owned(),
                ],
                final_score: public_view.score,
                round_breakdowns: vec![RoundOutcomeBreakdownView {
                    round_number: 6,
                    seat_0_rank: 4,
                    seat_1_rank: 4,
                    winner: None,
                    point_delta_seat_0: 0,
                    point_delta_seat_1: 0,
                    cumulative_score: Score {
                        seat_0: 0,
                        seat_1: 0,
                    },
                }],
            }
        }
    );
}

#[test]
fn terminal_win_rationale_uses_revealed_round_history_only() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let hidden_tail = card(12, Sigil::B);
    state.phase = Phase::Terminal;
    state.score = Score {
        seat_0: 2,
        seat_1: 1,
    };
    state.terminal_outcome = Some(TerminalOutcome::Win {
        seat: HighCardDuelSeat::Seat0,
    });
    state.deck = vec![hidden_tail];
    state.revealed_history = vec![
        high_card_duel::RevealedRound {
            round_number: 1,
            seat_0_card: card(9, Sigil::A),
            seat_1_card: card(4, Sigil::B),
            winner: Some(HighCardDuelSeat::Seat0),
        },
        high_card_duel::RevealedRound {
            round_number: 2,
            seat_0_card: card(3, Sigil::A),
            seat_1_card: card(10, Sigil::B),
            winner: Some(HighCardDuelSeat::Seat1),
        },
        high_card_duel::RevealedRound {
            round_number: 3,
            seat_0_card: card(11, Sigil::A),
            seat_1_card: card(8, Sigil::B),
            winner: Some(HighCardDuelSeat::Seat0),
        },
    ];

    let public_view = project_view(&state, &viewer(None));

    assert!(!public_view
        .stable_summary()
        .contains(&hidden_tail.stable_id()));
    assert_eq!(
        public_view.terminal,
        TerminalView::Win {
            winning_seat: HighCardDuelSeat::Seat0,
            rationale: OutcomeRationaleView {
                result_kind: "win".to_owned(),
                decisive_cause: "final_score_after_round_limit".to_owned(),
                template_key: "high_card_duel.final_score_win".to_owned(),
                decisive_rule_ids: vec![
                    "HCD-ROUND-005".to_owned(),
                    "HCD-END-001".to_owned(),
                    "HCD-END-002".to_owned(),
                ],
                final_score: public_view.score,
                round_breakdowns: vec![
                    RoundOutcomeBreakdownView {
                        round_number: 1,
                        seat_0_rank: 9,
                        seat_1_rank: 4,
                        winner: Some(HighCardDuelSeat::Seat0),
                        point_delta_seat_0: 1,
                        point_delta_seat_1: 0,
                        cumulative_score: Score {
                            seat_0: 1,
                            seat_1: 0,
                        },
                    },
                    RoundOutcomeBreakdownView {
                        round_number: 2,
                        seat_0_rank: 3,
                        seat_1_rank: 10,
                        winner: Some(HighCardDuelSeat::Seat1),
                        point_delta_seat_0: 0,
                        point_delta_seat_1: 1,
                        cumulative_score: Score {
                            seat_0: 1,
                            seat_1: 1,
                        },
                    },
                    RoundOutcomeBreakdownView {
                        round_number: 3,
                        seat_0_rank: 11,
                        seat_1_rank: 8,
                        winner: Some(HighCardDuelSeat::Seat0),
                        point_delta_seat_0: 1,
                        point_delta_seat_1: 0,
                        cumulative_score: Score {
                            seat_0: 2,
                            seat_1: 1,
                        },
                    },
                ],
            },
        }
    );
}

#[test]
fn effect_filtering_returns_correct_sets_for_observer_seat0_seat1() {
    let mut state =
        setup_match(Seed(7), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let lead_card = state.hand_for(HighCardDuelSeat::Seat0)[0];
    let effects = {
        let command = command(0, lead_card, state.freshness_token);
        let action = validate_command(&state, &command).expect("command validates");
        apply_action(&mut state, action)
    };
    let mut log = EffectLog::new();
    for effect in effects {
        log.push(effect);
    }

    let observer = log.since(EffectCursor(0), &viewer(None));
    let seat_0 = log.since(EffectCursor(0), &viewer(Some(0)));
    let seat_1 = log.since(EffectCursor(0), &viewer(Some(1)));

    assert_eq!(observer.len(), 1);
    assert_eq!(observer[0].envelope.payload.kind(), "hcd_commit_face_down");
    assert_eq!(seat_0.len(), 2);
    assert!(seat_0
        .iter()
        .any(|entry| entry.envelope.payload.kind() == "hcd_own_commit_confirmed"));
    assert_eq!(seat_1.len(), 1);
    assert_eq!(seat_1[0].envelope.payload.kind(), "hcd_commit_face_down");
}
