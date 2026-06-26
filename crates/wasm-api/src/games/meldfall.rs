//! Browser-bridge helpers for `meldfall_ledger`.

use engine_core::{ActionPath, Actor, EffectEnvelope, FreshnessToken, Seed, Viewer};
use meldfall_ledger::{
    actions::{
        DISCARD_SEGMENT_PREFIX, DRAW_DISCARD_SEGMENT_PREFIX, DRAW_STOCK_SEGMENT,
        FINISH_TURN_SEGMENT, GO_OUT_WITHOUT_DISCARD_SEGMENT, LAY_OFF_SEGMENT_PREFIX,
        MELD_NEW_SEGMENT_PREFIX,
    },
    bots::{legal_action_paths, legal_action_tree_for_seat, MeldfallL0Bot},
    cards::CardId,
    effects::{effect_stable_string, MeldfallEffect, MeldfallEffectEnvelope},
    ids::{canonical_seat_ids, supported_seat_count},
    replay_support::{export_viewer_snapshot, import_viewer_export, ViewerReplayExport},
    rules::{
        advance_to_next_round, discard_card, draw_from_discard, draw_from_stock,
        finish_turn_after_table_plays, lay_off_card, settle_round_stock_exhausted, table_new_meld,
    },
    scoring::settle_round,
    setup::{setup_match, SetupOptions},
    state::{MatchState, MeldId, TableCard, TurnOrdinal, TurnPhase},
    visibility::{
        project_action_tree_for_viewer, project_effects_for_viewer, project_view, MeldfallView,
        PrivateView, PublicMatchOutcomeView, PublicMeldGroupView, PublicSeatStandingView,
        PublicTableCardView, PublicTableauView,
    },
};

use crate::action_tree::action_tree_json;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::json_parse::{string_field, validate_json_object};
use crate::replay::{PublicTimelineReplay, PublicTimelineStep};
use crate::store::{next_replay_id, REPLAYS};
use crate::{effect_visible_to_viewer, visibility_json, AppliedCommand, ReplayRecord};

pub(crate) fn parse_meldfall_seat(value: &str) -> Result<usize, String> {
    let suffix = value.strip_prefix("seat_").ok_or_else(|| {
        diagnostic_string(
            "unknown_seat",
            &format!("unknown meldfall_ledger seat: {value}"),
        )
    })?;
    let index = suffix.parse::<usize>().map_err(|_| {
        diagnostic_string(
            "unknown_seat",
            &format!("unknown meldfall_ledger seat: {value}"),
        )
    })?;
    if index < meldfall_ledger::STANDARD_MAX_SEATS as usize {
        Ok(index)
    } else {
        Err(diagnostic_string(
            "unknown_seat",
            &format!("unknown meldfall_ledger seat: {value}"),
        ))
    }
}

pub(crate) fn trace_meldfall_seat(seat: usize) -> String {
    format!("seat_{seat}")
}

pub(crate) fn meldfall_actor_for_seat(state: &MatchState, seat: usize) -> Result<Actor, String> {
    state
        .seats
        .get(seat)
        .cloned()
        .map(|seat_id| Actor { seat_id })
        .ok_or_else(|| {
            diagnostic_string(
                "unknown_seat",
                &format!("seat not present: {}", trace_meldfall_seat(seat)),
            )
        })
}

pub(crate) fn meldfall_viewer_for_seat(
    state: &MatchState,
    viewer_seat: Option<&str>,
) -> Result<Viewer, String> {
    match viewer_seat {
        None => Ok(Viewer { seat_id: None }),
        Some(value) => {
            let seat = parse_meldfall_seat(value)?;
            state
                .seats
                .get(seat)
                .cloned()
                .map(|seat_id| Viewer {
                    seat_id: Some(seat_id),
                })
                .ok_or_else(|| {
                    diagnostic_string(
                        "unknown_seat",
                        &format!("seat not present: {}", trace_meldfall_seat(seat)),
                    )
                })
        }
    }
}

pub(crate) fn meldfall_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    seat: usize,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_meldfall_seat)
        .transpose()
        .map(|viewer| viewer == Some(seat))
}

