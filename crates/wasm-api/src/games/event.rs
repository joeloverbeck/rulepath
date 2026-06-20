//! Browser-bridge helpers for `event_frontier` (hidden-information event-deck game).

use engine_core::{EffectEnvelope, Viewer};
use event_frontier::resolve_reckoning as event_frontier_resolve_reckoning;
use event_frontier::visibility::reason_label as event_frontier_reason_label;
use event_frontier::{
    EventFrontierEffect, EventFrontierState, FactionId as EventFrontierFactionId,
};

use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{parse_public_replay_steps, PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{option_string_json, string_array, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_event_frontier_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(event_frontier::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && string_field(doc, "viewer").is_ok()
        && string_field(doc, "hidden_information_redaction").is_ok()
}

pub(crate) fn import_event_frontier_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    event_frontier::import_public_export_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != event_frontier::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != event_frontier::VARIANT_STANDARD_ID
        && variant != event_frontier::VARIANT_HARD_WINTER_ID
        && variant != event_frontier::VARIANT_LAND_RUSH_ID
    {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let viewer = string_field(doc, "viewer")?;
    let steps = parse_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = event_frontier::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: event_frontier::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        viewer,
        hidden_information: event_frontier::TRACE_HIDDEN_SURFACE.to_owned(),
        hidden_information_redaction: "undrawn_order_redacted".to_owned(),
        stochastic_game_rule_events: event_frontier::TRACE_STOCHASTIC_SURFACE.to_owned(),
        steps: steps
            .iter()
            .map(event_frontier_step_from_public_timeline)
            .collect(),
    };
    let imported = event_frontier::import_public_export(&export);
    let viewer = export.viewer.clone();
    let step_count = export.steps.len();
    let replay_id = next_replay_id(GAME_EVENT_FRONTIER);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_EVENT_FRONTIER.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_event_frontier)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0,\"raw_size\":{}}}",
        escape_json(&replay_id),
        escape_json(GAME_EVENT_FRONTIER),
        escape_json(&viewer),
        step_count,
        imported.raw_json.len()
    ))
}

