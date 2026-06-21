//! Browser-bridge helpers for `vow_tide`.

use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer, VisibilityScope,
};
use vow_tide::{
    actions::legal_action_tree as vow_legal_action_tree,
    bots::{VowTideL0Bot, VowTideL1Bot},
    effects::VowTideEffect,
    ids::{canonical_seat_ids, VowTideSeat},
    replay_support::{export_for_viewer, import_viewer_export, observer, ViewerExport},
    rules::{apply_bid, apply_play, validate_bid_command, validate_play_command},
    setup_match as vow_setup_match,
    state::VowTideState,
    visibility::{project_view as vow_project_view, PrivateView, PublicView},
    SetupOptions,
};

use crate::action_tree::action_tree_json;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn parse_vow_seat(value: &str) -> Result<VowTideSeat, String> {
    VowTideSeat::ALL
        .into_iter()
        .find(|seat| seat.as_str() == value || value == format!("seat-{}", seat.index()))
        .ok_or_else(|| diagnostic_string("unknown_seat", &format!("unknown seat: {value}")))
}

pub(crate) fn trace_vow_seat(seat: VowTideSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn vow_actor_for_seat(state: &VowTideState, seat: VowTideSeat) -> Result<Actor, String> {
    state
        .seats
        .get(seat.index())
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            diagnostic_string(
                "unknown_seat",
                &format!("seat not present: {}", seat.as_str()),
            )
        })
}

pub(crate) fn vow_viewer_for_seat(
    state: &VowTideState,
    viewer_seat: Option<&str>,
) -> Result<Viewer, String> {
    match viewer_seat {
        None => Ok(Viewer { seat_id: None }),
        Some(value) => {
            let seat = parse_vow_seat(value)?;
            state
                .seats
                .get(seat.index())
                .cloned()
                .map(|seat_id| Viewer {
                    seat_id: Some(seat_id),
                })
                .ok_or_else(|| {
                    diagnostic_string(
                        "unknown_seat",
                        &format!("seat not present: {}", seat.as_str()),
                    )
                })
        }
    }
}

pub(crate) fn vow_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    seat: VowTideSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_vow_seat)
        .transpose()
        .map(|viewer| viewer == Some(seat))
}

