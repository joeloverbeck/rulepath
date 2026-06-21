//! Browser-bridge helpers for `briar_circuit` (hidden-information four-seat trick taking).

use engine_core::{ActionPath, Actor, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};

use briar_circuit::{
    apply_pass_action, apply_play_action, canonical_seat_ids, effect_envelopes,
    filter_effects_for_viewer, legal_bot_actions, parse_pass_action_path, parse_play_action_path,
    project_view as briar_project_view, setup_match as briar_setup_match, validate_pass_command,
    validate_play_command, BriarCircuitBotAction, BriarCircuitEffect, BriarCircuitL1Bot,
    BriarCircuitSeat, BriarCircuitState, CardId, PassAction, PlayAction, SetupOptions,
};

use crate::constants::*;
use crate::json::{diagnostic_json, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{parse_public_replay_steps, PublicTimelineReplay};
use crate::store::{next_replay_id, REPLAYS};
use crate::{option_bool_json, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn parse_briar_seat(value: &str) -> Result<BriarCircuitSeat, String> {
    match value {
        "seat-0" => Ok(BriarCircuitSeat::Seat0),
        "seat-1" => Ok(BriarCircuitSeat::Seat1),
        "seat-2" => Ok(BriarCircuitSeat::Seat2),
        "seat-3" => Ok(BriarCircuitSeat::Seat3),
        _ => BriarCircuitSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_briar_seat(seat: BriarCircuitSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn briar_actor_for_seat(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"seat not present: {}\"}}",
                seat.as_str()
            )
        })
}

pub(crate) fn briar_viewer_for_seat(
    _state: &BriarCircuitState,
    viewer_seat: Option<&str>,
) -> Result<Viewer, String> {
    match viewer_seat {
        None => Ok(Viewer { seat_id: None }),
        Some(value) => {
            let seat = parse_briar_seat(value)?;
            Ok(Viewer {
                seat_id: Some(engine_core::SeatId(seat.as_str().to_owned())),
            })
        }
    }
}

pub(crate) fn briar_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    seat: BriarCircuitSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_briar_seat)
        .transpose()
        .map(|viewer| viewer == Some(seat))
}

pub(crate) fn create_briar_circuit_match(
    seed: u64,
    seat_count: usize,
) -> Result<BriarCircuitState, String> {
    if seat_count != briar_circuit::STANDARD_SEAT_COUNT as usize {
        return Err(format!(
            "{{\"code\":\"unsupported_seat_count\",\"message\":\"briar_circuit requires exactly {} seats\"}}",
            briar_circuit::STANDARD_SEAT_COUNT
        ));
    }
    briar_setup_match(Seed(seed), &canonical_seat_ids(), &SetupOptions::default())
        .map_err(diagnostic_json)
}

pub(crate) fn briar_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(BriarCircuitState, Vec<EffectEnvelope<BriarCircuitEffect>>), String> {
    let mut state = create_briar_circuit_match(seed, briar_circuit::STANDARD_SEAT_COUNT as usize)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_briar_seat(&command.actor_seat)?;
        let applied = briar_apply_command(
            &mut state,
            seat,
            ActionPath {
                segments: command.action_path.clone(),
            },
            command.freshness_token,
        )?;
        all_effects.extend(applied);
    }
    Ok((state, all_effects))
}

pub(crate) fn briar_apply_command(
    state: &mut BriarCircuitState,
    seat: BriarCircuitSeat,
    action_path: ActionPath,
    freshness_token: u64,
) -> Result<Vec<EffectEnvelope<BriarCircuitEffect>>, String> {
    let command = CommandEnvelope {
        actor: briar_actor_for_seat(state, seat)?,
        action_path,
        freshness_token: engine_core::FreshnessToken(freshness_token),
        rules_version: RulesVersion(RULES_VERSION),
    };

    let effects =
        match command.action_path.segments.first().map(String::as_str) {
            Some(briar_circuit::ACTION_PASS) => {
                let (validated_seat, action) =
                    validate_pass_command(state, &command).map_err(diagnostic_json)?;
                apply_pass_action(state, validated_seat, action)
                    .map_err(diagnostic_json)?
                    .effects
            }
            Some(briar_circuit::ACTION_PLAY) => {
                let (validated_seat, action) =
                    validate_play_command(state, &command).map_err(diagnostic_json)?;
                apply_play_action(state, validated_seat, action)
                    .map_err(diagnostic_json)?
                    .effects
            }
            _ => return Err(
                "{\"code\":\"unknown_action\",\"message\":\"unknown briar_circuit action family\"}"
                    .to_owned(),
            ),
        };

    Ok(effects.into_iter().flat_map(effect_envelopes).collect())
}

