use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, setup_match, validate_command,
    MaskedClaimsAction, MaskedClaimsSeat, Phase, ResponseChoice, SetupOptions, ValidatedAction,
    ACTION_CLAIM, ACTION_RESPOND_ACCEPT, ACTION_RESPOND_CHALLENGE,
};

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn command(
    state: &masked_claims::MaskedClaimsState,
    seat: MaskedClaimsSeat,
    segments: Vec<String>,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, seat),
        action_path: ActionPath { segments },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn first_claim_path(state: &masked_claims::MaskedClaimsState, declared: &str) -> ActionPath {
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
            declared.to_owned(),
        ],
    }
}

fn apply_first_claim(state: &mut masked_claims::MaskedClaimsState, declared: &str) -> String {
    let path = first_claim_path(state, declared);
    let hidden_tile = path.segments[1].clone();
    let ValidatedAction::Claim(claim) = validate_command(
        state,
        &command(state, MaskedClaimsSeat::Seat0, path.segments),
    )
    .expect("claim validates") else {
        panic!("expected claim");
    };
    apply_action(state, ValidatedAction::Claim(claim)).expect("claim applies");
    hidden_tile
}

#[test]
fn claim_and_reaction_legality_use_same_validation_path() {
    let mut state =
        setup_match(Seed(21), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim_path = first_claim_path(&state, "5");
    let validated = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, claim_path.segments),
    )
    .expect("claim validates");
    assert!(matches!(
        validated,
        ValidatedAction::Claim(masked_claims::ValidatedClaim { .. })
    ));
    apply_action(&mut state, validated).expect("claim applies");

    assert!(matches!(state.phase, Phase::Reaction { .. }));
    let claimant_tree = legal_action_tree(&state, &actor_for_seat(&state, MaskedClaimsSeat::Seat0));
    assert!(claimant_tree.root.choices.is_empty());

    for segment in [ACTION_RESPOND_ACCEPT, ACTION_RESPOND_CHALLENGE] {
        let validated = validate_command(
            &state,
            &command(&state, MaskedClaimsSeat::Seat1, vec![segment.to_owned()]),
        )
        .expect("response validates");
        assert!(matches!(
            validated,
            ValidatedAction::Response(masked_claims::ValidatedResponse { .. })
        ));
    }
}

#[test]
fn diagnostics_are_fail_closed_and_public_safe() {
    let mut state =
        setup_match(Seed(22), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let claim_path = first_claim_path(&state, "5");

    let wrong_seat = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat1, claim_path.segments.clone()),
    )
    .expect_err("wrong claimant rejected");
    assert_eq!(wrong_seat.code, "wrong_claimant");
    assert!(!wrong_seat.message.contains("mask_g"));

    let bad_grade = validate_command(
        &state,
        &command(
            &state,
            MaskedClaimsSeat::Seat0,
            vec![
                ACTION_CLAIM.to_owned(),
                claim_path.segments[1].clone(),
                "9".to_owned(),
            ],
        ),
    )
    .expect_err("bad grade rejected");
    assert_eq!(bad_grade.code, "invalid_grade");
    assert!(!bad_grade.message.contains("mask_g"));

    apply_first_claim(&mut state, "5");
    let wrong_phase = validate_command(
        &state,
        &command(
            &state,
            MaskedClaimsSeat::Seat0,
            vec![ACTION_RESPOND_ACCEPT.to_owned()],
        ),
    )
    .expect_err("wrong responder rejected");
    assert_eq!(wrong_phase.code, "wrong_responder");
    assert!(!wrong_phase.message.contains("mask_g"));
}

#[test]
fn accept_and_challenge_resolve_with_expected_action_shapes() {
    let mut accept_state =
        setup_match(Seed(23), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let accepted_tile = apply_first_claim(&mut accept_state, "4");
    let accept = validate_command(
        &accept_state,
        &command(
            &accept_state,
            MaskedClaimsSeat::Seat1,
            vec![ACTION_RESPOND_ACCEPT.to_owned()],
        ),
    )
    .expect("accept validates");
    let effects = apply_action(&mut accept_state, accept).expect("accept applies");
    assert!(matches!(accept_state.phase, Phase::Claim { .. }));
    assert!(effects.iter().any(|effect| {
        matches!(
            effect.payload,
            masked_claims::MaskedClaimsEffect::ClaimAccepted { .. }
        )
    }));
    assert!(!format!("{effects:?}").contains(&accepted_tile));

    let mut challenge_state =
        setup_match(Seed(24), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let challenged_tile = apply_first_claim(&mut challenge_state, "5");
    let challenge = validate_command(
        &challenge_state,
        &command(
            &challenge_state,
            MaskedClaimsSeat::Seat1,
            vec![ACTION_RESPOND_CHALLENGE.to_owned()],
        ),
    )
    .expect("challenge validates");
    let effects = apply_action(&mut challenge_state, challenge).expect("challenge applies");
    assert!(effects.iter().any(|effect| {
        matches!(
            &effect.payload,
            masked_claims::MaskedClaimsEffect::MaskRevealed { tile_id, .. } if tile_id == &challenged_tile
        )
    }));
}

#[test]
fn decision_parser_round_trips_claim_and_response_paths() {
    let claim = masked_claims::parse_action_path(&[
        ACTION_CLAIM.to_owned(),
        "mask_g1_a".to_owned(),
        "5".to_owned(),
    ]);
    assert!(matches!(
        claim,
        Some(MaskedClaimsAction::Claim {
            declared: masked_claims::Grade::Master,
            ..
        })
    ));
    assert_eq!(
        masked_claims::parse_action_path(&[ACTION_RESPOND_ACCEPT.to_owned()]),
        Some(MaskedClaimsAction::Response(ResponseChoice::Accept))
    );
}
