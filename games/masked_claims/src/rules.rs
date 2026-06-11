use engine_core::{Diagnostic, EffectEnvelope};

use crate::{
    actions::{self, ResponseChoice, ValidatedAction, ValidatedClaim, ValidatedResponse},
    effects::{
        challenge_declared_effect, challenge_resolved_effect, claim_accepted_effect,
        claim_placed_effect, mask_revealed_effect, reaction_window_opened_effect,
        score_changed_effect, terminal_effect, turn_advanced_effect, ChallengeOutcome,
        MaskedClaimsEffect,
    },
    ids::MaskedClaimsSeat,
    state::{ExposedMask, MaskedClaimsState, PendingClaim, Phase, TerminalOutcome, VeiledClaim},
};

pub fn apply_action(
    state: &mut MaskedClaimsState,
    action: ValidatedAction,
) -> Result<Vec<EffectEnvelope<MaskedClaimsEffect>>, Diagnostic> {
    match action {
        ValidatedAction::Claim(claim) => apply_claim(state, claim),
        ValidatedAction::Response(response) => apply_response(state, response),
    }
}

fn apply_claim(
    state: &mut MaskedClaimsState,
    claim: ValidatedClaim,
) -> Result<Vec<EffectEnvelope<MaskedClaimsEffect>>, Diagnostic> {
    ensure_claim_still_legal(state, claim)?;

    let actor_index = claim.actor.index();
    let tile_index = state.hands[actor_index]
        .iter()
        .position(|tile| *tile == claim.tile)
        .expect("validated claim tile must be in actor hand");
    let tile = state.hands[actor_index].remove(tile_index);
    let responder = claim.actor.other();

    state.pedestal = Some(PendingClaim {
        claimant: claim.actor,
        tile,
        declared: claim.declared,
    });
    state.phase = Phase::Reaction {
        turn_index: claim.turn_index,
        responder,
    };
    state.active_seat = Some(responder);
    state.freshness_token = state.freshness_token.next();

    let effects = vec![
        claim_placed_effect(claim.turn_index, claim.actor, claim.declared),
        reaction_window_opened_effect(claim.turn_index, responder, claim.declared),
    ];
    state
        .effect_history
        .extend(effects.iter().map(|effect| format!("{:?}", effect.payload)));

    Ok(effects)
}

fn apply_response(
    state: &mut MaskedClaimsState,
    response: ValidatedResponse,
) -> Result<Vec<EffectEnvelope<MaskedClaimsEffect>>, Diagnostic> {
    ensure_response_still_legal(state, response)?;
    let claim = state
        .pedestal
        .take()
        .expect("validated response requires a pending claim");

    let mut effects = Vec::new();
    match response.response {
        ResponseChoice::Accept => {
            let delta = claim.declared.value();
            let claimant_index = claim.claimant.index();
            state.scores[claimant_index] = state.scores[claimant_index].saturating_add(delta);
            state.veiled_gallery[claimant_index].push(VeiledClaim {
                declared: claim.declared,
            });
            effects.push(claim_accepted_effect(
                response.turn_index,
                claim.claimant,
                claim.declared,
            ));
            effects.push(score_changed_effect(
                claim.claimant,
                delta,
                state.scores[claimant_index],
                "accepted_claim",
            ));
        }
        ResponseChoice::Challenge => {
            effects.push(challenge_declared_effect(
                response.turn_index,
                response.actor,
            ));
            effects.push(mask_revealed_effect(
                response.turn_index,
                claim.tile,
                claim.tile.grade(),
            ));
            state.counters[response.actor.index()].challenges_declared += 1;
            let actual = claim.tile.grade();
            if actual.value() >= claim.declared.value() {
                let delta = actual.value().saturating_add(2);
                let claimant_index = claim.claimant.index();
                state.scores[claimant_index] = state.scores[claimant_index].saturating_add(delta);
                state.exposed_row[claimant_index].push(ExposedMask {
                    tile: claim.tile,
                    declared: claim.declared,
                    claimant: claim.claimant,
                    challenger: response.actor,
                });
                effects.push(challenge_resolved_effect(
                    response.turn_index,
                    ChallengeOutcome::Honest,
                    claim.claimant,
                    response.actor,
                    delta,
                    0,
                ));
                effects.push(score_changed_effect(
                    claim.claimant,
                    delta,
                    state.scores[claimant_index],
                    "honest_challenge",
                ));
            } else {
                let delta = claim.declared.value() - actual.value();
                let responder_index = response.actor.index();
                state.scores[responder_index] = state.scores[responder_index].saturating_add(delta);
                state.counters[claim.claimant.index()].exposed_lies += 1;
                state.counters[response.actor.index()].successful_challenges += 1;
                state.exposed_row[responder_index].push(ExposedMask {
                    tile: claim.tile,
                    declared: claim.declared,
                    claimant: claim.claimant,
                    challenger: response.actor,
                });
                effects.push(challenge_resolved_effect(
                    response.turn_index,
                    ChallengeOutcome::Exposed,
                    claim.claimant,
                    response.actor,
                    0,
                    delta,
                ));
                effects.push(score_changed_effect(
                    response.actor,
                    delta,
                    state.scores[responder_index],
                    "exposed_lie",
                ));
            }
        }
    }

    advance_after_resolution(state, response.turn_index, &mut effects);
    state.freshness_token = state.freshness_token.next();
    state
        .effect_history
        .extend(effects.iter().map(|effect| format!("{:?}", effect.payload)));
    Ok(effects)
}

