//! Browser-bridge helpers for `three_marks` (perfect-information mark game).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use three_marks::{
    apply_action as three_apply_action, project_view as three_project_view,
    replay_support::replay_commands as three_replay_commands, setup_match as three_setup_match,
    ThreeMarksEffect, ThreeMarksState,
};

use crate::actors::three_actor_for_seat;
use crate::commands::{command_record_json, single_segment_commands};
use crate::constants::*;
use crate::json::{diagnostic_json, escape_json};
use crate::seats::{canonical_seats_for_count, canonical_trace_seat_id, parse_three_seat, seats};
use crate::{visibility_json, AppliedCommand};

pub(crate) fn three_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(ThreeMarksState, Vec<EffectEnvelope<ThreeMarksEffect>>), String> {
    let seats = seats();
    let mut state = three_setup_match(Seed(seed), &seats, &three_marks::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_three_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: three_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = three_marks::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(three_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

pub(crate) fn three_effects_json(effects: &[EffectEnvelope<ThreeMarksEffect>]) -> String {
    let body = effects
        .iter()
        .map(three_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn three_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_segments = single_segment_commands(commands)?;
    let hashes = three_replay_commands(seed, &command_segments);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| three_command_record_json(index, command))
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
            three_marks::TerminalOutcome::Draw => {
                "{\"terminal\":true,\"winner\":null,\"kind\":\"draw\"}".to_owned()
            }
            three_marks::TerminalOutcome::Win { seat, line } => format!(
                "{{\"terminal\":true,\"winner\":\"{}\",\"kind\":\"win\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
                canonical_trace_seat_id(seat.index() as u32),
                line.cells[0].as_str(),
                line.cells[1].as_str(),
                line.cells[2].as_str()
            ),
        },
    );

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"Updated public view hash for VICEXPSHASUR-004 outcome rationale projection.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_private_view_hashes\":{{\"not_applicable\":\"three_marks is perfect-information and has no private-view API.\"}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_outcome\":{},\"expected_terminal_state\":{},\"not_applicable\":{{\"hidden_information\":\"three_marks is perfect-information and has no hidden state to redact.\",\"stochastic_game_events\":\"three_marks game rules use no randomness; bot RNG is not replayed from exported documents because resolved commands are recorded.\",\"private_view_hashes\":\"three_marks has no private-view API.\",\"preview_hashes\":\"three_marks has no Rust preview surface in Gate 4.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_THREE_MARKS),
        escape_json(THREE_MARKS_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_THREE_MARKS_STANDARD),
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

fn three_command_record_json(index: usize, command: &AppliedCommand) -> Result<String, String> {
    let seat = parse_three_seat(&command.actor_seat)?;
    Ok(command_record_json(
        index,
        &AppliedCommand {
            actor_seat: canonical_trace_seat_id(seat.index() as u32),
            action_path: command.action_path.clone(),
            freshness_token: command.freshness_token,
        },
    ))
}

pub(crate) fn three_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &ThreeMarksState,
    effects: &[EffectEnvelope<ThreeMarksEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        three_project_view(state, &Viewer { seat_id: None }).to_json(),
        three_effects_json(effects)
    )
}

pub(crate) fn three_effect_json(effect: &EffectEnvelope<ThreeMarksEffect>) -> String {
    let payload = match &effect.payload {
        ThreeMarksEffect::SetupComplete {
            game_id,
            variant_id,
            rules_version,
            seats,
        } => format!(
            "{{\"type\":\"setup_complete\",\"game_id\":\"{}\",\"variant_id\":\"{}\",\"rules_version\":\"{}\",\"seats\":[\"{}\",\"{}\"]}}",
            escape_json(game_id),
            escape_json(variant_id),
            escape_json(rules_version),
            escape_json(&seats[0]),
            escape_json(&seats[1])
        ),
        ThreeMarksEffect::MarkPlaced {
            seat,
            cell,
            ply,
            occupancy_summary,
        } => format!(
            "{{\"type\":\"mark_placed\",\"seat\":\"{}\",\"cell\":\"{}\",\"ply\":{},\"occupancy_summary\":\"{}\"}}",
            seat.as_str(),
            cell.as_str(),
            ply,
            escape_json(occupancy_summary)
        ),
        ThreeMarksEffect::ActivePlayerChanged {
            previous_seat,
            active_seat,
            ply,
        } => format!(
            "{{\"type\":\"active_player_changed\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"ply\":{}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            ply
        ),
        ThreeMarksEffect::PlacementRejected { reason, label } => format!(
            "{{\"type\":\"placement_rejected\",\"reason\":\"{}\",\"label\":\"{}\"}}",
            rejection_reason_json(*reason),
            escape_json(label)
        ),
        ThreeMarksEffect::LineCompleted { winning_seat, line } => format!(
            "{{\"type\":\"line_completed\",\"winning_seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
            winning_seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
        ThreeMarksEffect::DrawReached {
            final_ply,
            full_board,
        } => format!(
            "{{\"type\":\"draw_reached\",\"final_ply\":{},\"full_board\":{}}}",
            final_ply, full_board
        ),
        ThreeMarksEffect::GameEnded {
            outcome,
            final_ply,
            terminal_hash_ref,
        } => format!(
            "{{\"type\":\"game_ended\",\"outcome\":{},\"final_ply\":{},\"terminal_hash_ref\":\"{}\"}}",
            terminal_outcome_json(*outcome),
            final_ply,
            escape_json(terminal_hash_ref)
        ),
        ThreeMarksEffect::BotChoseAction {
            level,
            policy_id,
            action_id,
            cell,
            explanation,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"level\":{},\"policy_id\":\"{}\",\"action_id\":\"{}\",\"cell\":\"{}\",\"explanation\":\"{}\"}}",
            level,
            escape_json(policy_id),
            escape_json(action_id),
            cell.as_str(),
            escape_json(explanation)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

fn terminal_outcome_json(outcome: three_marks::TerminalOutcome) -> String {
    match outcome {
        three_marks::TerminalOutcome::Draw => "{\"kind\":\"draw\"}".to_owned(),
        three_marks::TerminalOutcome::Win { seat, line } => format!(
            "{{\"kind\":\"win\",\"seat\":\"{}\",\"line\":[\"{}\",\"{}\",\"{}\"]}}",
            seat.as_str(),
            line.cells[0].as_str(),
            line.cells[1].as_str(),
            line.cells[2].as_str()
        ),
    }
}

fn rejection_reason_json(reason: three_marks::RejectionReason) -> &'static str {
    match reason {
        three_marks::RejectionReason::Occupied => "occupied",
        three_marks::RejectionReason::Stale => "stale",
        three_marks::RejectionReason::InvalidCell => "invalid_cell",
        three_marks::RejectionReason::WrongActor => "wrong_actor",
        three_marks::RejectionReason::Terminal => "terminal",
        three_marks::RejectionReason::UnknownActor => "unknown_actor",
        three_marks::RejectionReason::InvalidPath => "invalid_path",
        three_marks::RejectionReason::InvalidAction => "invalid_action",
    }
}