pub(crate) fn create_meldfall_match(seed: u64, seat_count: usize) -> Result<MatchState, String> {
    if !supported_seat_count(seat_count) {
        return Err(diagnostic_json(
            meldfall_ledger::setup::invalid_seat_count_diagnostic(seat_count),
        ));
    }
    let setup = setup_match(
        Seed(seed),
        &canonical_seat_ids(seat_count),
        &SetupOptions::default(),
    )
    .map_err(diagnostic_json)?;
    Ok(MatchState::from_initial_setup(setup))
}

pub(crate) fn meldfall_replay_to_cursor(
    seed: u64,
    seat_count: usize,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<(MatchState, Vec<MeldfallEffectEnvelope>), String> {
    let mut state = create_meldfall_match(seed, seat_count)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_meldfall_seat(&command.actor_seat)?;
        let effects = meldfall_apply_command(
            &mut state,
            seat,
            ActionPath {
                segments: command.action_path.clone(),
            },
        )?;
        all_effects.extend(effects);
    }
    Ok((state, all_effects))
}

pub(crate) fn meldfall_apply_command(
    state: &mut MatchState,
    seat: usize,
    action_path: ActionPath,
) -> Result<Vec<MeldfallEffectEnvelope>, String> {
    meldfall_actor_for_seat(state, seat)?;
    let action = parse_meldfall_action(&action_path)?;
    let mut effects = match action {
        MeldfallCommand::DrawStock => {
            vec![draw_from_stock(&mut state.round, seat).map_err(diagnostic_json)?]
        }
        MeldfallCommand::DrawDiscard { index } => {
            vec![draw_from_discard(&mut state.round, seat, index).map_err(diagnostic_json)?]
        }
        MeldfallCommand::MeldNew { cards } => {
            let group = table_new_meld(&mut state.round, seat, &cards, TurnOrdinal(0))
                .map_err(diagnostic_json)?;
            vec![EffectEnvelope::public(MeldfallEffect::Meld {
                seat,
                meld_id: group.id,
                cards: group.cards,
            })]
        }
        MeldfallCommand::LayOff {
            card,
            target_meld,
            position,
        } => vec![lay_off_card(
            &mut state.round,
            seat,
            card,
            target_meld,
            position,
            TurnOrdinal(0),
        )
        .map_err(diagnostic_json)?],
        MeldfallCommand::Discard { card } => {
            vec![discard_card(&mut state.round, seat, card).map_err(diagnostic_json)?]
        }
        MeldfallCommand::FinishTurn | MeldfallCommand::GoOutWithoutDiscard => {
            finish_turn_after_table_plays(&mut state.round, seat).map_err(diagnostic_json)?;
            Vec::new()
        }
    };

    // ML-TURN-009: if the action leaves the active seat in the draw phase with no
    // legal draw (stock empty and no usable discard-pile pickup), the round can no
    // longer continue and must settle rather than deadlock.
    if state.round.phase == TurnPhase::Draw {
        let active_seat = state.round.active_seat_index;
        let draw_tree = legal_action_tree_for_seat(state, active_seat, FreshnessToken(0));
        if legal_action_paths(&draw_tree).is_empty() {
            settle_round_stock_exhausted(&mut state.round, active_seat).map_err(diagnostic_json)?;
        }
    }

    if state.round.phase == TurnPhase::RoundSettled {
        let round_index = round_score_index(state);
        let settlement = settle_round(state);
        let deltas = settlement
            .seats
            .iter()
            .map(|seat| seat.round_delta)
            .collect::<Vec<_>>();
        effects.push(EffectEnvelope::public(MeldfallEffect::RoundScore {
            round_index,
            deltas,
            cumulative_scores: state.cumulative_scores.clone(),
        }));
        if let Some(outcome) = settlement.terminal {
            state.round.phase = TurnPhase::MatchComplete;
            effects.push(EffectEnvelope::public(MeldfallEffect::MatchTerminal {
                outcome,
            }));
        } else {
            advance_to_next_round(state).map_err(diagnostic_json)?;
            effects.push(EffectEnvelope::public(MeldfallEffect::NextRoundDealt {
                next_round_number: state.rounds_settled + 1,
                next_lead_seat: state.round.active_seat_index,
                new_dealer: state.dealer_index,
            }));
        }
    }

    Ok(effects)
}

pub(crate) fn round_score_index(state: &MatchState) -> u32 {
    state.rounds_settled
}

