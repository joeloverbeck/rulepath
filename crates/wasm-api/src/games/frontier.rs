//! Browser-bridge helpers for `frontier_control` (perfect-information graph map).

use engine_core::{EffectEnvelope, Viewer};
use frontier_control::FrontierControlEffect;

use crate::constants::*;
use crate::json::{diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{parse_frontier_public_replay_steps, PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{
    option_bool_json, option_string_json, string_array, visibility_json, AppliedCommand,
    ReplayRecord,
};

pub(crate) fn is_frontier_control_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(frontier_control::GAME_ID)
    ) && string_field(doc, "rules_version_label").is_ok()
        && doc.contains("\"not_applicable\"")
}

pub(crate) fn import_frontier_control_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    frontier_control::import_public_export_json(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version_label")?;
    if rules_version != frontier_control::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != frontier_control::VARIANT_STANDARD_ID
        && variant != frontier_control::VARIANT_HIGHLANDS_ID
    {
        return Err(diagnostic_string(
            "unsupported_replay_variant",
            &format!("unsupported replay variant: {variant}"),
        ));
    }
    let steps = parse_frontier_public_replay_steps(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = frontier_control::PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        game_id: frontier_control::GAME_ID.to_owned(),
        rules_version_label: rules_version,
        variant,
        not_applicable: frontier_control::trace_not_applicable(),
        steps: steps
            .iter()
            .map(frontier_step_from_public_timeline)
            .collect(),
    };
    let imported = frontier_control::import_public_export(&export);
    let timeline_steps = export
        .steps
        .iter()
        .map(public_timeline_step_from_frontier)
        .collect::<Vec<_>>();
    let step_count = timeline_steps.len();
    let replay_id = next_replay_id(GAME_FRONTIER_CONTROL);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_FRONTIER_CONTROL.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: "observer".to_owned(),
                    steps: timeline_steps,
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"observer\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0,\"raw_size\":{}}}",
        escape_json(&replay_id),
        escape_json(GAME_FRONTIER_CONTROL),
        step_count,
        imported.raw_json.len()
    ))
}

pub(crate) fn frontier_step_from_public_timeline(
    step: &PublicTimelineStep,
) -> frontier_control::PublicReplayStep {
    frontier_control::PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn frontier_command_summary(command: &AppliedCommand) -> String {
    format!(
        "{}:{}:{}",
        command.actor_seat,
        command.action_path.join("/"),
        command.freshness_token
    )
}

pub(crate) fn frontier_view_json(view: &frontier_control::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"seats\":[{}],\"factions\":[{}],\"round_number\":{},\"active_faction\":\"{}\",\"active_seat\":{},\"phase\":{},\"sites\":[{}],\"scores\":{},\"terminal\":{},\"freshness_token\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        string_array(&view.seats),
        view.factions
            .iter()
            .map(frontier_faction_view_json)
            .collect::<Vec<_>>()
            .join(","),
        view.round_number,
        view.active_faction.as_str(),
        option_string_json(view.active_seat.as_deref()),
        frontier_phase_json(view.phase),
        view.sites
            .iter()
            .map(frontier_site_json)
            .collect::<Vec<_>>()
            .join(","),
        frontier_score_json(&view.scores),
        frontier_terminal_json(&view.terminal),
        view.freshness_token
    )
}

pub(crate) fn frontier_faction_view_json(faction: &frontier_control::FactionView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"faction\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&faction.seat),
        faction.faction.as_str(),
        escape_json(&faction.label)
    )
}

pub(crate) fn frontier_phase_json(phase: frontier_control::PhaseView) -> String {
    match phase {
        frontier_control::PhaseView::Action { budget_remaining } => {
            format!("{{\"kind\":\"action\",\"budget_remaining\":{budget_remaining}}}")
        }
        frontier_control::PhaseView::Terminal => {
            "{\"kind\":\"terminal\",\"budget_remaining\":0}".to_owned()
        }
    }
}

pub(crate) fn frontier_site_json(site: &frontier_control::SiteView) -> String {
    format!(
        "{{\"site\":\"{}\",\"label\":\"{}\",\"guards\":{},\"crews\":{},\"stake\":{},\"fort\":{},\"stake_value\":{},\"supplied\":{}}}",
        site.site.as_str(),
        escape_json(site.label),
        site.guards,
        site.crews,
        site.stake,
        site.fort,
        site.stake_value,
        option_bool_json(site.supplied)
    )
}

pub(crate) fn frontier_score_json(score: &frontier_control::ScoreView) -> String {
    format!(
        "{{\"garrison\":{},\"prospectors\":{}}}",
        score.garrison, score.prospectors
    )
}

