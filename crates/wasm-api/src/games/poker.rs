//! Browser-bridge helpers for `poker_lite` (hidden-information bounded-pledge game).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use poker_lite::replay_support::{
    import_public_export as poker_import_public_export, PublicReplayStep as PokerPublicReplayStep,
};
use poker_lite::{
    apply_action as poker_apply_action, project_view as poker_project_view,
    setup_match as poker_setup_match, PokerLiteEffect, PokerLiteSeat, PokerLiteState,
};

use crate::actors::poker_actor_for_seat;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::seats::{parse_poker_seat, seats};
use crate::store::{next_replay_id, REPLAYS};
use crate::{visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_poker_lite_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("viewer_scoped_observation_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(poker_lite::GAME_ID)
    )
}

pub(crate) fn import_poker_lite_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export =
        poker_lite::replay_support::PublicReplayExport::from_json(doc).map_err(|message| {
            diagnostic_string(
                "invalid_replay",
                &format!("invalid public replay document: {message}"),
            )
        })?;
    if export.rules_version != poker_lite::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {}", export.rules_version),
        ));
    }
    if export.variant != poker_lite::VARIANT_ID {
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
    let timeline = poker_import_public_export(&export);
    let replay_id = next_replay_id(GAME_POKER_LITE);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_POKER_LITE.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_poker)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_POKER_LITE),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