pub(crate) fn create_vow_tide_match(seed: u64, seat_count: usize) -> Result<VowTideState, String> {
    if !(3..=7).contains(&seat_count) {
        return Err(diagnostic_string(
            "unsupported_seat_count",
            "vow_tide requires 3, 4, 5, 6, or 7 seats",
        ));
    }
    vow_setup_match(
        Seed(seed),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .map_err(diagnostic_json)
}

pub(crate) fn vow_replay_to_cursor(
    seed: u64,
    seat_count: usize,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(VowTideState, Vec<EffectEnvelope<VowTideEffect>>), String> {
    let mut state = create_vow_tide_match(seed, seat_count)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_vow_seat(&command.actor_seat)?;
        let effects = vow_apply_command(
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

pub(crate) fn vow_apply_command(
    state: &mut VowTideState,
    seat: VowTideSeat,
    action_path: ActionPath,
    freshness_token: u64,
) -> Result<Vec<EffectEnvelope<VowTideEffect>>, String> {
    let command = CommandEnvelope {
        actor: vow_actor_for_seat(state, seat)?,
        action_path,
        freshness_token: engine_core::FreshnessToken(freshness_token),
        rules_version: RulesVersion(RULES_VERSION),
    };
    let effects = match command.action_path.segments.first().map(String::as_str) {
        Some(vow_tide::ids::ACTION_BID) => {
            let bid = validate_bid_command(state, &command).map_err(diagnostic_json)?;
            apply_bid(state, bid).map_err(diagnostic_json)?
        }
        Some(vow_tide::actions::ACTION_PLAY) => {
            let play = validate_play_command(state, &command).map_err(diagnostic_json)?;
            apply_play(state, play).map_err(diagnostic_json)?
        }
        _ => {
            return Err(diagnostic_string(
                "unknown_action",
                "unknown vow_tide action family",
            ))
        }
    };
    Ok(effects
        .into_iter()
        .map(|payload| EffectEnvelope {
            visibility: VisibilityScope::Public,
            payload,
        })
        .collect())
}

pub(crate) fn vow_action_tree_json(
    state: &VowTideState,
    seat: VowTideSeat,
) -> Result<String, String> {
    let actor = vow_actor_for_seat(state, seat)?;
    Ok(action_tree_json(&vow_legal_action_tree(state, &actor)))
}

pub(crate) fn vow_view_json(view: &PublicView, freshness_token: u64) -> String {
    let private_status = match &view.private_view {
        PrivateView::Observer => "observer",
        PrivateView::Seat(_) => "seat",
    };
    let own_hand = match &view.private_view {
        PrivateView::Seat(private) => private
            .own_hand
            .iter()
            .map(vow_card_json)
            .collect::<Vec<_>>()
            .join(","),
        PrivateView::Observer => String::new(),
    };
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"dealer\":\"{}\",\"hand_index\":{},\"hand_size\":{},\"hand_schedule\":{},\"trump_indicator\":{},\"hand_counts\":{},\"hidden_stock_count\":{},\"public_bids\":{},\"trick_counts\":{},\"current_trick\":{},\"captured_tricks_count\":{},\"completed_hand_count\":{},\"terminal\":{},\"freshness_token\":{},\"private_view_status\":\"{}\",\"own_hand\":[{}],\"hidden_fields\":[\"opponent_hands\",\"hidden_stock\",\"deck_order\"],\"ui\":{{\"action_families\":[\"bid\",\"play\"]}}}}",
        SCHEMA_VERSION,
        RULES_VERSION,
        escape_json(GAME_VOW_TIDE),
        escape_json(GAME_VOW_TIDE_DISPLAY_NAME),
        escape_json(VARIANT_VOW_TIDE_STANDARD),
        escape_json(vow_tide::ids::RULES_VERSION_LABEL),
        escape_json(&view.phase),
        option_vow_seat_json(view.active_seat),
        view.dealer.as_str(),
        view.hand_index,
        view.hand_size,
        u8_array_json(&view.hand_schedule),
        vow_card_json(&view.trump_indicator),
        vow_seat_u8_json(&view.hand_counts),
        view.hidden_stock_count,
        vow_bids_json(&view.public_bids),
        vow_seat_u8_json(&view.trick_counts),
        vow_current_trick_json(view),
        view.captured_tricks.len(),
        view.completed_hand_count,
        vow_terminal_json(view),
        freshness_token,
        private_status,
        own_hand
    )
}

pub(crate) fn vow_effects_json(
    effects: &[EffectEnvelope<VowTideEffect>],
    _viewer: &Viewer,
) -> String {
    format!(
        "[{}]",
        effects
            .iter()
            .map(vow_logged_effect_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn vow_logged_effect_json(effect: &EffectEnvelope<VowTideEffect>) -> String {
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        vow_effect_payload_json(&effect.payload)
    )
}

pub(crate) fn vow_replay_document_json(
    seed: u64,
    seat_count: usize,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let (state, effects) = vow_replay_to_cursor(seed, seat_count, commands, commands.len())?;
    let payloads = effects
        .iter()
        .map(|effect| effect.payload.clone())
        .collect::<Vec<_>>();
    let export = export_for_viewer(&state, &payloads, &observer());
    Ok(vow_viewer_export_json(&export, commands.last()))
}

pub(crate) fn is_vow_tide_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(string_field(doc, "game_id").as_deref(), Ok(GAME_VOW_TIDE))
}

pub(crate) fn import_vow_tide_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = import_viewer_export(&ViewerExport {
        schema_version: 1,
        game_id: GAME_VOW_TIDE.to_owned(),
        rules_version_label: VOW_TIDE_TRACE_RULES_VERSION.to_owned(),
        viewer: "observer".to_owned(),
        observations: Vec::new(),
    })
    .map_err(|message| diagnostic_string("invalid_replay", &message))?;
    let replay_id = next_replay_id(GAME_VOW_TIDE);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_VOW_TIDE.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: export.viewer,
                    steps: vec![PublicTimelineStep {
                        step_index: 0,
                        public_view_summary: "imported vow_tide public export".to_owned(),
                        public_effects: Vec::new(),
                        redacted_command_summary: "imported public export".to_owned(),
                        terminal: false,
                    }],
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"observer\",\"step_count\":1,\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_VOW_TIDE)
    ))
}

pub(crate) fn vow_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &VowTideState,
    effects: &[EffectEnvelope<VowTideEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        vow_view_json(&vow_project_view(state, &viewer), state.freshness_token.0),
        vow_effects_json(effects, &viewer)
    )
}

pub(crate) fn vow_select_bot_decision(
    state: &VowTideState,
    seat: VowTideSeat,
    bot_seed: u64,
) -> Result<vow_tide::bots::BotDecision, String> {
    if seat.index() % 2 == 0 {
        VowTideL1Bot::new(Seed(bot_seed))
            .select_decision(state, seat)
            .map_err(diagnostic_json)
    } else {
        VowTideL0Bot::new(Seed(bot_seed))
            .select_decision(state, seat)
            .map_err(diagnostic_json)
    }
}

fn vow_viewer_export_json(export: &ViewerExport, last: Option<&AppliedCommand>) -> String {
    format!(
        "{{\"export_class\":\"viewer_scoped_observation_v1\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"variant\":\"{}\",\"viewer\":\"{}\",\"steps\":[{{\"step_index\":0,\"public_view_summary\":\"{}\",\"public_effects\":[],\"redacted_command_summary\":\"{}\",\"terminal\":false}}]}}",
        escape_json(GAME_VOW_TIDE),
        escape_json(VOW_TIDE_TRACE_RULES_VERSION),
        escape_json(VARIANT_VOW_TIDE_STANDARD),
        escape_json(&export.viewer),
        escape_json(&export.stable_summary()),
        escape_json(&vow_redacted_command_summary(last))
    )
}

