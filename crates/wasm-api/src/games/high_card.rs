//! Browser-bridge helpers for `high_card_duel` (hidden-information duel).

use engine_core::{ActionPath, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};
use high_card_duel::{
    apply_action as high_card_apply_action, import_public_export as high_card_import_public_export,
    project_view as high_card_project_view, setup_match as high_card_setup_match,
    validate_command as high_card_validate_command, HighCardDuelEffect, HighCardDuelSeat,
    HighCardDuelState, PublicReplayExport, PublicReplayStep,
};

use crate::actors::high_card_actor_for_seat;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{parse_public_replay_steps, PublicTimelineReplay, PublicTimelineStep};
use crate::seats::{parse_high_card_seat, seats};
use crate::store::{next_replay_id, REPLAYS};
use crate::{effect_visible_to_viewer, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn is_high_card_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok("public_observer_projection_v1")
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(high_card_duel::GAME_ID)
    )
}

pub(crate) fn import_high_card_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let rules_version = string_field(doc, "rules_version")?;
    if rules_version != high_card_duel::RULES_VERSION_LABEL {
        return Err(diagnostic_string(
            "unsupported_replay_rules",
            &format!("unsupported replay rules version: {rules_version}"),
        ));
    }
    let variant = string_field(doc, "variant")?;
    if variant != high_card_duel::VARIANT_ID {
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
    let export = PublicReplayExport {
        schema_version: SCHEMA_VERSION,
        export_class: "public_observer_projection_v1".to_owned(),
        viewer: viewer.clone(),
        game_id: high_card_duel::GAME_ID.to_owned(),
        rules_version,
        variant,
        steps: steps
            .iter()
            .map(high_card_step_from_public_timeline)
            .collect(),
    };
    let timeline = high_card_import_public_export(&export);
    let replay_id = next_replay_id(GAME_HIGH_CARD_DUEL);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_HIGH_CARD_DUEL.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: timeline.viewer.clone(),
                    steps: timeline
                        .steps
                        .iter()
                        .map(public_timeline_step_from_high_card)
                        .collect(),
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"{}\",\"step_count\":{},\"command_count\":0,\"final_view\":null,\"effect_count\":0}}",
        escape_json(&replay_id),
        escape_json(GAME_HIGH_CARD_DUEL),
        escape_json(&timeline.viewer),
        timeline.steps.len()
    ))
}

pub(crate) fn high_card_step_from_public_timeline(step: &PublicTimelineStep) -> PublicReplayStep {
    PublicReplayStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}

pub(crate) fn high_card_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(HighCardDuelState, Vec<EffectEnvelope<HighCardDuelEffect>>), String> {
    let seats = seats();
    let mut state =
        high_card_setup_match(Seed(seed), &seats, &high_card_duel::SetupOptions::default())
            .map_err(diagnostic_json)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_high_card_seat(&command.actor_seat)?;
        let envelope = CommandEnvelope {
            actor: high_card_actor_for_seat(&state, seat)?,
            action_path: ActionPath {
                segments: command.action_path.clone(),
            },
            freshness_token: engine_core::FreshnessToken(command.freshness_token),
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = high_card_validate_command(&state, &envelope).map_err(diagnostic_json)?;
        all_effects.extend(high_card_apply_action(&mut state, action));
    }
    Ok((state, all_effects))
}

pub(crate) fn high_card_effects_json(
    effects: &[EffectEnvelope<HighCardDuelEffect>],
    viewer: &Viewer,
) -> String {
    let body = effects
        .iter()
        .filter(|effect| effect_visible_to_viewer(&effect.visibility, viewer))
        .map(high_card_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(crate) fn high_card_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &HighCardDuelState,
    effects: &[EffectEnvelope<HighCardDuelEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        high_card_view_json(&high_card_project_view(state, &viewer)),
        high_card_effects_json(effects, &viewer)
    )
}

pub(crate) fn high_card_view_json(view: &high_card_duel::PublicView) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"round_number\":{},\"round_limit\":{},\"phase\":\"{}\",\"active_seat\":{},\"lead_seat\":{},\"reply_seat\":{},\"score\":{},\"hand_counts\":{},\"deck_count\":{},\"commitments\":{},\"revealed_cards\":[{}],\"terminal_kind\":\"{}\",\"winning_seat\":{},\"freshness_token\":{},\"private_view\":{},\"ui\":{}}}",
        view.schema_version,
        view.rules_version,
        escape_json(&view.game_id),
        escape_json(&view.display_name),
        escape_json(&view.variant_id),
        escape_json(&view.rules_version_label),
        view.round_number,
        view.round_limit,
        view.phase.as_str(),
        option_high_card_seat_json(view.active_seat),
        option_high_card_seat_json(view.lead_seat),
        option_high_card_seat_json(view.reply_seat),
        high_card_score_json(view.score),
        high_card_hand_counts_json(&view.hand_counts),
        view.deck_count,
        high_card_commitments_json(&view.commitments),
        view.revealed_cards
            .iter()
            .map(high_card_revealed_round_json)
            .collect::<Vec<_>>()
            .join(","),
        high_card_terminal_kind(&view.terminal),
        option_high_card_seat_json(high_card_terminal_winner(&view.terminal)),
        view.freshness_token.0,
        high_card_private_view_json(&view.private_view),
        high_card_ui_json(&view.ui)
    )
}

