//! Browser-bridge helpers for `token_bazaar` (perfect-information economy game).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use token_bazaar::{
    apply_action as token_apply_action, project_view as token_project_view,
    replay_support::replay_commands as token_replay_commands, setup_match as token_setup_match,
    TokenBazaarEffect, TokenBazaarSeat, TokenBazaarState,
};

use crate::actors::token_actor_for_seat;
use crate::commands::command_record_json;
use crate::constants::*;
use crate::json::{diagnostic_json, escape_json};
use crate::seats::{parse_token_seat, seats};
use crate::{string_array, visibility_json, AppliedCommand};

pub(crate) fn token_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(TokenBazaarState, Vec<EffectEnvelope<TokenBazaarEffect>>), String> {
    let seats = seats();
    let mut state = token_setup_match(Seed(seed), &seats, &token_bazaar::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_token_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: token_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = token_bazaar::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(token_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

pub(crate) fn token_effects_json(effects: &[EffectEnvelope<TokenBazaarEffect>]) -> String {
    let body = effects
        .iter()
        .map(token_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn token_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let command_paths = commands
        .iter()
        .map(|command| command.action_path.clone())
        .collect::<Vec<_>>();
    let hashes = token_replay_commands(seed, &command_paths);
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
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
    let (terminal, winner, draw) = match hashes.terminal_outcome {
        Some(token_bazaar::TerminalOutcome::Win { seat }) => {
            (true, format!("\"{}\"", seat.as_str()), false)
        }
        Some(token_bazaar::TerminalOutcome::Draw) => (true, "null".to_owned(), true),
        None => (false, "null".to_owned(), false),
    };

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"token-bazaar-{}\",\"fixture_kind\":\"wasm\",\"purpose\":\"public_export_round_trip\",\"note\":\"Token Bazaar public_export_round_trip replay fixture.\",\"migration_update_note\":\"Updated expected hashes for VICEXPSHASUR-007 tiebreak rationale projection.\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat-0\",\"player_id\":\"player-0\"}},{{\"seat_id\":\"seat-1\",\"player_id\":\"player-1\"}}],\"commands\":[{}],\"checkpoints\":{},\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"all\":{}}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_diagnostic_hashes\":null,\"expected_public_export_hashes\":{{\"final\":{}}},\"expected_outcome\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"not_applicable\":{{\"hidden_information\":\"token_bazaar is fully public.\",\"stochastic_game_events\":\"token_bazaar game rules use no randomness.\",\"private_view_hashes\":\"token_bazaar observer and seat views are identical.\",\"preview_hashes\":\"token_bazaar uses legal action metadata rather than a separate preview hash.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_TOKEN_BAZAAR),
        escape_json(TOKEN_BAZAAR_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_TOKEN_BAZAAR_STANDARD),
        commands_json,
        checkpoints,
        hashes.final_state_hash.0,
        hashes.effect_hash.0,
        hashes.action_tree_hash.0,
        hashes.public_view_hash.0,
        hashes.replay_hash.0,
        hashes.replay_hash.0,
        terminal,
        winner,
        draw,
        terminal,
        winner,
        draw
    ))
}

pub(crate) fn token_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &TokenBazaarState,
    effects: &[EffectEnvelope<TokenBazaarEffect>],
) -> String {
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        token_view_json(&token_project_view(state, &Viewer { seat_id: None })),
        token_effects_json(effects)
    )
}

pub(crate) fn token_view_json(view: &token_bazaar::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"supply\":{},\"inventories\":[{},{}],\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"turns_taken\":{{\"seat_0\":{},\"seat_1\":{},\"turns_per_seat\":{}}},\"active_seat\":{},\"market_slots\":[{}],\"queue_remaining\":{},\"fulfilled\":{{\"seat_0\":[{}],\"seat_1\":[{}]}},\"legal_actions\":[{}],\"terminal\":{},\"freshness_token\":{},\"recent_effects\":[{}],\"private_view_status\":\"{}\",\"hidden_fields\":[{}],\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        token_supply_json(view.supply),
        token_inventory_json(&view.inventories[0]),
        token_inventory_json(&view.inventories[1]),
        view.scores[0],
        view.scores[1],
        view.turns_taken[0],
        view.turns_taken[1],
        view.turns_per_seat,
        option_token_seat_json(view.active_seat),
        view.market_slots
            .iter()
            .map(token_market_slot_json)
            .collect::<Vec<_>>()
            .join(","),
        view.queue_remaining,
        string_array(&view.fulfilled[0]),
        string_array(&view.fulfilled[1]),
        view.legal_actions
            .iter()
            .map(token_legal_action_json)
            .collect::<Vec<_>>()
            .join(","),
        token_terminal_json(&view.terminal),
        view.freshness_token.0,
        view.recent_effects
            .iter()
            .map(token_effect_view_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&view.private_view_status),
        string_array(&view.hidden_fields),
        token_ui_json(&view.ui)
    )
}

