//! Browser-bridge helpers for `masked_claims` (hidden-information bluffing game).

use engine_core::{EffectEnvelope, Viewer};
use masked_claims::{MaskedClaimsEffect, MaskedClaimsSeat};

use crate::constants::*;
use crate::json::{diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{string_array, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_masked_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(masked_claims::GAME_ID)
    )
}

pub(crate) fn import_masked_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = masked_claims::PublicReplayExport::from_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    if export.rules_version != masked_claims::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != masked_claims::VARIANT_ID {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {}", export.variant),
        ));
    }
    if export.viewer != "observer" {
        return Err(diagnostic_string(
            "unsupported_replay_viewer",
            &format!("unsupported replay viewer: {}", export.viewer),
        ));
    }
    let replay_id = next_replay_id(GAME_MASKED_CLAIMS);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_MASKED_CLAIMS.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: export.viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_masked)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_MASKED_CLAIMS),
        escape_json(&export.viewer),
        export.steps.len()
    ))
}

pub(crate) fn masked_redacted_command_summary(command: &AppliedCommand) -> String {
    match command.action_path.as_slice() {
        [family, _tile, declared] if family == masked_claims::ACTION_CLAIM => {
            format!("claim/grade-{declared}")
        }
        _ => command.action_path.join("/"),
    }
}

pub(crate) fn masked_view_json(view: &masked_claims::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"turn_index\":{},\"claimant\":\"{}\",\"hand_counts\":{},\"pedestal\":{},\"veiled_gallery\":[{},{}],\"exposed_rows\":[{},{}],\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"counters\":[{},{}],\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        option_masked_seat_json(view.active_seat),
        view.turn_index,
        view.claimant.as_str(),
        masked_counts_json(view.hand_counts.seat_0, view.hand_counts.seat_1),
        view.pedestal
            .map_or_else(|| "null".to_owned(), masked_pedestal_json),
        masked_veiled_json(&view.veiled_gallery[0]),
        masked_veiled_json(&view.veiled_gallery[1]),
        masked_exposed_json(&view.exposed_rows[0]),
        masked_exposed_json(&view.exposed_rows[1]),
        view.scores[0],
        view.scores[1],
        masked_counter_json(view.counters[0]),
        masked_counter_json(view.counters[1]),
        masked_terminal_json(&view.terminal),
        masked_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), masked_outcome_rationale_json),
        view.freshness_token.0,
        masked_private_view_json(&view.private_view),
        masked_ui_json(&view.ui)
    )
}