pub(crate) fn high_card_card_json(card: &high_card_duel::CardView) -> String {
    format!(
        "{{\"card_id\":\"{}\",\"rank\":{},\"sigil\":\"{}\",\"accessibility_label\":\"{}\"}}",
        escape_json(&card.card_id),
        card.rank,
        escape_json(&card.sigil),
        escape_json(&card.accessibility_label)
    )
}

pub(crate) fn high_card_score_json(score: high_card_duel::Score) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        score.seat_0, score.seat_1
    )
}

pub(crate) fn high_card_hand_counts_json(counts: &high_card_duel::HandCountsView) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        counts.seat_0, counts.seat_1
    )
}

pub(crate) fn high_card_commitments_json(commitments: &high_card_duel::CommitmentViews) -> String {
    format!(
        "{{\"seat_0\":{},\"seat_1\":{}}}",
        high_card_commitment_json(&commitments.seat_0),
        high_card_commitment_json(&commitments.seat_1)
    )
}

pub(crate) fn high_card_commitment_json(commitment: &high_card_duel::CommitmentView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"status\":\"{}\",\"card\":{},\"accessibility_label\":\"{}\"}}",
        commitment.seat.as_str(),
        escape_json(&commitment.status),
        commitment
            .card
            .as_ref()
            .map_or_else(|| "null".to_owned(), high_card_card_json),
        escape_json(&commitment.accessibility_label)
    )
}

pub(crate) fn high_card_revealed_round_json(round: &high_card_duel::RevealedRoundView) -> String {
    format!(
        "{{\"round_number\":{},\"seat_0_card\":{},\"seat_1_card\":{},\"winner\":{}}}",
        round.round_number,
        high_card_card_json(&round.seat_0_card),
        high_card_card_json(&round.seat_1_card),
        option_high_card_seat_json(round.winner)
    )
}

pub(crate) fn high_card_private_view_json(private_view: &high_card_duel::PrivateView) -> String {
    match private_view {
        high_card_duel::PrivateView::Observer => {
            "{\"status\":\"observer\",\"hand\":[],\"own_commitment\":null}".to_owned()
        }
        high_card_duel::PrivateView::Seat {
            seat,
            hand,
            own_commitment,
        } => {
            let hand = hand
                .iter()
                .map(high_card_card_json)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "{{\"status\":\"seat\",\"seat\":\"{}\",\"hand\":[{}],\"own_commitment\":{}}}",
                seat.as_str(),
                hand,
                own_commitment
                    .as_ref()
                    .map_or_else(|| "null".to_owned(), high_card_card_json)
            )
        }
    }
}

pub(crate) fn high_card_ui_json(ui: &high_card_duel::UiMetadata) -> String {
    format!(
        "{{\"table_label\":\"{}\",\"card_back_token\":\"{}\",\"own_card_token\":\"{}\",\"revealed_card_token\":\"{}\",\"empty_commitment_token\":\"{}\",\"face_down_commitment_token\":\"{}\",\"commit_action_label\":\"{}\",\"observer_disabled_reason\":\"{}\"}}",
        escape_json(&ui.table_label),
        escape_json(&ui.card_back_token),
        escape_json(&ui.own_card_token),
        escape_json(&ui.revealed_card_token),
        escape_json(&ui.empty_commitment_token),
        escape_json(&ui.face_down_commitment_token),
        escape_json(&ui.commit_action_label),
        escape_json(&ui.observer_disabled_reason)
    )
}