pub(crate) fn token_supply_json(supply: token_bazaar::ResourceSupplyView) -> String {
    format!(
        "{{\"amber\":{},\"jade\":{},\"iron\":{}}}",
        supply.amber, supply.jade, supply.iron
    )
}

pub(crate) fn token_inventory_json(inventory: &token_bazaar::InventoryView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"resources\":{}}}",
        inventory.seat.as_str(),
        token_supply_json(inventory.resources)
    )
}

pub(crate) fn token_market_slot_json(slot: &token_bazaar::MarketSlotView) -> String {
    format!(
        "{{\"slot\":\"{}\",\"slot_id\":\"{}\",\"contract\":{},\"is_empty\":{},\"accessibility_label\":\"{}\"}}",
        slot.slot.as_str(),
        escape_json(&slot.slot_id),
        slot.contract
            .as_ref()
            .map_or_else(|| "null".to_owned(), token_contract_json),
        slot.is_empty,
        escape_json(&slot.accessibility_label)
    )
}

pub(crate) fn token_contract_json(contract: &token_bazaar::ContractView) -> String {
    format!(
        "{{\"contract_id\":\"{}\",\"label\":\"{}\",\"cost\":{},\"points\":{},\"accessibility_label\":\"{}\"}}",
        escape_json(&contract.contract_id),
        escape_json(&contract.label),
        token_supply_json(contract.cost),
        contract.points,
        escape_json(&contract.accessibility_label)
    )
}

pub(crate) fn token_legal_action_json(action: &token_bazaar::LegalActionView) -> String {
    let metadata = action
        .metadata
        .iter()
        .map(|(key, value)| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                escape_json(key),
                escape_json(value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"action_segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[{}],\"freshness_token\":{}}}",
        escape_json(&action.action_segment),
        escape_json(&action.label),
        escape_json(&action.accessibility_label),
        metadata,
        action.freshness_token.0
    )
}

pub(crate) fn token_terminal_json(terminal: &token_bazaar::TerminalView) -> String {
    match terminal {
        token_bazaar::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"winner\":null,\"draw\":false}".to_owned()
        }
        token_bazaar::TerminalView::Win { winning_seat, .. } => format!(
            "{{\"terminal\":true,\"winner\":\"{}\",\"draw\":false}}",
            winning_seat.as_str()
        ),
        token_bazaar::TerminalView::Draw { .. } => {
            "{\"terminal\":true,\"winner\":null,\"draw\":true}".to_owned()
        }
    }
}

pub(crate) fn token_effect_view_json(effect: &token_bazaar::EffectView) -> String {
    format!(
        "{{\"kind\":\"{}\",\"summary\":\"{}\"}}",
        escape_json(&effect.kind),
        escape_json(&effect.summary)
    )
}

pub(crate) fn token_ui_json(ui: &token_bazaar::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"supply_label\":\"{}\",\"inventory_label\":\"{}\",\"market_label\":\"{}\",\"score_label\":\"{}\",\"turn_counter_label\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(&ui.table_label),
        escape_json(&ui.supply_label),
        escape_json(&ui.inventory_label),
        escape_json(&ui.market_label),
        escape_json(&ui.score_label),
        escape_json(&ui.turn_counter_label),
        escape_json(&ui.reduced_motion_token)
    )
}