pub(crate) fn briar_action_tree_json(state: &BriarCircuitState, seat: BriarCircuitSeat) -> String {
    let choices = legal_bot_actions(state, seat)
        .unwrap_or_default()
        .into_iter()
        .map(briar_action_choice_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"freshness_token\":{},\"choices\":[{}]}}",
        state.freshness_token.0, choices
    )
}

pub(crate) fn briar_view_json(
    view: &briar_circuit::BriarCircuitView,
    freshness_token: u64,
) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"viewer_seat\":{},\"phase\":\"{}\",\"dealer\":\"{}\",\"hand_index\":{},\"cumulative_scores\":{},\"hand_counts\":{},\"own_hand\":[{}],\"pass\":{},\"active_seat\":{},\"hearts_broken\":{},\"current_trick\":[{}],\"captured_tricks\":[{}],\"last_hand_summary\":{},\"freshness_token\":{},\"private_view_status\":\"{}\",\"hidden_fields\":[\"opponent_hands\",\"pass_provenance\",\"deck_order\"],\"ui\":{}}}",
        SCHEMA_VERSION,
        RULES_VERSION,
        escape_json(GAME_BRIAR_CIRCUIT),
        escape_json(GAME_BRIAR_CIRCUIT_DISPLAY_NAME),
        escape_json(VARIANT_BRIAR_CIRCUIT_STANDARD),
        escape_json(briar_circuit::RULES_VERSION_LABEL),
        option_briar_seat_json(view.viewer_seat),
        escape_json(&view.phase),
        view.dealer.as_str(),
        view.hand_index,
        briar_scores_json(view.cumulative_scores),
        briar_hand_counts_json(&view.hand_counts),
        view.own_hand
            .iter()
            .map(briar_card_json)
            .collect::<Vec<_>>()
            .join(","),
        view.pass
            .as_ref()
            .map_or_else(|| "null".to_owned(), briar_pass_json),
        option_briar_seat_json(view.active_seat),
        option_bool_json(view.hearts_broken),
        view.current_trick
            .iter()
            .map(briar_trick_play_json)
            .collect::<Vec<_>>()
            .join(","),
        view.captured_tricks
            .iter()
            .map(briar_captured_trick_json)
            .collect::<Vec<_>>()
            .join(","),
        view.last_hand_summary
            .as_ref()
            .map_or_else(|| "null".to_owned(), briar_hand_summary_json),
        freshness_token,
        if view.viewer_seat.is_some() { "seat" } else { "observer" },
        briar_ui_json()
    )
}

pub(crate) fn briar_effects_json(
    effects: &[EffectEnvelope<BriarCircuitEffect>],
    viewer: &Viewer,
) -> String {
    let visible = filter_effects_for_viewer(effects, viewer);
    let body = visible
        .iter()
        .map(briar_visible_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn briar_logged_effect_json(effect: &EffectEnvelope<BriarCircuitEffect>) -> String {
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        briar_effect_payload_json(&effect.payload)
    )
}

pub(crate) fn briar_replay_document_json(
    _replay_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let (state, effects) = briar_replay_to_cursor(seed, commands, commands.len())?;
    let viewer = Viewer { seat_id: None };
    let public_effects = filter_effects_for_viewer(&effects, &viewer)
        .iter()
        .map(briar_public_effect_text)
        .collect::<Vec<_>>();
    Ok(format!(
        "{{\"export_class\":\"viewer_scoped_observation_v1\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"viewer\":\"observer\",\"steps\":[{{\"step_index\":0,\"public_view_summary\":\"{}\",\"public_effects\":[{}],\"redacted_command_summary\":\"{}\",\"terminal\":{}}}]}}",
        escape_json(GAME_BRIAR_CIRCUIT),
        escape_json(BRIAR_CIRCUIT_TRACE_RULES_VERSION),
        escape_json(VARIANT_BRIAR_CIRCUIT_STANDARD),
        escape_json(&briar_public_view_summary(&state)),
        public_effects
            .iter()
            .map(|effect| format!("\"{}\"", escape_json(effect)))
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&briar_redacted_command_summary(commands.last())),
        matches!(
            state.phase,
            briar_circuit::Phase::Terminal(_)
        )
    ))
}