pub(crate) fn high_card_effect_json(effect: &EffectEnvelope<HighCardDuelEffect>) -> String {
    let payload = match &effect.payload {
        HighCardDuelEffect::DealPrivateCard { owner, card } => format!(
            "{{\"type\":\"deal_private_card\",\"owner\":\"{}\",\"card_id\":\"{}\"}}",
            owner.as_str(),
            card.stable_id()
        ),
        HighCardDuelEffect::HandCountChanged {
            seat_0_count,
            seat_1_count,
            deck_count,
        } => format!(
            "{{\"type\":\"hand_count_changed\",\"seat_0_count\":{},\"seat_1_count\":{},\"deck_count\":{}}}",
            seat_0_count, seat_1_count, deck_count
        ),
        HighCardDuelEffect::CommitFaceDown { seat, round_number } => format!(
            "{{\"type\":\"commit_face_down\",\"seat\":\"{}\",\"round_number\":{}}}",
            seat.as_str(),
            round_number
        ),
        HighCardDuelEffect::OwnCommitConfirmed {
            owner,
            card,
            round_number,
        } => format!(
            "{{\"type\":\"own_commit_confirmed\",\"owner\":\"{}\",\"card_id\":\"{}\",\"round_number\":{}}}",
            owner.as_str(),
            card.stable_id(),
            round_number
        ),
        HighCardDuelEffect::CardsRevealed {
            round_number,
            seat_0_card,
            seat_1_card,
        } => format!(
            "{{\"type\":\"cards_revealed\",\"round_number\":{},\"seat_0_card\":\"{}\",\"seat_1_card\":\"{}\"}}",
            round_number,
            seat_0_card.stable_id(),
            seat_1_card.stable_id()
        ),
        HighCardDuelEffect::RoundScored {
            round_number,
            winner,
            score,
        } => format!(
            "{{\"type\":\"round_scored\",\"round_number\":{},\"winner\":{},\"score\":{}}}",
            round_number,
            option_high_card_seat_json(*winner),
            high_card_score_json(*score)
        ),
        HighCardDuelEffect::RefillStarted {
            next_round_number,
            next_lead_seat,
        } => format!(
            "{{\"type\":\"refill_started\",\"next_round_number\":{},\"next_lead_seat\":\"{}\"}}",
            next_round_number,
            next_lead_seat.as_str()
        ),
        HighCardDuelEffect::Terminal { winner, score } => format!(
            "{{\"type\":\"terminal\",\"winner\":{},\"score\":{}}}",
            option_high_card_seat_json(*winner),
            high_card_score_json(*score)
        ),
        HighCardDuelEffect::PrivateDiagnostic {
            owner,
            code,
            private_message,
        } => format!(
            "{{\"type\":\"private_diagnostic\",\"owner\":\"{}\",\"code\":\"{}\",\"private_message\":\"{}\"}}",
            owner.as_str(),
            escape_json(code),
            escape_json(private_message)
        ),
        HighCardDuelEffect::PublicDiagnostic {
            code,
            public_message,
        } => format!(
            "{{\"type\":\"public_diagnostic\",\"code\":\"{}\",\"public_message\":\"{}\"}}",
            escape_json(code),
            escape_json(public_message)
        ),
    };
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        payload
    )
}

pub(crate) fn high_card_terminal_kind(terminal: &high_card_duel::TerminalView) -> &'static str {
    match terminal {
        high_card_duel::TerminalView::NonTerminal => "non_terminal",
        high_card_duel::TerminalView::Win { .. } => "win",
        high_card_duel::TerminalView::Draw { .. } => "draw",
    }
}

pub(crate) fn high_card_terminal_winner(
    terminal: &high_card_duel::TerminalView,
) -> Option<HighCardDuelSeat> {
    match terminal {
        high_card_duel::TerminalView::Win { winning_seat, .. } => Some(*winning_seat),
        _ => None,
    }
}

pub(crate) fn option_high_card_seat_json(seat: Option<HighCardDuelSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", seat.as_str()),
    )
}

fn public_timeline_step_from_high_card(step: &PublicReplayStep) -> PublicTimelineStep {
    PublicTimelineStep {
        step_index: step.step_index,
        public_view_summary: step.public_view_summary.clone(),
        public_effects: step.public_effects.clone(),
        redacted_command_summary: step.redacted_command_summary.clone(),
        terminal: step.terminal,
    }
}
