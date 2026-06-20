//! Browser-bridge helpers for `secret_draft` (hidden-information simultaneous draft).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Viewer};
use secret_draft::replay_support::{
    import_public_export as secret_import_public_export, PublicReplayStep as SecretPublicReplayStep,
};
use secret_draft::{
    apply_action as secret_apply_action, project_view as secret_project_view,
    setup_match as secret_setup_match, SecretDraftEffect, SecretDraftState,
};

use crate::actors::secret_actor_for_seat;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::seats::{parse_secret_seat, seats};
use crate::store::{next_replay_id, REPLAYS};
use crate::{visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_secret_draft_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(secret_draft::GAME_ID)
    )
}

pub(crate) fn import_secret_draft_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        secret_draft::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != secret_draft::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != secret_draft::VARIANT_ID {
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
    let timeline = secret_import_public_export(&export);
    let replay_id = next_replay_id(GAME_SECRET_DRAFT);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_SECRET_DRAFT.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_secret)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_SECRET_DRAFT),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

pub(crate) fn public_timeline_step_from_secret(
    step: &SecretPublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn secret_replay_to_cursor(
    _seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(SecretDraftState, Vec<EffectEnvelope<SecretDraftEffect>>), String> {
    let seats = seats();
    let mut state = secret_setup_match(&seats, &secret_draft::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_secret_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: secret_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action =
            secret_draft::actions::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(secret_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

pub(crate) fn secret_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &SecretDraftState,
    effects: &[EffectEnvelope<SecretDraftEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        secret_view_json(&secret_project_view(state, &viewer)),
        secret_effects_json(effects, &viewer)
    )
}

pub(crate) fn secret_view_json(view: &secret_draft::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"round_number\":{},\"round_limit\":{},\"phase\":\"{}\",\"active_seat\":{},\"priority_seat\":\"{}\",\"visible_pool\":[{}],\"drafted\":{},\"commitments\":{},\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"revealed_history\":[{}],\"terminal\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.round_number,
        view.round_limit,
        view.phase.as_str(),
        secret_active_seat_json(view),
        view.priority_seat.as_str(),
        view.visible_pool
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(","),
        secret_drafted_json(&view.drafted),
        secret_commitments_json(&view.commitments),
        view.scores[0],
        view.scores[1],
        view.revealed_history
            .iter()
            .map(secret_revealed_round_json)
            .collect::<Vec<_>>()
            .join(","),
        secret_terminal_json(&view.terminal),
        view.freshness_token.0,
        secret_private_view_json(&view.private_view),
        secret_ui_json(&view.ui)
    )
}

pub(crate) fn secret_active_seat_json(view: &secret_draft::PublicView) -> String {
    if !matches!(
        view.terminal,
        secret_draft::visibility::TerminalView::NonTerminal
    ) {
        return "null".to_owned();
    }
    if !view.commitments.seat_0.committed {
        return "\"seat_0\"".to_owned();
    }
    if !view.commitments.seat_1.committed {
        return "\"seat_1\"".to_owned();
    }
    "null".to_owned()
}

pub(crate) fn secret_item_json(item: &secret_draft::visibility::DraftItemView) -> String {
    format!(
        "{{\"item_id\":\"{}\",\"label\":\"{}\",\"thread\":\"{}\",\"value\":{},\"accessibility_label\":\"{}\"}}",
        escape_json(&item.item_id),
        escape_json(&item.label),
        escape_json(&item.thread),
        item.value,
        escape_json(&item.accessibility_label)
    )
}

pub(crate) fn secret_drafted_json(
    drafted: &secret_draft::visibility::DraftedCollectionsView,
) -> String {
    format!(
        "{{\"seat_0\":[{}],\"seat_1\":[{}]}}",
        drafted
            .seat_0
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(","),
        drafted
            .seat_1
            .iter()
            .map(secret_item_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn secret_commitments_json(
    commitments: &secret_draft::visibility::CommitmentViews,
) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{},\"copy\":\"{}\"}}",
        secret_commitment_json(&commitments.seat_0),
        secret_commitment_json(&commitments.seat_1),
        escape_json(&commitments.copy)
    )
}

pub(crate) fn secret_commitment_json(
    commitment: &secret_draft::visibility::CommitmentView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"committed\":{},\"status\":\"{}\",\"accessibility_label\":\"{}\"}}",
        commitment.seat.as_str(),
        commitment.committed,
        escape_json(&commitment.status),
        escape_json(&commitment.accessibility_label)
    )
}

pub(crate) fn secret_revealed_round_json(
    round: &secret_draft::visibility::RevealedRoundView,
) -> String {
    format!(
        "{{\"round_number\":{},\"seat_0_choice\":{},\"seat_1_choice\":{},\"seat_0_award\":{},\"seat_1_award\":{},\"priority_seat\":\"{}\",\"contested\":{}}}",
        round.round_number,
        secret_item_json(&round.seat_0_choice),
        secret_item_json(&round.seat_1_choice),
        secret_item_json(&round.seat_0_award),
        secret_item_json(&round.seat_1_award),
        round.priority_seat.as_str(),
        round.contested
    )
}

pub(crate) fn secret_terminal_json(terminal: &secret_draft::visibility::TerminalView) -> String {
    match terminal {
        secret_draft::visibility::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"winner\":null,\"draw\":false}".to_owned()
        }
        secret_draft::visibility::TerminalView::Win { winning_seat, .. } => format!(
            "{{\"terminal\":true,\"winner\":\"{}\",\"draw\":false}}",
            winning_seat.as_str()
        ),
        secret_draft::visibility::TerminalView::Draw { .. } => {
            "{\"terminal\":true,\"winner\":null,\"draw\":true}".to_owned()
        }
    }
}

pub(crate) fn secret_private_view_json(
    private_view: &secret_draft::visibility::PrivateView,
) -> String {
    match private_view {
        secret_draft::visibility::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_committed\":false,\"waiting_copy\":\"\"}".to_owned()
        }
        secret_draft::visibility::PrivateView::Seat {
            seat,
            own_committed,
            waiting_copy,
        } => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_committed\":{},\"waiting_copy\":\"{}\"}}",
            seat.as_str(),
            own_committed,
            escape_json(waiting_copy)
        ),
    }
}

