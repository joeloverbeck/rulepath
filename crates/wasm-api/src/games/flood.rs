//! Browser-bridge helpers for `flood_watch` (cooperative hidden-information game).

use engine_core::{EffectEnvelope, Viewer};
use flood_watch::FloodWatchEffect;

use crate::constants::*;
use crate::json::{diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{parse_public_replay_steps, PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{option_string_json, string_array, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_flood_watch_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(flood_watch::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && string_field(doc, "viewer").is_ok()
}

pub(crate) fn import_flood_watch_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != flood_watch::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != flood_watch::VARIANT_STANDARD_ID && variant != flood_watch::VARIANT_DELUGE_ID {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let viewer = string_field(doc, "viewer")?;
    if viewer != "observer" {
        return Err(diagnostic_string(
            "unsupported_replay_viewer",
            &format!("unsupported replay viewer: {viewer}"),
        ));
    }
    let steps = parse_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = flood_watch::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: flood_watch::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        viewer,
        steps: steps.iter().map(flood_step_from_public_timeline).collect(),
    };
    let _timeline = flood_watch::import_public_export(&export);
    let viewer = export.viewer.clone();
    let step_count = export.steps.len();
    let replay_id = next_replay_id(GAME_FLOOD_WATCH);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_FLOOD_WATCH.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: viewer.clone(),
                    steps: export
                        .steps
                        .iter()
                        .map(public_timeline_step_from_flood)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_FLOOD_WATCH),
        escape_json(&viewer),
        step_count
    ))
}

pub(crate) fn flood_step_from_public_timeline(
    step: &PublicTimelineStep,
) -> flood_watch::PublicReplayStep {
    flood_watch::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn flood_redacted_command_summary(command: &AppliedCommand) -> String {
    command.action_path.join("/")
}

pub(crate) fn flood_view_json(view: &flood_watch::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"roles\":[{}],\"turn_number\":{},\"active_seat\":\"{}\",\"phase\":{},\"districts\":[{}],\"drawn_cards\":[{}],\"forecast\":{},\"remaining_composition\":{},\"undrawn_count\":{},\"terminal\":{},\"freshness_token\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        view.roles
            .iter()
            .map(flood_role_json)
            .collect::<Vec<_>>()
            .join(","),
        view.turn_number,
        escape_json(&view.active_seat),
        flood_phase_json(view.phase),
        view.districts
            .iter()
            .map(flood_district_json)
            .collect::<Vec<_>>()
            .join(","),
        view.drawn_cards
            .iter()
            .map(flood_card_face_json)
            .collect::<Vec<_>>()
            .join(","),
        option_flood_card_face_json(view.forecast.as_ref()),
        flood_composition_json(&view.remaining_composition),
        view.undrawn_count,
        flood_terminal_json(&view.terminal),
        view.freshness_token,
        flood_ui_json(&view.ui)
    )
}

pub(crate) fn option_flood_card_face_json(card: Option<&flood_watch::CardFaceView>) -> String {
    card.map(flood_card_face_json)
        .unwrap_or_else(|| "null".to_owned())
}

pub(crate) fn flood_card_face_json(card: &flood_watch::CardFaceView) -> String {
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

pub(crate) fn flood_role_json(role: &flood_watch::visibility::RoleView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"role\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&role.seat),
        role.role.as_str(),
        escape_json(&role.label)
    )
}

pub(crate) fn flood_phase_json(phase: flood_watch::visibility::PhaseView) -> String {
    match phase {
        flood_watch::visibility::PhaseView::Action { budget_remaining } => {
            format!("{{\"kind\":\"action\",\"budget_remaining\":{budget_remaining}}}")
        }
        flood_watch::visibility::PhaseView::Terminal => {
            "{\"kind\":\"terminal\",\"budget_remaining\":0}".to_owned()
        }
    }
}

pub(crate) fn flood_district_json(district: &flood_watch::DistrictView) -> String {
    format!(
        "{{\"district\":\"{}\",\"label\":\"{}\",\"flood_level\":{},\"levees\":{}}}",
        district.district.as_str(),
        escape_json(district.label),
        district.flood_level,
        district.levees
    )
}

pub(crate) fn flood_composition_json(composition: &flood_watch::CompositionView) -> String {
    format!(
        "{{\"downpours_per_district\":[{}],\"surges_per_district\":[{}],\"reprieves\":{}}}",
        composition
            .downpours_per_district
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(","),
        composition
            .surges_per_district
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(","),
        composition.reprieves
    )
}