fn ensure_claim_still_legal(
    state: &MaskedClaimsState,
    claim: ValidatedClaim,
) -> Result<(), Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    if !matches!(state.phase, Phase::Claim { .. }) {
        return Err(actions::wrong_phase_diagnostic());
    }
    if state.active_seat != Some(claim.actor) || state.claimant != claim.actor {
        return Err(actions::wrong_claimant_diagnostic());
    }
    if state.pedestal.is_some() {
        return Err(actions::unavailable_action_diagnostic());
    }
    if !state.hands[claim.actor.index()].contains(&claim.tile) {
        return Err(actions::mask_not_in_hand_diagnostic());
    }
    Ok(())
}

fn ensure_response_still_legal(
    state: &MaskedClaimsState,
    response: ValidatedResponse,
) -> Result<(), Diagnostic> {
    if state.phase == Phase::Terminal || state.terminal_outcome.is_some() {
        return Err(actions::terminal_diagnostic());
    }
    let Phase::Reaction {
        turn_index,
        responder,
    } = state.phase
    else {
        return Err(actions::wrong_phase_diagnostic());
    };
    if turn_index != response.turn_index || responder != response.actor {
        return Err(actions::wrong_responder_diagnostic());
    }
    if state.active_seat != Some(response.actor) {
        return Err(actions::wrong_responder_diagnostic());
    }
    if state.pedestal.is_none() {
        return Err(actions::unavailable_action_diagnostic());
    }
    Ok(())
}

fn advance_after_resolution(
    state: &mut MaskedClaimsState,
    resolved_turn_index: u8,
    effects: &mut Vec<EffectEnvelope<MaskedClaimsEffect>>,
) {
    if resolved_turn_index + 1 >= crate::STANDARD_MAX_TURNS {
        state.phase = Phase::Terminal;
        state.active_seat = None;
        let outcome = resolve_terminal(state);
        state.terminal_outcome = Some(outcome);
        effects.push(terminal_effect(outcome, state.scores));
        return;
    }

    let next_turn = resolved_turn_index + 1;
    let next_claimant = state.claimant.other();
    state.turn_index = next_turn;
    state.claimant = next_claimant;
    state.phase = Phase::Claim {
        turn_index: next_turn,
    };
    state.active_seat = Some(next_claimant);
    effects.push(turn_advanced_effect(next_turn, next_claimant));
}

pub fn resolve_terminal(state: &MaskedClaimsState) -> TerminalOutcome {
    if state.scores[0] > state.scores[1] {
        return TerminalOutcome::ScoreWin {
            winner: MaskedClaimsSeat::Seat0,
            scores: state.scores,
        };
    }
    if state.scores[1] > state.scores[0] {
        return TerminalOutcome::ScoreWin {
            winner: MaskedClaimsSeat::Seat1,
            scores: state.scores,
        };
    }

    if state.counters[0].exposed_lies < state.counters[1].exposed_lies {
        return tiebreak(MaskedClaimsSeat::Seat0, state.scores, "fewer_exposed_lies");
    }
    if state.counters[1].exposed_lies < state.counters[0].exposed_lies {
        return tiebreak(MaskedClaimsSeat::Seat1, state.scores, "fewer_exposed_lies");
    }
    if state.counters[0].successful_challenges > state.counters[1].successful_challenges {
        return tiebreak(
            MaskedClaimsSeat::Seat0,
            state.scores,
            "more_successful_challenges",
        );
    }
    if state.counters[1].successful_challenges > state.counters[0].successful_challenges {
        return tiebreak(
            MaskedClaimsSeat::Seat1,
            state.scores,
            "more_successful_challenges",
        );
    }
    if state.counters[0].challenges_declared < state.counters[1].challenges_declared {
        return tiebreak(
            MaskedClaimsSeat::Seat0,
            state.scores,
            "fewer_challenges_declared",
        );
    }
    if state.counters[1].challenges_declared < state.counters[0].challenges_declared {
        return tiebreak(
            MaskedClaimsSeat::Seat1,
            state.scores,
            "fewer_challenges_declared",
        );
    }

    TerminalOutcome::Draw {
        scores: state.scores,
    }
}

