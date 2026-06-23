use engine_core::{ActionPath, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, Viewer};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, project_view,
    replay_support::{action_tree_v1_bytes, action_tree_v1_hash},
    setup_match, validate_command, MaskedClaimsLevel1Bot, MaskedClaimsSeat, Phase,
    PublicReplayExport, PublicReplayStep, SetupOptions, ACTION_CLAIM, ACTION_RESPOND_ACCEPT,
    ACTION_RESPOND_CHALLENGE,
};

const GOLDEN_TRACES: [(&str, &str); 17] = [
    (
        "shortest-normal.trace.json",
        include_str!("golden_traces/shortest-normal.trace.json"),
    ),
    (
        "claim-pending-window.trace.json",
        include_str!("golden_traces/claim-pending-window.trace.json"),
    ),
    (
        "accept-resolution.trace.json",
        include_str!("golden_traces/accept-resolution.trace.json"),
    ),
    (
        "challenge-honest-reveal.trace.json",
        include_str!("golden_traces/challenge-honest-reveal.trace.json"),
    ),
    (
        "challenge-exposed-lie.trace.json",
        include_str!("golden_traces/challenge-exposed-lie.trace.json"),
    ),
    (
        "underclaim-trap-reveal.trace.json",
        include_str!("golden_traces/underclaim-trap-reveal.trace.json"),
    ),
    (
        "certain-lie-challenge.trace.json",
        include_str!("golden_traces/certain-lie-challenge.trace.json"),
    ),
    (
        "terminal-tie-break.trace.json",
        include_str!("golden_traces/terminal-tie-break.trace.json"),
    ),
    (
        "draw-after-tie-breaks.trace.json",
        include_str!("golden_traces/draw-after-tie-breaks.trace.json"),
    ),
    (
        "stale-diagnostic.trace.json",
        include_str!("golden_traces/stale-diagnostic.trace.json"),
    ),
    (
        "wrong-phase-claim-diagnostic.trace.json",
        include_str!("golden_traces/wrong-phase-claim-diagnostic.trace.json"),
    ),
    (
        "wrong-seat-response-diagnostic.trace.json",
        include_str!("golden_traces/wrong-seat-response-diagnostic.trace.json"),
    ),
    (
        "unowned-tile-diagnostic.trace.json",
        include_str!("golden_traces/unowned-tile-diagnostic.trace.json"),
    ),
    (
        "public-observer-no-leak.trace.json",
        include_str!("golden_traces/public-observer-no-leak.trace.json"),
    ),
    (
        "accepted-mask-never-revealed.trace.json",
        include_str!("golden_traces/accepted-mask-never-revealed.trace.json"),
    ),
    (
        "bot-claim-and-response.trace.json",
        include_str!("golden_traces/bot-claim-and-response.trace.json"),
    ),
    (
        "public-replay-export-import.trace.json",
        include_str!("golden_traces/public-replay-export-import.trace.json"),
    ),
];

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn command(
    state: &masked_claims::MaskedClaimsState,
    seat: MaskedClaimsSeat,
    action_path: ActionPath,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path,
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayRun {
    state_hashes: Vec<HashValue>,
    effect_hashes: Vec<HashValue>,
    action_tree_hashes: Vec<HashValue>,
    view_hashes: Vec<HashValue>,
    export_json: String,
}

#[test]
fn same_seed_and_bot_stream_reproduce_replay_surfaces() {
    let left = replay_run(31);
    let right = replay_run(31);

    assert_eq!(left, right);
    assert!(left.export_json.contains("claim/grade-"));
    assert!(!left.export_json.contains("claim/mask_g"));
}

#[test]
fn challenge_reveal_appears_after_public_claim_effect() {
    let mut state =
        setup_match(Seed(32), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim = MaskedClaimsLevel1Bot::new(Seed(7))
        .select_decision(&state, MaskedClaimsSeat::Seat0)
        .expect("claim decision");
    let claim_command = command(&state, MaskedClaimsSeat::Seat0, claim.action_path);
    let claim_validated = validate_command(&state, &claim_command).expect("claim validates");
    let claim_effects = apply_action(&mut state, claim_validated).expect("claim applies");
    assert!(!format!("{claim_effects:?}").contains("MaskRevealed"));

    let challenge = ActionPath {
        segments: vec![masked_claims::ACTION_RESPOND_CHALLENGE.to_owned()],
    };
    let challenge_command = command(&state, MaskedClaimsSeat::Seat1, challenge);
    let challenge_validated =
        validate_command(&state, &challenge_command).expect("challenge validates");
    let response_effects =
        apply_action(&mut state, challenge_validated).expect("challenge applies");
    let rendered = format!("{response_effects:?}");
    assert!(rendered.find("ChallengeDeclared") < rendered.find("MaskRevealed"));
}

#[test]
fn golden_traces_are_present_parseable_and_viewer_safe() {
    assert_eq!(GOLDEN_TRACES.len(), 17);
    for (name, trace) in GOLDEN_TRACES {
        assert_trace_field(name, trace, "\"schema_version\":1");
        assert_trace_field(name, trace, "\"game_id\":\"masked_claims\"");
        assert_trace_field(name, trace, "\"rules_version\":\"masked-claims-rules-v1\"");
        assert_trace_field(name, trace, "\"variant\":\"masked_claims_standard\"");
        assert_trace_field(name, trace, "\"expected_state_hashes\"");
        assert_trace_field(name, trace, "\"expected_replay_hashes\"");
        assert!(
            trace.contains("\"note\":\"") && trace.contains("\"migration_update_note\":\""),
            "{name} carries trace maintenance notes"
        );
        assert!(
            !trace.contains("claim/mask_g"),
            "{name} redacts claim command tile ids"
        );
        if trace.contains("\"public_no_leak\":true") {
            assert!(
                !trace.contains("mask_g"),
                "{name} public no-leak trace must not contain tile ids"
            );
        }
        if name == "accepted-mask-never-revealed.trace.json" {
            assert_trace_field(name, trace, "\"accepted_revealed_tiles\":[]");
            assert!(!trace.contains("mask_g"));
        }
        if name == "public-replay-export-import.trace.json" {
            assert_trace_field(name, trace, "\"public_export_contains_tile_ids\":false");
            assert_trace_field(name, trace, "\"claim/grade-4\"");
        }
    }
}

#[test]
fn action_tree_v1_bytes_and_hashes_are_pinned_for_claim_and_response_shapes() {
    let mut state =
        setup_match(Seed(31), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim_tree = legal_action_tree(&state, &actor_for_seat(&state, MaskedClaimsSeat::Seat0));

    assert_eq!(choice_segments(&claim_tree), vec![ACTION_CLAIM]);
    assert_eq!(action_tree_v1_bytes(&claim_tree).len(), 15326);
    assert_eq!(
        action_tree_v1_hash(&claim_tree),
        HashValue(3772732430772540101)
    );

    let claim_path = first_claim_path(&claim_tree, "5");
    let validated = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, claim_path),
    )
    .expect("claim validates");
    apply_action(&mut state, validated).expect("claim applies");

    let response_tree = legal_action_tree(&state, &actor_for_seat(&state, MaskedClaimsSeat::Seat1));

    assert_eq!(
        choice_segments(&response_tree),
        vec![ACTION_RESPOND_ACCEPT, ACTION_RESPOND_CHALLENGE]
    );
    assert_eq!(action_tree_v1_bytes(&response_tree).len(), 1100);
    assert_eq!(
        action_tree_v1_hash(&response_tree),
        HashValue(689297409234037920)
    );
}

fn replay_run(seed: u64) -> ReplayRun {
    let mut state =
        setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let mut state_hashes = Vec::new();
    let mut effect_hashes = Vec::new();
    let mut action_tree_hashes = Vec::new();
    let mut view_hashes = Vec::new();
    let mut steps = Vec::new();

    while !matches!(state.phase, Phase::Terminal) {
        let seat = match state.phase {
            Phase::Claim { .. } => state.active_seat.expect("active claimant"),
            Phase::Reaction { responder, .. } => responder,
            Phase::Terminal => unreachable!(),
        };
        let tree = legal_action_tree(&state, &actor_for_seat(&state, seat));
        action_tree_hashes.push(hash(&format!("{tree:?}")));
        view_hashes.push(hash(
            &project_view(&state, &Viewer { seat_id: None }).stable_summary(),
        ));
        let decision = MaskedClaimsLevel1Bot::new(Seed(seed + 300))
            .select_decision(&state, seat)
            .expect("bot decision");
        let summary = redacted_summary(&decision.action_path);
        let validated = validate_command(&state, &command(&state, seat, decision.action_path))
            .expect("decision validates");
        let effects = apply_action(&mut state, validated).expect("action applies");
        effect_hashes.push(hash(&format!("{effects:?}")));
        state_hashes.push(hash(&state.stable_internal_summary()));
        steps.push(PublicReplayStep::from_view(
            steps.len(),
            &project_view(&state, &Viewer { seat_id: None }),
            effects
                .iter()
                .map(|effect| format!("{:?}", effect.payload))
                .collect(),
            &summary,
            matches!(state.phase, Phase::Terminal),
        ));
    }

    let export_json = PublicReplayExport::new("observer", steps).to_json();
    assert_eq!(
        PublicReplayExport::from_json(&export_json)
            .expect("export imports")
            .to_json(),
        export_json
    );

    ReplayRun {
        state_hashes,
        effect_hashes,
        action_tree_hashes,
        view_hashes,
        export_json,
    }
}

fn hash(value: &str) -> HashValue {
    HashValue::from_stable_bytes(value.as_bytes())
}

fn first_claim_path(tree: &engine_core::ActionTree, declared: &str) -> ActionPath {
    let claim = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == ACTION_CLAIM)
        .expect("claim family");
    let tile = &claim.next.as_ref().expect("tile choices").choices[0];
    ActionPath {
        segments: vec![
            ACTION_CLAIM.to_owned(),
            tile.segment.clone(),
            declared.to_owned(),
        ],
    }
}

fn choice_segments(tree: &engine_core::ActionTree) -> Vec<&str> {
    tree.root
        .choices
        .iter()
        .map(|choice| choice.segment.as_str())
        .collect()
}

fn assert_trace_field(name: &str, trace: &str, needle: &str) {
    assert!(trace.contains(needle), "{name} missing {needle}");
}

fn redacted_summary(action_path: &ActionPath) -> String {
    match action_path.segments.as_slice() {
        [family, _tile, declared] if family == masked_claims::ACTION_CLAIM => {
            format!("claim/grade-{declared}")
        }
        _ => action_path.segments.join("/"),
    }
}
