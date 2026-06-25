//! Browser-bridge helpers for `blackglass_pact`.

use blackglass_pact::{
    apply_bid_action, apply_blind_nil_action, apply_play_action, canonical_seat_ids,
    legal_action_tree, parse_bid_action_path, parse_blind_nil_action_path, parse_play_action_path,
    score_completed_hand, setup_match, Bid, BlackglassL1Bot, BlackglassPactEffect,
    BlackglassPactState, BlackglassSeat, BlackglassViewer, BotDecision, CardId, PublicPhase,
    PublicView, SeatScoreBreakdown, SeatView, SetupOptions, TeamId, TeamScoreBreakdown, ViewerView,
};
use engine_core::{ActionPath, Actor, CommandEnvelope, EffectEnvelope, RulesVersion, Seed, Viewer};

use crate::action_tree::action_tree_json;
use crate::commands::command_record_json;
use crate::constants::*;
use crate::json::{diagnostic_json, diagnostic_string, escape_json};
use crate::{visibility_json, AppliedCommand};

pub(crate) fn parse_blackglass_seat(value: &str) -> Result<BlackglassSeat, String> {
    BlackglassSeat::parse(value).ok_or_else(|| {
        diagnostic_string(
            "unknown_seat",
            &format!("unknown blackglass_pact seat: {value}"),
        )
    })
}