const fn tiebreak(
    winner: MaskedClaimsSeat,
    scores: [u8; 2],
    tiebreak: &'static str,
) -> TerminalOutcome {
    TerminalOutcome::TiebreakWin {
        winner,
        scores,
        tiebreak,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        actions::{validate_command, ValidatedAction},
        ids::{Grade, MaskTileId},
        setup::{setup_match, SetupOptions},
    };
    use engine_core::{ActionPath, Actor, CommandEnvelope, RulesVersion, SeatId, Seed, SeededRng};

    fn actor(seat: &str) -> Actor {
        Actor {
            seat_id: SeatId(seat.to_owned()),
        }
    }

    fn command(state: &MaskedClaimsState, seat: &str, segments: Vec<&str>) -> CommandEnvelope {
        CommandEnvelope {
            actor: actor(seat),
            action_path: ActionPath {
                segments: segments.into_iter().map(str::to_owned).collect(),
            },
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        }
    }

    fn setup_state_with_hands(
        hand_0: Vec<MaskTileId>,
        hand_1: Vec<MaskTileId>,
    ) -> MaskedClaimsState {
        MaskedClaimsState::new_after_deal(
            crate::Variant::masked_claims_standard(),
            [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            [hand_0, hand_1],
            Vec::new(),
            SeededRng::from_seed(Seed(0)),
        )
    }

    fn claim_then_response(
        state: &mut MaskedClaimsState,
        tile: MaskTileId,
        declared: &str,
        response: &str,
    ) -> Vec<EffectEnvelope<MaskedClaimsEffect>> {
        let ValidatedAction::Claim(claim) = validate_command(
            state,
            &command(state, "seat_0", vec!["claim", tile.as_str(), declared]),
        )
        .expect("claim validates") else {
            panic!("expected claim");
        };
        apply_action(state, ValidatedAction::Claim(claim)).expect("claim applies");
        let ValidatedAction::Response(response) =
            validate_command(state, &command(state, "seat_1", vec![response]))
                .expect("response validates")
        else {
            panic!("expected response");
        };
        apply_action(state, ValidatedAction::Response(response)).expect("response applies")
    }

    #[test]
    fn accept_scores_declared_grade_and_never_reveals_tile() {
        let mut state =
            setup_state_with_hands(vec![MaskTileId::MaskG5A], vec![MaskTileId::MaskG1A]);
        let effects = claim_then_response(
            &mut state,
            MaskTileId::MaskG5A,
            "4",
            crate::ACTION_RESPOND_ACCEPT,
        );

        assert_eq!(state.scores, [4, 0]);
        assert_eq!(
            state.veiled_gallery[0],
            vec![VeiledClaim {
                declared: Grade::Jeweled
            }]
        );
        assert!(state.exposed_row[0].is_empty());
        assert_eq!(state.phase, Phase::Claim { turn_index: 1 });
        assert_eq!(state.active_seat, Some(MaskedClaimsSeat::Seat1));

        let rendered = format!("{effects:?}");
        assert!(!rendered.contains(MaskTileId::MaskG5A.as_str()));
        assert!(!rendered.contains(&MaskTileId::MaskG5A.label()));
    }

    #[test]
    fn honest_challenge_reveals_tile_and_scores_actual_plus_bonus() {
        let mut state =
            setup_state_with_hands(vec![MaskTileId::MaskG4A], vec![MaskTileId::MaskG1A]);
        let effects = claim_then_response(
            &mut state,
            MaskTileId::MaskG4A,
            "3",
            crate::ACTION_RESPOND_CHALLENGE,
        );

        assert_eq!(state.scores, [6, 0]);
        assert_eq!(state.exposed_row[0][0].tile, MaskTileId::MaskG4A);
        assert_eq!(state.counters[1].challenges_declared, 1);
        assert_eq!(state.counters[1].successful_challenges, 0);

        let rendered = format!("{effects:?}");
        assert!(rendered.contains("MaskRevealed"));
        assert!(rendered.contains(MaskTileId::MaskG4A.as_str()));
        assert!(rendered.contains("Honest"));
    }

    #[test]
    fn exposed_challenge_scores_gap_for_responder_and_tracks_counters() {
        let mut state =
            setup_state_with_hands(vec![MaskTileId::MaskG2A], vec![MaskTileId::MaskG1A]);
        let effects = claim_then_response(
            &mut state,
            MaskTileId::MaskG2A,
            "5",
            crate::ACTION_RESPOND_CHALLENGE,
        );

        assert_eq!(state.scores, [0, 3]);
        assert_eq!(state.exposed_row[1][0].tile, MaskTileId::MaskG2A);
        assert_eq!(state.counters[0].exposed_lies, 1);
        assert_eq!(state.counters[1].successful_challenges, 1);
        assert_eq!(state.counters[1].challenges_declared, 1);
        assert!(format!("{effects:?}").contains("Exposed"));
    }

    #[test]
    fn terminal_resolves_after_eighth_turn_without_hidden_reveal() {
        let mut state = setup_match(
            Seed(3),
            &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
            &SetupOptions::default(),
        )
        .expect("setup succeeds");
        state.turn_index = 7;
        state.claimant = MaskedClaimsSeat::Seat1;
        state.active_seat = Some(MaskedClaimsSeat::Seat1);
        state.phase = Phase::Claim { turn_index: 7 };
        state.hands[1] = vec![MaskTileId::MaskG3A];
        state.scores = [2, 2];

        let ValidatedAction::Claim(claim) = validate_command(
            &state,
            &command(&state, "seat_1", vec!["claim", "mask_g3_a", "3"]),
        )
        .expect("claim validates") else {
            panic!("expected claim");
        };
        apply_action(&mut state, ValidatedAction::Claim(claim)).expect("claim applies");
        let ValidatedAction::Response(response) = validate_command(
            &state,
            &command(&state, "seat_0", vec![crate::ACTION_RESPOND_ACCEPT]),
        )
        .expect("response validates") else {
            panic!("expected response");
        };
        let effects = apply_action(&mut state, ValidatedAction::Response(response))
            .expect("response applies");

        assert_eq!(state.phase, Phase::Terminal);
        assert!(matches!(
            state.terminal_outcome,
            Some(TerminalOutcome::ScoreWin {
                winner: MaskedClaimsSeat::Seat1,
                scores: [2, 5]
            })
        ));
        let rendered = format!("{effects:?}");
        assert!(rendered.contains("Terminal"));
        assert!(!rendered.contains("mask_g3_a"));
    }

    #[test]
    fn terminal_tiebreak_ladder_is_deterministic() {
        let mut state = setup_state_with_hands(Vec::new(), Vec::new());
        state.scores = [4, 4];
        state.counters[0].exposed_lies = 0;
        state.counters[1].exposed_lies = 1;
        assert!(matches!(
            resolve_terminal(&state),
            TerminalOutcome::TiebreakWin {
                winner: MaskedClaimsSeat::Seat0,
                tiebreak: "fewer_exposed_lies",
                ..
            }
        ));

        state.counters[1].exposed_lies = 0;
        state.counters[0].successful_challenges = 1;
        state.counters[1].successful_challenges = 0;
        assert!(matches!(
            resolve_terminal(&state),
            TerminalOutcome::TiebreakWin {
                winner: MaskedClaimsSeat::Seat0,
                tiebreak: "more_successful_challenges",
                ..
            }
        ));

        state.counters[0].successful_challenges = 0;
        state.counters[0].challenges_declared = 0;
        state.counters[1].challenges_declared = 1;
        assert!(matches!(
            resolve_terminal(&state),
            TerminalOutcome::TiebreakWin {
                winner: MaskedClaimsSeat::Seat0,
                tiebreak: "fewer_challenges_declared",
                ..
            }
        ));

        state.counters[1].challenges_declared = 0;
        assert!(matches!(
            resolve_terminal(&state),
            TerminalOutcome::Draw { scores: [4, 4] }
        ));
    }
}
