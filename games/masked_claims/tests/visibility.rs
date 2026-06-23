use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use game_test_support::no_leak::{assert_pairwise_no_leak, ExposureExpectation, LeakProbe};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, project_view, setup_match, validate_command,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, PrivateView, PublicReplayExport, PublicReplayStep,
    SetupOptions, ACTION_CLAIM, ACTION_RESPOND_ACCEPT, ACTION_RESPOND_CHALLENGE,
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

fn first_claim_path_for(
    state: &masked_claims::MaskedClaimsState,
    seat: MaskedClaimsSeat,
) -> ActionPath {
    let tree = legal_action_tree(state, &actor_for_seat(state, seat));
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

fn first_claim_path(state: &masked_claims::MaskedClaimsState) -> ActionPath {
    first_claim_path_for(state, MaskedClaimsSeat::Seat0)
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

    fn seat(self) -> Option<MaskedClaimsSeat> {
        match self {
            MatrixViewer::Observer => None,
            MatrixViewer::Seat0 => Some(MaskedClaimsSeat::Seat0),
            MatrixViewer::Seat1 => Some(MaskedClaimsSeat::Seat1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MatrixSurface {
    PendingView(MaskedClaimsSeat),
    PendingResponderActionTree(MaskedClaimsSeat),
    PendingEffect(MaskedClaimsSeat),
    PendingPublicExport(MaskedClaimsSeat),
    PendingBot(MaskedClaimsSeat),
    AcceptedSecretView(MaskedClaimsSeat),
    AcceptedSecretPublicExport(MaskedClaimsSeat),
    ChallengeRevealView(MaskedClaimsSeat),
    ChallengeRevealEffect(MaskedClaimsSeat),
    ChallengeRevealPublicExport(MaskedClaimsSeat),
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
        MatrixSurface::PendingView(MaskedClaimsSeat::Seat0),
        MatrixSurface::PendingResponderActionTree(MaskedClaimsSeat::Seat0),
        MatrixSurface::PendingEffect(MaskedClaimsSeat::Seat0),
        MatrixSurface::PendingPublicExport(MaskedClaimsSeat::Seat0),
        MatrixSurface::PendingBot(MaskedClaimsSeat::Seat0),
        MatrixSurface::AcceptedSecretView(MaskedClaimsSeat::Seat0),
        MatrixSurface::AcceptedSecretPublicExport(MaskedClaimsSeat::Seat0),
        MatrixSurface::ChallengeRevealView(MaskedClaimsSeat::Seat0),
        MatrixSurface::ChallengeRevealEffect(MaskedClaimsSeat::Seat0),
        MatrixSurface::ChallengeRevealPublicExport(MaskedClaimsSeat::Seat0),
        MatrixSurface::PendingView(MaskedClaimsSeat::Seat1),
        MatrixSurface::PendingResponderActionTree(MaskedClaimsSeat::Seat1),
        MatrixSurface::PendingEffect(MaskedClaimsSeat::Seat1),
        MatrixSurface::PendingPublicExport(MaskedClaimsSeat::Seat1),
        MatrixSurface::PendingBot(MaskedClaimsSeat::Seat1),
        MatrixSurface::AcceptedSecretView(MaskedClaimsSeat::Seat1),
        MatrixSurface::AcceptedSecretPublicExport(MaskedClaimsSeat::Seat1),
        MatrixSurface::ChallengeRevealView(MaskedClaimsSeat::Seat1),
        MatrixSurface::ChallengeRevealEffect(MaskedClaimsSeat::Seat1),
        MatrixSurface::ChallengeRevealPublicExport(MaskedClaimsSeat::Seat1),
    ]
}

fn advance_to_claimant(state: &mut masked_claims::MaskedClaimsState, claimant: MaskedClaimsSeat) {
    if claimant == MaskedClaimsSeat::Seat0 {
        return;
    }

    let path = first_claim_path_for(state, MaskedClaimsSeat::Seat0);
    let claim = validate_command(state, &command(state, MaskedClaimsSeat::Seat0, path))
        .expect("setup claim validates");
    apply_action(state, claim).expect("setup claim applies");
    let accept = validate_command(
        state,
        &command(
            state,
            MaskedClaimsSeat::Seat1,
            ActionPath {
                segments: vec![ACTION_RESPOND_ACCEPT.to_owned()],
            },
        ),
    )
    .expect("setup accept validates");
    apply_action(state, accept).expect("setup accept applies");
}

fn pending_claim_state(
    claimant: MaskedClaimsSeat,
) -> (masked_claims::MaskedClaimsState, String, Vec<String>) {
    let mut state =
        setup_match(Seed(51), &seats(), &SetupOptions::default()).expect("setup succeeds");
    advance_to_claimant(&mut state, claimant);
    let claim_path = first_claim_path_for(&state, claimant);
    let hidden_tile = claim_path.segments[1].clone();
    let claim =
        validate_command(&state, &command(&state, claimant, claim_path)).expect("claim validates");
    let effects = apply_action(&mut state, claim)
        .expect("claim applies")
        .iter()
        .map(|effect| format!("{:?}", effect.payload))
        .collect();
    (state, hidden_tile, effects)
}

fn resolved_claim_state(
    claimant: MaskedClaimsSeat,
    response: &str,
) -> (masked_claims::MaskedClaimsState, String, Vec<String>) {
    let (mut state, hidden_tile, _) = pending_claim_state(claimant);
    let responder = claimant.other();
    let response = validate_command(
        &state,
        &command(
            &state,
            responder,
            ActionPath {
                segments: vec![response.to_owned()],
            },
        ),
    )
    .expect("response validates");
    let effects = apply_action(&mut state, response)
        .expect("response applies")
        .iter()
        .map(|effect| format!("{:?}", effect.payload))
        .collect();
    (state, hidden_tile, effects)
}

fn matrix_probes() -> Vec<LeakProbe<MaskedClaimsSeat, &'static str, String>> {
    MaskedClaimsSeat::ALL
        .into_iter()
        .map(|source_seat| {
            let (_, hidden_tile, _) = pending_claim_state(source_seat);
            LeakProbe {
                source_seat,
                canary_id: "claimed_tile",
                canary: hidden_tile,
            }
        })
        .collect()
}

fn public_export(
    viewer: &MatrixViewer,
    state: &masked_claims::MaskedClaimsState,
    effects: Vec<String>,
    terminal: bool,
) -> String {
    PublicReplayExport::new(
        match viewer {
            MatrixViewer::Observer => "observer",
            MatrixViewer::Seat0 => "seat_0",
            MatrixViewer::Seat1 => "seat_1",
        },
        vec![PublicReplayStep::from_view(
            0,
            &project_view(state, &viewer.viewer()),
            effects,
            "claim/grade-5",
            terminal,
        )],
    )
    .to_json()
}

fn matrix_snapshot(viewer: &MatrixViewer, surface: &MatrixSurface) -> String {
    match *surface {
        MatrixSurface::PendingView(source) => {
            let (state, _, _) = pending_claim_state(source);
            project_view(&state, &viewer.viewer()).stable_summary()
        }
        MatrixSurface::PendingResponderActionTree(source) => {
            let (state, _, _) = pending_claim_state(source);
            let responder = source.other();
            format!(
                "{:?}",
                legal_action_tree(&state, &actor_for_seat(&state, responder))
            )
        }
        MatrixSurface::PendingEffect(source) => {
            let (_, _, effects) = pending_claim_state(source);
            effects.join("|")
        }
        MatrixSurface::PendingPublicExport(source) => {
            let (state, _, effects) = pending_claim_state(source);
            public_export(&MatrixViewer::Observer, &state, effects, false)
        }
        MatrixSurface::PendingBot(source) => {
            let (state, _, _) = pending_claim_state(source);
            match viewer.seat() {
                Some(seat) => MaskedClaimsLevel1Bot::new(Seed(9))
                    .select_decision(&state, seat)
                    .map_or_else(|_| String::new(), |decision| decision.rationale),
                None => String::new(),
            }
        }
        MatrixSurface::AcceptedSecretView(source) => {
            let (state, _, _) = resolved_claim_state(source, ACTION_RESPOND_ACCEPT);
            project_view(&state, &viewer.viewer()).stable_summary()
        }
        MatrixSurface::AcceptedSecretPublicExport(source) => {
            let (state, _, effects) = resolved_claim_state(source, ACTION_RESPOND_ACCEPT);
            public_export(viewer, &state, effects, false)
        }
        MatrixSurface::ChallengeRevealView(source) => {
            let (state, _, _) = resolved_claim_state(source, ACTION_RESPOND_CHALLENGE);
            project_view(&state, &viewer.viewer()).stable_summary()
        }
        MatrixSurface::ChallengeRevealEffect(source) => {
            let (_, _, effects) = resolved_claim_state(source, ACTION_RESPOND_CHALLENGE);
            effects.join("|")
        }
        MatrixSurface::ChallengeRevealPublicExport(source) => {
            let (state, _, effects) = resolved_claim_state(source, ACTION_RESPOND_CHALLENGE);
            public_export(viewer, &state, effects, false)
        }
    }
}

fn matrix_expectation(
    source: &MaskedClaimsSeat,
    _viewer: &MatrixViewer,
    surface: &MatrixSurface,
    _canary_id: &&'static str,
) -> ExposureExpectation {
    match *surface {
        MatrixSurface::PendingView(surface_source)
        | MatrixSurface::PendingResponderActionTree(surface_source)
        | MatrixSurface::PendingEffect(surface_source)
        | MatrixSurface::PendingPublicExport(surface_source)
        | MatrixSurface::PendingBot(surface_source)
        | MatrixSurface::AcceptedSecretView(surface_source)
        | MatrixSurface::AcceptedSecretPublicExport(surface_source) => {
            if surface_source == *source {
                ExposureExpectation::MustBeAbsent
            } else {
                ExposureExpectation::NotApplicable
            }
        }
        MatrixSurface::ChallengeRevealView(surface_source)
        | MatrixSurface::ChallengeRevealEffect(surface_source)
        | MatrixSurface::ChallengeRevealPublicExport(surface_source) => {
            if surface_source == *source {
                ExposureExpectation::MustBePresent
            } else {
                ExposureExpectation::NotApplicable
            }
        }
    }
}

#[test]
fn pairwise_no_leak_matrix_covers_pending_accepted_and_challenge_surfaces() {
    assert_pairwise_no_leak(
        matrix_viewers(),
        matrix_surfaces(),
        matrix_probes(),
        matrix_snapshot,
        matrix_expectation,
        |snapshot, canary| snapshot.contains(canary),
    )
    .expect("masked claims pairwise no-leak matrix passes");
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
