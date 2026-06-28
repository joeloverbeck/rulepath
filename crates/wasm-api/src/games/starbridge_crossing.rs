//! Browser-bridge helpers for `starbridge_crossing`.

use engine_core::{
    ActionPath, CommandEnvelope, FreshnessToken, RulesVersion, SeatId, Seed, Viewer,
};
use starbridge_crossing::{
    apply_jump_command, apply_pass_blocked_command, apply_step_command, legal_action_tree,
    parse_bot_action, setup_match, StarbridgeAction, StarbridgeEffect, StarbridgeEffectEnvelope,
    StarbridgeOutcomeRationaleView, StarbridgeOutcomeStandingView, StarbridgePublicView,
    StarbridgeState,
};

use crate::action_tree::action_tree_json;
use crate::commands::command_record_json;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::{AppliedCommand, ReplayRecord};

pub(crate) fn parse_starbridge_seat(value: &str) -> Result<usize, String> {
    let suffix = value.strip_prefix("seat_").ok_or_else(|| {
        diagnostic_string(
            "unknown_seat",
            &format!("unknown starbridge_crossing seat: {value}"),
        )
    })?;
    let index = suffix.parse::<usize>().map_err(|_| {
        diagnostic_string(
            "unknown_seat",
            &format!("unknown starbridge_crossing seat: {value}"),
        )
    })?;
    if index < usize::from(starbridge_crossing::STANDARD_MAX_SEATS) {
        Ok(index)
    } else {
        Err(diagnostic_string(
            "unknown_seat",
            &format!("unknown starbridge_crossing seat: {value}"),
        ))
    }
}

pub(crate) fn trace_starbridge_seat(seat: usize) -> String {
    format!("seat_{seat}")
}

pub(crate) fn starbridge_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

pub(crate) fn create_starbridge_match(
    seed: u64,
    seat_count: usize,
) -> Result<StarbridgeState, String> {
    if !starbridge_crossing::supported_seat_count(seat_count) {
        return Err(diagnostic_string(
            "invalid_seat_count",
            &format!("starbridge_crossing supports exactly 2, 3, 4, or 6 seats; got {seat_count}"),
        ));
    }
    setup_match(
        Seed(seed),
        &starbridge_seats_for_count(seat_count),
        &starbridge_crossing::SetupOptions::default(),
    )
    .map_err(diagnostic_json)
}

pub(crate) fn starbridge_viewer_for_seat(
    state: &StarbridgeState,
    viewer_seat: Option<&str>,
) -> Result<Viewer, String> {
    match viewer_seat {
        None => Ok(Viewer { seat_id: None }),
        Some(value) => {
            let seat = parse_starbridge_seat(value)?;
            state
                .seats
                .get(seat)
                .map(|assignment| Viewer {
                    seat_id: Some(assignment.seat_id.clone()),
                })
                .ok_or_else(|| {
                    diagnostic_string(
                        "unknown_seat",
                        &format!("seat not present: {}", trace_starbridge_seat(seat)),
                    )
                })
        }
    }
}

pub(crate) fn starbridge_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    seat: usize,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_starbridge_seat)
        .transpose()
        .map(|viewer| viewer == Some(seat))
}

pub(crate) fn starbridge_actor_for_seat(
    state: &StarbridgeState,
    seat: usize,
) -> Result<engine_core::Actor, String> {
    state
        .seats
        .get(seat)
        .map(|assignment| engine_core::Actor {
            seat_id: assignment.seat_id.clone(),
        })
        .ok_or_else(|| {
            diagnostic_string(
                "unknown_seat",
                &format!("seat not present: {}", trace_starbridge_seat(seat)),
            )
        })
}

pub(crate) fn starbridge_action_tree_json(
    state: &StarbridgeState,
    seat: usize,
) -> Result<String, String> {
    let actor = starbridge_actor_for_seat(state, seat)?;
    Ok(action_tree_json(&legal_action_tree(state, &actor)))
}

pub(crate) fn starbridge_apply_command(
    state: &mut StarbridgeState,
    seat: usize,
    action_path: ActionPath,
    freshness_token: u64,
) -> Result<Vec<StarbridgeEffectEnvelope>, String> {
    let actor = starbridge_actor_for_seat(state, seat)?;
    let command = CommandEnvelope {
        actor,
        action_path: action_path.clone(),
        freshness_token: FreshnessToken(freshness_token),
        rules_version: RulesVersion(RULES_VERSION),
    };
    match parse_bot_action(&action_path).map_err(diagnostic_json)? {
        StarbridgeAction::Step { .. } => {
            apply_step_command(state, &command).map_err(diagnostic_json)
        }
        StarbridgeAction::Jump { .. } => {
            apply_jump_command(state, &command).map_err(diagnostic_json)
        }
        StarbridgeAction::PassBlocked => {
            apply_pass_blocked_command(state, &command).map_err(diagnostic_json)
        }
    }
}