enum MeldfallCommand {
    DrawStock,
    DrawDiscard {
        index: usize,
    },
    MeldNew {
        cards: Vec<CardId>,
    },
    LayOff {
        card: CardId,
        target_meld: MeldId,
        position: meldfall_ledger::actions::LayoffPosition,
    },
    Discard {
        card: CardId,
    },
    GoOutWithoutDiscard,
    FinishTurn,
}

fn parse_meldfall_action(path: &ActionPath) -> Result<MeldfallCommand, String> {
    if path.segments.len() != 1 {
        return Err(diagnostic_string(
            "unknown_action",
            "meldfall_ledger action paths use one segment",
        ));
    }
    let segment = path.segments[0].as_str();
    if segment == DRAW_STOCK_SEGMENT {
        return Ok(MeldfallCommand::DrawStock);
    }
    if let Some(index) = segment.strip_prefix(&format!("{DRAW_DISCARD_SEGMENT_PREFIX}-")) {
        return Ok(MeldfallCommand::DrawDiscard {
            index: index
                .parse()
                .map_err(|_| diagnostic_string("unknown_action", "invalid discard index"))?,
        });
    }
    if let Some(cards) = segment.strip_prefix(&format!("{MELD_NEW_SEGMENT_PREFIX}-")) {
        return Ok(MeldfallCommand::MeldNew {
            cards: parse_card_list(cards)?,
        });
    }
    if let Some(rest) = segment.strip_prefix(&format!("{LAY_OFF_SEGMENT_PREFIX}-")) {
        let parts = rest.split('-').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(diagnostic_string(
                "unknown_action",
                "invalid lay-off action",
            ));
        }
        let position = match parts[2] {
            "prepend" => meldfall_ledger::actions::LayoffPosition::Prepend,
            "append" => meldfall_ledger::actions::LayoffPosition::Append,
            _ => {
                return Err(diagnostic_string(
                    "unknown_action",
                    "invalid lay-off position",
                ))
            }
        };
        let meld_id = parts[1]
            .strip_prefix("meld_")
            .ok_or_else(|| diagnostic_string("unknown_action", "invalid meld id"))?
            .parse::<u32>()
            .map_err(|_| diagnostic_string("unknown_action", "invalid meld id"))?;
        return Ok(MeldfallCommand::LayOff {
            card: parse_card(parts[0])?,
            target_meld: MeldId(meld_id),
            position,
        });
    }
    if let Some(card) = segment.strip_prefix(&format!("{DISCARD_SEGMENT_PREFIX}-")) {
        return Ok(MeldfallCommand::Discard {
            card: parse_card(card)?,
        });
    }
    if segment == GO_OUT_WITHOUT_DISCARD_SEGMENT {
        return Ok(MeldfallCommand::GoOutWithoutDiscard);
    }
    if segment == FINISH_TURN_SEGMENT {
        return Ok(MeldfallCommand::FinishTurn);
    }
    Err(diagnostic_string(
        "unknown_action",
        "unknown meldfall_ledger action family",
    ))
}

fn parse_card_list(value: &str) -> Result<Vec<CardId>, String> {
    let parts = value.split('_').collect::<Vec<_>>();
    if parts.len() % 2 != 0 {
        return Err(diagnostic_string(
            "unknown_card",
            "meldfall_ledger card list must contain rank_suit pairs",
        ));
    }
    parts
        .chunks(2)
        .map(|chunk| parse_card(&format!("{}_{}", chunk[0], chunk[1])))
        .collect()
}

fn parse_card(value: &str) -> Result<CardId, String> {
    CardId::parse(value).ok_or_else(|| {
        diagnostic_string(
            "unknown_card",
            &format!("unknown meldfall_ledger card id: {value}"),
        )
    })
}

pub(crate) fn meldfall_action_tree_json(
    state: &MatchState,
    seat: usize,
    viewer: &Viewer,
) -> Result<String, String> {
    let tree = legal_action_tree_for_seat(state, seat, FreshnessToken(0));
    Ok(action_tree_json(&project_action_tree_for_viewer(
        &tree, state, viewer,
    )))
}

