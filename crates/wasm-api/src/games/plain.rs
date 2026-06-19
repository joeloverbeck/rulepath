//! Browser-bridge helpers for `plain_tricks` (hidden-information trick-taking game).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use plain_tricks::replay_support::{
    import_public_export as plain_import_public_export, PublicReplayStep as PlainPublicReplayStep,
};
use plain_tricks::{
    apply_action as plain_apply_action, project_view as plain_project_view,
    setup_match as plain_setup_match, PlainTricksEffect, PlainTricksSeat, PlainTricksState,
};

use crate::actors::plain_actor_for_seat;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::seats::{parse_plain_seat, plain_seats};
use crate::store::{next_replay_id, REPLAYS};
use crate::{
    effect_visible_to_viewer, option_string_json, string_array, visibility_json, AppliedCommand,
    ReplayRecord,
};

pub(crate) fn is_plain_tricks_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(plain_tricks::GAME_ID)
    )
}

pub(crate) fn import_plain_tricks_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        plain_tricks::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != plain_tricks::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != plain_tricks::VARIANT_ID {
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
    let timeline = plain_import_public_export(&export);
    let replay_id = next_replay_id(GAME_PLAIN_TRICKS);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_PLAIN_TRICKS.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_plain)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_PLAIN_TRICKS),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

pub(crate) fn public_timeline_step_from_plain(step: &PlainPublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn plain_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(PlainTricksState, Vec<EffectEnvelope<PlainTricksEffect>>), String> {
    let seats = plain_seats();
    let mut state = plain_setup_match(Seed(seed), &seats, &plain_tricks::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_plain_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: plain_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = plain_tricks::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(plain_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

pub(crate) fn plain_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &PlainTricksState,
    effects: &[EffectEnvelope<PlainTricksEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        plain_view_json(&plain_project_view(state, &viewer)),
        plain_effects_json(effects, &viewer)
    )
}

pub(crate) fn plain_view_json(view: &plain_tricks::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"round_index\":{},\"trick_index\":{},\"round_leader\":\"{}\",\"current_leader\":\"{}\",\"hand_counts\":{},\"current_trick\":{},\"trick_history\":[{}],\"round_trick_counts\":{},\"total_trick_counts\":{},\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        option_plain_seat_json(view.active_seat),
        view.round_index,
        view.trick_index,
        view.round_leader.as_str(),
        view.current_leader.as_str(),
        plain_counts_json(view.hand_counts.seat_0, view.hand_counts.seat_1),
        plain_current_trick_json(&view.current_trick),
        view.trick_history
            .iter()
            .map(plain_completed_trick_json)
            .collect::<Vec<_>>()
            .join(","),
        plain_counts_json(view.round_trick_counts.seat_0, view.round_trick_counts.seat_1),
        plain_counts_json(view.total_trick_counts.seat_0, view.total_trick_counts.seat_1),
        plain_terminal_json(&view.terminal),
        plain_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), plain_outcome_rationale_json),
        view.freshness_token.0,
        plain_private_view_json(&view.private_view),
        plain_ui_json(&view.ui)
    )
}

pub(crate) fn plain_counts_json(seat_0: u8, seat_1: u8) -> String {
    format!("{{\"seat_0\":{},\"seat_1\":{}}}", seat_0, seat_1)
}