pub(crate) fn token_effect_json(effect: &EffectEnvelope<TokenBazaarEffect>) -> String {
    let payload = match &effect.payload {
        TokenBazaarEffect::ResourceCollected {
            seat,
            bundle,
            gain,
            inventory_after,
            supply_after,
        } => format!(
            "{{\"type\":\"resource_collected\",\"seat\":\"{}\",\"bundle\":\"{}\",\"gain\":{},\"inventory_after\":{},\"supply_after\":{}}}",
            seat.as_str(),
            bundle.as_str(),
            token_counts_json(*gain),
            token_counts_json(*inventory_after),
            token_counts_json(*supply_after)
        ),
        TokenBazaarEffect::ResourceExchanged {
            seat,
            paid_resource,
            taken_resource,
            cost,
            gain,
            inventory_after,
            supply_after,
        } => format!(
            "{{\"type\":\"resource_exchanged\",\"seat\":\"{}\",\"paid_resource\":\"{}\",\"taken_resource\":\"{}\",\"cost\":{},\"gain\":{},\"inventory_after\":{},\"supply_after\":{}}}",
            seat.as_str(),
            paid_resource.as_str(),
            taken_resource.as_str(),
            token_counts_json(*cost),
            token_counts_json(*gain),
            token_counts_json(*inventory_after),
            token_counts_json(*supply_after)
        ),
        TokenBazaarEffect::ContractFulfilled {
            seat,
            slot,
            contract,
            cost,
            points,
            score_after,
            fulfilled_count_after,
        } => format!(
            "{{\"type\":\"contract_fulfilled\",\"seat\":\"{}\",\"slot\":\"{}\",\"contract\":\"{}\",\"cost\":{},\"points\":{},\"score_after\":{},\"fulfilled_count_after\":{}}}",
            seat.as_str(),
            slot.as_str(),
            contract.as_str(),
            token_counts_json(*cost),
            points,
            score_after,
            fulfilled_count_after
        ),
        TokenBazaarEffect::SlotRefilled {
            slot,
            contract,
            remaining_queue_len,
        } => format!(
            "{{\"type\":\"slot_refilled\",\"slot\":\"{}\",\"contract\":\"{}\",\"remaining_queue_len\":{}}}",
            slot.as_str(),
            contract.as_str(),
            remaining_queue_len
        ),
        TokenBazaarEffect::SlotEmptied {
            slot,
            remaining_queue_len,
        } => format!(
            "{{\"type\":\"slot_emptied\",\"slot\":\"{}\",\"remaining_queue_len\":{}}}",
            slot.as_str(),
            remaining_queue_len
        ),
        TokenBazaarEffect::PassAccepted { seat } => {
            format!("{{\"type\":\"pass_accepted\",\"seat\":\"{}\"}}", seat.as_str())
        }
        TokenBazaarEffect::TurnAdvanced {
            previous_seat,
            active_seat,
            turns_taken,
        } => format!(
            "{{\"type\":\"turn_advanced\",\"previous_seat\":\"{}\",\"active_seat\":\"{}\",\"turns_taken\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            previous_seat.as_str(),
            active_seat.as_str(),
            turns_taken[0],
            turns_taken[1]
        ),
        TokenBazaarEffect::Terminal {
            outcome,
            scores,
            fulfilled_counts,
            inventory_totals,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"scores\":{{\"seat_0\":{},\"seat_1\":{}}},\"fulfilled_counts\":{{\"seat_0\":{},\"seat_1\":{}}},\"inventory_totals\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            token_terminal_outcome_json(*outcome),
            scores[0],
            scores[1],
            fulfilled_counts[0],
            fulfilled_counts[1],
            inventory_totals[0],
            inventory_totals[1]
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn token_counts_json(counts: token_bazaar::ResourceCounts) -> String {
    format!(
        "{{\"amber\":{},\"jade\":{},\"iron\":{}}}",
        counts.amber, counts.jade, counts.iron
    )
}

pub(crate) fn token_terminal_outcome_json(outcome: token_bazaar::TerminalOutcome) -> String {
    match outcome {
        token_bazaar::TerminalOutcome::Win { seat } => {
            format!("{{\"kind\":\"win\",\"winner\":\"{}\"}}", seat.as_str())
        }
        token_bazaar::TerminalOutcome::Draw => "{\"kind\":\"draw\",\"winner\":null}".to_owned(),
    }
}

pub(crate) fn option_token_seat_json(seat: Option<TokenBazaarSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}
