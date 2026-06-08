use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Viewer};
use secret_draft::{
    actions::{commit_segment, legal_action_metadata, legal_action_tree, validate_command},
    apply_action,
    bots::{actor_for_seat, SecretDraftLevel1Bot},
    replay_support::{
        effect_stable_string, export_public_replay, import_public_export, PublicReplayExport,
        ReplayCommand, SecretDraftInternalTrace,
    },
    visibility::filter_effects_for_viewer,
    DraftItemId, SecretDraftSeat, SecretDraftState, SetupOptions, GAME_ID, RULES_VERSION_LABEL,
    VARIANT_ID,
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
    assert_no_hidden("public import timeline", &format!("{imported:?}"), hidden);
    assert_no_hidden(
        "public export reparsed",
        &format!(
            "{:?}",
            PublicReplayExport::from_json(&export_json).expect("export parses")
        ),
        hidden,
    );
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
