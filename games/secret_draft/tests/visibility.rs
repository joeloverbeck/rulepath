use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Viewer};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use secret_draft::{
    actions::{commit_segment, legal_action_metadata, legal_action_tree, validate_command},
    apply_action,
    bots::{actor_for_seat, SecretDraftLevel1Bot},
    determine_terminal_outcome_from_summary,
    replay_support::{
        effect_stable_string, export_public_replay, import_public_export, PublicReplayExport,
        ReplayCommand, SecretDraftInternalTrace,
    },
    terminal_tie_break_summary,
    visibility::filter_effects_for_viewer,
    DraftItemId, Phase, SecretDraftSeat, SecretDraftState, SetupOptions, TerminalView, GAME_ID,
    RULES_VERSION_LABEL, VARIANT_ID,
};

fn setup() -> SecretDraftState {
    secret_draft::setup_match(
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn command(state: &SecretDraftState, seat: SecretDraftSeat, item: DraftItemId) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path: ActionPath {
            segments: vec![commit_segment(item)],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn one_commit_state(item: DraftItemId) -> (SecretDraftState, Vec<String>) {
    let mut state = setup();
    let envelope = command(&state, SecretDraftSeat::Seat0, item);
    let validated = validate_command(&state, &envelope).expect("command validates");
    let effects = apply_action(&mut state, validated).expect("commit applies");
    (
        state,
        effects.iter().map(effect_stable_string).collect::<Vec<_>>(),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixViewer {
    Observer,
    Seat0,
    Seat1,
}

impl MatrixViewer {
    fn viewer(self) -> Viewer {
        match self {
            MatrixViewer::Observer => Viewer { seat_id: None },
            MatrixViewer::Seat0 => Viewer {
                seat_id: Some(SeatId("seat_0".to_owned())),
            },
            MatrixViewer::Seat1 => Viewer {
                seat_id: Some(SeatId("seat_1".to_owned())),
            },
        }
    }

    fn seat(self) -> Option<SecretDraftSeat> {
        match self {
            MatrixViewer::Observer => None,
            MatrixViewer::Seat0 => Some(SecretDraftSeat::Seat0),
            MatrixViewer::Seat1 => Some(SecretDraftSeat::Seat1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixSurface {
    PreCommitView(SecretDraftSeat),
    PreCommitActionTree(SecretDraftSeat),
    PreCommitDiagnostic(SecretDraftSeat),
    PreCommitEffect(SecretDraftSeat),
    PreCommitPublicExport(SecretDraftSeat),
    PreCommitSeatPrivateExport(SecretDraftSeat),
    PreCommitBot(SecretDraftSeat),
    PostRevealView,
    PostRevealEffect,
    PostRevealPublicExport,
    PostRevealSeatPrivateExport,
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
        MatrixSurface::PreCommitView(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitActionTree(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitDiagnostic(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitEffect(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitPublicExport(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitSeatPrivateExport(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitBot(SecretDraftSeat::Seat0),
        MatrixSurface::PreCommitView(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitActionTree(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitDiagnostic(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitEffect(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitPublicExport(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitSeatPrivateExport(SecretDraftSeat::Seat1),
        MatrixSurface::PreCommitBot(SecretDraftSeat::Seat1),
        MatrixSurface::PostRevealView,
        MatrixSurface::PostRevealEffect,
        MatrixSurface::PostRevealPublicExport,
        MatrixSurface::PostRevealSeatPrivateExport,
    ]
}

fn hidden_item(source: SecretDraftSeat) -> DraftItemId {
    match source {
        SecretDraftSeat::Seat0 => DraftItemId::Ember4,
        SecretDraftSeat::Seat1 => DraftItemId::Grove4,
    }
}

fn matrix_probes() -> Vec<LeakProbe<SecretDraftSeat, &'static str, DraftItemId>> {
    SecretDraftSeat::ALL
        .into_iter()
        .map(|source_seat| LeakProbe {
            source_seat,
            canary_id: "committed_item",
            canary: hidden_item(source_seat),
        })
        .collect()
}

fn apply_commit(
    state: &mut SecretDraftState,
    seat: SecretDraftSeat,
    item: DraftItemId,
) -> Vec<String> {
    let envelope = command(state, seat, item);
    let validated = validate_command(state, &envelope).expect("commit validates");
    apply_action(state, validated)
        .expect("commit applies")
        .iter()
        .map(effect_stable_string)
        .collect()
}

fn pre_commit_state(source: SecretDraftSeat) -> (SecretDraftState, Vec<String>) {
    let mut state = setup();
    let effects = apply_commit(&mut state, source, hidden_item(source));
    (state, effects)
}

fn post_reveal_state() -> (SecretDraftState, Vec<String>) {
    let mut state = setup();
    apply_commit(
        &mut state,
        SecretDraftSeat::Seat0,
        hidden_item(SecretDraftSeat::Seat0),
    );
    let reveal_effects = apply_commit(
        &mut state,
        SecretDraftSeat::Seat1,
        hidden_item(SecretDraftSeat::Seat1),
    );
    (state, reveal_effects)
}

fn trace_for_commits(commits: &[(SecretDraftSeat, DraftItemId)]) -> SecretDraftInternalTrace {
    SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: 44,
        commands: commits
            .iter()
            .map(|(seat, item)| ReplayCommand {
                actor: seat.as_str().to_owned(),
                path: vec![commit_segment(*item)],
            })
            .collect(),
    }
}

fn matrix_snapshot(viewer: &MatrixViewer, surface: &MatrixSurface) -> String {
    match *surface {
        MatrixSurface::PreCommitView(source) => {
            let (state, _) = pre_commit_state(source);
            let view = secret_draft::project_view(&state, &viewer.viewer());
            format!("{:?}|{:?}", view.commitments, view.private_view)
        }
        MatrixSurface::PreCommitActionTree(source) => {
            let (state, _) = pre_commit_state(source);
            match viewer.seat() {
                Some(seat) => format!(
                    "{:?}",
                    legal_action_metadata(&state, &actor_for_seat(&state, seat))
                ),
                None => format!(
                    "{:?}",
                    legal_action_metadata(
                        &state,
                        &engine_core::Actor {
                            seat_id: SeatId("observer".to_owned()),
                        }
                    )
                ),
            }
        }
        MatrixSurface::PreCommitDiagnostic(source) => {
            let (state, _) = pre_commit_state(source);
            format!(
                "{:?}",
                validate_command(&state, &command(&state, source, hidden_item(source)))
            )
        }
        MatrixSurface::PreCommitEffect(source) => {
            let (_, effects) = pre_commit_state(source);
            effects.join("|")
        }
        MatrixSurface::PreCommitPublicExport(source) => export_public_replay(
            &trace_for_commits(&[(source, hidden_item(source))]),
            &Viewer { seat_id: None },
        )
        .to_json(),
        MatrixSurface::PreCommitSeatPrivateExport(source) => {
            let viewer = viewer.viewer();
            export_public_replay(
                &trace_for_commits(&[(source, hidden_item(source))]),
                &viewer,
            )
            .to_json()
        }
        MatrixSurface::PreCommitBot(source) => {
            let (state, _) = pre_commit_state(source);
            match viewer.seat() {
                Some(seat) => SecretDraftLevel1Bot::new(engine_core::Seed(7))
                    .select_decision(&state, seat)
                    .map_or_else(|_| String::new(), |decision| decision.rationale),
                None => String::new(),
            }
        }
        MatrixSurface::PostRevealView => {
            let (state, _) = post_reveal_state();
            format!("{:?}", secret_draft::project_view(&state, &viewer.viewer()))
        }
        MatrixSurface::PostRevealEffect => {
            let (_, effects) = post_reveal_state();
            effects.join("|")
        }
        MatrixSurface::PostRevealPublicExport => export_public_replay(
            &trace_for_commits(&[
                (SecretDraftSeat::Seat0, hidden_item(SecretDraftSeat::Seat0)),
                (SecretDraftSeat::Seat1, hidden_item(SecretDraftSeat::Seat1)),
            ]),
            &Viewer { seat_id: None },
        )
        .to_json(),
        MatrixSurface::PostRevealSeatPrivateExport => {
            let viewer = viewer.viewer();
            export_public_replay(
                &trace_for_commits(&[
                    (SecretDraftSeat::Seat0, hidden_item(SecretDraftSeat::Seat0)),
                    (SecretDraftSeat::Seat1, hidden_item(SecretDraftSeat::Seat1)),
                ]),
                &viewer,
            )
            .to_json()
        }
    }
}

fn matrix_expectation(
    source: &SecretDraftSeat,
    _viewer: &MatrixViewer,
    surface: &MatrixSurface,
    _canary_id: &&'static str,
) -> ExposureExpectation {
    match *surface {
        MatrixSurface::PreCommitView(pre_source)
        | MatrixSurface::PreCommitActionTree(pre_source)
        | MatrixSurface::PreCommitDiagnostic(pre_source)
        | MatrixSurface::PreCommitEffect(pre_source)
        | MatrixSurface::PreCommitPublicExport(pre_source)
        | MatrixSurface::PreCommitSeatPrivateExport(pre_source)
        | MatrixSurface::PreCommitBot(pre_source) => {
            if pre_source == *source {
                ExposureExpectation::MustBeAbsent
            } else {
                ExposureExpectation::NotApplicable
            }
        }
        MatrixSurface::PostRevealView
        | MatrixSurface::PostRevealEffect
        | MatrixSurface::PostRevealPublicExport
        | MatrixSurface::PostRevealSeatPrivateExport => ExposureExpectation::MustBePresent,
    }
}

// `&String` is required: this is passed as the `ContainsFn` callback to
// `assert_pairwise_no_leak`, whose `FnMut(&Snapshot, _)` bound pins `Snapshot`
// to `String` (the `matrix_snapshot` return type). `&str` fails to satisfy it.
#[allow(clippy::ptr_arg)]
fn snapshot_contains_item(snapshot: &String, item: &DraftItemId) -> bool {
    snapshot.contains(item.as_str())
        || snapshot.contains(item.label())
        || snapshot.contains(&format!("{item:?}"))
}

#[test]
fn pairwise_no_leak_matrix_covers_pre_commit_and_post_reveal_surfaces() {
    assert_pairwise_no_leak(
        matrix_viewers(),
        matrix_surfaces(),
        matrix_probes(),
        matrix_snapshot,
        matrix_expectation,
        snapshot_contains_item,
    )
    .expect("secret draft pairwise no-leak matrix passes");
}

#[test]
fn pending_views_commitment_fields_and_metadata_do_not_reveal_committed_item() {
    let hidden = DraftItemId::Ember4;
    let (state, _) = one_commit_state(hidden);
    let viewers = [
        Viewer { seat_id: None },
        Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
        Viewer {
            seat_id: Some(SeatId("seat_1".to_owned())),
        },
    ];

    for viewer in viewers {
        let view = secret_draft::project_view(&state, &viewer);
        assert!(view.commitments.seat_0.committed);
        assert!(!view.commitments.seat_1.committed);
        assert_no_hidden(
            "commitment view",
            &format!("{:?}", view.commitments),
            hidden,
        );
        assert_no_hidden("private view", &format!("{:?}", view.private_view), hidden);
    }

    let committed_actor_tree =
        legal_action_tree(&state, &actor_for_seat(&state, SecretDraftSeat::Seat0));
    assert!(committed_actor_tree.root.choices.is_empty());
    assert_no_hidden(
        "committed action metadata",
        &format!(
            "{:?}",
            legal_action_metadata(&state, &actor_for_seat(&state, SecretDraftSeat::Seat0))
        ),
        hidden,
    );
}

#[test]
fn pending_effects_diagnostics_and_public_export_redact_committed_item() {
    let hidden = DraftItemId::Ember4;
    let (state, effects) = one_commit_state(hidden);

    for effect in effects {
        assert_no_hidden("pending effect", &effect, hidden);
    }

    let already_committed = validate_command(
        &state,
        &command(&state, SecretDraftSeat::Seat0, DraftItemId::Tide4),
    )
    .expect_err("already committed rejected");
    assert_no_hidden("diagnostic", &format!("{already_committed:?}"), hidden);

    let trace = SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: 44,
        commands: vec![ReplayCommand {
            actor: SecretDraftSeat::Seat0.as_str().to_owned(),
            path: vec![commit_segment(hidden)],
        }],
    };
    let export = export_public_replay(&trace, &Viewer { seat_id: None });
    let imported = import_public_export(&export);
    let export_json = export.to_json();

    assert!(export_json.contains("commit_redacted"));
    assert_no_hidden("public export json", &export_json, hidden);
    assert!(!export_json.contains("seed"));
    assert!(!export_json.contains("44"));
    assert_no_hidden("public import timeline", &format!("{imported:?}"), hidden);
    assert_no_hidden(
        "public export reparsed",
        &format!(
            "{:?}",
            PublicReplayExport::from_json(&export_json).expect("export parses")
        ),
        hidden,
    );

    for seat in ["seat_0", "seat_1"] {
        let seat_export = export_public_replay(
            &trace,
            &Viewer {
                seat_id: Some(SeatId(seat.to_owned())),
            },
        );
        assert_eq!(seat_export.viewer, seat);
        let seat_json = seat_export.to_json();
        assert!(seat_json.contains("commit_redacted"));
        assert_no_hidden("seat-private public export json", &seat_json, hidden);
        assert!(!seat_json.contains("seed"));
        assert!(!seat_json.contains("44"));
    }
}

#[test]
fn viewer_filtered_effects_and_bot_explanations_do_not_reveal_opponent_commitment() {
    let hidden = DraftItemId::Grove4;
    let mut state = setup();
    let envelope = command(&state, SecretDraftSeat::Seat0, hidden);
    let validated = validate_command(&state, &envelope).expect("command validates");
    let effects = apply_action(&mut state, validated).expect("commit applies");
    let seat_1_viewer = Viewer {
        seat_id: Some(SeatId("seat_1".to_owned())),
    };

    let filtered = filter_effects_for_viewer(&effects, &seat_1_viewer);
    assert_no_hidden("filtered effects", &format!("{filtered:?}"), hidden);

    let decision = SecretDraftLevel1Bot::new(engine_core::Seed(7))
        .select_decision(&state, SecretDraftSeat::Seat1)
        .expect("bot decision selected");
    assert_no_hidden("bot rationale", &decision.rationale, hidden);
    assert!(!decision.rationale.to_lowercase().contains("opponent chose"));
    assert!(!decision.rationale.to_lowercase().contains("hidden"));
}

#[test]
fn terminal_rationale_identifies_each_public_tie_break_rung() {
    let cases = [
        (
            terminal_state(
                [vec![DraftItemId::Ember4], vec![DraftItemId::Ember3]],
                [0, 0],
            ),
            SecretDraftSeat::Seat0,
            "score",
            "secret_draft.score_win",
        ),
        (
            terminal_state(
                [
                    vec![DraftItemId::Ember1, DraftItemId::Tide1, DraftItemId::Grove1],
                    vec![DraftItemId::Ember2, DraftItemId::Ember4],
                ],
                [0, 0],
            ),
            SecretDraftSeat::Seat0,
            "complete_sets",
            "secret_draft.complete_sets_tiebreak",
        ),
        (
            terminal_state(
                [
                    vec![DraftItemId::Ember2],
                    vec![DraftItemId::Ember1, DraftItemId::Tide1],
                ],
                [0, 0],
            ),
            SecretDraftSeat::Seat0,
            "highest_single_value",
            "secret_draft.highest_single_tiebreak",
        ),
        (
            terminal_state(
                [
                    vec![DraftItemId::Ember1, DraftItemId::Tide2],
                    vec![DraftItemId::Grove1, DraftItemId::Grove2],
                ],
                [0, 0],
            ),
            SecretDraftSeat::Seat0,
            "distinct_threads",
            "secret_draft.distinct_threads_tiebreak",
        ),
        (
            terminal_state(
                [vec![DraftItemId::Ember1], vec![DraftItemId::Tide1]],
                [1, 0],
            ),
            SecretDraftSeat::Seat1,
            "fewer_priority_conflict_wins",
            "secret_draft.fewer_priority_conflict_wins_tiebreak",
        ),
    ];

    for (state, winner, decisive_cause, template_key) in cases {
        let view = secret_draft::project_view(&state, &Viewer { seat_id: None });
        let TerminalView::Win {
            winning_seat,
            rationale,
        } = view.terminal
        else {
            panic!("expected terminal win view");
        };

        assert_eq!(winning_seat, winner);
        assert_eq!(rationale.result_kind, "win");
        assert_eq!(rationale.decisive_cause, decisive_cause);
        assert_eq!(rationale.template_key, template_key);
        assert_eq!(rationale.terminal_trigger, "sixth_reveal_complete");
        assert_eq!(rationale.terminal_trigger_rule_id, "SD-END-001");
        assert_eq!(
            rationale.ladder.iter().filter(|rung| rung.decisive).count(),
            1
        );
        let decisive = rationale
            .ladder
            .iter()
            .find(|rung| rung.decisive)
            .expect("one decisive rung");
        assert_eq!(decisive.rung, decisive_cause);
        assert_eq!(decisive.winner, Some(winner));
        assert!(rationale
            .decisive_rule_ids
            .contains(&"SD-END-001".to_owned()));
        assert!(rationale
            .decisive_rule_ids
            .contains(&"SD-END-002".to_owned()));
    }
}

#[test]
fn terminal_rationale_identifies_all_tied_draw() {
    let state = terminal_state(
        [
            vec![DraftItemId::Ember2, DraftItemId::Tide2],
            vec![DraftItemId::Ember2, DraftItemId::Tide2],
        ],
        [0, 0],
    );
    let view = secret_draft::project_view(&state, &Viewer { seat_id: None });
    let TerminalView::Draw { rationale } = view.terminal else {
        panic!("expected terminal draw view");
    };

    assert_eq!(rationale.result_kind, "draw");
    assert_eq!(rationale.decisive_cause, "all_tied_draw");
    assert_eq!(rationale.template_key, "secret_draft.all_tied_draw");
    assert_eq!(
        rationale.ladder.iter().filter(|rung| rung.decisive).count(),
        1
    );
    let decisive = rationale
        .ladder
        .iter()
        .find(|rung| rung.decisive)
        .expect("draw rung decisive");
    assert_eq!(decisive.rung, "all_tied_draw");
    assert_eq!(decisive.winner, None);
    assert_eq!(decisive.seat_0_value, None);
    assert_eq!(decisive.seat_1_value, None);
}

#[test]
fn pending_terminal_surface_does_not_create_hidden_rationale() {
    let hidden = DraftItemId::Ember4;
    let (state, _) = one_commit_state(hidden);
    let view = secret_draft::project_view(&state, &Viewer { seat_id: None });

    assert_eq!(view.terminal, TerminalView::NonTerminal);
    assert_no_hidden("terminal view", &format!("{:?}", view.terminal), hidden);
}

#[test]
fn raw_internal_trace_is_the_only_checked_surface_that_keeps_private_command_authority() {
    let hidden = DraftItemId::Ember4;
    let trace = SecretDraftInternalTrace {
        schema_version: 1,
        game_id: GAME_ID.to_owned(),
        rules_version: RULES_VERSION_LABEL.to_owned(),
        variant: VARIANT_ID.to_owned(),
        seed_evidence: 9,
        commands: vec![ReplayCommand {
            actor: SecretDraftSeat::Seat0.as_str().to_owned(),
            path: vec![commit_segment(hidden)],
        }],
    };

    assert!(trace.to_json().contains(hidden.as_str()));
}

fn terminal_state(
    drafted: [Vec<DraftItemId>; 2],
    priority_conflict_wins: [u8; 2],
) -> SecretDraftState {
    let mut state = setup();
    state.phase = Phase::Terminal;
    state.round_number = 6;
    state.visible_pool.clear();
    state.drafted = drafted;
    state.priority_conflict_wins = priority_conflict_wins;
    let summary = terminal_tie_break_summary(&state);
    state.scores = summary.scores;
    state.terminal_outcome = Some(determine_terminal_outcome_from_summary(summary));
    state
}

fn assert_no_hidden(surface: &str, value: &str, hidden: DraftItemId) {
    assert!(
        !value.contains(hidden.as_str()),
        "{surface} leaked {} in {value}",
        hidden.as_str()
    );
    assert!(
        !value.contains(hidden.label()),
        "{surface} leaked {} in {value}",
        hidden.label()
    );
    assert!(
        !value.contains(&format!("{hidden:?}")),
        "{surface} leaked {hidden:?} in {value}"
    );
}
