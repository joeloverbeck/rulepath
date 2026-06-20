//! Browser-bridge helpers for `river_ledger` (hidden-information multi-seat ledger game).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use river_ledger::{
    apply_action as river_apply_action, project_view as river_project_view,
    setup_match as river_setup_match, RiverLedgerEffect, RiverLedgerSeat, RiverLedgerState,
};

use crate::actors::river_actor_for_seat;
use crate::constants::*;
use crate::json::{diagnostic_json, escape_json};
use crate::seats::{parse_river_seat, river_seats_for_count};
use crate::{option_string_json, AppliedCommand};

pub(crate) fn river_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(RiverLedgerState, Vec<EffectEnvelope<RiverLedgerEffect>>), String> {
    let seats = river_seats_for_count(6);
    let mut state = river_setup_match(Seed(seed), &seats, &river_ledger::SetupOptions::default())
        .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_river_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: river_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = river_ledger::validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(river_apply_action(&mut state, action).map_err(diagnostic_json)?);
    }
    Ok((state, all_effects))
}

pub(crate) fn river_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &RiverLedgerState,
    effects: &[EffectEnvelope<RiverLedgerEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        river_view_json(&river_project_view(state, &viewer)),
        river_effects_json(effects, &viewer)
    )
}

pub(crate) fn river_view_json(view: &river_ledger::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"phase\":\"{}\",\"active_seat\":{},\"active_seat_labels\":[{}],\"button\":\"{}\",\"small_blind\":\"{}\",\"big_blind\":\"{}\",\"pot_total\":{},\"seats\":[{}],\"board\":[{}],\"board_slots\":[{}],\"terminal\":{},\"terminal_rationale\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        river_phase_label(view.phase),
        option_river_seat_json(view.active_seat),
        view.active_seat_labels
            .iter()
            .map(river_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&view.button.as_str()),
        escape_json(&view.small_blind.as_str()),
        escape_json(&view.big_blind.as_str()),
        view.pot_total,
        view.seats
            .iter()
            .map(river_seat_view_json)
            .collect::<Vec<_>>()
            .join(","),
        view.board
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        view.board_slots
            .iter()
            .map(river_board_slot_json)
            .collect::<Vec<_>>()
            .join(","),
        river_terminal_json(&view.terminal),
        view.terminal_rationale
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_rationale_json),
        view.freshness_token.0,
        river_private_view_json(&view.private_view),
        river_ui_json(&view.ui)
    )
}

pub(crate) fn river_phase_label(phase: river_ledger::Phase) -> &'static str {
    match phase {
        river_ledger::Phase::Setup => "setup",
        river_ledger::Phase::Betting { street } => street.as_str(),
        river_ledger::Phase::Showdown => "showdown",
        river_ledger::Phase::Terminal => "terminal",
    }
}

pub(crate) fn option_river_seat_json(seat: Option<RiverLedgerSeat>) -> String {
    seat.map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
        .unwrap_or_else(|| "null".to_owned())
}

pub(crate) fn river_seat_view_json(seat: &river_ledger::visibility::SeatView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"status\":\"{}\",\"street_contribution\":{},\"total_contribution\":{},\"hidden_hole_count\":{},\"ledger_display\":{}}}",
        escape_json(&seat.seat.as_str()),
        river_seat_status_label(seat.status),
        seat.street_contribution,
        seat.total_contribution,
        seat.hidden_hole_count,
        river_seat_ledger_display_json(&seat.ledger_display)
    )
}

pub(crate) fn river_seat_ledger_display_json(
    display: &river_ledger::ui::RiverLedgerSeatLedgerDisplay,
) -> String {
    format!(
        "{{\"round_contribution\":{},\"hand_contribution\":{},\"hole_card_summary\":{},\"role_badges\":[{}],\"status_label\":\"{}\"}}",
        river_seat_ledger_field_json(&display.round_contribution),
        river_seat_ledger_field_json(&display.hand_contribution),
        river_seat_ledger_field_json(&display.hole_card_summary),
        display
            .role_badges
            .iter()
            .map(|badge| format!("\"{}\"", escape_json(badge)))
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&display.status_label)
    )
}

pub(crate) fn river_seat_ledger_field_json(
    field: &river_ledger::ui::RiverLedgerSeatLedgerField,
) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&field.label),
        escape_json(&field.value),
        escape_json(&field.accessibility_label)
    )
}

pub(crate) fn river_seat_status_label(status: river_ledger::SeatStatus) -> &'static str {
    match status {
        river_ledger::SeatStatus::Live => "live",
        river_ledger::SeatStatus::Folded => "folded",
        river_ledger::SeatStatus::ShowdownEligible => "showdown_eligible",
    }
}