pub(crate) fn public_timeline_step_from_poker(step: &PokerPublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn poker_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(PokerLiteState, Vec<EffectEnvelope<PokerLiteEffect>>), String> {
    let seats = seats();
    let mut state = poker_setup_match(Seed(seed), &seats, &poker_lite::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_poker_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: poker_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = poker_lite::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(poker_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

pub(crate) fn poker_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &PokerLiteState,
    effects: &[EffectEnvelope<PokerLiteEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        poker_view_json(&poker_project_view(state, &viewer)),
        poker_effects_json(effects, &viewer)
    )
}

pub(crate) fn poker_view_json(view: &poker_lite::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"round\":{},\"private_counts\":{{\"seat_0\":{},\"seat_1\":{}}},\"center\":{},\"showdown\":{},\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.phase.as_str(),
        poker_optional_seat_json(view.active_seat),
        view.shared_pool,
        view.contributions[0],
        view.contributions[1],
        poker_round_json(&view.round),
        view.private_counts[0],
        view.private_counts[1],
        poker_center_json(&view.center),
        view.showdown
            .as_ref()
            .map_or_else(|| "null".to_owned(), poker_showdown_json),
        poker_terminal_json(&view.terminal),
        poker_terminal_rationale(&view.terminal)
            .map_or_else(|| "null".to_owned(), poker_rationale_json),
        view.freshness_token.0,
        poker_private_view_json(&view.private_view),
        poker_ui_json(&view.ui)
    )
}

pub(crate) fn poker_round_json(round: &poker_lite::visibility::RoundView) -> String {
    format!(
        "{{\"round_index\":{},\"round_unit\":{},\"outstanding_actor\":{},\"outstanding_amount\":{},\"lift_cap_remaining\":{}}}",
        round.round_index,
        round.round_unit,
        poker_optional_seat_json(round.outstanding_actor),
        round.outstanding_amount,
        round.lift_cap_remaining
    )
}

pub(crate) fn poker_center_json(center: &poker_lite::visibility::CenterView) -> String {
    match center {
        poker_lite::visibility::CenterView::Hidden { status } => {
            format!("{{\"status\":\"{}\",\"card\":null}}", escape_json(status))
        }
        poker_lite::visibility::CenterView::Revealed(card) => {
            format!(
                "{{\"status\":\"revealed\",\"card\":{}}}",
                poker_card_json(card)
            )
        }
    }
}

pub(crate) fn poker_showdown_json(showdown: &poker_lite::visibility::ShowdownView) -> String {
    format!(
        "{{\"seat_0_private\":{},\"seat_1_private\":{},\"center\":{},\"winner\":{}}}",
        poker_card_json(&showdown.seat_0_private),
        poker_card_json(&showdown.seat_1_private),
        poker_card_json(&showdown.center),
        poker_optional_seat_json(showdown.winner)
    )
}

pub(crate) fn poker_card_json(card: &poker_lite::visibility::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"copy\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.copy),
        escape_json(&card.label),
        escape_json(&card.accessibility_label)
    )
}

pub(crate) fn poker_terminal_json(terminal: &poker_lite::visibility::TerminalView) -> String {
    match terminal {
        poker_lite::visibility::TerminalView::NonTerminal => {
            "{\"terminal\":false,\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}".to_owned()
        }
        poker_lite::visibility::TerminalView::YieldWin {
            winner,
            loser,
            shared_pool,
            ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"yield_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"draw\":false,\"shared_pool\":{}}}",
            winner.as_str(),
            loser.as_str(),
            shared_pool
        ),
        poker_lite::visibility::TerminalView::ShowdownWin {
            winner,
            shared_pool,
            ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"showdown_win\",\"winner\":\"{}\",\"draw\":false,\"shared_pool\":{}}}",
            winner.as_str(),
            shared_pool
        ),
        poker_lite::visibility::TerminalView::Split {
            shared_pool, each, ..
        } => format!(
            "{{\"terminal\":true,\"kind\":\"split\",\"winner\":null,\"draw\":true,\"shared_pool\":{},\"each\":{}}}",
            shared_pool, each
        ),
    }
}

pub(crate) fn poker_terminal_rationale(
    terminal: &poker_lite::visibility::TerminalView,
) -> Option<&poker_lite::visibility::OutcomeRationaleView> {
    match terminal {
        poker_lite::visibility::TerminalView::NonTerminal => None,
        poker_lite::visibility::TerminalView::YieldWin { rationale, .. }
        | poker_lite::visibility::TerminalView::ShowdownWin { rationale, .. }
        | poker_lite::visibility::TerminalView::Split { rationale, .. } => Some(rationale),
    }
}

pub(crate) fn poker_rationale_json(
    rationale: &poker_lite::visibility::OutcomeRationaleView,
) -> String {
    let decisive_rule_ids = rationale
        .decisive_rule_ids
        .iter()
        .map(|rule_id| format!("\"{}\"", escape_json(rule_id)))
        .collect::<Vec<_>>()
        .join(",");
    let final_standing = rationale
        .per_seat
        .iter()
        .map(poker_rationale_standing_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"decisive_rule_ids\":[{}],\"final_standing\":[{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        decisive_rule_ids,
        final_standing
    )
}

pub(crate) fn poker_rationale_standing_json(
    breakdown: &poker_lite::visibility::SeatOutcomeBreakdownView,
) -> String {
    let mut values = vec![
        format!(
            "{{\"label\":\"Contribution\",\"value\":{}}}",
            breakdown.contribution
        ),
        format!(
            "{{\"label\":\"Allocation\",\"value\":{}}}",
            breakdown.allocation
        ),
    ];
    if let Some(strength) = &breakdown.strength {
        values.push(format!(
            "{{\"label\":\"Pair\",\"value\":\"{}\"}}",
            escape_json(&strength.pair_bucket)
        ));
        values.push(format!(
            "{{\"label\":\"Private rank\",\"value\":\"{}\"}}",
            escape_json(&strength.private_rank)
        ));
    }

    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"result\":\"{}\",\"emphasized\":{},\"values\":[{}]}}",
        breakdown.seat.as_str(),
        breakdown.seat.as_str(),
        escape_json(&breakdown.result),
        breakdown.result == "win",
        values.join(",")
    )
}

pub(crate) fn poker_private_view_json(
    private_view: &poker_lite::visibility::PrivateView,
) -> String {
    match private_view {
        poker_lite::visibility::PrivateView::Observer => {
            "{\"status\":\"observer\",\"own_private\":null,\"own_strength_bucket\":null}".to_owned()
        }
        poker_lite::visibility::PrivateView::Seat(private) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"own_private\":{},\"own_strength_bucket\":{}}}",
            private.seat.as_str(),
            private
                .own_private
                .as_ref()
                .map_or_else(|| "null".to_owned(), poker_card_json),
            private.own_strength_bucket.as_ref().map_or_else(
                || "null".to_owned(),
                |bucket| format!("\"{}\"", escape_json(bucket))
            )
        ),
    }
}