pub(crate) fn is_briar_circuit_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(GAME_BRIAR_CIRCUIT)
    )
}

pub(crate) fn import_briar_circuit_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        crate::json::diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    if string_field(doc, "rules_version")? != BRIAR_CIRCUIT_TRACE_RULES_VERSION {
        return Err(crate::json::diagnostic_string(
            "unsupported_replay_rules",
            "unsupported briar_circuit replay rules version",
        ));
    }
    if string_field(doc, "variant")? != VARIANT_BRIAR_CIRCUIT_STANDARD {
        return Err(crate::json::diagnostic_string(
            "unsupported_replay_variant",
            "unsupported briar_circuit replay variant",
        ));
    }
    let viewer = string_field(doc, "viewer")?;
    if viewer != "observer" {
        return Err(crate::json::diagnostic_string(
            "unsupported_replay_viewer",
            "briar_circuit public replay import only supports observer exports",
        ));
    }
    let steps = parse_public_replay_steps(doc).map_err(|message| {
        crate::json::diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let replay_id = next_replay_id(GAME_BRIAR_CIRCUIT);
    let step_count = steps.len();
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_BRIAR_CIRCUIT.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay { viewer, steps }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"observer\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_BRIAR_CIRCUIT),
        step_count
    ))
}

pub(crate) fn briar_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &BriarCircuitState,
    effects: &[EffectEnvelope<BriarCircuitEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        briar_view_json(&briar_project_view(state, &viewer), state.freshness_token.0),
        briar_effects_json(effects, &viewer)
    )
}

pub(crate) fn briar_bot_action_path(action: BriarCircuitBotAction) -> Vec<String> {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            vec!["pass".to_owned(), "select".to_owned(), card.as_str()]
        }
        BriarCircuitBotAction::Pass(PassAction::Unselect(card)) => {
            vec!["pass".to_owned(), "unselect".to_owned(), card.as_str()]
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => {
            vec!["pass".to_owned(), "confirm".to_owned()]
        }
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            vec!["play".to_owned(), card.as_str()]
        }
    }
}

pub(crate) fn briar_select_bot_action(
    state: &BriarCircuitState,
    seat: BriarCircuitSeat,
    seed: u64,
) -> Result<(String, Vec<String>), String> {
    let decision = BriarCircuitL1Bot::new(Seed(seed))
        .select_decision(state, seat)
        .map_err(diagnostic_json)?;
    Ok((decision.explanation, decision.action_path))
}

fn briar_action_choice_json(action: BriarCircuitBotAction) -> String {
    let path = briar_bot_action_path(action);
    let segment = path.first().cloned().unwrap_or_default();
    let label = briar_action_label(action);
    let next = briar_action_next_json(&path);
    format!(
        "{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[],\"presentation\":null,\"tags\":[],\"next\":{}}}",
        escape_json(&segment),
        escape_json(&label),
        escape_json(&label),
        next
    )
}

fn briar_action_next_json(path: &[String]) -> String {
    if path.len() <= 1 {
        return "null".to_owned();
    }
    let mut next = "null".to_owned();
    for segment in path.iter().skip(1).rev() {
        let label = segment.replace('_', " ");
        next = format!(
            "{{\"choices\":[{{\"segment\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\",\"metadata\":[],\"presentation\":null,\"tags\":[],\"next\":{}}}]}}",
            escape_json(segment),
            escape_json(&label),
            escape_json(&label),
            next
        );
    }
    next
}