pub(crate) fn event_frontier_step_from_public_timeline(
    step: &PublicTimelineStep,
) -> event_frontier::PublicReplayStep {
    event_frontier::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn public_timeline_step_from_event_frontier(
    step: &event_frontier::PublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn event_frontier_command_summary(command: &AppliedCommand) -> String {
    format!(
        "{}:{}:{}",
        command.actor_seat,
        command.action_path.join("/"),
        command.freshness_token
    )
}

pub(crate) fn event_frontier_effects_json(
    effects: &[EffectEnvelope<EventFrontierEffect>],
    viewer: &Viewer,
) -> String {
    let body = event_frontier::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(event_frontier_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn event_frontier_finish_automated_phases(
    state: &mut EventFrontierState,
) -> Result<Vec<EffectEnvelope<EventFrontierEffect>>, String> {
    let mut effects = Vec::new();
    while state.card_phase == event_frontier::CardPhase::Reckoning {
        effects.extend(
            event_frontier_resolve_reckoning(state)
                .map_err(diagnostic_json)?
                .effects,
        );
    }
    Ok(effects)
}

pub(crate) fn event_frontier_view_json(view: &event_frontier::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"factions\":[{}],\"active_seat\":{},\"sites\":[{}],\"adjacency\":[{}],\"resources\":{},\"scores\":{},\"eligibility\":[{}],\"current_card\":{},\"next_public_card\":{},\"discard\":[{}],\"active_edicts\":[{}],\"epoch\":{},\"reckoning_count\":{},\"victory_distance\":{},\"terminal\":{},\"terminal_rationale\":{},\"ui\":{},\"freshness_token\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(GAME_EVENT_FRONTIER_DISPLAY_NAME),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        string_array(&view.factions),
        option_string_json(view.active_seat.as_deref()),
        view.sites
            .iter()
            .map(event_frontier_site_json)
            .collect::<Vec<_>>()
            .join(","),
        view.adjacency
            .iter()
            .map(event_frontier_adjacency_json)
            .collect::<Vec<_>>()
            .join(","),
        event_frontier_resources_json(&view.resources),
        event_frontier_scores_json(&view.scores),
        view.eligibility
            .iter()
            .map(event_frontier_eligibility_json)
            .collect::<Vec<_>>()
            .join(","),
        option_event_frontier_card_face_json(view.current_card.as_ref()),
        option_event_frontier_card_face_json(view.next_public_card.as_ref()),
        view.discard
            .iter()
            .map(event_frontier_card_face_json)
            .collect::<Vec<_>>()
            .join(","),
        string_array(&view.active_edicts),
        view.epoch,
        view.reckoning_count,
        event_frontier_victory_distance_json(&view.victory_distance),
        event_frontier_terminal_json(&view.terminal),
        event_frontier_terminal_rationale_json(view),
        event_frontier_ui_json(&view.ui),
        view.freshness_token
    )
}

pub(crate) fn option_event_frontier_card_face_json(
    card: Option<&event_frontier::CardFaceView>,
) -> String {
    card.map(event_frontier_card_face_json)
        .unwrap_or_else(|| "null".to_owned())
}

pub(crate) fn event_frontier_card_face_json(card: &event_frontier::CardFaceView) -> String {
    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"summary\":\"{}\",\"details\":{},\"family\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.id),
        escape_json(&card.label),
        escape_json(&card.summary),
        option_string_json(card.details.as_deref()),
        escape_json(&card.family),
        escape_json(&card.accessibility_label)
    )
}

pub(crate) fn event_frontier_ui_json(ui: &event_frontier::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"event_deck_label\":\"{}\",\"current_card_label\":\"{}\",\"next_card_label\":\"{}\",\"discard_label\":\"{}\",\"face_down_label\":\"{}\",\"face_down_summary\":\"{}\",\"reduced_motion_token\":\"{}\",\"seat_labels\":[{}],\"faction_labels\":[{}],\"action_affordance_templates\":[{}]}}",
        escape_json(&ui.table_label),
        escape_json(&ui.event_deck_label),
        escape_json(&ui.current_card_label),
        escape_json(&ui.next_card_label),
        escape_json(&ui.discard_label),
        escape_json(&ui.face_down_label),
        escape_json(&ui.face_down_summary),
        escape_json(&ui.reduced_motion_token),
        ui.seat_labels
            .iter()
            .map(event_frontier_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        ui.faction_labels
            .iter()
            .map(event_frontier_faction_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        ui.action_affordance_templates
            .iter()
            .map(event_frontier_action_affordance_template_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn event_frontier_seat_display_label_json(
    label: &event_frontier::ui::SeatDisplayLabel,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.seat),
        escape_json(&label.label)
    )
}

pub(crate) fn event_frontier_faction_display_label_json(
    label: &event_frontier::ui::FactionDisplayLabel,
) -> String {
    format!(
        "{{\"faction\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.faction),
        escape_json(&label.label)
    )
}

pub(crate) fn event_frontier_action_affordance_template_json(
    template: &event_frontier::ActionAffordanceTemplate,
) -> String {
    format!(
        "{{\"id\":\"{}\",\"text\":\"{}\"}}",
        escape_json(&template.id),
        escape_json(&template.text)
    )
}

pub(crate) fn event_frontier_site_json(site: &event_frontier::visibility::SiteView) -> String {
    format!(
        "{{\"site\":\"{}\",\"label\":\"{}\",\"agents\":{},\"settlers\":{},\"depot\":{},\"cache_count\":{}}}",
        site.site.as_str(),
        escape_json(&site.label),
        site.agents,
        site.settlers,
        site.depot,
        site.cache_count
    )
}

pub(crate) fn event_frontier_adjacency_json(
    (site, neighbors): &(event_frontier::SiteId, Vec<event_frontier::SiteId>),
) -> String {
    format!(
        "{{\"site\":\"{}\",\"neighbors\":[{}]}}",
        site.as_str(),
        neighbors
            .iter()
            .map(|neighbor| format!("\"{}\"", neighbor.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn event_frontier_resources_json(
    resources: &event_frontier::visibility::ResourceView,
) -> String {
    format!(
        "{{\"funds\":{},\"provisions\":{}}}",
        resources.funds, resources.provisions
    )
}

pub(crate) fn event_frontier_scores_json(scores: &event_frontier::visibility::ScoreView) -> String {
    format!(
        "{{\"charter\":{},\"freeholders\":{}}}",
        scores.charter, scores.freeholders
    )
}

pub(crate) fn event_frontier_eligibility_json(
    (faction, eligibility): &(EventFrontierFactionId, event_frontier::Eligibility),
) -> String {
    format!(
        "{{\"faction\":\"{}\",\"eligible\":\"{}\"}}",
        faction.as_str(),
        eligibility.as_str()
    )
}

pub(crate) fn event_frontier_victory_distance_json(
    distance: &event_frontier::visibility::VictoryDistanceView,
) -> String {
    format!(
        "{{\"charter_sites_needed\":{},\"freeholder_caches_needed\":{}}}",
        distance.charter_sites_needed, distance.freeholder_caches_needed
    )
}

pub(crate) fn event_frontier_terminal_json(
    terminal: &event_frontier::visibility::TerminalView,
) -> String {
    match terminal {
        event_frontier::visibility::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null}".to_owned()
        }
        event_frontier::visibility::TerminalView::Complete {
            winner,
            victory_type,
            scores,
            decisive_rule,
        } => format!(
            "{{\"kind\":\"winner\",\"winner\":\"{}\",\"victory_type\":\"{}\",\"scores\":{},\"decisive_rule\":\"{}\"}}",
            winner.as_str(),
            escape_json(victory_type),
            event_frontier_scores_json(scores),
            escape_json(decisive_rule)
        ),
    }
}

pub(crate) fn event_frontier_terminal_rationale_json(view: &event_frontier::PublicView) -> String {
    let event_frontier::visibility::TerminalView::Complete {
        winner,
        victory_type,
        scores,
        decisive_rule,
    } = &view.terminal
    else {
        return "null".to_owned();
    };
    let cause = if decisive_rule == "EF-END-003" {
        "both_met_freeholder"
    } else if victory_type == "charter_instant" {
        "charter_instant"
    } else if victory_type == "freeholder_instant" {
        "freeholder_instant"
    } else if scores.charter == scores.freeholders {
        "final_fallback_tiebreak"
    } else {
        "final_fallback_score"
    };
    let template_key = match cause {
        "charter_instant" => "event_frontier.charter_instant",
        "freeholder_instant" => "event_frontier.freeholder_instant",
        "both_met_freeholder" => "event_frontier.both_met_freeholder",
        "final_fallback_tiebreak" => "event_frontier.final_fallback_tiebreak",
        _ => "event_frontier.final_fallback_score",
    };
    format!(
        "{{\"result_kind\":\"win\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"template_params\":{{\"winner\":\"{}\",\"charter_score\":{},\"freeholder_score\":{}}},\"decisive_rule_ids\":[\"{}\"],\"final_standing\":[{{\"seat\":\"faction_charter\",\"label\":\"Charter\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{{\"label\":\"Score\",\"value\":{}}}]}},{{\"seat\":\"faction_freeholders\",\"label\":\"Freeholders\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{{\"label\":\"Score\",\"value\":{}}}]}}],\"breakdown_sections\":[{{\"id\":\"event-frontier-terminal\",\"heading\":\"Rust terminal cause\",\"rows\":[{{\"label\":\"Victory type\",\"value\":\"{}\"}},{{\"label\":\"Decisive rule\",\"value\":\"{}\"}},{{\"label\":\"Reckonings\",\"value\":{}}}]}}]}}",
        cause,
        template_key,
        escape_json(winner.as_str()),
        scores.charter,
        scores.freeholders,
        escape_json(decisive_rule),
        if *winner == EventFrontierFactionId::Charter { "win" } else { "loss" },
        *winner == EventFrontierFactionId::Charter,
        scores.charter,
        if *winner == EventFrontierFactionId::Freeholders { "win" } else { "loss" },
        *winner == EventFrontierFactionId::Freeholders,
        scores.freeholders,
        escape_json(victory_type),
        escape_json(decisive_rule),
        view.reckoning_count
    )
}

pub(crate) fn event_frontier_effect_json(effect: &EffectEnvelope<EventFrontierEffect>) -> String {
    let payload = match &effect.payload {
        EventFrontierEffect::EventResolved { card, summary } => format!(
            "{{\"type\":\"event_resolved\",\"card\":\"{}\",\"summary\":\"{}\"}}",
            card.as_str(),
            escape_json(summary)
        ),
        EventFrontierEffect::EdictActivated { card, edict } => format!(
            "{{\"type\":\"edict_activated\",\"card\":\"{}\",\"edict\":\"{}\"}}",
            card.as_str(),
            escape_json(edict)
        ),
        EventFrontierEffect::EdictExpired { edict } => format!(
            "{{\"type\":\"edict_expired\",\"edict\":\"{}\"}}",
            escape_json(edict)
        ),
        EventFrontierEffect::CardRevealed { card, next_public } => format!(
            "{{\"type\":\"card_revealed\",\"card\":\"{}\",\"next_public\":{}}}",
            card.as_str(),
            option_string_json(next_public.map(|card| card.as_str()))
        ),
        EventFrontierEffect::ChoiceTaken { faction, choice } => format!(
            "{{\"type\":\"choice_taken\",\"faction\":\"{}\",\"choice\":\"{}\"}}",
            faction.as_str(),
            escape_json(choice)
        ),
        EventFrontierEffect::CardDiscarded { card, reason } => format!(
            "{{\"type\":\"card_discarded\",\"card\":\"{}\",\"reason\":\"{}\"}}",
            card.as_str(),
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::EligibilityChanged {
            faction,
            eligible,
            reason,
        } => format!(
            "{{\"type\":\"eligibility_changed\",\"faction\":\"{}\",\"eligible\":{},\"reason\":\"{}\"}}",
            faction.as_str(),
            eligible,
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::ResourcesChanged {
            faction,
            previous,
            new,
            reason,
        } => format!(
            "{{\"type\":\"resources_changed\",\"faction\":\"{}\",\"previous\":{},\"new\":{},\"reason\":\"{}\"}}",
            faction.as_str(),
            previous,
            new,
            escape_json(event_frontier_reason_label(reason))
        ),
        EventFrontierEffect::OpResolved { faction, op, sites } => format!(
            "{{\"type\":\"op_resolved\",\"faction\":\"{}\",\"op\":\"{}\",\"sites\":[{}]}}",
            faction.as_str(),
            escape_json(op),
            sites
                .iter()
                .map(|site| format!("\"{}\"", site.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        EventFrontierEffect::AgentPlaced { site, new_count } => format!(
            "{{\"type\":\"agent_placed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::AgentRemoved { site, new_count } => format!(
            "{{\"type\":\"agent_removed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::DepotBuilt { site } => format!(
            "{{\"type\":\"depot_built\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        EventFrontierEffect::CacheRemoved { site, new_count } => format!(
            "{{\"type\":\"cache_removed\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::SettlerMoved {
            from,
            to,
            from_count,
            to_count,
        } => format!(
            "{{\"type\":\"settler_moved\",\"from\":\"{}\",\"to\":\"{}\",\"from_count\":{},\"to_count\":{}}}",
            from.as_str(),
            to.as_str(),
            from_count,
            to_count
        ),
        EventFrontierEffect::CacheLaid { site, new_count } => format!(
            "{{\"type\":\"cache_laid\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::SettlerRallied { site, new_count } => format!(
            "{{\"type\":\"settler_rallied\",\"site\":\"{}\",\"new_count\":{}}}",
            site.as_str(),
            new_count
        ),
        EventFrontierEffect::ReckoningResolved {
            round,
            victory_check,
            site_breakdown,
            income,
            expired_edicts,
        } => format!(
            "{{\"type\":\"reckoning_resolved\",\"round\":{},\"victory_check\":\"{}\",\"site_breakdown\":[{}],\"income\":{{\"funds\":{},\"provisions\":{}}},\"expired_edicts\":[{}]}}",
            round,
            escape_json(victory_check),
            site_breakdown
                .iter()
                .map(event_frontier_site_score_json)
                .collect::<Vec<_>>()
                .join(","),
            income.0,
            income.1,
            expired_edicts
                .iter()
                .map(|edict| format!("\"{}\"", escape_json(edict)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        EventFrontierEffect::Terminal {
            winner,
            victory_type,
            totals,
            summary,
        } => format!(
            "{{\"type\":\"terminal\",\"winner\":\"{}\",\"victory_type\":\"{}\",\"totals\":{{\"charter\":{},\"freeholders\":{}}},\"summary\":\"{}\"}}",
            winner.as_str(),
            escape_json(victory_type),
            totals.0,
            totals.1,
            escape_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn event_frontier_site_score_json(
    breakdown: &event_frontier::SiteScoreBreakdown,
) -> String {
    format!(
        "{{\"site\":\"{}\",\"charter_presence\":{},\"freeholder_presence\":{},\"awarded_to\":{}}}",
        breakdown.site.as_str(),
        breakdown.charter_presence,
        breakdown.freeholder_presence,
        option_string_json(breakdown.awarded_to.map(|faction| faction.as_str()))
    )
}
