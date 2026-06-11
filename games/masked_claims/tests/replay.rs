use engine_core::{ActionPath, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, Viewer};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, project_view, setup_match, validate_command,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, Phase, PublicReplayExport, PublicReplayStep,
    SetupOptions,
};

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

fn redacted_summary(action_path: &ActionPath) -> String {
    match action_path.segments.as_slice() {
        [family, _tile, declared] if family == masked_claims::ACTION_CLAIM => {
            format!("claim/grade-{declared}")
        }
        _ => action_path.segments.join("/"),
    }
}