pub(crate) fn option_masked_seat_json(seat: Option<MaskedClaimsSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

pub(crate) fn masked_counts_json(seat_0: u8, seat_1: u8) -> String {
    format!("{{\"seat_0\":{},\"seat_1\":{}}}", seat_0, seat_1)
}

pub(crate) fn masked_pedestal_json(pedestal: masked_claims::PedestalView) -> String {
    format!(
        "{{\"claimant\":\"{}\",\"declared_grade\":\"{}\",\"declared_label\":\"{}\"}}",
        pedestal.claimant.as_str(),
        pedestal.declared_grade.as_str(),
        pedestal.declared_grade.label()
    )
}

pub(crate) fn masked_veiled_json(veiled: &[masked_claims::VeiledClaimView]) -> String {
    let body = veiled
        .iter()
        .map(|claim| {
            format!(
                "{{\"declared_grade\":\"{}\",\"declared_label\":\"{}\"}}",
                claim.declared_grade.as_str(),
                claim.declared_grade.label()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn masked_exposed_json(exposed: &[masked_claims::ExposedMaskView]) -> String {
    let body = exposed
        .iter()
        .map(|mask| {
            format!(
                "{{\"tile_id\":\"{}\",\"actual_grade\":\"{}\",\"declared_grade\":\"{}\",\"claimant\":\"{}\",\"challenger\":\"{}\"}}",
                escape_json(&mask.tile_id),
                mask.actual_grade.as_str(),
                mask.declared_grade.as_str(),
                mask.claimant.as_str(),
                mask.challenger.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn masked_counter_json(counter: masked_claims::CounterView) -> String {
    format!(
        "{{\"exposed_lies\":{},\"successful_challenges\":{},\"challenges_declared\":{}}}",
        counter.exposed_lies, counter.successful_challenges, counter.challenges_declared
    )
}

pub(crate) fn masked_terminal_json(terminal: &masked_claims::TerminalView) -> String {
    match terminal {
        masked_claims::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        masked_claims::TerminalView::Complete { outcome, .. } => {
            masked_terminal_outcome_json(*outcome)
        }
    }
}

pub(crate) fn masked_terminal_rationale(
    terminal: &masked_claims::TerminalView,
) -> Option<&masked_claims::OutcomeRationaleView> {
    match terminal {
        masked_claims::TerminalView::NonTerminal => None,
        masked_claims::TerminalView::Complete { rationale, .. } => Some(rationale),
    }
}

pub(crate) fn masked_outcome_rationale_json(
    rationale: &masked_claims::OutcomeRationaleView,
) -> String {
    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"decisive_rule_ids\":[{}],\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        string_array(&rationale.decisive_rule_ids),
        rationale.final_scores[0],
        rationale.final_scores[1]
    )
}

pub(crate) fn masked_private_view_json(private: &masked_claims::PrivateView) -> String {
    match private {
        masked_claims::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_hand\":[]}".to_owned()
        }
        masked_claims::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_hand\":[{}]}}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(masked_mask_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

pub(crate) fn masked_mask_json(mask: &masked_claims::MaskView) -> String {
    format!(
        "{{\"tile_id\":\"{}\",\"grade\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&mask.tile_id),
        mask.grade.as_str(),
        escape_json(&mask.label),
        escape_json(&mask.accessibility_label)
    )
}

pub(crate) fn masked_ui_json(ui: &masked_claims::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"variant_id\":\"{}\",\"display_name\":\"{}\",\"grade_labels\":[{}],\"claim_preview_template\":\"{}\",\"reaction_prompt_template\":\"{}\"}}",
        escape_json(&ui.game_id),
        escape_json(&ui.variant_id),
        escape_json(&ui.display_name),
        string_array(&ui.grade_labels),
        escape_json(&ui.claim_preview_template),
        escape_json(&ui.reaction_prompt_template)
    )
}

pub(crate) fn masked_terminal_outcome_json(outcome: masked_claims::TerminalOutcome) -> String {
    match outcome {
        masked_claims::TerminalOutcome::ScoreWin { winner, scores } => format!(
            "{{\"kind\":\"score_win\",\"winner\":\"{}\",\"draw\":false,\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(), scores[0], scores[1]
        ),
        masked_claims::TerminalOutcome::TiebreakWin {
            winner,
            scores,
            tiebreak,
        } => format!(
            "{{\"kind\":\"tiebreak_win\",\"winner\":\"{}\",\"draw\":false,\"tiebreak\":\"{}\",\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(),
            escape_json(tiebreak),
            scores[0],
            scores[1]
        ),
        masked_claims::TerminalOutcome::Draw { scores } => format!(
            "{{\"kind\":\"draw\",\"winner\":null,\"draw\":true,\"scores\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            scores[0], scores[1]
        ),
    }
}

pub(crate) fn masked_effects_json(
    effects: &[EffectEnvelope<MaskedClaimsEffect>],
    viewer: &Viewer,
) -> String {
    let body = masked_claims::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(masked_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn masked_effect_json(effect: &EffectEnvelope<MaskedClaimsEffect>) -> String {
    let payload = match &effect.payload {
        MaskedClaimsEffect::ClaimPlaced {
            turn,
            claimant,
            declared_grade,
            ..
        } => format!(
            "{{\"type\":\"claim_placed\",\"turn\":{},\"claimant\":\"{}\",\"declared_grade\":\"{}\"}}",
            turn,
            claimant.as_str(),
            declared_grade.as_str()
        ),
        MaskedClaimsEffect::ReactionWindowOpened {
            turn,
            responder,
            declared_grade,
            choices,
            ..
        } => format!(
            "{{\"type\":\"reaction_window_opened\",\"turn\":{},\"responder\":\"{}\",\"declared_grade\":\"{}\",\"choices\":[{}]}}",
            turn,
            responder.as_str(),
            declared_grade.as_str(),
            string_array(choices)
        ),
        MaskedClaimsEffect::ClaimAccepted {
            turn,
            claimant,
            declared_grade,
            score_delta,
            ..
        } => format!(
            "{{\"type\":\"claim_accepted\",\"turn\":{},\"claimant\":\"{}\",\"declared_grade\":\"{}\",\"score_delta\":{}}}",
            turn,
            claimant.as_str(),
            declared_grade.as_str(),
            score_delta
        ),
        MaskedClaimsEffect::ChallengeDeclared {
            turn, responder, ..
        } => format!(
            "{{\"type\":\"challenge_declared\",\"turn\":{},\"responder\":\"{}\"}}",
            turn,
            responder.as_str()
        ),
        MaskedClaimsEffect::MaskRevealed {
            turn,
            tile_id,
            actual_grade,
            ..
        } => format!(
            "{{\"type\":\"mask_revealed\",\"turn\":{},\"tile_id\":\"{}\",\"actual_grade\":\"{}\"}}",
            turn,
            escape_json(tile_id),
            actual_grade.as_str()
        ),
        MaskedClaimsEffect::ChallengeResolved {
            turn,
            outcome,
            claimant,
            responder,
            claimant_award,
            responder_award,
            ..
        } => format!(
            "{{\"type\":\"challenge_resolved\",\"turn\":{},\"outcome\":\"{}\",\"claimant\":\"{}\",\"responder\":\"{}\",\"claimant_award\":{},\"responder_award\":{}}}",
            turn,
            outcome.as_str(),
            claimant.as_str(),
            responder.as_str(),
            claimant_award,
            responder_award
        ),
        MaskedClaimsEffect::ScoreChanged {
            seat,
            delta,
            total,
            reason,
        } => format!(
            "{{\"type\":\"claim_score_changed\",\"seat\":\"{}\",\"delta\":{},\"total\":{},\"reason\":\"{}\"}}",
            seat.as_str(),
            delta,
            total,
            escape_json(reason)
        ),
        MaskedClaimsEffect::TurnAdvanced { turn, claimant, .. } => format!(
            "{{\"type\":\"claim_turn_advanced\",\"turn\":{},\"claimant\":\"{}\"}}",
            turn,
            claimant.as_str()
        ),
        MaskedClaimsEffect::Terminal {
            outcome,
            final_scores,
            tiebreak_summary,
            ..
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tiebreak_summary\":\"{}\"}}",
            masked_terminal_outcome_json(*outcome),
            final_scores[0],
            final_scores[1],
            escape_json(tiebreak_summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn public_timeline_step_from_masked(step: &masked_claims::PublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}