pub(crate) fn river_card_json(card: &river_ledger::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":\"{}\",\"rank_value\":{},\"suit\":\"{}\",\"label\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        escape_json(&card.rank),
        card.rank_value,
        escape_json(&card.suit),
        escape_json(&card.label),
        escape_json(&format!("{} of {}", card.rank, card.suit))
    )
}

pub(crate) fn river_board_slot_json(slot: &river_ledger::visibility::BoardSlotView) -> String {
    format!(
        "{{\"slot\":\"{}\",\"reveal_state\":\"{}\",\"street_label\":\"{}\",\"visual_placeholder_label\":\"{}\",\"accessibility_label\":\"{}\",\"card\":{}}}",
        escape_json(&slot.slot),
        escape_json(&slot.reveal_state),
        escape_json(&slot.street_label),
        escape_json(&slot.visual_placeholder_label),
        escape_json(&slot.accessibility_label),
        slot.card
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_card_json)
    )
}

pub(crate) fn river_terminal_json(terminal: &river_ledger::visibility::TerminalView) -> String {
    match terminal {
        river_ledger::visibility::TerminalView::NonTerminal => {
            "{\"kind\":\"non_terminal\",\"terminal\":false,\"winners\":[],\"pot_total\":0,\"allocations\":[],\"explanations\":[]}".to_owned()
        }
        river_ledger::visibility::TerminalView::LastLiveHand { winner, pot_total } => format!(
            "{{\"kind\":\"last_live_hand\",\"terminal\":true,\"winners\":[\"{}\"],\"pot_total\":{},\"allocations\":[{{\"seat\":\"{}\",\"amount\":{}}}],\"explanations\":[]}}",
            escape_json(&winner.as_str()),
            pot_total,
            escape_json(&winner.as_str()),
            pot_total
        ),
        river_ledger::visibility::TerminalView::Showdown {
            winners,
            pot_total,
            allocations,
            explanations,
            presentation_v2,
        } => format!(
            "{{\"kind\":\"showdown\",\"terminal\":true,\"winners\":[{}],\"pot_total\":{},\"allocations\":[{}],\"explanations\":[{}],\"presentation_v2\":{}}}",
            winners
                .iter()
                .map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
                .collect::<Vec<_>>()
                .join(","),
            pot_total,
            allocations
                .iter()
                .map(|(seat, amount)| format!(
                    "{{\"seat\":\"{}\",\"amount\":{}}}",
                    escape_json(&seat.as_str()),
                    amount
                ))
                .collect::<Vec<_>>()
                .join(","),
            explanations
                .iter()
                .map(|explanation| format!("\"{}\"", escape_json(explanation)))
                .collect::<Vec<_>>()
                .join(","),
            river_showdown_presentation_v2_json(presentation_v2)
        ),
    }
}

