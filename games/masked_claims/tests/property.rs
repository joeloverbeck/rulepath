use engine_core::{ActionPath, CommandEnvelope, RulesVersion, SeatId, Seed, Viewer};
use masked_claims::{
    action_from_decision, actor_for_seat, apply_action, setup_match, validate_command,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, Phase, SetupOptions, TerminalView, STANDARD_MAX_TURNS,
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
fn many_seed_level1_sequences_preserve_core_invariants() {
    for seed in 1..20 {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds");
        let initial_inventory = mask_token_count(&state.stable_internal_summary());

        while !matches!(state.phase, Phase::Terminal) {
            let seat = active_seat(&state).expect("active seat before terminal");
            let decision = MaskedClaimsLevel1Bot::new(Seed(seed + 100))
                .select_decision(&state, seat)
                .expect("bot decision");
            assert!(action_from_decision(&decision).is_some());
            let validated = validate_command(&state, &command(&state, seat, decision.action_path))
                .expect("decision validates");
            let effects = apply_action(&mut state, validated).expect("action applies");

            assert_eq!(
                mask_token_count(&state.stable_internal_summary()),
                initial_inventory
            );
            assert_single_window(&state);
            if format!("{effects:?}").contains("ClaimAccepted") {
                assert!(!format!("{effects:?}").contains("mask_g"));
            }
            assert!(state.turn_index <= STANDARD_MAX_TURNS);
        }

        let view = masked_claims::project_view(&state, &Viewer { seat_id: None });
        assert!(matches!(view.terminal, TerminalView::Complete { .. }));
    }
}

fn assert_single_window(state: &masked_claims::MaskedClaimsState) {
    match state.phase {
        Phase::Claim { .. } => {
            assert!(state.pedestal.is_none());
            assert!(state.active_seat.is_some());
        }
        Phase::Reaction { responder, .. } => {
            assert!(state.pedestal.is_some());
            assert_eq!(state.active_seat, Some(responder));
        }
        Phase::Terminal => {
            assert!(state.pedestal.is_none());
            assert!(state.active_seat.is_none());
        }
    }
}

fn mask_token_count(summary: &str) -> usize {
    let current_state = summary
        .split(";effects=")
        .next()
        .expect("state summary prefix");
    current_state.matches("mask_g").count() + veiled_slot_count(current_state)
}

fn veiled_slot_count(summary: &str) -> usize {
    let Some(rest) = summary.split(";veiled=").nth(1) else {
        return 0;
    };
    let galleries = rest
        .split(";exposed=")
        .next()
        .expect("veiled field terminates before exposed");
    galleries
        .split('|')
        .map(|gallery| {
            if gallery == "none" {
                0
            } else {
                gallery.split(',').count()
            }
        })
        .sum()
}