pub(crate) fn meldfall_view_json(view: &MeldfallView, freshness_token: u64) -> String {
    let private_status = match &view.private {
        PrivateView::Observer => "observer",
        PrivateView::Seat(_) => "seat",
    };
    let own_hand = match &view.private {
        PrivateView::Observer => String::new(),
        PrivateView::Seat(private) => string_array(&private.hand),
    };
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"active_seat\":\"{}\",\"active_seat_index\":{},\"dealer\":\"{}\",\"dealer_index\":{},\"phase\":\"{}\",\"stock_count\":{},\"discard\":[{}],\"hand_counts\":{},\"cumulative_scores\":{},\"round_played_scores\":{},\"tableau\":{},\"round_end\":{},\"terminal\":{},\"freshness_token\":{},\"private_view_status\":\"{}\",\"own_hand\":[{}],\"hidden_fields\":[\"opponent_hands\",\"stock_order\",\"private_drawn_cards\"]}}",
        SCHEMA_VERSION,
        RULES_VERSION,
        escape_json(GAME_MELDFALL_LEDGER),
        escape_json(GAME_MELDFALL_LEDGER_DISPLAY_NAME),
        escape_json(VARIANT_MELDFALL_LEDGER_STANDARD),
        escape_json(&view.rules_version_label),
        trace_meldfall_seat(view.active_seat_index),
        view.active_seat_index,
        trace_meldfall_seat(view.dealer_index),
        view.dealer_index,
        escape_json(&view.phase),
        view.stock_count,
        string_array(&view.discard),
        usize_array(&view.hand_counts),
        i32_array(&view.cumulative_scores),
        i32_array(&view.round_played_scores),
        tableau_json(&view.tableau),
        view.round_end.as_ref().map_or_else(|| "null".to_owned(), |value| format!("\"{}\"", escape_json(value))),
        view.terminal.as_ref().map_or_else(|| "null".to_owned(), outcome_json),
        freshness_token,
        private_status,
        own_hand
    )
}

pub(crate) fn meldfall_effects_json(effects: &[MeldfallEffectEnvelope], viewer: &Viewer) -> String {
    let visible = project_effects_for_viewer(effects, viewer)
        .iter()
        .map(meldfall_logged_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{visible}]")
}

pub(crate) fn meldfall_logged_effect_json(effect: &MeldfallEffectEnvelope) -> String {
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        meldfall_effect_payload_json(&effect.payload)
    )
}

pub(crate) fn meldfall_effect_json(effect: &MeldfallEffectEnvelope, viewer: &Viewer) -> String {
    if effect_visible_to_viewer(&effect.visibility, viewer) {
        meldfall_logged_effect_json(effect)
    } else {
        "{\"visibility\":\"redacted\",\"payload\":{\"kind\":\"redacted\"}}".to_owned()
    }
}