pub(crate) fn starbridge_replay_to_cursor(
    seed: u64,
    seat_count: usize,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(StarbridgeState, Vec<StarbridgeEffectEnvelope>), String> {
    let mut state = create_starbridge_match(seed, seat_count)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_starbridge_seat(&command.actor_seat)?;
        let effects = starbridge_apply_command(
            &mut state,
            seat,
            ActionPath {
                segments: command.action_path.clone(),
            },
            command.freshness_token,
        )?;
        all_effects.extend(effects);
    }
    Ok((state, all_effects))
}

pub(crate) fn starbridge_view_json(view: &StarbridgePublicView, freshness_token: u64) -> String {
    let spaces = view
        .spaces
        .iter()
        .map(|space| {
            let occupant = space.occupant.as_ref().map_or_else(
                || "null".to_owned(),
                |peg| {
                    format!(
                        "{{\"peg\":\"{}\",\"owner_seat\":\"seat_{}\",\"owner_seat_index\":{}}}",
                        escape_json(&peg.peg),
                        peg.owner_seat_index,
                        peg.owner_seat_index
                    )
                },
            );
            format!(
                "{{\"space\":\"{}\",\"coord\":{{\"q\":{},\"r\":{},\"s\":{}}},\"zone\":\"{}\",\"occupant\":{},\"ui\":{{\"coordinate_label\":\"{}\",\"zone_label\":\"{}\",\"anchor\":{{\"x\":{},\"y\":{}}}}}}}",
                escape_json(&space.space),
                space.coord.0,
                space.coord.1,
                space.coord.2,
                escape_json(&space.zone),
                occupant,
                escape_json(&space.ui.coordinate_label),
                escape_json(&space.ui.zone_label),
                space.ui.anchor.x,
                space.ui.anchor.y
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let seats = view
        .seats
        .iter()
        .map(|seat| {
            let finish_rank = seat
                .finish_rank
                .map_or_else(|| "null".to_owned(), |rank| rank.to_string());
            format!(
                "{{\"seat_id\":\"{}\",\"seat_index\":{},\"home\":\"{}\",\"target\":\"{}\",\"finish_rank\":{}}}",
                escape_json(&seat.seat_id),
                seat.seat_index,
                escape_json(&seat.home),
                escape_json(&seat.target),
                finish_rank
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let finish_ranks = view
        .finish_ranks
        .iter()
        .map(|rank| {
            format!(
                "{{\"seat\":\"seat_{}\",\"seat_index\":{},\"rank\":{}}}",
                rank.seat_index, rank.seat_index, rank.rank
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let active_seat = view.active_seat.as_ref().map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", escape_json(seat)),
    );
    let terminal = view.terminal.as_ref().map_or_else(
        || "null".to_owned(),
        |value| format!("\"{}\"", escape_json(value)),
    );
    let terminal_rationale = view
        .terminal_rationale
        .as_ref()
        .map_or_else(|| "null".to_owned(), starbridge_rationale_json);
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"data_version_label\":\"{}\",\"freshness_token\":{},\"active_seat\":{},\"terminal\":{},\"terminal_rationale\":{},\"ply_count\":{},\"command_count\":{},\"spaces\":[{}],\"seats\":[{}],\"finish_ranks\":[{}],\"audit\":{{\"redaction_class\":\"{}\",\"private_fields\":[],\"rationale\":\"{}\"}}}}",
        escape_json(&view.game_id),
        escape_json(GAME_STARBRIDGE_CROSSING_DISPLAY_NAME),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        escape_json(&view.data_version_label),
        freshness_token,
        active_seat,
        terminal,
        terminal_rationale,
        view.ply_count,
        view.command_count,
        spaces,
        seats,
        finish_ranks,
        escape_json(&view.audit.redaction_class),
        escape_json(&view.audit.rationale)
    )
}

fn starbridge_rationale_json(rationale: &StarbridgeOutcomeRationaleView) -> String {
    let decisive_rule_ids = rationale
        .decisive_rule_ids
        .iter()
        .map(|rule_id| format!("\"{}\"", escape_json(rule_id)))
        .collect::<Vec<_>>()
        .join(",");
    let final_standing = rationale
        .final_standing
        .iter()
        .map(starbridge_rationale_standing_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"headline\":null,\"decisive_comparison\":null,\"comparison_basis\":null,\"decisive_rule_ids\":[{}],\"final_standing\":[{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        decisive_rule_ids,
        final_standing
    )
}

fn starbridge_rationale_standing_json(standing: &StarbridgeOutcomeStandingView) -> String {
    let mut values = Vec::new();
    if let Some(rank) = standing.finish_rank {
        values.push(format!("{{\"label\":\"Rank\",\"value\":{rank}}}"));
    }
    values.push(format!(
        "{{\"label\":\"Finished\",\"value\":\"{}\"}}",
        if standing.finished { "yes" } else { "no" }
    ));
    if let Some(progress) = standing.progress {
        values.push(format!("{{\"label\":\"Progress\",\"value\":{progress}}}"));
    }

    format!(
        "{{\"id\":\"{}\",\"label\":\"Seat {}\",\"result\":\"{}\",\"emphasized\":{},\"strength\":null,\"values\":[{}]}}",
        escape_json(&standing.seat.0),
        u16::from(standing.seat_index) + 1,
        if standing.winner { "win" } else { "ranked" },
        standing.winner,
        values.join(",")
    )
}

pub(crate) fn starbridge_effects_json(effects: &[StarbridgeEffectEnvelope]) -> String {
    format!(
        "[{}]",
        effects
            .iter()
            .map(starbridge_effect_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn starbridge_effect_json(effect: &StarbridgeEffectEnvelope) -> String {
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        crate::visibility_json(&effect.visibility),
        starbridge_effect_payload_json(&effect.payload)
    )
}

fn starbridge_effect_payload_json(effect: &StarbridgeEffect) -> String {
    match effect {
        StarbridgeEffect::Step {
            seat_index,
            peg,
            from,
            to,
        } => format!(
            "{{\"kind\":\"step\",\"seat\":\"seat_{}\",\"seat_index\":{},\"peg\":\"{}\",\"from\":\"{}\",\"to\":\"{}\"}}",
            seat_index,
            seat_index,
            escape_json(&peg.stable_id()),
            from,
            to
        ),
        StarbridgeEffect::JumpChain {
            seat_index,
            peg,
            from,
            hops,
        } => {
            let hops = hops
                .iter()
                .map(|hop| format!("{{\"over\":\"{}\",\"to\":\"{}\"}}", hop.over, hop.to))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"kind\":\"jump_chain\",\"seat\":\"seat_{}\",\"seat_index\":{},\"peg\":\"{}\",\"from\":\"{}\",\"hops\":[{}]}}",
                seat_index,
                seat_index,
                escape_json(&peg.stable_id()),
                from,
                hops
            )
        }
        StarbridgeEffect::FinishAssigned { seat_index, rank } => format!(
            "{{\"kind\":\"finish_assigned\",\"seat\":\"seat_{}\",\"seat_index\":{},\"rank\":{}}}",
            seat_index, seat_index, rank
        ),
        StarbridgeEffect::PassBlocked { seat_index } => format!(
            "{{\"kind\":\"pass_blocked\",\"seat\":\"seat_{}\",\"seat_index\":{}}}",
            seat_index, seat_index
        ),
        StarbridgeEffect::Terminal { reason } => format!(
            "{{\"kind\":\"terminal\",\"reason\":\"{}\"}}",
            escape_json(reason)
        ),
    }
}

pub(crate) fn starbridge_replay_document_json(
    trace_id: &str,
    seed: u64,
    seat_count: usize,
    commands: &[AppliedCommand],
) -> String {
    let command_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let seats = (0..seat_count)
        .map(|seat| format!("\"{}\"", trace_starbridge_seat(seat)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"public-export\",\"purpose\":\"wasm public replay export\",\"note\":\"Starbridge Crossing all-public wasm export\",\"migration_update_note\":\"No migration authorized\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"seats\":[{}],\"commands\":[{}],\"not_applicable\":{{\"hidden_information\":\"Starbridge Crossing has no private game facts\",\"stochastic_game_events\":\"Setup is deterministic from seed and variant\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        GAME_STARBRIDGE_CROSSING,
        STARBRIDGE_CROSSING_TRACE_RULES_VERSION,
        ENGINE_VERSION,
        DATA_VERSION,
        seed,
        VARIANT_STARBRIDGE_CROSSING_STANDARD,
        seats,
        command_json
    )
}

pub(crate) fn starbridge_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &StarbridgeState,
    effects: &[StarbridgeEffectEnvelope],
) -> String {
    let view = starbridge_crossing::project_view(state, &Viewer { seat_id: None });
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        starbridge_view_json(&view, state.freshness_token.0),
        starbridge_effects_json(effects)
    )
}

pub(crate) fn starbridge_replay_seat_count(record: &ReplayRecord) -> usize {
    record
        .commands
        .iter()
        .filter_map(|command| parse_starbridge_seat(&command.actor_seat).ok())
        .max()
        .map_or(
            usize::from(starbridge_crossing::STANDARD_DEFAULT_SEATS),
            |seat| seat + 1,
        )
        .max(usize::from(starbridge_crossing::STANDARD_DEFAULT_SEATS))
}