fn briar_action_label(action: BriarCircuitBotAction) -> String {
    match action {
        BriarCircuitBotAction::Pass(PassAction::Select(card)) => {
            format!("Select {}", card.as_str().replace('_', " "))
        }
        BriarCircuitBotAction::Pass(PassAction::Unselect(card)) => {
            format!("Unselect {}", card.as_str().replace('_', " "))
        }
        BriarCircuitBotAction::Pass(PassAction::Confirm) => "Confirm pass".to_owned(),
        BriarCircuitBotAction::Play(PlayAction::Play(card)) => {
            format!("Play {}", card.as_str().replace('_', " "))
        }
    }
}

fn option_briar_seat_json(seat: Option<BriarCircuitSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn briar_scores_json(scores: [u16; 4]) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{},\"seat_2\":{},\"seat_3\":{}}}",
        scores[0], scores[1], scores[2], scores[3]
    )
}

fn briar_raw_points_json(values: [u8; 4]) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{},\"seat_2\":{},\"seat_3\":{}}}",
        values[0], values[1], values[2], values[3]
    )
}

fn briar_hand_summary_json(summary: &briar_circuit::HandSummaryView) -> String {
    format!(
        "{{\"raw_points\":{},\"hand_additions\":{},\"cumulative_after\":{},\"moon_shooter\":{}}}",
        briar_raw_points_json(summary.raw_points),
        briar_raw_points_json(summary.hand_additions),
        briar_scores_json(summary.cumulative_after),
        option_briar_seat_json(summary.moon_shooter)
    )
}

fn briar_hand_counts_json(counts: &[(BriarCircuitSeat, usize)]) -> String {
    let body = counts
        .iter()
        .map(|(seat, count)| format!("\"{}\":{}", seat.as_str(), count))
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{body}}}")
}

fn briar_pass_json(pass: &briar_circuit::PassView) -> String {
    format!(
        "{{\"direction\":\"{}\",\"committed_count\":{},\"pending_count\":{},\"own_selection\":[{}],\"own_committed\":{}}}",
        escape_json(&pass.direction),
        pass.committed_count,
        pass.pending_count,
        pass.own_selection
            .iter()
            .map(briar_card_json)
            .collect::<Vec<_>>()
            .join(","),
        pass.own_committed
    )
}

fn briar_card_json(card: &CardId) -> String {
    let detail = card.card();
    format!(
        "{{\"card_id\":\"{}\",\"suit\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.as_str()),
        detail.suit.as_str(),
        detail.rank.as_str(),
        detail.rank.value(),
        escape_json(&detail.public_label()),
        escape_json(&detail.public_label())
    )
}

fn briar_card_id_array_json(cards: &[CardId]) -> String {
    cards
        .iter()
        .map(|card| format!("\"{}\"", escape_json(&card.as_str())))
        .collect::<Vec<_>>()
        .join(",")
}

fn briar_trick_play_json(play: &briar_circuit::TrickPlay) -> String {
    format!(
        "{{\"seat\":\"{}\",\"card\":\"{}\"}}",
        play.seat.as_str(),
        escape_json(&play.card.as_str())
    )
}