pub(crate) fn frontier_terminal_json(terminal: &frontier_control::TerminalView) -> String {
    match terminal {
        frontier_control::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null}".to_owned()
        }
        frontier_control::TerminalView::Winner {
            faction,
            scores,
            garrison_tiebreak,
            summary,
        } => format!(
            "{{\"kind\":\"winner\",\"winner\":\"{}\",\"scores\":{},\"garrison_tiebreak\":{},\"summary\":\"{}\"}}",
            faction.as_str(),
            frontier_score_json(scores),
            garrison_tiebreak,
            escape_json(summary)
        ),
    }
}

pub(crate) fn frontier_effects_json(
    effects: &[EffectEnvelope<FrontierControlEffect>],
    viewer: &Viewer,
) -> String {
    let body = frontier_control::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(frontier_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn frontier_effect_json(effect: &EffectEnvelope<FrontierControlEffect>) -> String {
    let payload = match &effect.payload {
        FrontierControlEffect::CrewMarched { from, to } => format!(
            "{{\"type\":\"crew_marched\",\"from\":\"{}\",\"to\":\"{}\"}}",
            from.as_str(),
            to.as_str()
        ),
        FrontierControlEffect::GuardPatrolled { from, to } => format!(
            "{{\"type\":\"guard_patrolled\",\"from\":\"{}\",\"to\":\"{}\"}}",
            from.as_str(),
            to.as_str()
        ),
        FrontierControlEffect::ClashResolved {
            site,
            guard_removed,
            crew_removed,
            entering_faction,
        } => format!(
            "{{\"type\":\"clash_resolved\",\"site\":\"{}\",\"guard_removed\":{},\"crew_removed\":{},\"entering_faction\":\"{}\"}}",
            site.as_str(),
            guard_removed,
            crew_removed,
            entering_faction.as_str()
        ),
        FrontierControlEffect::StakePlaced { site } => format!(
            "{{\"type\":\"stake_placed\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        FrontierControlEffect::StakeDismantled { site } => format!(
            "{{\"type\":\"stake_dismantled\",\"site\":\"{}\"}}",
            site.as_str()
        ),
        FrontierControlEffect::CrewMustered { site, crews } => format!(
            "{{\"type\":\"crew_mustered\",\"site\":\"{}\",\"crews\":{}}}",
            site.as_str(),
            crews
        ),
        FrontierControlEffect::GuardReinforced { site, guards } => format!(
            "{{\"type\":\"guard_reinforced\",\"site\":\"{}\",\"guards\":{}}}",
            site.as_str(),
            guards
        ),
        FrontierControlEffect::TurnEnded { faction, round } => format!(
            "{{\"type\":\"turn_ended\",\"faction\":\"{}\",\"round\":{}}}",
            faction.as_str(),
            round
        ),
        FrontierControlEffect::RoundScored {
            round,
            garrison_points,
            prospector_points,
            fort_breakdown,
            stake_breakdown,
        } => format!(
            "{{\"type\":\"round_scored\",\"round\":{},\"garrison_points\":{},\"prospector_points\":{},\"fort_breakdown\":[{}],\"stake_breakdown\":[{}]}}",
            round,
            garrison_points,
            prospector_points,
            fort_breakdown
                .iter()
                .map(frontier_fort_score_json)
                .collect::<Vec<_>>()
                .join(","),
            stake_breakdown
                .iter()
                .map(frontier_stake_score_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        FrontierControlEffect::Terminal {
            winner,
            garrison_total,
            prospector_total,
            tiebreak_applied,
            summary,
        } => format!(
            "{{\"type\":\"terminal\",\"winner\":\"{}\",\"garrison_total\":{},\"prospector_total\":{},\"tiebreak_applied\":{},\"summary\":\"{}\"}}",
            winner.as_str(),
            garrison_total,
            prospector_total,
            tiebreak_applied,
            escape_json(summary)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn frontier_fort_score_json(breakdown: &frontier_control::FortScoreBreakdown) -> String {
    format!(
        "{{\"site\":\"{}\",\"held\":{},\"points\":{}}}",
        breakdown.site.as_str(),
        breakdown.held,
        breakdown.points
    )
}

pub(crate) fn frontier_stake_score_json(
    breakdown: &frontier_control::StakeScoreBreakdown,
) -> String {
    format!(
        "{{\"site\":\"{}\",\"value\":{},\"supplied\":{},\"points\":{}}}",
        breakdown.site.as_str(),
        breakdown.value,
        breakdown.supplied,
        breakdown.points
    )
}

pub(crate) fn public_timeline_step_from_frontier(
    step: &frontier_control::PublicReplayStep,
) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.command_summary.clone(),
        terminal: step.terminal,
    }
}