fn vow_redacted_command_summary(command: Option<&AppliedCommand>) -> String {
    command.map_or_else(
        || "no commands".to_owned(),
        |command| format!("{}:{}", command.actor_seat, command.action_path.join("/")),
    )
}

fn option_vow_seat_json(seat: Option<VowTideSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn vow_card_json(card: &vow_tide::visibility::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"suit\":\"{}\",\"rank\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.suit),
        escape_json(&card.rank),
        escape_json(&card.label)
    )
}

fn u8_array_json(values: &[u8]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn vow_seat_u8_json(values: &[(VowTideSeat, u8)]) -> String {
    format!(
        "{{{}}}",
        values
            .iter()
            .map(|(seat, value)| format!("\"{}\":{}", seat.as_str(), value))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn vow_bids_json(values: &[(VowTideSeat, Option<u8>)]) -> String {
    format!(
        "{{{}}}",
        values
            .iter()
            .map(|(seat, value)| {
                format!(
                    "\"{}\":{}",
                    seat.as_str(),
                    value.map_or_else(|| "null".to_owned(), |bid| bid.to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn vow_current_trick_json(view: &PublicView) -> String {
    format!(
        "[{}]",
        view.current_trick
            .iter()
            .map(|play| {
                format!(
                    "{{\"seat\":\"{}\",\"card\":{}}}",
                    play.seat.as_str(),
                    vow_card_json(&play.card)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn vow_terminal_json(view: &PublicView) -> String {
    view.terminal.as_ref().map_or_else(
        || "{\"kind\":\"non_terminal\"}".to_owned(),
        |terminal| {
            format!(
                "{{\"kind\":\"terminal\",\"winners\":[{}],\"hands_played\":{}}}",
                terminal
                    .winners
                    .iter()
                    .map(|seat| format!("\"{}\"", seat.as_str()))
                    .collect::<Vec<_>>()
                    .join(","),
                terminal.hands_played
            )
        },
    )
}

fn vow_effect_payload_json(effect: &VowTideEffect) -> String {
    format!(
        "{{\"kind\":\"{}\",\"summary\":\"{}\"}}",
        vow_effect_kind(effect),
        escape_json(&format!("{effect:?}"))
    )
}

fn vow_effect_kind(effect: &VowTideEffect) -> &'static str {
    match effect {
        VowTideEffect::BidAccepted { .. } => "bid_accepted",
        VowTideEffect::DealerHookConstrained { .. } => "dealer_hook_constrained",
        VowTideEffect::BiddingCompleted { .. } => "bidding_completed",
        VowTideEffect::CardPlayed { .. } => "card_played",
        VowTideEffect::TrickCaptured { .. } => "trick_captured",
        VowTideEffect::HandScored { .. } => "hand_scored",
        VowTideEffect::HandAdvanced { .. } => "hand_advanced",
        VowTideEffect::MatchCompleted { .. } => "match_completed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_views_do_not_leak_other_hands_or_hidden_stock() {
        for seat_count in 3..=7 {
            let state = create_vow_tide_match(1700 + seat_count as u64, seat_count).unwrap();
            let hidden_stock = state
                .hidden_stock
                .iter()
                .map(|card| card.as_str())
                .collect::<Vec<_>>();

            let observer_json = vow_view_json(
                &vow_project_view(&state, &Viewer { seat_id: None }),
                state.freshness_token.0,
            );
            assert_hidden_cards_absent(&observer_json, &hidden_stock);
            for (_, hand) in &state.private_hands {
                let hand_ids = hand.iter().map(|card| card.as_str()).collect::<Vec<_>>();
                assert_hidden_cards_absent(&observer_json, &hand_ids);
            }

            for viewer_seat in VowTideSeat::ALL.into_iter().take(seat_count) {
                let viewer = vow_viewer_for_seat(&state, Some(viewer_seat.as_str())).unwrap();
                let view_json =
                    vow_view_json(&vow_project_view(&state, &viewer), state.freshness_token.0);
                assert_hidden_cards_absent(&view_json, &hidden_stock);

                for (seat, hand) in &state.private_hands {
                    if *seat == viewer_seat {
                        continue;
                    }
                    let hand_ids = hand.iter().map(|card| card.as_str()).collect::<Vec<_>>();
                    assert_hidden_cards_absent(&view_json, &hand_ids);
                }
            }
        }
    }

    fn assert_hidden_cards_absent(json: &str, hidden_cards: &[String]) {
        for card in hidden_cards {
            assert!(
                !json.contains(card),
                "projected Vow Tide JSON leaked hidden card {card}: {json}"
            );
        }
    }
}
