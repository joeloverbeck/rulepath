use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, project_view, setup_match, validate_command,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, PrivateView, PublicReplayExport, PublicReplayStep,
    SetupOptions, ACTION_CLAIM, ACTION_RESPOND_ACCEPT,
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

fn first_claim_path(state: &masked_claims::MaskedClaimsState) -> ActionPath {
    let tree = legal_action_tree(state, &actor_for_seat(state, MaskedClaimsSeat::Seat0));
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
            "5".to_owned(),
        ],
    }
}

#[test]
fn public_and_opponent_surfaces_hide_unrevealed_tile_ids() {
    let mut state =
        setup_match(Seed(51), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim_path = first_claim_path(&state);
    let hidden_tile = claim_path.segments[1].clone();
    let validated = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, claim_path.clone()),
    )
    .expect("claim validates");
    let effects = apply_action(&mut state, validated).expect("claim applies");

    let public = project_view(&state, &Viewer { seat_id: None }).stable_summary();
    let opponent = project_view(
        &state,
        &Viewer {
            seat_id: Some(SeatId("seat_1".to_owned())),
        },
    )
    .stable_summary();
    let action_tree = format!(
        "{:?}",
        legal_action_tree(&state, &actor_for_seat(&state, MaskedClaimsSeat::Seat1))
    );
    let export = PublicReplayExport::new(
        "observer",
        vec![PublicReplayStep::from_view(
            0,
            &project_view(&state, &Viewer { seat_id: None }),
            effects
                .iter()
                .map(|effect| format!("{:?}", effect.payload))
                .collect(),
            "claim/grade-5",
            false,
        )],
    )
    .to_json();

    for surface in [
        public,
        opponent,
        action_tree,
        format!("{effects:?}"),
        export,
    ] {
        assert!(!surface.contains(&hidden_tile), "{surface}");
    }
}

#[test]
fn accepted_masks_remain_hidden_after_resolution_and_bot_rationale_is_safe() {
    let mut state =
        setup_match(Seed(52), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim_path = first_claim_path(&state);
    let accepted_tile = claim_path.segments[1].clone();
    let claim = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, claim_path),
    )
    .expect("claim validates");
    apply_action(&mut state, claim).expect("claim applies");
    let accept = validate_command(
        &state,
        &command(
            &state,
            MaskedClaimsSeat::Seat1,
            ActionPath {
                segments: vec![ACTION_RESPOND_ACCEPT.to_owned()],
            },
        ),
    )
    .expect("accept validates");
    apply_action(&mut state, accept).expect("accept applies");

    let public = project_view(&state, &Viewer { seat_id: None });
    assert!(!public.stable_summary().contains(&accepted_tile));
    assert!(matches!(public.private_view, PrivateView::Observer));

    let decision = MaskedClaimsLevel1Bot::new(Seed(9))
        .select_decision(&state, MaskedClaimsSeat::Seat1)
        .expect("bot decision");
    assert!(!decision.rationale.contains("mask_g"));
    assert!(!decision.rationale.contains("reserve"));
    assert!(!decision.rationale.contains("opponent hand"));
}