pub(crate) fn trace_blackglass_seat(seat: BlackglassSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn blackglass_actor_for_seat(
    state: &BlackglassPactState,
    seat: BlackglassSeat,
) -> Result<Actor, String> {
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

pub(crate) fn blackglass_viewer_for_seat(viewer_seat: Option<&str>) -> Result<Viewer, String> {
    match viewer_seat {
        None => Ok(Viewer { seat_id: None }),
        Some(value) => {
            let seat = parse_blackglass_seat(value)?;
            Ok(Viewer {
                seat_id: Some(engine_core::SeatId(seat.as_str().to_owned())),
            })
        }
    }
}

pub(crate) fn blackglass_viewer_authorizes_actor(
    viewer_seat: Option<&str>,
    seat: BlackglassSeat,
) -> Result<bool, String> {
    viewer_seat
        .map(parse_blackglass_seat)
        .transpose()
        .map(|viewer| viewer == Some(seat))
}

pub(crate) fn create_blackglass_pact_match(
    seed: u64,
    seat_count: usize,
) -> Result<BlackglassPactState, String> {
    if blackglass_pact::validate_standard_seat_count(seat_count).is_err() {
        return Err(diagnostic_string(
            "unsupported_seat_count",
            "blackglass_pact requires exactly 4 seats",
        ));
    }
    setup_match(Seed(seed), &canonical_seat_ids(), &SetupOptions::default())
        .map_err(diagnostic_json)
}

pub(crate) fn blackglass_replay_to_cursor(
    seed: u64,
    commands: &[AppliedCommand],
    cursor: usize,
) -> Result<
    (
        BlackglassPactState,
        Vec<EffectEnvelope<BlackglassPactEffect>>,
    ),
    String,
> {
    let mut state =
        create_blackglass_pact_match(seed, blackglass_pact::STANDARD_SEAT_COUNT as usize)?;
    let mut all_effects = Vec::new();
    for command in commands.iter().take(cursor) {
        let seat = parse_blackglass_seat(&command.actor_seat)?;
        let effects = blackglass_apply_command(
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

pub(crate) fn blackglass_apply_command(
    state: &mut BlackglassPactState,
    seat: BlackglassSeat,
    action_path: ActionPath,
    freshness_token: u64,
) -> Result<Vec<EffectEnvelope<BlackglassPactEffect>>, String> {
    let command = CommandEnvelope {
        actor: blackglass_actor_for_seat(state, seat)?,
        action_path,
        freshness_token: engine_core::FreshnessToken(freshness_token),
        rules_version: RulesVersion(RULES_VERSION),
    };
    let mut effects = match command.action_path.segments.first().map(String::as_str) {
        Some(blackglass_pact::ACTION_BLIND_NIL) => {
            let action = parse_blind_nil_action_path(&command.action_path.segments)
                .map_err(diagnostic_json)?;
            apply_blind_nil_action(state, &command.actor, action).map_err(diagnostic_json)?
        }
        Some(blackglass_pact::ACTION_BID) => {
            let action =
                parse_bid_action_path(&command.action_path.segments).map_err(diagnostic_json)?;
            apply_bid_action(state, &command.actor, action).map_err(diagnostic_json)?
        }
        Some(blackglass_pact::ACTION_PLAY) => {
            let action =
                parse_play_action_path(&command.action_path.segments).map_err(diagnostic_json)?;
            apply_play_action(state, &command.actor, action).map_err(diagnostic_json)?
        }
        _ => {
            return Err(diagnostic_string(
                "unknown_action",
                "unknown blackglass_pact action family",
            ))
        }
    };

    if matches!(state.phase, blackglass_pact::Phase::HandScoring { .. }) {
        effects.extend(score_completed_hand(state).map_err(diagnostic_json)?);
    }

    Ok(effects
        .into_iter()
        .map(blackglass_effect_envelope)
        .collect())
}

fn blackglass_effect_envelope(
    effect: BlackglassPactEffect,
) -> EffectEnvelope<BlackglassPactEffect> {
    match &effect {
        BlackglassPactEffect::PrivateHandReceived { seat, .. } => {
            EffectEnvelope::private_to(engine_core::SeatId(seat.as_str().to_owned()), effect)
        }
        _ => EffectEnvelope::public(effect),
    }
}

pub(crate) fn blackglass_action_tree_json(
    state: &BlackglassPactState,
    seat: BlackglassSeat,
) -> Result<String, String> {
    let actor = blackglass_actor_for_seat(state, seat)?;
    Ok(action_tree_json(&legal_action_tree(state, &actor)))
}

pub(crate) fn blackglass_view_json(view: &ViewerView, freshness_token: u64) -> String {
    match view {
        ViewerView::Public(public) => blackglass_public_view_json(public, None, freshness_token),
        ViewerView::Seat(seat) => blackglass_seat_view_json(seat, freshness_token),
    }
}

fn blackglass_seat_view_json(view: &SeatView, freshness_token: u64) -> String {
    format!(
        "{}",
        with_seat_private_fields(
            blackglass_public_view_json(&view.public, Some(view.seat), freshness_token),
            &view.own_hand
        )
    )
}

fn with_seat_private_fields(mut json: String, own_hand: &[CardId]) -> String {
    if json.ends_with('}') {
        json.pop();
        json.push_str(&format!(
            ",\"own_hand\":[{}],\"private_view_status\":\"seat\"}}",
            own_hand
                .iter()
                .map(|card| blackglass_card_json(*card))
                .collect::<Vec<_>>()
                .join(",")
        ));
    }
    json
}

fn blackglass_public_view_json(
    view: &PublicView,
    viewer_seat: Option<BlackglassSeat>,
    freshness_token: u64,
) -> String {
    format!(
        "{{\"schema_version\":{},\"rules_version\":{},\"game_id\":\"{}\",\"display_name\":\"{}\",\"variant_id\":\"{}\",\"rules_version_label\":\"{}\",\"viewer_seat\":{},\"phase\":{},\"dealer\":\"{}\",\"hand_index\":{},\"spades_broken\":{},\"active_seat\":{},\"hand_counts\":{},\"bids\":{},\"team_contracts\":{},\"team_scores\":{},\"team_bags\":{},\"current_trick\":{},\"last_hand_score\":{},\"outcome\":{},\"freshness_token\":{},\"private_view_status\":\"{}\",\"hidden_fields\":[\"opponent_hands\",\"deck_order\",\"future_deals\"]}}",
        SCHEMA_VERSION,
        RULES_VERSION,
        escape_json(GAME_BLACKGLASS_PACT),
        escape_json(GAME_BLACKGLASS_PACT_DISPLAY_NAME),
        escape_json(VARIANT_BLACKGLASS_PACT_STANDARD),
        escape_json(blackglass_pact::RULES_VERSION_LABEL),
        option_blackglass_seat_json(viewer_seat),
        blackglass_phase_json(&view.phase),
        view.dealer.as_str(),
        view.hand_index,
        view.spades_broken,
        option_blackglass_seat_json(blackglass_active_seat(view)),
        blackglass_hand_counts_json(&view.hand_counts),
        blackglass_bids_json(&view.bids),
        blackglass_team_contracts_json(&view.team_contracts),
        blackglass_team_i32_json(&view.team_scores),
        blackglass_team_u8_json(&view.team_bags),
        blackglass_played_cards_json(&view.current_trick),
        view.last_hand_score
            .as_ref()
            .map_or_else(|| "null".to_owned(), blackglass_hand_score_json),
        view.outcome
            .as_ref()
            .map_or_else(|| "null".to_owned(), |outcome| format!("\"{}\"", escape_json(&format!("{:?}", outcome)))),
        freshness_token,
        if viewer_seat.is_some() { "seat" } else { "observer" }
    )
}

fn blackglass_active_seat(view: &PublicView) -> Option<BlackglassSeat> {
    match view.phase {
        PublicPhase::BlindNilCommitment { active, .. } => active,
        PublicPhase::Bidding { next } => Some(next),
        PublicPhase::PlayingTrick { next, .. } => Some(next),
        PublicPhase::HandScoring { .. } | PublicPhase::Terminal { .. } => None,
    }
}

pub(crate) fn blackglass_effects_json(
    effects: &[EffectEnvelope<BlackglassPactEffect>],
    viewer: &Viewer,
) -> String {
    let visible = effects
        .iter()
        .filter(|effect| blackglass_effect_visible(effect, viewer))
        .map(blackglass_logged_effect_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{visible}]")
}

fn blackglass_effect_visible(
    effect: &EffectEnvelope<BlackglassPactEffect>,
    viewer: &Viewer,
) -> bool {
    match &effect.visibility {
        engine_core::VisibilityScope::Public => true,
        engine_core::VisibilityScope::PrivateToSeat(seat) => viewer.seat_id.as_ref() == Some(seat),
    }
}

pub(crate) fn blackglass_logged_effect_json(
    effect: &EffectEnvelope<BlackglassPactEffect>,
) -> String {
    format!(
        "{{\"visibility\":{},\"payload\":{}}}",
        visibility_json(&effect.visibility),
        blackglass_effect_payload_json(&effect.payload)
    )
}

pub(crate) fn blackglass_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> String {
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":{},\"trace_id\":\"{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported_replay\",\"note\":\"Replay exported by the Rulepath WASM API from the Rust command log.\",\"migration_update_note\":\"none\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat_0\",\"player_id\":\"player_0\"}},{{\"seat_id\":\"seat_1\",\"player_id\":\"player_1\"}},{{\"seat_id\":\"seat_2\",\"player_id\":\"player_2\"}},{{\"seat_id\":\"seat_3\",\"player_id\":\"player_3\"}}],\"commands\":[{}],\"checkpoints\":[{{\"id\":\"final\",\"after_command_index\":{}}}],\"expected_state_hashes\":{{\"final\":null}},\"expected_effect_hashes\":{{\"final\":null}},\"expected_action_tree_hashes\":{{\"final\":null}},\"expected_public_view_hashes\":{{\"observer\":null}},\"expected_private_view_hashes\":{{\"seat_0\":null,\"seat_1\":null,\"seat_2\":null,\"seat_3\":null}},\"expected_replay_hashes\":{{\"final\":null}},\"expected_outcome\":{{\"terminal\":false,\"winner\":null,\"draw\":false}},\"expected_terminal_state\":{{\"terminal\":false,\"winner\":null,\"draw\":false}},\"not_applicable\":{{\"hidden_information\":\"Blackglass Pact has private hands; viewer-scoped views and effects redact unauthorized card identities.\",\"preview_hashes\":\"Blackglass Pact uses action metadata rather than a separate preview hash.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_BLACKGLASS_PACT),
        escape_json(BLACKGLASS_PACT_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(blackglass_pact::DATA_VERSION_LABEL),
        seed,
        escape_json(VARIANT_BLACKGLASS_PACT_STANDARD),
        commands_json,
        commands.len().saturating_sub(1)
    )
}

pub(crate) fn blackglass_replay_step_json(
    replay_id: &str,
    cursor: usize,
    command_count: usize,
    state: &BlackglassPactState,
    effects: &[EffectEnvelope<BlackglassPactEffect>],
) -> String {
    let viewer = Viewer { seat_id: None };
    format!(
        "{{\"replay_id\":\"{}\",\"cursor\":{},\"command_count\":{},\"done\":{},\"view\":{},\"effects\":{}}}",
        escape_json(replay_id),
        cursor,
        command_count,
        cursor >= command_count,
        blackglass_view_json(&blackglass_pact::viewer_view(state, BlackglassViewer::Observer), state.freshness_token.0),
        blackglass_effects_json(effects, &viewer)
    )
}

pub(crate) fn blackglass_select_bot_decision(
    state: &BlackglassPactState,
    seat: BlackglassSeat,
    _bot_seed: u64,
) -> Result<BotDecision, String> {
    BlackglassL1Bot
        .select_decision(state, seat)
        .map_err(diagnostic_json)
}

fn blackglass_effect_payload_json(effect: &BlackglassPactEffect) -> String {
    match effect {
        BlackglassPactEffect::BlindNilWindowOpened { pending, threshold } => format!(
            "{{\"type\":\"blind_nil_window_opened\",\"pending\":{},\"threshold\":{}}}",
            blackglass_seats_json(pending),
            threshold
        ),
        BlackglassPactEffect::BlindNilDeclared { seat, team } => format!(
            "{{\"type\":\"blind_nil_declared\",\"seat\":\"{}\",\"team\":\"{}\"}}",
            seat.as_str(),
            team.as_str()
        ),
        BlackglassPactEffect::BlindNilDeclined { seat } => format!(
            "{{\"type\":\"blind_nil_declined\",\"seat\":\"{}\"}}",
            seat.as_str()
        ),
        BlackglassPactEffect::DealCompleted {
            dealer,
            hand_index,
            counts,
            next_bidder,
        } => format!(
            "{{\"type\":\"deal_completed\",\"dealer\":\"{}\",\"hand_index\":{},\"counts\":{},\"next_bidder\":\"{}\"}}",
            dealer.as_str(),
            hand_index,
            blackglass_counts_json(counts),
            next_bidder.as_str()
        ),
        BlackglassPactEffect::PrivateHandReceived { seat, cards } => format!(
            "{{\"type\":\"private_hand_received\",\"seat\":\"{}\",\"cards\":[{}]}}",
            seat.as_str(),
            cards
                .iter()
                .map(|card| blackglass_card_json(*card))
                .collect::<Vec<_>>()
                .join(",")
        ),
        BlackglassPactEffect::BidAccepted { seat, team, bid } => format!(
            "{{\"type\":\"bid_accepted\",\"seat\":\"{}\",\"team\":\"{}\",\"bid\":{}}}",
            seat.as_str(),
            team.as_str(),
            blackglass_bid_json(*bid)
        ),
        BlackglassPactEffect::CardPlayed {
            seat,
            card,
            trick_index,
        } => format!(
            "{{\"type\":\"card_played\",\"seat\":\"{}\",\"card\":{},\"trick_index\":{}}}",
            seat.as_str(),
            blackglass_card_json(*card),
            trick_index
        ),
        BlackglassPactEffect::SpadesBroken {
            seat,
            card,
            trick_index,
        } => format!(
            "{{\"type\":\"spades_broken\",\"seat\":\"{}\",\"card\":{},\"trick_index\":{}}}",
            seat.as_str(),
            blackglass_card_json(*card),
            trick_index
        ),
        BlackglassPactEffect::TrickCaptured {
            winner,
            trick_index,
            plays,
        } => format!(
            "{{\"type\":\"trick_captured\",\"winner\":\"{}\",\"trick_index\":{},\"plays\":{}}}",
            winner.as_str(),
            trick_index,
            blackglass_played_cards_json(plays)
        ),
        BlackglassPactEffect::HandScored { breakdown } => format!(
            "{{\"type\":\"hand_scored\",\"breakdown\":{}}}",
            blackglass_hand_score_json(breakdown)
        ),
        BlackglassPactEffect::BagPenaltyApplied {
            team,
            penalty_count,
            points_deducted,
            next_bags,
        } => format!(
            "{{\"type\":\"bag_penalty_applied\",\"team\":\"{}\",\"penalty_count\":{},\"points_deducted\":{},\"next_bags\":{}}}",
            team.as_str(),
            penalty_count,
            points_deducted,
            next_bags
        ),
        BlackglassPactEffect::DealerAdvanced { dealer, hand_index } => format!(
            "{{\"type\":\"dealer_advanced\",\"dealer\":\"{}\",\"hand_index\":{}}}",
            dealer.as_str(),
            hand_index
        ),
        BlackglassPactEffect::MatchCompleted { outcome } => format!(
            "{{\"type\":\"match_completed\",\"outcome\":\"{}\"}}",
            escape_json(&format!("{:?}", outcome))
        ),
    }
}

fn blackglass_phase_json(phase: &PublicPhase) -> String {
    match phase {
        PublicPhase::BlindNilCommitment {
            active,
            pending_count,
        } => format!(
            "{{\"kind\":\"blind_nil_commitment\",\"active\":{},\"pending_count\":{}}}",
            option_blackglass_seat_json(*active),
            pending_count
        ),
        PublicPhase::Bidding { next } => {
            format!("{{\"kind\":\"bidding\",\"next\":\"{}\"}}", next.as_str())
        }
        PublicPhase::PlayingTrick {
            leader,
            next,
            trick_index,
        } => format!(
            "{{\"kind\":\"playing_trick\",\"leader\":\"{}\",\"next\":\"{}\",\"trick_index\":{}}}",
            leader.as_str(),
            next.as_str(),
            trick_index
        ),
        PublicPhase::HandScoring { completed_tricks } => format!(
            "{{\"kind\":\"hand_scoring\",\"completed_tricks\":{}}}",
            completed_tricks
        ),
        PublicPhase::Terminal { winning_team } => format!(
            "{{\"kind\":\"terminal\",\"winning_team\":\"{}\"}}",
            winning_team.as_str()
        ),
    }
}

fn option_blackglass_seat_json(seat: Option<BlackglassSeat>) -> String {
    seat.map_or_else(
        || "null".to_owned(),
        |seat| format!("\"{}\"", escape_json(seat.as_str())),
    )
}

fn blackglass_seats_json(seats: &[BlackglassSeat]) -> String {
    format!(
        "[{}]",
        seats
            .iter()
            .map(|seat| format!("\"{}\"", seat.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_card_json(card: CardId) -> String {
    format!(
        "{{\"id\":\"{}\",\"label\":\"{}\",\"rank\":\"{}\",\"suit\":\"{}\"}}",
        escape_json(&card.as_str()),
        escape_json(&card.card().public_label()),
        card.card().rank.as_str(),
        card.card().suit.as_str()
    )
}

fn blackglass_played_cards_json(cards: &[blackglass_pact::PlayedCard]) -> String {
    format!(
        "[{}]",
        cards
            .iter()
            .map(|play| format!(
                "{{\"seat\":\"{}\",\"card\":{}}}",
                play.seat.as_str(),
                blackglass_card_json(play.card)
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_hand_counts_json(counts: &[blackglass_pact::PublicHandCount]) -> String {
    format!(
        "[{}]",
        counts
            .iter()
            .map(|entry| format!(
                "{{\"seat\":\"{}\",\"count\":{}}}",
                entry.seat.as_str(),
                entry.count
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_counts_json(counts: &[(BlackglassSeat, usize)]) -> String {
    format!(
        "[{}]",
        counts
            .iter()
            .map(|(seat, count)| format!("{{\"seat\":\"{}\",\"count\":{}}}", seat.as_str(), count))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_bid_json(bid: Bid) -> String {
    match bid {
        Bid::Tricks(value) => format!("{{\"kind\":\"tricks\",\"value\":{value}}}"),
        Bid::Nil => "{\"kind\":\"nil\",\"value\":0}".to_owned(),
        Bid::BlindNil => "{\"kind\":\"blind_nil\",\"value\":0}".to_owned(),
    }
}

fn blackglass_bids_json(rows: &[blackglass_pact::PublicBidRow]) -> String {
    format!(
        "[{}]",
        rows.iter()
            .map(|row| format!(
                "{{\"seat\":\"{}\",\"team\":\"{}\",\"bid\":{}}}",
                row.seat.as_str(),
                row.team.as_str(),
                row.bid
                    .map_or_else(|| "null".to_owned(), blackglass_bid_json)
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_team_contracts_json(rows: &[blackglass_pact::PublicTeamContract]) -> String {
    format!(
        "[{}]",
        rows.iter()
            .map(|row| format!(
                "{{\"team\":\"{}\",\"ordinary_contract\":{}}}",
                row.team.as_str(),
                row.ordinary_contract
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_team_i32_json(rows: &[(TeamId, i32)]) -> String {
    format!(
        "{{{}}}",
        rows.iter()
            .map(|(team, value)| format!("\"{}\":{}", team.as_str(), value))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_team_u8_json(rows: &[(TeamId, u8)]) -> String {
    format!(
        "{{{}}}",
        rows.iter()
            .map(|(team, value)| format!("\"{}\":{}", team.as_str(), value))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_hand_score_json(score: &blackglass_pact::HandScoreBreakdown) -> String {
    format!(
        "{{\"hand_index\":{},\"teams\":[{}],\"seats\":[{}]}}",
        score.hand_index,
        score
            .team_breakdowns
            .iter()
            .map(blackglass_team_score_json)
            .collect::<Vec<_>>()
            .join(","),
        score
            .seat_breakdowns
            .iter()
            .map(blackglass_seat_score_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn blackglass_team_score_json(score: &TeamScoreBreakdown) -> String {
    format!(
        "{{\"team\":\"{}\",\"contract\":{},\"ordinary_tricks\":{},\"ordinary_made\":{},\"hand_delta\":{},\"prior_score\":{},\"next_score\":{},\"next_bags\":{}}}",
        score.team.as_str(),
        score.contract,
        score.ordinary_tricks,
        score.ordinary_made,
        score.hand_delta,
        score.prior_score,
        score.next_score,
        score.next_bags
    )
}

fn blackglass_seat_score_json(score: &SeatScoreBreakdown) -> String {
    format!(
        "{{\"seat\":\"{}\",\"team\":\"{}\",\"bid\":{},\"tricks\":{},\"nil_result\":{}}}",
        score.seat.as_str(),
        score.team.as_str(),
        score
            .bid
            .map_or_else(|| "null".to_owned(), blackglass_bid_json),
        score.tricks,
        score.nil_result.map_or_else(
            || "null".to_owned(),
            |result| format!("\"{}\"", escape_json(&format!("{:?}", result)))
        )
    )
}