pub(crate) fn plain_current_trick_json(trick: &plain_tricks::CurrentTrickView) -> String {
    format!(
        "{{\"led_suit\":{},\"plays\":[{}]}}",
        option_string_json(trick.led_suit.as_deref()),
        trick
            .plays
            .iter()
            .map(plain_played_card_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn plain_completed_trick_json(trick: &plain_tricks::CompletedTrickView) -> String {
    format!(
        "{{\"round_index\":{},\"trick_index\":{},\"leader\":\"{}\",\"plays\":[{},{}],\"winner\":\"{}\",\"trick_counts_after\":{}}}",
        trick.round_index,
        trick.trick_index,
        trick.leader.as_str(),
        plain_played_card_json(&trick.plays[0]),
        plain_played_card_json(&trick.plays[1]),
        trick.winner.as_str(),
        plain_counts_json(
            trick.trick_counts_after.seat_0,
            trick.trick_counts_after.seat_1
        )
    )
}

pub(crate) fn plain_played_card_json(play: &plain_tricks::PlayedCardView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"card\":{}}}",
        play.seat.as_str(),
        plain_card_json(&play.card)
    )
}

pub(crate) fn plain_card_json(card: &plain_tricks::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"suit\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.suit),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.label),
        escape_json(&card.accessibility_label)
    )
}

pub(crate) fn plain_terminal_json(terminal: &plain_tricks::TerminalView) -> String {
    match terminal {
        plain_tricks::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        plain_tricks::TerminalView::TrickWin { winner, totals, .. } => format!(
            "{{\"kind\":\"trick_win\",\"winner\":\"{}\",\"draw\":false,\"totals\":{}}}",
            winner.as_str(),
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
        plain_tricks::TerminalView::Split { each, totals, .. } => format!(
            "{{\"kind\":\"split\",\"winner\":null,\"draw\":true,\"each\":{},\"totals\":{}}}",
            each,
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
    }
}

pub(crate) fn plain_terminal_rationale(
    terminal: &plain_tricks::TerminalView,
) -> Option<&plain_tricks::OutcomeRationaleView> {
    match terminal {
        plain_tricks::TerminalView::NonTerminal => None,
        plain_tricks::TerminalView::TrickWin { rationale, .. }
        | plain_tricks::TerminalView::Split { rationale, .. } => Some(rationale),
    }
}

pub(crate) fn plain_outcome_rationale_json(
    rationale: &plain_tricks::OutcomeRationaleView,
) -> String {
    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"decisive_rule_ids\":[{}],\"per_seat\":[{},{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        string_array(&rationale.decisive_rule_ids),
        plain_outcome_breakdown_json(&rationale.per_seat[0]),
        plain_outcome_breakdown_json(&rationale.per_seat[1])
    )
}

pub(crate) fn plain_outcome_breakdown_json(
    breakdown: &plain_tricks::SeatOutcomeBreakdownView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"total_tricks\":{},\"result\":\"{}\"}}",
        breakdown.seat.as_str(),
        breakdown.total_tricks,
        escape_json(&breakdown.result)
    )
}

pub(crate) fn plain_private_view_json(private: &plain_tricks::PrivateView) -> String {
    match private {
        plain_tricks::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_hand\":[]}".to_owned()
        }
        plain_tricks::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_hand\":[{}]}}",
            view.seat.as_str(),
            view.own_hand
                .iter()
                .map(plain_card_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

pub(crate) fn plain_ui_json(ui: &plain_tricks::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"table_label\":\"{}\",\"own_hand_label\":\"{}\",\"opponent_hand_label\":\"{}\",\"current_trick_label\":\"{}\",\"trick_history_label\":\"{}\",\"score_label\":\"{}\",\"play_action_label\":\"{}\",\"observer_disabled_reason\":\"{}\",\"reduced_motion_note\":\"{}\",\"rules_summary\":[{}]}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.table_label),
        escape_json(&ui.own_hand_label),
        escape_json(&ui.opponent_hand_label),
        escape_json(&ui.current_trick_label),
        escape_json(&ui.trick_history_label),
        escape_json(&ui.score_label),
        escape_json(&ui.play_action_label),
        escape_json(&ui.observer_disabled_reason),
        escape_json(&ui.reduced_motion_note),
        string_array(&ui.rules_summary)
    )
}

pub(crate) fn plain_effects_json(
    effects: &[EffectEnvelope<PlainTricksEffect>],
    viewer: &Viewer,
) -> String {
    let body = effects
        .iter()
        .filter(|effect| effect_visible_to_viewer(&effect.visibility, viewer))
        .map(plain_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn plain_effect_json(effect: &EffectEnvelope<PlainTricksEffect>) -> String {
    let payload = match &effect.payload {
        PlainTricksEffect::DealStarted {
            round_index,
            cards_per_seat,
            tail_count,
        } => format!(
            "{{\"type\":\"deal_started\",\"round_index\":{},\"cards_per_seat\":{},\"tail_count\":{}}}",
            round_index, cards_per_seat, tail_count
        ),
        PlainTricksEffect::HandDealt { owner, cards } => format!(
            "{{\"type\":\"hand_dealt\",\"owner\":\"{}\",\"cards\":[{}]}}",
            owner.as_str(),
            cards
                .iter()
                .map(|card| format!("\"{}\"", card.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        PlainTricksEffect::DealCompleted {
            round_index,
            cards_per_seat,
            tail_count,
            leader,
        } => format!(
            "{{\"type\":\"deal_completed\",\"round_index\":{},\"cards_per_seat\":{},\"tail_count\":{},\"leader\":\"{}\"}}",
            round_index, cards_per_seat, tail_count, leader.as_str()
        ),
        PlainTricksEffect::CardPlayed {
            seat,
            card,
            round_index,
            trick_index,
            led,
        } => format!(
            "{{\"type\":\"card_played\",\"seat\":\"{}\",\"card\":\"{}\",\"round_index\":{},\"trick_index\":{},\"led\":{}}}",
            seat.as_str(),
            card.as_str(),
            round_index,
            trick_index,
            led
        ),
        PlainTricksEffect::TrickResolved {
            round_index,
            trick_index,
            plays,
            winner,
            trick_counts,
        } => format!(
            "{{\"type\":\"trick_resolved\",\"round_index\":{},\"trick_index\":{},\"plays\":[{},{}],\"winner\":\"{}\",\"trick_counts\":{}}}",
            round_index,
            trick_index,
            plain_trick_play_json(plays[0]),
            plain_trick_play_json(plays[1]),
            winner.as_str(),
            plain_counts_json(trick_counts.seat_0, trick_counts.seat_1)
        ),
        PlainTricksEffect::RoundScored {
            round_index,
            round_counts,
            total_counts,
        } => format!(
            "{{\"type\":\"round_scored\",\"round_index\":{},\"round_counts\":{},\"total_counts\":{}}}",
            round_index,
            plain_counts_json(round_counts.seat_0, round_counts.seat_1),
            plain_counts_json(total_counts.seat_0, total_counts.seat_1)
        ),
        PlainTricksEffect::DealRotated {
            round_index,
            leader,
        } => format!(
            "{{\"type\":\"deal_rotated\",\"round_index\":{},\"leader\":\"{}\"}}",
            round_index,
            leader.as_str()
        ),
        PlainTricksEffect::MatchResolved {
            totals,
            decisive_cause,
        } => format!(
            "{{\"type\":\"match_resolved\",\"totals\":{},\"decisive_cause\":\"{}\"}}",
            plain_counts_json(totals.seat_0, totals.seat_1),
            escape_json(decisive_cause)
        ),
        PlainTricksEffect::Terminal {
            outcome,
            decisive_cause,
        } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{},\"decisive_cause\":\"{}\"}}",
            plain_terminal_outcome_json(*outcome),
            escape_json(decisive_cause)
        ),
        PlainTricksEffect::BotChoseActionPublic {
            policy_id,
            action_family,
        } => format!(
            "{{\"type\":\"bot_chose_action_public\",\"policy_id\":\"{}\",\"action_family\":\"{}\"}}",
            escape_json(policy_id),
            escape_json(action_family)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn plain_trick_play_json(play: plain_tricks::TrickPlay) -> String {
    format!(
        "{{\"seat\":\"{}\",\"card\":\"{}\"}}",
        play.seat.as_str(),
        play.card.as_str()
    )
}

pub(crate) fn plain_terminal_outcome_json(outcome: plain_tricks::TerminalOutcome) -> String {
    match outcome {
        plain_tricks::TerminalOutcome::TrickWin { winner, totals } => format!(
            "{{\"kind\":\"trick_win\",\"winner\":\"{}\",\"totals\":{}}}",
            winner.as_str(),
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
        plain_tricks::TerminalOutcome::Split { each, totals } => format!(
            "{{\"kind\":\"split\",\"winner\":null,\"each\":{},\"totals\":{}}}",
            each,
            plain_counts_json(totals.seat_0, totals.seat_1)
        ),
    }
}

pub(crate) fn option_plain_seat_json(seat: Option<PlainTricksSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}