fn meldfall_effect_payload_json(effect: &MeldfallEffect) -> String {
    match effect {
        MeldfallEffect::Draw {
            seat,
            source,
            cards_moved,
            stock_count_after,
            discard_count_after,
        } => format!(
            "{{\"kind\":\"draw\",\"seat\":\"{}\",\"source\":\"{}\",\"cards_moved\":{},\"stock_count_after\":{},\"discard_count_after\":{}}}",
            trace_meldfall_seat(*seat),
            escape_json(&source.stable_string()),
            cards_moved,
            stock_count_after,
            discard_count_after
        ),
        MeldfallEffect::StockDrawPrivate {
            seat,
            card,
            stock_count_after,
        } => format!(
            "{{\"kind\":\"stock_draw_private\",\"seat\":\"{}\",\"card\":\"{}\",\"stock_count_after\":{}}}",
            trace_meldfall_seat(*seat),
            escape_json(&card.as_str()),
            stock_count_after
        ),
        MeldfallEffect::Meld {
            seat,
            meld_id,
            cards,
        } => format!(
            "{{\"kind\":\"meld\",\"seat\":\"{}\",\"meld_id\":\"{}\",\"cards\":[{}]}}",
            trace_meldfall_seat(*seat),
            escape_json(&meld_id.as_string()),
            cards.iter().map(table_card_json).collect::<Vec<_>>().join(",")
        ),
        MeldfallEffect::LayOff {
            seat,
            meld_id,
            card,
            position,
        } => format!(
            "{{\"kind\":\"lay_off\",\"seat\":\"{}\",\"meld_id\":\"{}\",\"card\":{},\"position\":\"{}\"}}",
            trace_meldfall_seat(*seat),
            escape_json(&meld_id.as_string()),
            table_card_json(card),
            position.as_str()
        ),
        MeldfallEffect::Discard {
            seat,
            card,
            discard_count_after,
        } => format!(
            "{{\"kind\":\"discard\",\"seat\":\"{}\",\"card\":\"{}\",\"discard_count_after\":{}}}",
            trace_meldfall_seat(*seat),
            escape_json(&card.as_str()),
            discard_count_after
        ),
        MeldfallEffect::RoundScore {
            round_index,
            deltas,
            cumulative_scores,
        } => format!(
            "{{\"kind\":\"round_score\",\"round_index\":{},\"deltas\":{},\"cumulative_scores\":{}}}",
            round_index,
            i32_array(deltas),
            i32_array(cumulative_scores)
        ),
        MeldfallEffect::NextRoundDealt {
            next_round_number,
            next_lead_seat,
            new_dealer,
        } => format!(
            "{{\"kind\":\"next_round_dealt\",\"next_round_number\":{},\"next_lead_seat\":\"{}\",\"new_dealer\":\"{}\"}}",
            next_round_number,
            trace_meldfall_seat(*next_lead_seat),
            trace_meldfall_seat(*new_dealer)
        ),
        MeldfallEffect::MatchTerminal { outcome } => {
            format!("{{\"kind\":\"match_terminal\",\"outcome\":{}}}", outcome_json(&PublicMatchOutcomeView {
                standings: outcome.standings.iter().map(|standing| PublicSeatStandingView {
                    seat_index: standing.seat_index,
                    rank: standing.rank,
                    cumulative_score: standing.cumulative_score,
                    latest_round_delta: standing.latest_round_delta,
                    winner: standing.winner,
                }).collect(),
            }))
        }
    }
}

pub(crate) fn meldfall_replay_document_json(
    state: &MatchState,
    effects: &[MeldfallEffectEnvelope],
) -> String {
    let viewer = Viewer { seat_id: None };
    let tree = legal_action_tree_for_seat(state, state.round.active_seat_index, FreshnessToken(0));
    let export = export_viewer_snapshot(state, &tree, effects, &viewer);
    export.to_json()
}

pub(crate) fn is_meldfall_public_export(doc: &str) -> bool {
    matches!(
        string_field(doc, "export_class").as_deref(),
        Ok(meldfall_ledger::replay_support::VIEWER_EXPORT_CLASS)
    ) && matches!(
        string_field(doc, "game_id").as_deref(),
        Ok(GAME_MELDFALL_LEDGER)
    )
}

pub(crate) fn import_meldfall_public_replay(doc: &str) -> Result<String, String> {
    validate_json_object(doc).map_err(|message| {
        diagnostic_string(
            "invalid_replay",
            &format!("invalid public replay document: {message}"),
        )
    })?;
    let export = import_viewer_export(
        &ViewerReplayExport {
            schema_version: 1,
            export_format_version: meldfall_ledger::replay_support::EXPORT_FORMAT_VERSION,
            export_class: meldfall_ledger::replay_support::VIEWER_EXPORT_CLASS.to_owned(),
            viewer: "observer".to_owned(),
            game_id: GAME_MELDFALL_LEDGER.to_owned(),
            rules_version: MELDFALL_LEDGER_TRACE_RULES_VERSION.to_owned(),
            data_version: meldfall_ledger::DATA_VERSION_LABEL.to_owned(),
            variant: VARIANT_MELDFALL_LEDGER_STANDARD.to_owned(),
            steps: vec![meldfall_ledger::replay_support::ViewerReplayStep {
                step_index: 0,
                view_summary: "imported meldfall_ledger public export".to_owned(),
                action_tree_hash: engine_core::HashValue(0),
                effect_summaries: Vec::new(),
                terminal: false,
            }],
        },
        &Viewer { seat_id: None },
    )
    .map_err(|message| diagnostic_string("invalid_replay", &message))?;
    let replay_id = next_replay_id(GAME_MELDFALL_LEDGER);
    REPLAYS.with(|replays| {
        replays.borrow_mut().insert(
            replay_id.clone(),
            ReplayRecord {
                game_id: GAME_MELDFALL_LEDGER.to_owned(),
                seed: 0,
                commands: Vec::new(),
                public_timeline: Some(PublicTimelineReplay {
                    viewer: export.viewer,
                    steps: vec![PublicTimelineStep {
                        step_index: 0,
                        public_view_summary: "imported meldfall_ledger public export".to_owned(),
                        public_effects: Vec::new(),
                        redacted_command_summary: "imported public export".to_owned(),
                        terminal: false,
                    }],
                }),
            },
        );
    });
    Ok(format!(
        "{{\"replay_id\":\"{}\",\"game_id\":\"{}\",\"public_export\":true,\"viewer\":\"observer\"}}",
        escape_json(&replay_id),
        escape_json(GAME_MELDFALL_LEDGER)
    ))
}