pub(crate) fn secret_ui_json(ui: &secret_draft::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"table_label\":\"{}\",\"visible_pool_label\":\"{}\",\"drafted_label\":\"{}\",\"pending_label\":\"{}\",\"score_label\":\"{}\",\"reveal_group_token\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(ui.game_id),
        escape_json(ui.display_name),
        escape_json(&ui.table_label),
        escape_json(&ui.visible_pool_label),
        escape_json(&ui.drafted_label),
        escape_json(&ui.pending_label),
        escape_json(&ui.score_label),
        escape_json(&ui.reveal_group_token),
        escape_json(&ui.reduced_motion_token)
    )
}

pub(crate) fn secret_effects_json(
    effects: &[EffectEnvelope<SecretDraftEffect>],
    viewer: &Viewer,
) -> String {
    let body = secret_draft::visibility::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(secret_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn secret_effect_json(effect: &EffectEnvelope<SecretDraftEffect>) -> String {
    let payload = match &effect.payload {
        SecretDraftEffect::CommitmentPlaced { seat, round } => format!(
            "{{\"type\":\"commitment_placed\",\"seat\":\"{}\",\"round\":{}}}",
            seat.as_str(),
            round
        ),
        SecretDraftEffect::OwnCommitAccepted { seat, round } => format!(
            "{{\"type\":\"own_commit_accepted\",\"seat\":\"{}\",\"round\":{}}}",
            seat.as_str(),
            round
        ),
        SecretDraftEffect::PendingSeatsChanged {
            round,
            seat_0_committed,
            seat_1_committed,
        } => format!(
            "{{\"type\":\"pending_seats_changed\",\"round\":{},\"seat_0_committed\":{},\"seat_1_committed\":{}}}",
            round, seat_0_committed, seat_1_committed
        ),
        SecretDraftEffect::RevealBatchStarted { round, group_id } => format!(
            "{{\"type\":\"reveal_batch_started\",\"round\":{},\"group_id\":\"{}\"}}",
            round,
            escape_json(group_id)
        ),
        SecretDraftEffect::ChoicesRevealed {
            round,
            seat_0_item,
            seat_1_item,
        } => format!(
            "{{\"type\":\"choices_revealed\",\"round\":{},\"seat_0_item\":\"{}\",\"seat_1_item\":\"{}\"}}",
            round,
            seat_0_item.as_str(),
            seat_1_item.as_str()
        ),
        SecretDraftEffect::DraftResolved {
            round,
            seat_0_award,
            seat_1_award,
            removed_items,
            conflict,
        } => format!(
            "{{\"type\":\"draft_resolved\",\"round\":{},\"seat_0_award\":\"{}\",\"seat_1_award\":\"{}\",\"removed_items\":[\"{}\",\"{}\"],\"conflict\":{}}}",
            round,
            seat_0_award.as_str(),
            seat_1_award.as_str(),
            removed_items[0].as_str(),
            removed_items[1].as_str(),
            conflict.map_or_else(|| "null".to_owned(), secret_conflict_json)
        ),
        SecretDraftEffect::PoolChanged { remaining_count } => format!(
            "{{\"type\":\"pool_changed\",\"remaining_count\":{}}}",
            remaining_count
        ),
        SecretDraftEffect::ScoreChanged {
            scores,
            tie_break_summary,
        } => format!(
            "{{\"type\":\"score_changed\",\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tie_break_summary\":{}}}",
            scores[0],
            scores[1],
            secret_tie_break_json(*tie_break_summary)
        ),
        SecretDraftEffect::RoundAdvanced {
            next_round,
            priority_seat,
        } => format!(
            "{{\"type\":\"round_advanced\",\"next_round\":{},\"priority_seat\":\"{}\"}}",
            next_round,
            priority_seat.as_str()
        ),
        SecretDraftEffect::Terminal {
            outcome,
            final_scores,
            tie_break_summary,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"final_scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"tie_break_summary\":{}}}",
            secret_terminal_outcome_json(*outcome),
            final_scores[0],
            final_scores[1],
            secret_tie_break_json(*tie_break_summary)
        ),
        SecretDraftEffect::PublicDiagnostic { code, message } => format!(
            "{{\"type\":\"public_diagnostic\",\"code\":\"{}\",\"message\":\"{}\"}}",
            escape_json(code),
            escape_json(message)
        ),
        SecretDraftEffect::PrivateDiagnostic {
            seat,
            code,
            message,
        } => format!(
            "{{\"type\":\"private_diagnostic\",\"seat\":\"{}\",\"code\":\"{}\",\"message\":\"{}\"}}",
            seat.as_str(),
            escape_json(code),
            escape_json(message)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn secret_conflict_json(conflict: secret_draft::effects::ConflictSummary) -> String {
    format!(
        "{{\"contested_item\":\"{}\",\"priority_seat\":\"{}\",\"fallback_item\":\"{}\"}}",
        conflict.contested_item.as_str(),
        conflict.priority_seat.as_str(),
        conflict.fallback_item.as_str()
    )
}

pub(crate) fn secret_tie_break_json(summary: secret_draft::effects::TieBreakSummary) -> String {
    format!(
        "{{\"scores\":[{},{}],\"complete_sets\":[{},{}],\"highest_single_values\":[{},{}],\"distinct_threads\":[{},{}],\"priority_conflict_wins\":[{},{}]}}",
        summary.scores[0],
        summary.scores[1],
        summary.complete_sets[0],
        summary.complete_sets[1],
        summary.highest_single_values[0],
        summary.highest_single_values[1],
        summary.distinct_threads[0],
        summary.distinct_threads[1],
        summary.priority_conflict_wins[0],
        summary.priority_conflict_wins[1]
    )
}

pub(crate) fn secret_terminal_outcome_json(outcome: secret_draft::TerminalOutcome) -> String {
    match outcome {
        secret_draft::TerminalOutcome::Win { seat } => {
            format!("{{\"kind\":\"win\",\"winner\":\"{}\"}}", seat.as_str())
        }
        secret_draft::TerminalOutcome::Draw => "{\"kind\":\"draw\",\"winner\":null}".to_owned(),
    }
}
