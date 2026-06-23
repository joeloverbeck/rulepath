//! Browser-bridge helpers for `draughts_lite` (perfect-information board game).

use draughts_lite::{
    apply_action as draughts_apply_action, project_view as draughts_project_view,
    replay_support::replay_commands as draughts_replay_commands,
    setup_match as draughts_setup_match, DraughtsLiteEffect, DraughtsLiteSeat, DraughtsLiteState,
};
use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};

use crate::actors::draughts_actor_for_seat;
use crate::commands::command_record_json;
use crate::constants::*;
use crate::json::{diagnostic_json, escape_json};
use crate::seats::{
    canonical_seats_for_count, canonical_trace_seat_id, parse_draughts_seat, seats,
};
use crate::{option_string_json, string_array, visibility_json, AppliedCommand};

pub(crate) fn draughts_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(DraughtsLiteState, Vec<EffectEnvelope<DraughtsLiteEffect>>), String> {
    let seats = seats();
    let mut state =
        draughts_setup_match(Seed(seed), &seats, &draughts_lite::SetupOptions::default())
            .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_draughts_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: draughts_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = draughts_lite::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(draughts_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

pub(crate) fn draughts_effects_json(effects: &[EffectEnvelope<DraughtsLiteEffect>]) -> String {
    let body = effects
        .iter()
        .map(draughts_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn draughts_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_paths = commands
        .iter()
        .map(|command| command.action_path.clone())
        .collect::<Vec<_>>();
    let hashes = draughts_replay_commands(seed, &command_paths);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| draughts_command_record_json(index, command))
        .collect::<Result<Vec<_>, _>>()?
        .join(",");
    let seats_json = canonical_seats_for_count(2)
        .iter()
        .enumerate()
        .map(|(index, seat)| {
            format!(
                "{{\"seat_id\":\"{}\",\"player_id\":\"player-{}\"}}",
                escape_json(&seat.0),
                index
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let outcome = hashes.outcome.map_or_else(
        || "{\"terminal\":false,\"winner\":null,\"kind\":\"none\"}".to_owned(),
        |outcome| match outcome {
            draughts_lite::TerminalOutcome::Win { seat } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\"}}",
                canonical_trace_seat_id(seat.index() as u32)
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log with ordered multi-segment action paths preserved.\",\"migration_update_note\":\"Updated expected hashes for VICEXPSHASUR-006 terminal reason projection.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"draughts_lite is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"draughts_lite is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"draughts_lite game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"draughts_lite has no private-view API.\",\"preview_hashes\":\"draughts_lite uses action-tree metadata and semantic effects rather than a separate preview hash surface in Gate 7.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_DRAUGHTS_LITE),
        escape_json(DRAUGHTS_LITE_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_DRAUGHTS_LITE_STANDARD),
        seats_json,
        commands_json,
        checkpoints,
        hashes.state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.view_hash.0,
        hashes.replay_hash.0,
        outcome,
        outcome
    ))
}

fn draughts_command_record_json(index: usize, command: &AppliedCommand) -> Result<String, String> {
    let seat = parse_draughts_seat(&command.actor_seat)?;
    Ok(command_record_json(
        index,
        &AppliedCommand {
            actor_seat: canonical_trace_seat_id(seat.index() as u32),
            action_path: command.action_path.clone(),
            freshness_token: command.freshness_token,
        },
    ))
}

pub(crate) fn draughts_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &DraughtsLiteState,
    effects: &[EffectEnvelope<DraughtsLiteEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        draughts_view_json(&draughts_project_view(state, &Viewer { seat_id: None })),
        draughts_effects_json(effects)
    )
}

pub(crate) fn draughts_view_json(view: &draughts_lite::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"board_rows\":{},\"board_columns\":{},\"cells\":[{}],\"active_seat\":{},\"ply_count\":{},\"command_count\":{},\"status_label\":\"{}\",\"freshness_token\":{},\"terminal_kind\":\"{}\",\"winning_seat\":{},\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"ui\":{},\"replay_step_index\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.board_rows,
        view.board_columns,
        view.cells
            .iter()
            .map(draughts_cell_json)
            .collect::<Vec<_>>()
            .join(","),
        option_draughts_seat_json(view.active_seat),
        view.ply_count,
        view.command_count,
        escape_json(&view.status_label),
        view.freshness_token.0,
        draughts_terminal_kind(&view.terminal),
        option_draughts_seat_json(draughts_terminal_winner(&view.terminal)),
        escape_json(&view.private_view.status),
        string_array(&view.private_view.hidden_fields),
        draughts_ui_json(&view.ui),
        view.replay_step_index
            .map_or_else(|| "null".to_owned(), |step| step.to_string())
    )
}

pub(crate) fn draughts_cell_json(cell: &draughts_lite::CellView) -> String {
    format!(
        "{{\"cell\":\"{}\",\"cell_id\":\"{}\",\"row\":{},\"column\":{},\"playable\":{},\"presentation_token\":\"{}\",\"accessibility_label\":\"{}\",\"occupancy\":\"{}\",\"owner\":{},\"piece_id\":{},\"piece_kind\":{},\"piece_token_key\":{},\"piece_shape_label\":{},\"piece_pattern_label\":{},\"piece_label\":{},\"piece_accessibility_label\":{}}}",
        cell.cell.id(),
        escape_json(&cell.cell_id),
        cell.row,
        cell.column,
        cell.playable,
        escape_json(&cell.presentation_token),
        escape_json(&cell.accessibility_label),
        escape_json(&cell.occupancy),
        option_draughts_seat_json(cell.owner),
        option_string_json(cell.piece_id.as_deref()),
        cell.piece_kind
            .map_or_else(|| "null".to_owned(), |kind| format!("\"{}\"", kind.as_str())),
        option_string_json(cell.piece_token_key.as_deref()),
        option_string_json(cell.piece_shape_label.as_deref()),
        option_string_json(cell.piece_pattern_label.as_deref()),
        option_string_json(cell.piece_label.as_deref()),
        option_string_json(cell.piece_accessibility_label.as_deref())
    )
}

pub(crate) fn draughts_ui_json(ui: &draughts_lite::UiMetadata) -> String {
    format!(
        "{{\"board_label\":\"{}\",\"row_count\":{},\"column_count\":{},\"playable_cell_token\":\"{}\",\"non_playable_cell_token\":\"{}\",\"first_man_token_key\":\"{}\",\"first_man_shape_label\":\"{}\",\"first_crown_token_key\":\"{}\",\"first_crown_shape_label\":\"{}\",\"second_man_token_key\":\"{}\",\"second_man_shape_label\":\"{}\",\"second_crown_token_key\":\"{}\",\"second_crown_shape_label\":\"{}\"}}",
        escape_json(&ui.board_label),
        ui.row_count,
        ui.column_count,
        escape_json(&ui.playable_cell_token),
        escape_json(&ui.non_playable_cell_token),
        escape_json(&ui.first_man_token_key),
        escape_json(&ui.first_man_shape_label),
        escape_json(&ui.first_crown_token_key),
        escape_json(&ui.first_crown_shape_label),
        escape_json(&ui.second_man_token_key),
        escape_json(&ui.second_man_shape_label),
        escape_json(&ui.second_crown_token_key),
        escape_json(&ui.second_crown_shape_label)
    )
}

pub(crate) fn draughts_effect_json(effect: &EffectEnvelope<DraughtsLiteEffect>) -> String {
    let payload = match &effect.payload {
        DraughtsLiteEffect::MoveCommitted {
            action_path,
            seat,
            piece_id,
            start_cell,
            final_cell,
            move_kind,
            path_length,
        } => format!(
            "{{\"type\":\"move_committed\",\"action_path\":[{}],\"seat\":\"{}\",\"piece_id\":\"{}\",\"start_cell\":\"{}\",\"final_cell\":\"{}\",\"move_kind\":\"{}\",\"path_length\":{}}}",
            string_array(action_path),
            seat.as_str(),
            piece_id.stable_id(),
            start_cell.id(),
            final_cell.id(),
            draughts_move_kind(*move_kind),
            path_length
        ),
        DraughtsLiteEffect::QuietStep {
            piece_id,
            origin,
            landing,
            piece_kind_before,
            piece_kind_after,
        } => format!(
            "{{\"type\":\"quiet_step\",\"piece_id\":\"{}\",\"origin\":\"{}\",\"landing\":\"{}\",\"piece_kind_before\":\"{}\",\"piece_kind_after\":\"{}\"}}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            piece_kind_before.as_str(),
            piece_kind_after.as_str()
        ),
        DraughtsLiteEffect::CaptureStep {
            piece_id,
            origin,
            landing,
            captured_cell,
            captured_piece_id,
            captured_owner,
        } => format!(
            "{{\"type\":\"capture_step\",\"piece_id\":\"{}\",\"origin\":\"{}\",\"landing\":\"{}\",\"captured_cell\":\"{}\",\"captured_piece_id\":\"{}\",\"captured_owner\":\"{}\"}}",
            piece_id.stable_id(),
            origin.id(),
            landing.id(),
            captured_cell.id(),
            captured_piece_id.stable_id(),
            captured_owner.as_str()
        ),
        DraughtsLiteEffect::Promotion {
            piece_id,
            seat,
            cell,
            from,
            to,
            during_capture,
        } => format!(
            "{{\"type\":\"promotion\",\"piece_id\":\"{}\",\"seat\":\"{}\",\"cell\":\"{}\",\"from\":\"{}\",\"to\":\"{}\",\"during_capture\":{}}}",
            piece_id.stable_id(),
            seat.as_str(),
            cell.id(),
            from.as_str(),
            to.as_str(),
            during_capture
        ),
        DraughtsLiteEffect::ForcedCaptureAvailable {
            active_seat,
            capture_origin_count,
            explanation,
        } => format!(
            "{{\"type\":\"forced_capture_available\",\"active_seat\":\"{}\",\"capture_origin_count\":{},\"explanation\":\"{}\"}}",
            active_seat.as_str(),
            capture_origin_count,
            escape_json(explanation)
        ),
        DraughtsLiteEffect::ForcedContinuationRequired {
            piece_id,
            current_landing,
            continuation_destination_count,
            explanation,
        } => format!(
            "{{\"type\":\"forced_continuation_required\",\"piece_id\":\"{}\",\"current_landing\":\"{}\",\"continuation_destination_count\":{},\"explanation\":\"{}\"}}",
            piece_id.stable_id(),
            current_landing.id(),
            continuation_destination_count,
            escape_json(explanation)
        ),
        DraughtsLiteEffect::InvalidCommand {
            code,
            public_message,
            rejected_action_path,
        } => format!(
            "{{\"type\":\"invalid_command\",\"code\":\"{}\",\"public_message\":\"{}\",\"rejected_action_path\":[{}]}}",
            escape_json(code),
            escape_json(public_message),
            string_array(rejected_action_path)
        ),
        DraughtsLiteEffect::TerminalWin {
            winner,
            loser,
            reason,
        } => format!(
            "{{\"type\":\"terminal_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"reason\":\"{}\"}}",
            winner.as_str(),
            loser.as_str(),
            draughts_terminal_reason(*reason)
        ),
        DraughtsLiteEffect::BotChoseAction {
            level,
            policy_id,
            action_path,
            rationale,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_path\":[{}],\"rationale\":\"{}\"}}",
            level,
            escape_json(policy_id),
            string_array(action_path),
            escape_json(rationale)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn draughts_terminal_kind(terminal: &draughts_lite::TerminalView) -> &'static str {
    match terminal {
        draughts_lite::TerminalView::NonTerminal => "non_terminal",
        draughts_lite::TerminalView::Win { .. } => "win",
    }
}

pub(crate) fn draughts_terminal_winner(
    terminal: &draughts_lite::TerminalView,
) -> Option<DraughtsLiteSeat> {
    match terminal {
        draughts_lite::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
        _ => None,
    }
}

pub(crate) fn draughts_move_kind(kind: draughts_lite::MoveKind) -> &'static str {
    match kind {
        draughts_lite::MoveKind::Quiet => "quiet",
        draughts_lite::MoveKind::Capture => "capture",
    }
}

pub(crate) fn draughts_terminal_reason(reason: draughts_lite::TerminalWinReason) -> &'static str {
    match reason {
        draughts_lite::TerminalWinReason::OpponentNoPieces => "opponent_no_pieces",
        draughts_lite::TerminalWinReason::OpponentNoLegalMove => "opponent_no_legal_move",
    }
}

pub(crate) fn option_draughts_seat_json(seat: Option<DraughtsLiteSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}