pub(crate) fn poker_ui_json(ui: &poker_lite::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"surface_label\":\"{}\",\"shared_pool_label\":\"{}\",\"hidden_center_label\":\"{}\",\"hidden_private_label\":\"{}\",\"hold_label\":\"{}\",\"press_label\":\"{}\",\"lift_label\":\"{}\",\"match_label\":\"{}\",\"yield_label\":\"{}\",\"reduced_motion_note\":\"{}\"}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.surface_label),
        escape_json(&ui.shared_pool_label),
        escape_json(&ui.hidden_center_label),
        escape_json(&ui.hidden_private_label),
        escape_json(&ui.hold_label),
        escape_json(&ui.press_label),
        escape_json(&ui.lift_label),
        escape_json(&ui.match_label),
        escape_json(&ui.yield_label),
        escape_json(&ui.reduced_motion_note)
    )
}

pub(crate) fn poker_optional_seat_json(seat: Option<PokerLiteSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

pub(crate) fn poker_effects_json(
    effects: &[EffectEnvelope<PokerLiteEffect>],
    viewer: &Viewer,
) -> String {
    let body = poker_lite::visibility::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(poker_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn poker_effect_json(effect: &EffectEnvelope<PokerLiteEffect>) -> String {
    let payload = match &effect.payload {
        PokerLiteEffect::PrivateCrestDealt { owner, card } => format!(
            "{{\"type\":\"private_crest_dealt\",\"owner\":\"{}\",\"card\":{}}}",
            owner.as_str(),
            poker_raw_card_json(*card)
        ),
        PokerLiteEffect::CrestDealStarted {
            private_count_per_seat,
            center_count,
            deck_tail_count,
        } => format!(
            "{{\"type\":\"crest_deal_started\",\"private_count_per_seat\":{},\"center_count\":{},\"deck_tail_count\":{}}}",
            private_count_per_seat, center_count, deck_tail_count
        ),
        PokerLiteEffect::OpeningPoolSet {
            contributions,
            shared_pool,
        } => format!(
            "{{\"type\":\"opening_pool_set\",\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"shared_pool\":{}}}",
            contributions[0], contributions[1], shared_pool
        ),
        PokerLiteEffect::PledgeHeld { actor, round_index } => format!(
            "{{\"type\":\"pledge_held\",\"actor\":\"{}\",\"round_index\":{}}}",
            actor.as_str(),
            round_index
        ),
        PokerLiteEffect::PledgePressed {
            actor,
            round_index,
            amount,
            shared_pool_after,
        } => format!(
            "{{\"type\":\"pledge_pressed\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after
        ),
        PokerLiteEffect::PledgeLifted {
            actor,
            round_index,
            amount,
            shared_pool_after,
            lift_cap_consumed,
        } => format!(
            "{{\"type\":\"pledge_lifted\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{},\"lift_cap_consumed\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after,
            lift_cap_consumed
        ),
        PokerLiteEffect::PledgeMatched {
            actor,
            round_index,
            amount,
            shared_pool_after,
        } => format!(
            "{{\"type\":\"pledge_matched\",\"actor\":\"{}\",\"round_index\":{},\"amount\":{},\"shared_pool_after\":{}}}",
            actor.as_str(),
            round_index,
            amount,
            shared_pool_after
        ),
        PokerLiteEffect::SeatYielded {
            actor,
            winner,
            shared_pool,
        } => format!(
            "{{\"type\":\"seat_yielded\",\"actor\":\"{}\",\"winner\":\"{}\",\"shared_pool\":{}}}",
            actor.as_str(),
            winner.as_str(),
            shared_pool
        ),
        PokerLiteEffect::CenterRevealStarted { group_id } => format!(
            "{{\"type\":\"center_reveal_started\",\"group_id\":\"{}\"}}",
            escape_json(group_id)
        ),
        PokerLiteEffect::CenterRevealed { group_id, center } => format!(
            "{{\"type\":\"center_revealed\",\"group_id\":\"{}\",\"center\":{}}}",
            escape_json(group_id),
            poker_raw_card_json(*center)
        ),
        PokerLiteEffect::ShowdownRevealStarted { group_id } => format!(
            "{{\"type\":\"showdown_reveal_started\",\"group_id\":\"{}\"}}",
            escape_json(group_id)
        ),
        PokerLiteEffect::ShowdownRevealed { group_id, reveal } => format!(
            "{{\"type\":\"showdown_revealed\",\"group_id\":\"{}\",\"reveal\":{}}}",
            escape_json(group_id),
            poker_reveal_json(*reveal)
        ),
        PokerLiteEffect::LedgerResolved {
            shared_pool,
            contributions,
            allocation,
        } => format!(
            "{{\"type\":\"ledger_resolved\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"allocation\":{}}}",
            shared_pool,
            contributions[0],
            contributions[1],
            poker_allocation_json(*allocation)
        ),
        PokerLiteEffect::Terminal { outcome } => format!(
            "{{\"type\":\"terminal\",\"outcome\":{}}}",
            poker_terminal_outcome_json(*outcome)
        ),
        PokerLiteEffect::BotChoseActionPublic {
            policy_id,
            action_family,
        } => format!(
            "{{\"type\":\"bot_chose_action\",\"policy_id\":\"{}\",\"action_family\":\"{}\"}}",
            escape_json(policy_id),
            escape_json(action_family)
        ),
        PokerLiteEffect::BotChoseActionPrivate {
            owner,
            policy_id,
            action_family,
            strength_bucket,
        } => format!(
            "{{\"type\":\"bot_chose_action_private\",\"owner\":\"{}\",\"policy_id\":\"{}\",\"action_family\":\"{}\",\"strength_bucket\":\"{}\"}}",
            owner.as_str(),
            escape_json(policy_id),
            escape_json(action_family),
            escape_json(strength_bucket)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn poker_raw_card_json(card: poker_lite::CrestCardId) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"copy\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        card.as_str(),
        card.rank().as_str(),
        card.rank().value(),
        card.rank_copy().as_str(),
        escape_json(&card.label()),
        escape_json(&poker_lite::card_accessibility_label(card))
    )
}

pub(crate) fn poker_reveal_json(reveal: poker_lite::ShowdownReveal) -> String {
    format!(
        "{{\"seat_0_private\":{},\"seat_1_private\":{},\"center\":{}}}",
        poker_raw_card_json(reveal.seat_0_private),
        poker_raw_card_json(reveal.seat_1_private),
        poker_raw_card_json(reveal.center)
    )
}

pub(crate) fn poker_allocation_json(allocation: poker_lite::effects::LedgerAllocation) -> String {
    match allocation {
        poker_lite::effects::LedgerAllocation::Winner { seat, amount } => format!(
            "{{\"kind\":\"winner\",\"seat\":\"{}\",\"amount\":{}}}",
            seat.as_str(),
            amount
        ),
        poker_lite::effects::LedgerAllocation::Split { each } => {
            format!("{{\"kind\":\"split\",\"each\":{each}}}")
        }
    }
}

pub(crate) fn poker_terminal_outcome_json(outcome: poker_lite::TerminalOutcome) -> String {
    match outcome {
        poker_lite::TerminalOutcome::YieldWin {
            winner,
            loser,
            shared_pool,
            contributions,
        } => format!(
            "{{\"kind\":\"yield_win\",\"winner\":\"{}\",\"loser\":\"{}\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}}}}",
            winner.as_str(),
            loser.as_str(),
            shared_pool,
            contributions[0],
            contributions[1]
        ),
        poker_lite::TerminalOutcome::ShowdownWin {
            winner,
            shared_pool,
            contributions,
            reveal,
        } => format!(
            "{{\"kind\":\"showdown_win\",\"winner\":\"{}\",\"shared_pool\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"reveal\":{}}}",
            winner.as_str(),
            shared_pool,
            contributions[0],
            contributions[1],
            poker_reveal_json(reveal)
        ),
        poker_lite::TerminalOutcome::Split {
            shared_pool,
            each,
            contributions,
            reveal,
        } => format!(
            "{{\"kind\":\"split\",\"shared_pool\":{},\"each\":{},\"contributions\":{{\"seat_0\":{},\"seat_1\":{}}},\"reveal\":{}}}",
            shared_pool,
            each,
            contributions[0],
            contributions[1],
            poker_reveal_json(reveal)
        ),
    }
}
