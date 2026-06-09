use directional_flip::{
    apply_action, project_view, setup_match, validate_command, DirectionalFlipSeat,
    OutcomeRationaleView, ScoreView, SetupOptions, TerminalReason, TerminalView,
};
use engine_core::{
    ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, StableSerialize, Viewer,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn command(state: &directional_flip::DirectionalFlipState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: state.seats[state.active_seat.index()].clone(),
        },
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn df_view_001_public_view_is_viewer_safe_and_stable() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    let action = validate_command(&state, &command(&state, "place/r3c4")).unwrap();
    apply_action(&mut state, action);

    let view = project_view(
        &state,
        &Viewer {
            seat_id: Some(state.seats[DirectionalFlipSeat::Seat0.index()].clone()),
        },
    );
    let summary = view.stable_summary();

    assert_eq!(
        view.private_view.status,
        "not_applicable_perfect_information"
    );
    assert!(view.private_view.hidden_fields.is_empty());
    assert!(view
        .legal_targets
        .iter()
        .all(|target| !target.explanation.is_empty()));
    assert!(!summary.contains("DirectionalFlipState"));
    assert!(!summary.contains("consecutive_forced_passes"));
    assert!(!summary.contains("debug"));
    assert_eq!(view.stable_hash(), view.stable_hash());
}

#[test]
fn df_term_001_terminal_public_view_has_no_legal_targets() {
    let mut state = setup_match(Seed(1), &seats(), &SetupOptions::default()).unwrap();
    state.terminal_outcome = Some(directional_flip::TerminalOutcome::Draw);
    state.terminal_reason = Some(TerminalReason::DoubleForcedPass);
    let view = project_view(&state, &Viewer { seat_id: None });

    assert!(view.legal_targets.is_empty());
    assert_eq!(
        view.terminal,
        TerminalView::Draw {
            final_score: view.score.clone(),
            rationale: OutcomeRationaleView {
                result_kind: "draw".to_owned(),
                decisive_cause: "final_score_comparison".to_owned(),
                template_key: "directional_flip.final_score_draw".to_owned(),
                decisive_rule_ids: vec!["DF-SCORE-002".to_owned(), "DF-END-003".to_owned()],
                terminal_trigger: "double_forced_pass".to_owned(),
                final_score: ScoreView {
                    seat_0: view.score.seat_0,
                    seat_1: view.score.seat_1,
                },
            },
        }
    );
    assert_eq!(view.active_seat, None);
}