pub(crate) fn meldfall_select_bot_decision(
    state: &MatchState,
    seat: usize,
    bot_seed: u64,
) -> Result<meldfall_ledger::bots::MeldfallBotDecision, String> {
    MeldfallL0Bot::new(Seed(bot_seed))
        .select_decision(state, seat)
        .map_err(diagnostic_json)
}

pub(crate) fn meldfall_replay_step_json(
    replay_id: &str,
    cursor: usize,
    total_commands: usize,
    state: &MatchState,
    effects: &[MeldfallEffectEnvelope],
) -> String {
    let viewer = Viewer { seat_id: None };
    let visible_effects = project_effects_for_viewer(effects, &viewer)
        .iter()
        .map(|effect| format!("\"{}\"", escape_json(&effect_stable_string(effect))))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"total_commands\":{},\"view\":{},\"effects\":[{}]}}",
        escape_json(replay_id),
        cursor,
        total_commands,
        meldfall_view_json(&project_view(state, &viewer), 0),
        visible_effects
    )
}

fn tableau_json(tableau: &PublicTableauView) -> String {
    format!(
        "{{\"groups\":[{}]}}",
        tableau
            .groups
            .iter()
            .map(meld_group_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn meld_group_json(group: &PublicMeldGroupView) -> String {
    format!(
        "{{\"id\":\"{}\",\"kind\":\"{}\",\"origin_seat\":\"{}\",\"origin_seat_index\":{},\"cards\":[{}]}}",
        escape_json(&group.id),
        escape_json(&group.kind),
        trace_meldfall_seat(group.origin_seat),
        group.origin_seat,
        group.cards.iter().map(public_table_card_json).collect::<Vec<_>>().join(",")
    )
}

fn public_table_card_json(card: &PublicTableCardView) -> String {
    format!(
        "{{\"card\":\"{}\",\"played_by\":\"{}\",\"played_by_index\":{},\"score_credit_owner\":\"{}\",\"score_credit_owner_index\":{},\"play_turn\":{}}}",
        escape_json(&card.card),
        trace_meldfall_seat(card.played_by),
        card.played_by,
        trace_meldfall_seat(card.score_credit_owner),
        card.score_credit_owner,
        card.play_turn
    )
}

fn table_card_json(card: &TableCard) -> String {
    format!(
        "{{\"card\":\"{}\",\"played_by\":\"{}\",\"played_by_index\":{},\"score_credit_owner\":\"{}\",\"score_credit_owner_index\":{},\"play_turn\":{}}}",
        escape_json(&card.card.as_str()),
        trace_meldfall_seat(card.played_by),
        card.played_by,
        trace_meldfall_seat(card.score_credit_owner),
        card.score_credit_owner,
        card.play_turn.0
    )
}

fn outcome_json(outcome: &PublicMatchOutcomeView) -> String {
    format!(
        "{{\"standings\":[{}]}}",
        outcome
            .standings
            .iter()
            .map(standing_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn standing_json(standing: &PublicSeatStandingView) -> String {
    format!(
        "{{\"seat\":\"{}\",\"seat_index\":{},\"rank\":{},\"cumulative_score\":{},\"latest_round_delta\":{},\"winner\":{}}}",
        trace_meldfall_seat(standing.seat_index),
        standing.seat_index,
        standing.rank,
        standing.cumulative_score,
        standing.latest_round_delta,
        standing.winner
    )
}

fn string_array(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn usize_array(values: &[usize]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn i32_array(values: &[i32]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}