fn briar_captured_trick_json(trick: &briar_circuit::CapturedTrick) -> String {
    format!(
        "{{\"hand_index\":{},\"trick_index\":{},\"winner\":\"{}\",\"plays\":[{}]}}",
        trick.hand_index,
        trick.trick_index,
        trick.winner.as_str(),
        trick
            .plays
            .iter()
            .map(briar_trick_play_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn briar_visible_effect_json(effect: &BriarCircuitEffect) -> String {
    format!("{{\"payload\":{}}}", briar_effect_payload_json(effect))
}

fn briar_effect_payload_json(effect: &BriarCircuitEffect) -> String {
    match effect {
        BriarCircuitEffect::PassSelectionUpdated {
            seat,
            selected_count,
            selected_cards,
        } => format!(
            "{{\"type\":\"pass_selection_updated\",\"seat\":\"{}\",\"selected_count\":{},\"selected_cards\":[{}]}}",
            seat.as_str(),
            selected_count,
            briar_card_id_array_json(selected_cards)
        ),
        BriarCircuitEffect::PassCommitmentPublic(status) => format!(
            "{{\"type\":\"pass_commitment_public\",\"direction\":\"{}\",\"committed_count\":{},\"pending_count\":{}}}",
            status.direction.as_str(),
            status.committed_count,
            status.pending_count
        ),
        BriarCircuitEffect::PassExchangePublic { direction } => format!(
            "{{\"type\":\"pass_exchange_public\",\"direction\":\"{}\"}}",
            direction.as_str()
        ),
        BriarCircuitEffect::PassExchangePrivate {
            seat,
            sent_cards,
            received_cards,
        } => format!(
            "{{\"type\":\"pass_exchange_private\",\"seat\":\"{}\",\"sent_cards\":[{}],\"received_cards\":[{}]}}",
            seat.as_str(),
            briar_card_id_array_json(sent_cards),
            briar_card_id_array_json(received_cards)
        ),
        BriarCircuitEffect::CardPlayed { seat, card } => format!(
            "{{\"type\":\"card_played\",\"seat\":\"{}\",\"card\":\"{}\"}}",
            seat.as_str(),
            escape_json(&card.as_str())
        ),
        BriarCircuitEffect::HeartsBroken { seat } => format!(
            "{{\"type\":\"hearts_broken\",\"seat\":\"{}\"}}",
            seat.as_str()
        ),
        BriarCircuitEffect::TrickCaptured {
            trick_index,
            winner,
            cards,
        } => format!(
            "{{\"type\":\"trick_captured\",\"trick_index\":{},\"winner\":\"{}\",\"cards\":[{}]}}",
            trick_index,
            winner.as_str(),
            briar_card_id_array_json(cards)
        ),
    }
}

fn briar_public_view_summary(state: &BriarCircuitState) -> String {
    let view = briar_project_view(state, &Viewer { seat_id: None });
    format!(
        "phase={} hand_index={} active_seat={} captured_tricks={}",
        view.phase,
        view.hand_index,
        view.active_seat.map_or("none", BriarCircuitSeat::as_str),
        view.captured_tricks.len()
    )
}

fn briar_redacted_command_summary(command: Option<&AppliedCommand>) -> String {
    command.map_or_else(
        || "setup".to_owned(),
        |command| {
            format!(
                "{} {}",
                command.actor_seat,
                command
                    .action_path
                    .first()
                    .map_or("unknown_action", String::as_str)
            )
        },
    )
}

fn briar_public_effect_text(effect: &BriarCircuitEffect) -> String {
    match effect {
        BriarCircuitEffect::PassCommitmentPublic(status) => format!(
            "pass commitment {}/{} {}",
            status.committed_count,
            status.committed_count + status.pending_count,
            status.direction.as_str()
        ),
        BriarCircuitEffect::PassExchangePublic { direction } => {
            format!("pass exchange completed {}", direction.as_str())
        }
        BriarCircuitEffect::CardPlayed { seat, .. } => {
            format!("{} played a public card", seat.as_str())
        }
        BriarCircuitEffect::HeartsBroken { seat } => {
            format!("hearts broken by {}", seat.as_str())
        }
        BriarCircuitEffect::TrickCaptured {
            trick_index,
            winner,
            ..
        } => format!("trick {} captured by {}", trick_index, winner.as_str()),
        BriarCircuitEffect::PassSelectionUpdated { .. }
        | BriarCircuitEffect::PassExchangePrivate { .. } => "private update".to_owned(),
    }
}

fn briar_ui_json() -> String {
    "{\"table_label\":\"Briar Circuit\",\"own_hand_label\":\"Your hand\",\"current_trick_label\":\"Current trick\",\"captured_tricks_label\":\"Captured tricks\",\"score_label\":\"Scores\",\"play_action_label\":\"Play card\"}".to_owned()
}

#[allow(dead_code)]
fn _parse_helpers_stay_linked(path: &[String]) -> bool {
    parse_pass_action_path(path).is_ok() || parse_play_action_path(path).is_ok()
}