pub(crate) fn flood_composition_entry_json(entry: &(flood_watch::DistrictId, u8)) -> String {
    format!(
        "{{\"district\":\"{}\",\"count\":{}}}",
        entry.0.as_str(),
        entry.1
    )
}

pub(crate) fn flood_terminal_json(terminal: &flood_watch::TerminalView) -> String {
    match terminal {
        flood_watch::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"outcome\":null,\"summary\":null}".to_owned()
        }
        flood_watch::TerminalView::Complete { outcome, summary } => format!(
            "{{\"kind\":\"complete\",\"outcome\":\"{}\",\"summary\":{}}}",
            escape_json(outcome),
            flood_terminal_summary_json(summary)
        ),
    }
}

pub(crate) fn flood_terminal_summary_json(summary: &flood_watch::TerminalSummary) -> String {
    format!(
        "{{\"rule_id\":\"{}\",\"public_summary\":\"{}\",\"drawn_card_count\":{},\"surviving_levels\":[{}]}}",
        escape_json(&summary.rule_id),
        escape_json(&summary.public_summary),
        summary.drawn_card_count,
        summary
            .surviving_levels
            .iter()
            .map(flood_composition_entry_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn flood_ui_json(ui: &flood_watch::ui::UiMetadata) -> String {
    format!(
        "{{\"display_name\":\"{}\",\"event_deck_label\":\"{}\",\"forecast_label\":\"{}\",\"drawn_label\":\"{}\",\"face_down_label\":\"{}\",\"face_down_summary\":\"{}\",\"reduced_motion_token\":\"{}\"}}",
        escape_json(&ui.display_name),
        escape_json(&ui.event_deck_label),
        escape_json(&ui.forecast_label),
        escape_json(&ui.drawn_label),
        escape_json(&ui.face_down_label),
        escape_json(&ui.face_down_summary),
        escape_json(&ui.reduced_motion_token)
    )
}

pub(crate) fn flood_effects_json(
    effects: &[EffectEnvelope<FloodWatchEffect>],
    viewer: &Viewer,
) -> String {
    let body = flood_watch::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(flood_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn flood_effect_json(effect: &EffectEnvelope<FloodWatchEffect>) -> String {
    let payload = match &effect.payload {
        FloodWatchEffect::DistrictBailed { district, amount } => format!(
            "{{\"type\":\"district_bailed\",\"district\":\"{}\",\"amount\":{}}}",
            district.as_str(),
            amount
        ),
        FloodWatchEffect::LeveePlaced { district, amount } => format!(
            "{{\"type\":\"levee_placed\",\"district\":\"{}\",\"amount\":{}}}",
            district.as_str(),
            amount
        ),
        FloodWatchEffect::ForecastRevealed { card } => format!(
            "{{\"type\":\"forecast_revealed\",\"card\":\"{}\"}}",
            escape_json(&card.id())
        ),
        FloodWatchEffect::EnvironmentPhaseBegan { turn, draws } => format!(
            "{{\"type\":\"environment_phase_began\",\"turn\":{},\"draws\":{}}}",
            turn,
            draws
        ),
        FloodWatchEffect::EventDrawn { index, card } => format!(
            "{{\"type\":\"event_drawn\",\"index\":{},\"card\":\"{}\"}}",
            index,
            escape_json(&card.id())
        ),
        FloodWatchEffect::LeveeAbsorbed {
            district,
            amount,
            remaining_levees,
        } => format!(
            "{{\"type\":\"levee_absorbed\",\"district\":\"{}\",\"amount\":{},\"remaining_levees\":{}}}",
            district.as_str(),
            amount,
            remaining_levees
        ),
        FloodWatchEffect::FloodLevelRose {
            district,
            amount,
            new_level,
        } => format!(
            "{{\"type\":\"flood_level_rose\",\"district\":\"{}\",\"amount\":{},\"new_level\":{}}}",
            district.as_str(),
            amount,
            new_level
        ),
        FloodWatchEffect::DistrictInundated { district } => format!(
            "{{\"type\":\"district_inundated\",\"district\":\"{}\"}}",
            district.as_str()
        ),
        FloodWatchEffect::DeckExhausted => "{\"type\":\"deck_exhausted\"}".to_owned(),
        FloodWatchEffect::Terminal { outcome, summary } => format!(
            "{{\"type\":\"terminal\",\"outcome\":\"{}\",\"summary\":{}}}",
            escape_json(outcome),
            flood_terminal_summary_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn public_timeline_step_from_flood(
    step: &flood_watch::PublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}
