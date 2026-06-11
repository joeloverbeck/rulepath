use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed};
use masked_claims::{
    actor_for_seat, apply_action, setup_match, validate_command, MaskedClaimsLevel1Bot,
    MaskedClaimsRandomBot, MaskedClaimsSeat, Phase, SetupOptions,
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

fn active_seat(state: &masked_claims::MaskedClaimsState) -> Option<MaskedClaimsSeat> {
    match state.phase {
        Phase::Claim { .. } => state.active_seat,
        Phase::Reaction { responder, .. } => Some(responder),
        Phase::Terminal => None,
    }
}

#[test]
fn level0_and_level1_validate_in_both_phases() {
    let mut state =
        setup_match(Seed(61), &seats(), &SetupOptions::default()).expect("setup succeeds");
    let random = MaskedClaimsRandomBot::new(Seed(1))
        .select_decision(&state, MaskedClaimsSeat::Seat0)
        .expect("random claim");
    validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, random.action_path),
    )
    .expect("random claim validates");

    let level1 = MaskedClaimsLevel1Bot::new(Seed(2))
        .select_decision(&state, MaskedClaimsSeat::Seat0)
        .expect("level1 claim");
    let validated = validate_command(
        &state,
        &command(&state, MaskedClaimsSeat::Seat0, level1.action_path),
    )
    .expect("level1 claim validates");
    apply_action(&mut state, validated).expect("claim applies");

    for decision in [
        MaskedClaimsRandomBot::new(Seed(3))
            .select_decision(&state, MaskedClaimsSeat::Seat1)
            .expect("random response"),
        MaskedClaimsLevel1Bot::new(Seed(4))
            .select_decision(&state, MaskedClaimsSeat::Seat1)
            .expect("level1 response"),
    ] {
        validate_command(
            &state,
            &command(&state, MaskedClaimsSeat::Seat1, decision.action_path),
        )
        .expect("response validates");
    }
}

#[test]
fn level1_is_deterministic_and_finishes_many_games_legally() {
    for seed in 62..72 {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
        let bot = MaskedClaimsLevel1Bot::new(Seed(17));
        let first = bot
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("first decision");
        let second = bot
            .select_decision(&state, MaskedClaimsSeat::Seat0)
            .expect("second decision");
        assert_eq!(first, second);

        while !matches!(state.phase, Phase::Terminal) {
            let seat = active_seat(&state).expect("active seat");
            let decision = bot.select_decision(&state, seat).expect("bot decision");
            assert!(!decision.rationale.contains("mask_g"));
            assert!(!decision.rationale.contains("reserve"));
            let validated = validate_command(&state, &command(&state, seat, decision.action_path))
                .expect("decision validates");
            apply_action(&mut state, validated).expect("decision applies");
        }
    }
}
