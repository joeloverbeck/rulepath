use high_card_duel::{
    export_public_observer_replay, generate_internal_full_trace, import_public_export,
    replay_internal_full_trace,
};

#[test]
fn replaying_internal_full_trace_reproduces_revealed_sequence() {
    let trace = generate_internal_full_trace(9);

    let left = replay_internal_full_trace(&trace);
    let right = replay_internal_full_trace(&trace);

    assert_eq!(left.revealed_sequence, right.revealed_sequence);
    assert_eq!(left.state_hash, right.state_hash);
    assert_eq!(left.effect_hash, right.effect_hash);
    assert_eq!(left.terminal_outcome, right.terminal_outcome);
    assert!(trace
        .command_paths
        .iter()
        .flatten()
        .any(|segment| segment.contains("hcd:r")));
}

#[test]
fn public_replay_export_has_no_unrevealed_internal_card_identities() {
    let trace = generate_internal_full_trace(12);
    let replay = replay_internal_full_trace(&trace);
    let export = export_public_observer_replay(&trace);
    let export_json = export.to_json();

    assert!(!export_json.contains("\"seed\""));
    assert!(!export_json.contains("commit/hcd:r"));
    for hidden_card in replay.final_state.deck.iter() {
        assert!(!export_json.contains(&hidden_card.stable_id()));
    }
    for hand in &replay.final_state.hands {
        for hidden_card in hand {
            assert!(!export_json.contains(&hidden_card.stable_id()));
        }
    }
    assert!(export_json.contains("commit_redacted"));
}

#[test]
fn import_public_export_produces_public_timeline_without_hidden_reconstruction() {
    let trace = generate_internal_full_trace(15);
    let export = export_public_observer_replay(&trace);
    let timeline = import_public_export(&export);

    assert_eq!(timeline.viewer, "observer");
    assert_eq!(timeline.steps, export.steps);
    assert_eq!(timeline.steps.len(), trace.command_paths.len() + 1);
    assert!(timeline
        .steps
        .iter()
        .all(|step| !step.redacted_command_summary.contains("hcd:r")));
    assert!(timeline.steps.iter().any(|step| step.terminal));
}