pub(crate) fn river_showdown_presentation_v2_json(
    presentation: &river_ledger::visibility::ShowdownPresentationV2View,
) -> String {
    format!(
        "{{\"result_banner\":{},\"decisive_reason\":{},\"board_cards\":[{}],\"standings\":[{}],\"folded_rows\":[{}]}}",
        river_showdown_result_banner_json(&presentation.result_banner),
        river_showdown_decisive_reason_json(&presentation.decisive_reason),
        presentation
            .board_cards
            .iter()
            .map(river_showdown_board_card_json)
            .collect::<Vec<_>>()
            .join(","),
        presentation
            .standings
            .iter()
            .map(river_showdown_standing_json)
            .collect::<Vec<_>>()
            .join(","),
        presentation
            .folded_rows
            .iter()
            .map(river_showdown_folded_row_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn river_showdown_result_banner_json(
    banner: &river_ledger::visibility::ShowdownResultBannerView,
) -> String {
    format!(
        "{{\"headline\":\"{}\",\"subheadline\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&banner.headline),
        escape_json(&banner.subheadline),
        escape_json(&banner.accessibility_label)
    )
}

pub(crate) fn river_showdown_decisive_reason_json(
    reason: &river_ledger::visibility::ShowdownDecisiveReasonView,
) -> String {
    format!(
        "{{\"short_text\":\"{}\",\"contrast_seat\":{},\"contrast_seat_label\":{},\"rule_refs\":[{}]}}",
        escape_json(&reason.short_text),
        reason
            .contrast_seat
            .map(|seat| format!("\"{}\"", escape_json(&seat.as_str())))
            .unwrap_or_else(|| "null".to_owned()),
        reason
            .contrast_seat_label
            .as_ref()
            .map(|label| format!("\"{}\"", escape_json(label)))
            .unwrap_or_else(|| "null".to_owned()),
        reason
            .rule_refs
            .iter()
            .map(|rule| format!("\"{}\"", escape_json(rule)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn river_showdown_board_card_json(
    card: &river_ledger::visibility::ShowdownBoardCardPresentationView,
) -> String {
    format!(
        "{{\"slot\":\"{}\",\"card\":{},\"public_label\":\"{}\",\"used_by_selected\":[{}]}}",
        escape_json(&card.slot),
        river_card_json(&card.card),
        escape_json(&card.public_label),
        card.used_by_selected
            .iter()
            .map(|seat| format!("\"{}\"", escape_json(seat)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn river_showdown_standing_json(
    standing: &river_ledger::visibility::ShowdownStandingPresentationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"rank\":{},\"result_label\":\"{}\",\"allocation_label\":\"{}\",\"hand_name\":\"{}\",\"short_comparison_note\":\"{}\",\"rank_ladder_label\":\"{}\",\"hole_cards\":[{}],\"board_cards\":[{}],\"best_five\":[{}],\"best_five_accessibility_label\":\"{}\",\"detail_rows\":[{}],\"default_expanded\":{}}}",
        escape_json(&standing.seat.as_str()),
        escape_json(&standing.seat_label),
        standing.rank,
        escape_json(&standing.result_label),
        escape_json(&standing.allocation_label),
        escape_json(&standing.hand_name),
        escape_json(&standing.short_comparison_note),
        escape_json(&standing.rank_ladder_label),
        standing
            .hole_cards
            .iter()
            .map(river_showdown_card_usage_json)
            .collect::<Vec<_>>()
            .join(","),
        standing
            .board_cards
            .iter()
            .map(river_showdown_card_usage_json)
            .collect::<Vec<_>>()
            .join(","),
        standing
            .best_five
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&standing.best_five_accessibility_label),
        standing
            .detail_rows
            .iter()
            .map(river_showdown_detail_row_json)
            .collect::<Vec<_>>()
            .join(","),
        standing.default_expanded
    )
}

pub(crate) fn river_showdown_card_usage_json(
    mark: &river_ledger::visibility::ShowdownCardUsageMarkView,
) -> String {
    format!(
        "{{\"card\":{},\"public_label\":\"{}\",\"used_in_best_five\":{}}}",
        river_card_json(&mark.card),
        escape_json(&mark.public_label),
        mark.used_in_best_five
    )
}

pub(crate) fn river_showdown_detail_row_json(
    row: &river_ledger::visibility::ShowdownDetailRowView,
) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\"}}",
        escape_json(&row.label),
        escape_json(&row.value)
    )
}

pub(crate) fn river_showdown_folded_row_json(
    row: &river_ledger::visibility::ShowdownFoldedRowPresentationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"redaction_label\":\"{}\"}}",
        escape_json(&row.seat.as_str()),
        escape_json(&row.seat_label),
        escape_json(&row.redaction_label)
    )
}

pub(crate) fn river_rationale_json(
    rationale: &river_ledger::visibility::OutcomeRationaleView,
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
        .map(river_rationale_standing_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"result_kind\":\"{}\",\"decisive_cause\":\"{}\",\"template_key\":\"{}\",\"headline\":{},\"decisive_comparison\":{},\"comparison_basis\":{},\"decisive_rule_ids\":[{}],\"final_standing\":[{}]}}",
        escape_json(&rationale.result_kind),
        escape_json(&rationale.decisive_cause),
        escape_json(&rationale.template_key),
        option_string_json(rationale.headline.as_deref()),
        option_string_json(rationale.decisive_comparison.as_deref()),
        option_string_json(rationale.comparison_basis.as_deref()),
        decisive_rule_ids,
        final_standing
    )
}

pub(crate) fn river_rationale_standing_json(
    breakdown: &river_ledger::visibility::SeatOutcomeBreakdownView,
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
            "{{\"label\":\"Result label\",\"value\":\"{}\"}}",
            escape_json(&strength.result_label)
        ));
        values.push(format!(
            "{{\"label\":\"Hand\",\"value\":\"{}\"}}",
            escape_json(&strength.hand_name)
        ));
        values.push(format!(
            "{{\"label\":\"Rank explanation\",\"value\":\"{}\"}}",
            escape_json(&strength.rank_explanation)
        ));
        values.push(format!(
            "{{\"label\":\"Comparison\",\"value\":\"{}\"}}",
            escape_json(&strength.comparison_note)
        ));
        values.push(format!(
            "{{\"label\":\"Category\",\"value\":\"{}\"}}",
            escape_json(&strength.category)
        ));
        values.push(format!(
            "{{\"label\":\"Tie break\",\"value\":\"{}\"}}",
            escape_json(
                &strength
                    .tie_break_vector
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            )
        ));
        values.push(format!(
            "{{\"label\":\"Best five\",\"value\":\"{}\"}}",
            escape_json(
                &strength
                    .best_five
                    .iter()
                    .map(|card| card.label.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        ));
    }

    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"result\":\"{}\",\"emphasized\":{},\"strength\":{},\"values\":[{}]}}",
        breakdown.seat.as_str(),
        breakdown.seat.as_str(),
        escape_json(&breakdown.result),
        matches!(breakdown.result.as_str(), "win" | "split"),
        breakdown
            .strength
            .as_ref()
            .map_or_else(|| "null".to_owned(), river_showdown_strength_json),
        values.join(",")
    )
}

pub(crate) fn river_showdown_strength_json(
    strength: &river_ledger::visibility::ShowdownStrengthView,
) -> String {
    format!(
        "{{\"category\":\"{}\",\"tie_break_vector\":[{}],\"best_five\":[{}],\"category_ladder_position\":{},\"result_label\":\"{}\",\"hand_name\":\"{}\",\"rank_explanation\":\"{}\",\"comparison_note\":\"{}\",\"best_five_accessibility_label\":\"{}\"}}",
        escape_json(&strength.category),
        strength
            .tie_break_vector
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(","),
        strength
            .best_five
            .iter()
            .map(river_card_json)
            .collect::<Vec<_>>()
            .join(","),
        river_category_ladder_position_json(&strength.category_ladder_position),
        escape_json(&strength.result_label),
        escape_json(&strength.hand_name),
        escape_json(&strength.rank_explanation),
        escape_json(&strength.comparison_note),
        escape_json(&strength.best_five_accessibility_label)
    )
}

pub(crate) fn river_category_ladder_position_json(
    position: &river_ledger::state::CategoryLadderPosition,
) -> String {
    format!(
        "{{\"position\":{},\"total\":{},\"description\":\"{}\"}}",
        position.position,
        position.total,
        escape_json(&position.description)
    )
}

pub(crate) fn river_private_view_json(private_view: &river_ledger::PrivateView) -> String {
    match private_view {
        river_ledger::PrivateView::Observer => {
            "{\"status\":\"observer\",\"seat\":null,\"hole_cards\":[]}".to_owned()
        }
        river_ledger::PrivateView::Seat(view) => format!(
            "{{\"status\":\"seat\",\"seat\":\"{}\",\"hole_cards\":[{}]}}",
            escape_json(&view.seat.as_str()),
            view.hole_cards
                .iter()
                .map(river_card_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

pub(crate) fn river_ui_json(ui: &river_ledger::ui::UiMetadata) -> String {
    format!(
        "{{\"game_id\":\"{}\",\"display_name\":\"{}\",\"surface_label\":\"{}\",\"viewer_modes\":[{}],\"min_seats\":{},\"default_seats\":{},\"max_seats\":{},\"seat_labels\":[{}],\"seat_metadata_label\":\"{}\",\"action_hint_label\":\"{}\",\"outcome_explanation_label\":\"{}\",\"contribution_label\":\"{}\",\"board_label\":\"{}\",\"hidden_hole_label\":\"{}\",\"reduced_motion_note\":\"{}\",\"hand_rankings\":[{}]}}",
        escape_json(&ui.game_id),
        escape_json(&ui.display_name),
        escape_json(&ui.surface_label),
        ui.viewer_modes
            .iter()
            .map(|mode| format!("\"{}\"", escape_json(mode)))
            .collect::<Vec<_>>()
            .join(","),
        ui.min_seats,
        ui.default_seats,
        ui.max_seats,
        ui.seat_labels
            .iter()
            .map(river_seat_display_label_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&ui.seat_metadata_label),
        escape_json(&ui.action_hint_label),
        escape_json(&ui.outcome_explanation_label),
        escape_json(&ui.contribution_label),
        escape_json(&ui.board_label),
        escape_json(&ui.hidden_hole_label),
        escape_json(&ui.reduced_motion_note),
        ui.hand_rankings
            .iter()
            .map(river_hand_ranking_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn river_seat_display_label_json(label: &river_ledger::ui::SeatDisplayLabel) -> String {
    format!(
        "{{\"seat\":\"{}\",\"label\":\"{}\"}}",
        escape_json(&label.seat),
        escape_json(&label.label)
    )
}

pub(crate) fn river_hand_ranking_json(row: &river_ledger::ui::HandRankingMetadata) -> String {
    format!(
        "{{\"category\":\"{}\",\"label\":\"{}\",\"definition\":\"{}\"}}",
        escape_json(&row.category),
        escape_json(&row.label),
        escape_json(&row.definition)
    )
}

pub(crate) fn river_bot_decision_public_explanation_json(
    explanation: &river_ledger::visibility::BotDecisionPublicExplanationView,
) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_label\":\"{}\",\"action_label\":\"{}\",\"short_reason\":\"{}\",\"public_facts\":[{}],\"hidden_information_notice\":\"{}\"}}",
        escape_json(&explanation.seat.as_str()),
        escape_json(&explanation.seat_label),
        escape_json(&explanation.action_label),
        escape_json(&explanation.short_reason),
        explanation
            .public_facts
            .iter()
            .map(river_bot_decision_public_fact_json)
            .collect::<Vec<_>>()
            .join(","),
        escape_json(&explanation.hidden_information_notice)
    )
}

pub(crate) fn river_bot_decision_public_fact_json(
    fact: &river_ledger::visibility::BotDecisionPublicFactView,
) -> String {
    format!(
        "{{\"label\":\"{}\",\"value\":\"{}\"}}",
        escape_json(&fact.label),
        escape_json(&fact.value)
    )
}

pub(crate) fn river_effects_json(
    effects: &[EffectEnvelope<RiverLedgerEffect>],
    viewer: &Viewer,
) -> String {
    let body = river_ledger::filter_effects_for_viewer(effects, viewer)
        .iter()
        .map(river_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn river_effect_json(effect: &EffectEnvelope<RiverLedgerEffect>) -> String {
    let payload = match &effect.payload {
        RiverLedgerEffect::PrivateCardsDealt { owner, cards } => format!(
            "{{\"type\":\"river_ledger_private_cards_dealt\",\"owner\":\"{}\",\"cards\":[{}]}}",
            escape_json(&river_effect_seat_label(*owner)),
            cards
                .iter()
                .map(|card| format!("\"{}\"", escape_json(&card.id())))
                .collect::<Vec<_>>()
                .join(",")
        ),
        RiverLedgerEffect::DealStarted {
            private_count_per_seat,
            reserved_community_count,
            deck_tail_count,
        } => format!(
            "{{\"type\":\"river_ledger_deal_started\",\"private_count_per_seat\":{},\"reserved_community_count\":{},\"deck_tail_count\":{}}}",
            private_count_per_seat, reserved_community_count, deck_tail_count
        ),
        RiverLedgerEffect::ContributionChanged {
            seat,
            amount_added,
            pot_total,
        } => format!(
            "{{\"type\":\"river_ledger_contribution_changed\",\"actor\":\"{}\",\"amount_added\":{},\"pot_total\":{}}}",
            escape_json(&river_effect_seat_label(*seat)),
            amount_added,
            pot_total
        ),
        RiverLedgerEffect::StreetAdvanced {
            street,
            public_board,
        } => format!(
            "{{\"type\":\"river_ledger_street_advanced\",\"street\":\"{}\",\"public_board_count\":{},\"public_board\":[{}]}}",
            street.as_str(),
            public_board.len(),
            public_board
                .iter()
                .map(|card| format!("\"{}\"", escape_json(&card.id())))
                .collect::<Vec<_>>()
                .join(",")
        ),
        RiverLedgerEffect::ShowdownResolved { outcome } => {
            let (kind, winner_count, pot_total) = match outcome {
                river_ledger::TerminalOutcome::LastLiveHand { pot_total, .. } => {
                    ("last_live_hand", 1usize, *pot_total)
                }
                river_ledger::TerminalOutcome::Showdown {
                    winners,
                    pot_total,
                    ..
                } => ("showdown", winners.len(), *pot_total),
            };
            format!(
                "{{\"type\":\"river_ledger_showdown_resolved\",\"kind\":\"{}\",\"winner_count\":{},\"pot_total\":{}}}",
                kind, winner_count, pot_total
            )
        }
    };
    format!("{{\"payload\":{payload}}}")
}

pub(crate) fn river_effect_seat_label(seat: RiverLedgerSeat) -> String {
    format!("Seat {}", seat.index())
}
