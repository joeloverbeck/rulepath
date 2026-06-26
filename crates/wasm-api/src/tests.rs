use super::*;
use crate::wasm_abi::{rulepath_feature_report, rulepath_list_games, LAST_OUTPUT};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct NoLeakSurface {
    viewer: Option<String>,
    name: &'static str,
    payload: String,
}

#[derive(Clone, Debug)]
struct PairwiseNoLeakCase {
    seats: Vec<String>,
    private_terms_by_seat: BTreeMap<String, Vec<String>>,
    surfaces: Vec<NoLeakSurface>,
}

impl PairwiseNoLeakCase {
    fn deterministic_summary(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.seats.join("|"));
        for (seat, terms) in &self.private_terms_by_seat {
            output.push_str(seat);
            output.push(':');
            output.push_str(&terms.join(","));
            output.push(';');
        }
        for surface in &self.surfaces {
            output.push_str(surface.name);
            output.push('@');
            output.push_str(surface.viewer.as_deref().unwrap_or("observer"));
            output.push('=');
            output.push_str(&surface.payload);
            output.push(';');
        }
        output
    }
}

fn assert_pairwise_no_leak(case: &PairwiseNoLeakCase) {
    if let Err(message) = pairwise_no_leak_result(case) {
        panic!("{message}");
    }
}

fn pairwise_no_leak_result(case: &PairwiseNoLeakCase) -> Result<(), String> {
    for source_seat in &case.seats {
        let Some(private_terms) = case.private_terms_by_seat.get(source_seat) else {
            return Err(format!("missing private terms for {source_seat}"));
        };
        if private_terms.is_empty() {
            return Err(format!("no private terms registered for {source_seat}"));
        }
        for surface in &case.surfaces {
            if surface.viewer.as_deref() == Some(source_seat.as_str()) {
                continue;
            }
            for term in private_terms {
                if surface.payload.contains(term) {
                    return Err(format!(
                        "private term {term} for {source_seat} leaked to {} via {}",
                        surface.viewer.as_deref().unwrap_or("observer"),
                        surface.name
                    ));
                }
            }
        }
    }
    Ok(())
}

fn synthetic_n_seat_no_leak_case(seat_count: usize) -> PairwiseNoLeakCase {
    let seats = (0..seat_count)
        .map(|index| format!("seat_{index}"))
        .collect::<Vec<_>>();
    let mut private_terms_by_seat = BTreeMap::new();
    for seat in &seats {
        private_terms_by_seat.insert(seat.clone(), vec![format!("private::{seat}::seed-1701")]);
    }

    let mut surfaces = vec![NoLeakSurface {
        viewer: None,
        name: "replay_export",
        payload: format!(
            "viewer=observer;seat_count={seat_count};redacted=true;dom_test_id=seat-frame"
        ),
    }];
    for viewer in &seats {
        let own = private_terms_by_seat
            .get(viewer)
            .and_then(|terms| terms.first())
            .expect("synthetic private term");
        for name in [
            "payload",
            "action_tree",
            "preview",
            "effect_log",
            "bot_explanation",
            "candidate_ranking",
            "dom_test_id",
            "storage",
            "log",
        ] {
            surfaces.push(NoLeakSurface {
                    viewer: Some(viewer.clone()),
                    name,
                    payload: format!(
                        "viewer={viewer};seat_count={seat_count};own_private={own};other_private=redacted"
                    ),
                });
        }
    }

    PairwiseNoLeakCase {
        seats,
        private_terms_by_seat,
        surfaces,
    }
}

#[test]
fn placeholder_version_is_stable() {
    assert_eq!(placeholder_version(), "rulepath-wasm-api/0.1.0");
}

#[test]
fn list_games_reports_registered_games() {
    let games = list_games().expect("games listed");
    assert!(games.contains("\"game_id\":\"race_to_n\""));
    assert!(games.contains("\"game_id\":\"three_marks\""));
    assert!(games.contains("\"game_id\":\"column_four\""));
    assert!(games.contains("\"game_id\":\"directional_flip\""));
    assert!(games.contains("\"game_id\":\"draughts_lite\""));
    assert!(games.contains("\"game_id\":\"high_card_duel\""));
    assert!(games.contains("\"game_id\":\"masked_claims\""));
    assert!(games.contains("\"game_id\":\"flood_watch\""));
    assert!(games.contains("\"game_id\":\"token_bazaar\""));
    assert!(games.contains("\"game_id\":\"poker_lite\""));
    assert!(games.contains("\"game_id\":\"plain_tricks\""));
    assert!(games.contains("\"min_seats\":2"));
    assert!(games.contains("\"max_seats\":2"));
    assert!(games.contains("\"default_seats\":2"));
    assert!(games.contains("\"supported_seats\":[2]"));
    assert!(games.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"}]"
        ));
    assert!(games.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Charter\"},{\"seat\":\"seat_1\",\"label\":\"Freeholders\"}]"
        ));
    assert!(games.contains("\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\"]"));
    assert!(games
        .contains("\"variants\":[{\"id\":\"three_marks_standard\",\"label\":\"Three Marks\"}]"));
    assert!(games
        .contains("\"variants\":[{\"id\":\"column_four_standard\",\"label\":\"Column Four\"}]"));
    assert!(games.contains(
        "\"variants\":[{\"id\":\"directional_flip_standard\",\"label\":\"Directional Flip\"}]"
    ));
    assert!(games.contains(
        "\"variants\":[{\"id\":\"draughts_lite_standard\",\"label\":\"Draughts Lite\"}]"
    ));
    assert!(games.contains(
        "\"variants\":[{\"id\":\"high_card_duel_standard\",\"label\":\"High Card Duel\"}]"
    ));
    assert!(games.contains(
        "\"variants\":[{\"id\":\"masked_claims_standard\",\"label\":\"Masked Claims\"}]"
    ));
    assert!(games.contains("\"id\":\"flood_watch_deluge\""));
    assert!(games.contains("\"label\":\"Flood Watch: Deluge\""));
    assert!(games.contains("\"description\":\"Higher water starts and heavier surges create a tighter shared rescue.\""));
    assert!(games.contains("\"id\":\"frontier_control_highlands\""));
    assert!(games.contains("\"label\":\"Frontier Control: Highlands\""));
    assert!(games.contains(
        "\"description\":\"Highlands shifts table pressure toward quarry routes and high ground.\""
    ));
    assert!(games.contains("\"id\":\"event_frontier_hard_winter\""));
    assert!(games.contains("\"label\":\"Event Frontier: Hard Winter\""));
    assert!(games.contains("\"description\":\"Leaner opening resources make recovery feel tighter from the first turn.\""));
    assert!(games.contains("\"id\":\"event_frontier_land_rush\""));
    assert!(games.contains("\"label\":\"Event Frontier: Land Rush\""));
    assert!(games
        .contains("\"description\":\"Broader opening reach creates a faster public buildup.\""));
    assert!(games
        .contains("\"variants\":[{\"id\":\"token_bazaar_standard\",\"label\":\"Token Bazaar\"}]"));
    assert!(games
        .contains("\"variants\":[{\"id\":\"poker_lite_standard\",\"label\":\"Crest Ledger\"}]"));
    assert!(games
        .contains("\"variants\":[{\"id\":\"plain_tricks_standard\",\"label\":\"Plain Tricks\"}]"));
    assert!(games.contains("\"hidden_information\":true"));
    assert!(games.contains("\"public_replay_export\""));
    assert!(games.contains("\"reaction_window\""));
    assert!(games.contains("\"cooperative\":true"));
    assert!(games.contains("\"environment_automation\""));
    assert!(games.contains("\"bounded_pledge\""));
    assert!(games.contains("\"trick_taking\""));
}

#[test]
fn default_catalog_seat_labels_are_one_based() {
    assert_eq!(
        crate::catalog::catalog_seat_labels_json(4),
        "[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"}]"
    );
}

#[test]
fn briar_circuit_catalog_uses_one_based_default_seat_labels() {
    let games = list_games().expect("games listed");
    let briar_start = games
        .find("\"game_id\":\"briar_circuit\"")
        .expect("briar catalog entry present");
    let briar = &games[briar_start..];
    assert!(briar.contains(
        "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"}]"
    ));
}

#[test]
fn meldfall_ledger_catalog_exposes_variable_hidden_info_metadata() {
    let games = list_games().expect("games listed");
    let meldfall_start = games
        .find("\"game_id\":\"meldfall_ledger\"")
        .expect("meldfall catalog entry present");
    let meldfall = &games[meldfall_start..];
    assert!(meldfall.contains("\"display_name\":\"Meldfall Ledger\""));
    assert!(meldfall.contains(
        "\"variants\":[{\"id\":\"classic_500_single_deck_v1\",\"label\":\"Meldfall Ledger\",\"description\":\"Single-deck public-meld race to 500\"}]"
    ));
    assert!(meldfall.contains("\"hidden_information\":true"));
    assert!(meldfall.contains("\"min_seats\":2"));
    assert!(meldfall.contains("\"max_seats\":6"));
    assert!(meldfall.contains("\"default_seats\":4"));
    assert!(meldfall.contains("\"supported_seats\":[2,3,4,5,6]"));
    assert!(meldfall.contains("\"viewer_modes\":[\"observer\",\"seat_0\",\"seat_1\",\"seat_2\",\"seat_3\",\"seat_4\",\"seat_5\"]"));
}

#[test]
fn race_to_n_and_directional_flip_catalogs_use_player_labels() {
    let games = list_games().expect("games listed");
    let race_start = games
        .find("\"game_id\":\"race_to_n\"")
        .expect("race catalog entry present");
    let three_marks_start = games
        .find("\"game_id\":\"three_marks\"")
        .expect("three marks catalog entry present");
    let race = &games[race_start..three_marks_start];
    assert!(race.contains(
        "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Player 1\"},{\"seat\":\"seat_1\",\"label\":\"Player 2\"}]"
    ));

    let directional_start = games
        .find("\"game_id\":\"directional_flip\"")
        .expect("directional catalog entry present");
    let draughts_start = games
        .find("\"game_id\":\"draughts_lite\"")
        .expect("draughts catalog entry present");
    let directional = &games[directional_start..draughts_start];
    assert!(directional.contains(
        "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Player 1\"},{\"seat\":\"seat_1\",\"label\":\"Player 2\"}]"
    ));
}

#[test]
fn meldfall_ledger_wasm_surface_filters_hidden_cards_and_authorizes_actor() {
    let created =
        new_match_with_seat_count("meldfall_ledger", 19019, 4).expect("meldfall match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"classic_500_single_deck_v1\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat view returned");
    assert!(observer.contains("\"game_id\":\"meldfall_ledger\""));
    assert!(observer.contains("\"private_view_status\":\"observer\""));
    assert!(observer.contains("\"own_hand\":[]"));
    assert!(observer.contains("\"stock_count\":"));
    assert!(observer.contains("\"discard\":["));
    assert!(seat_1.contains("\"private_view_status\":\"seat\""));
    assert!(seat_1.contains("\"own_hand\":["));

    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_1", Some("seat_0"))
        .expect("unauthorized tree returned");
    assert!(unauthorized.contains("\"choices\":[]"));
    let authorized = get_action_tree_for_viewer(&match_id, "seat_1", Some("seat_1"))
        .expect("authorized tree returned");
    assert!(authorized.contains("\"segment\":\"draw-stock\""));

    let applied = apply_action(&match_id, "seat_1", "draw-stock", 0).expect("stock draw applies");
    assert!(applied.contains("\"kind\":\"draw\""));
    assert!(applied.contains("\"source\":\"stock\""));
    assert!(applied.contains("\"phase\":\"table\""));
    assert!(!applied.contains("\"stock_draw_private\""));

    let observer_effects = get_effects(&match_id, 0, None).expect("observer effects returned");
    assert!(observer_effects.contains("\"kind\":\"draw\""));
    assert!(!observer_effects.contains("\"stock_draw_private\""));

    let exported = export_replay(&match_id).expect("viewer replay exported");
    assert!(exported.contains("\"game_id\":\"meldfall_ledger\""));
    assert!(exported.contains("\"export_class\":\"meldfall_ledger_viewer_scoped_observation_v1\""));
    let imported = import_replay(&exported).expect("viewer replay imported");
    assert!(imported.contains("\"public_export\":true"));
    assert!(imported.contains("\"game_id\":\"meldfall_ledger\""));

    let bot_created =
        new_match_with_seat_count("meldfall_ledger", 19020, 4).expect("meldfall bot match created");
    let bot_match_id = extract_match_id(&bot_created);
    let bot = run_bot_turn(&bot_match_id, "seat_1", 7).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"policy_id\":\"meldfall-ledger-l0-random-legal-v1\""));
}

#[test]
fn meldfall_random_legal_play_always_settles_without_deadlock() {
    use crate::games::meldfall::{
        create_meldfall_match, meldfall_apply_command, meldfall_select_bot_decision,
    };
    use engine_core::FreshnessToken;
    use meldfall_ledger::bots::{legal_action_paths, legal_action_tree_for_seat};
    use meldfall_ledger::state::TurnPhase;

    // Smaller stocks exhaust faster: 6 seats deal 42 cards, leaving a 9-card
    // stock, so stock-exhaustion turns are reached quickly. Every round must end
    // in a settled/terminal state (ML-TURN-009); the active seat must never be
    // left in a draw/table/discard phase with zero legal actions.
    for seat_count in [2, 4, 6] {
        for seed in 0..120u64 {
            let mut state = create_meldfall_match(seed, seat_count)
                .unwrap_or_else(|err| panic!("seed {seed} seats {seat_count}: {err}"));
            let mut steps = 0;
            loop {
                let phase = state.round.phase;
                if matches!(phase, TurnPhase::RoundSettled | TurnPhase::MatchComplete) {
                    break;
                }
                let active = state.round.active_seat_index;
                let paths = legal_action_paths(&legal_action_tree_for_seat(
                    &state,
                    active,
                    FreshnessToken(0),
                ));
                assert!(
                    !paths.is_empty(),
                    "seed {seed} seats {seat_count}: deadlock in phase {phase:?} at seat {active} after {steps} steps (stock {}, discard {})",
                    state.round.stock.len(),
                    state.round.discard.len(),
                );
                let decision = meldfall_select_bot_decision(&state, active, seed)
                    .unwrap_or_else(|err| panic!("seed {seed}: bot decision failed: {err}"));
                meldfall_apply_command(&mut state, active, decision.action_path)
                    .unwrap_or_else(|err| panic!("seed {seed}: apply failed: {err}"));
                steps += 1;
                assert!(
                    steps < 1000,
                    "seed {seed} seats {seat_count}: round did not settle"
                );
            }
        }
    }
}

#[test]
fn meldfall_round_score_index_is_the_round_not_the_finishing_seat() {
    use crate::games::meldfall::{create_meldfall_match, round_score_index};
    use meldfall_ledger::state::{RoundEndReason, RoundEndSummary};

    let mut state = create_meldfall_match(7, 4).expect("meldfall match created");
    // A non-zero seat ends the only round. The scored round is still round 0;
    // the round index must not be confused with the finishing seat's index.
    state.round.round_end = Some(RoundEndSummary {
        reason: RoundEndReason::GoOutWithoutDiscard,
        seat_index: 3,
    });

    assert_eq!(
        round_score_index(&state),
        0,
        "round_score effect must report the round index (0), not the finishing seat"
    );
}

#[test]
fn feature_report_lists_ops() {
    let report = feature_report().expect("feature report returned");
    assert!(report.contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));
    for operation in SUPPORTED_OPERATIONS {
        assert!(
            report.contains(&format!("\"{operation}\"")),
            "missing operation {operation} in {report}"
        );
    }
    assert!(report
        .contains("\"features\":[\"catalog\",\"match_store\",\"legal_action_tree\",\"effects\"]"));
}

#[test]
fn new_match_for_variant_starts_multi_variant_games() {
    let event_created =
        new_match_for_variant(GAME_EVENT_FRONTIER, Some("event_frontier_hard_winter"), 7)
            .expect("event frontier variant match created");
    let event_match_id = extract_match_id(&event_created);
    assert!(event_created.contains("\"variant_id\":\"event_frontier_hard_winter\""));
    let event_view = get_view(&event_match_id, Some("seat_0")).expect("event variant view");
    assert!(event_view.contains("\"variant_id\":\"event_frontier_hard_winter\""));

    let flood_created = new_match_for_variant(GAME_FLOOD_WATCH, Some("flood_watch_deluge"), 7)
        .expect("flood watch variant match created");
    assert!(flood_created.contains("\"variant_id\":\"flood_watch_deluge\""));

    let frontier_created =
        new_match_for_variant(GAME_FRONTIER_CONTROL, Some("frontier_control_highlands"), 7)
            .expect("frontier control variant match created");
    assert!(frontier_created.contains("\"variant_id\":\"frontier_control_highlands\""));
}

#[test]
fn bridge_seat_builder_uses_deterministic_labels() {
    let seats = seats_for_count(3);
    assert_eq!(
        seats,
        vec![
            SeatId("seat-0".to_owned()),
            SeatId("seat-1".to_owned()),
            SeatId("seat-2".to_owned())
        ]
    );

    let underscore = masked_seats_for_count(3);
    assert_eq!(
        underscore,
        vec![
            SeatId("seat_0".to_owned()),
            SeatId("seat_1".to_owned()),
            SeatId("seat_2".to_owned())
        ]
    );
}

#[test]
fn new_match_with_seat_count_surfaces_game_setup_diagnostic() {
    let created =
        new_match_with_seat_count(GAME_RACE_TO_N, 11, 2).expect("two-seat setup succeeds");
    assert!(created.contains("\"game_id\":\"race_to_n\""));

    let diagnostic = new_match_with_seat_count(GAME_RACE_TO_N, 11, 3)
        .expect_err("three-seat setup is rejected by the game");
    assert!(diagnostic.contains("\"code\":\"invalid_seat_count\""));
    assert!(diagnostic.contains("race_to_n requires exactly two seats"));
}

#[test]
fn new_ops_use_status_output_convention() {
    assert_eq!(rulepath_feature_report(), 0);
    assert!(last_output_string().contains("\"api_version\":\"rulepath-wasm-api/0.1.0\""));

    assert_eq!(rulepath_list_games(), 0);
    assert!(last_output_string().contains("\"game_id\":\"race_to_n\""));
}

#[test]
fn surface_drives_minimal_turn_loop() {
    let created = new_match("race_to_n", 11).expect("match created");
    let match_id = extract_match_id(&created);

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"counter\":0"));
    assert!(!view.contains("seat-0"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"add-1\""));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied = apply_action(&match_id, "seat_0", "add-1", 0).expect("human action applies");
    assert!(applied.contains("\"counter\":1"));
    assert!(applied.contains("\"type\":\"counter_advanced\""));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"cursor\":1"));
    assert!(effects.contains("\"visibility\":\"public\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"active_seat\":\"seat_0\""));
}

#[test]
fn get_view_honors_viewer_for_existing_perfect_information_games() {
    let created = new_match("column_four", 42).expect("match created");
    let match_id = extract_match_id(&created);

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat_0 view returned");
    let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat_1 view returned");

    assert_eq!(observer, seat_0);
    assert_eq!(observer, seat_1);
    assert!(get_view(&match_id, Some("seat_2")).is_err());
}

#[test]
fn get_action_tree_requires_viewer_authorization() {
    let created = new_match("three_marks", 32).expect("match created");
    let match_id = extract_match_id(&created);

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized action tree returned");
    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
        .expect("unauthorized action tree redacted");
    let observer = get_action_tree_for_viewer(&match_id, "seat_0", None)
        .expect("observer action tree redacted");

    assert!(authorized.contains("\"segment\":\"place/r1c1\""));
    assert!(unauthorized.contains("\"choices\":[]"));
    assert!(observer.contains("\"choices\":[]"));
    assert!(get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_2")).is_err());
}

#[test]
fn three_marks_surface_drives_operation_group() {
    let created = new_match("three_marks", 31).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"three_marks_standard\""));

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"game_id\":\"three_marks\""));
    assert!(view.contains("\"variant_id\":\"three_marks_standard\""));
    assert!(view.contains("\"freshness_token\":0"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"place/r1c1\""));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied = apply_action(&match_id, "seat_0", "place/r1c1", 0).expect("human action applies");
    assert!(applied.contains("\"type\":\"mark_placed\""));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"type\":\"bot_chose_action\""));
    assert!(bot.contains("\"active_seat\":\"seat_0\""));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"type\":\"mark_placed\""));
    assert!(effects.contains("\"type\":\"bot_chose_action\""));

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"game_id\":\"three_marks\""));
    assert!(exported.contains("\"expected_replay_hashes\""));
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    assert!(exported.contains("\"private_view_hashes\":\"three_marks has no private-view API.\""));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"three_marks\""));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported =
        import_replay(&legacy_exported).expect("legacy hyphen three marks export imports");
    assert!(legacy_imported.contains("\"game_id\":\"three_marks\""));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"ply_count\":0"));

    let step = replay_step(&replay_id, 1).expect("replay stepped");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"ply_count\":1"));
}

#[test]
fn column_four_surface_drives_operation_group() {
    let created = new_match("column_four", 41).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"column_four_standard\""));

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"game_id\":\"column_four\""));
    assert!(view.contains("\"variant_id\":\"column_four_standard\""));
    assert!(view.contains("\"board_rows\":6"));
    assert!(view.contains("\"board_columns\":7"));
    assert!(view.contains("\"freshness_token\":0"));
    assert!(view.contains("\"hidden_fields\":[]"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"drop/c4\""));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied = apply_action(&match_id, "seat_0", "drop/c4", 0).expect("human action applies");
    assert!(applied.contains("\"type\":\"piece_landed\""));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"type\":\"bot_chose_action\""));
    assert!(bot.contains("\"ply_count\":2"));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"type\":\"piece_landed\""));
    assert!(effects.contains("\"type\":\"bot_chose_action\""));

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"game_id\":\"column_four\""));
    assert!(exported.contains("\"rules_version\":\"column_four-rules-v1\""));
    assert!(exported.contains("\"expected_replay_hashes\""));
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    assert!(exported.contains("\"private_view_hashes\":\"column_four has no private-view API.\""));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"column_four\""));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported =
        import_replay(&legacy_exported).expect("legacy hyphen column four export imports");
    assert!(legacy_imported.contains("\"game_id\":\"column_four\""));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"ply_count\":0"));

    let step = replay_step(&replay_id, 1).expect("replay stepped");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"ply_count\":1"));
}

#[test]
fn directional_flip_surface_drives_operation_group() {
    let created = new_match("directional_flip", 51).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"directional_flip_standard\""));

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"game_id\":\"directional_flip\""));
    assert!(view.contains("\"variant_id\":\"directional_flip_standard\""));
    assert!(view.contains("\"board_rows\":8"));
    assert!(view.contains("\"board_columns\":8"));
    assert!(view.contains("\"freshness_token\":0"));
    assert!(view.contains("\"score\":{\"seat_0\":2,\"seat_1\":2}"));
    assert!(view.contains("\"hidden_fields\":[]"));
    assert!(view.contains("\"ordered_flip_cells\""));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"place/"));
    assert!(tree.contains("\"freshness_token\":0"));

    let action_segment = tree
        .split("\"segment\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("placement segment present")
        .to_owned();
    let applied =
        apply_action(&match_id, "seat_0", &action_segment, 0).expect("human action applies");
    assert!(applied.contains("\"type\":\"disc_placed\""));
    assert!(applied.contains("\"type\":\"discs_flipped\""));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"type\":\"bot_chose_action\""));
    assert!(bot.contains("\"ply_count\":2"));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"type\":\"disc_placed\""));
    assert!(effects.contains("\"type\":\"bot_chose_action\""));

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"game_id\":\"directional_flip\""));
    assert!(exported.contains("\"rules_version\":\"directional_flip-rules-v1\""));
    assert!(exported.contains("\"expected_replay_hashes\""));
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    assert!(
        exported.contains("\"private_view_hashes\":\"directional_flip has no private-view API.\"")
    );
    assert!(!exported.contains("initial_snapshot"));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"directional_flip\""));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported =
        import_replay(&legacy_exported).expect("legacy hyphen directional export imports");
    assert!(legacy_imported.contains("\"game_id\":\"directional_flip\""));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"ply_count\":0"));

    let step = replay_step(&replay_id, 1).expect("replay stepped");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"ply_count\":1"));
}

#[test]
fn draughts_lite_surface_preserves_multi_segment_paths() {
    let created = new_match("draughts_lite", 61).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"draughts_lite_standard\""));

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"game_id\":\"draughts_lite\""));
    assert!(view.contains("\"variant_id\":\"draughts_lite_standard\""));
    assert!(view.contains("\"board_rows\":8"));
    assert!(view.contains("\"board_columns\":8"));
    assert!(view.contains("\"freshness_token\":0"));
    assert!(view.contains("\"private_view_status\":\"not_applicable_perfect_information\""));
    assert!(view.contains("\"hidden_fields\":[]"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"from/"));
    assert!(tree.contains("\"next\":{\"choices\":["));
    assert!(tree.contains("\"segment\":\"to/"));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied = apply_action(&match_id, "seat_0", "from/r3c2>to/r4c1", 0)
        .expect("multi-segment human action applies");
    assert!(applied.contains("\"type\":\"move_committed\""));
    assert!(applied.contains("\"action_path\":[\"from/r3c2\",\"to/r4c1\"]"));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"type\":\"bot_chose_action\""));
    assert!(bot.contains("\"ply_count\":2"));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"type\":\"move_committed\""));
    assert!(effects.contains("\"type\":\"bot_chose_action\""));

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"game_id\":\"draughts_lite\""));
    assert!(exported.contains("\"rules_version\":\"draughts_lite-rules-v1\""));
    assert!(exported.contains("\"expected_replay_hashes\""));
    assert!(exported.contains("\"action_path\":[\"from/r3c2\",\"to/r4c1\"]"));
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    assert!(exported.contains("\"private_view_hashes\":\"draughts_lite has no private-view API.\""));
    assert!(!exported.contains("initial_snapshot"));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"draughts_lite\""));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported =
        import_replay(&legacy_exported).expect("legacy hyphen draughts export imports");
    assert!(legacy_imported.contains("\"game_id\":\"draughts_lite\""));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"ply_count\":0"));

    let step = replay_step(&replay_id, 1).expect("replay stepped");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"ply_count\":1"));
}

#[test]
fn high_card_duel_surface_filters_hidden_information() {
    let created = new_match("high_card_duel", 71).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"high_card_duel_standard\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    assert!(observer.contains("\"game_id\":\"high_card_duel\""));
    assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
    assert!(!observer.contains("hcd:r"));

    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
    assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
    assert!(seat_0.contains("hcd:r"));

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized tree returned");
    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
        .expect("unauthorized tree returned");
    let observer_tree =
        get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");

    assert!(authorized.contains("\"segment\":\"commit/hcd:r"));
    assert!(unauthorized.contains("\"choices\":[]"));
    assert!(observer_tree.contains("\"choices\":[]"));

    let action_segment = authorized
        .split("\"segment\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("commit segment present")
        .to_owned();
    let applied = apply_action(&match_id, "seat_0", &action_segment, 0).expect("commit applies");
    assert!(applied.contains("\"type\":\"own_commit_confirmed\""));
    assert!(applied.contains("hcd:r"));
    assert!(applied.contains("\"private_to_seat\":\"seat-0\""));

    let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
    assert!(observer_effects.contains("\"type\":\"commit_face_down\""));
    assert!(!observer_effects.contains("hcd:r"));

    let seat_0_effects = get_effects(&match_id, 0, Some("seat_0")).expect("seat effects");
    assert!(seat_0_effects.contains("\"type\":\"own_commit_confirmed\""));
    assert!(seat_0_effects.contains("hcd:r"));

    let seat_1_effects = get_effects(&match_id, 0, Some("seat_1")).expect("other effects");
    assert!(seat_1_effects.contains("\"type\":\"commit_face_down\""));
    assert!(!seat_1_effects.contains("hcd:r"));

    let exported = export_replay(&match_id).expect("public replay exported");
    assert!(exported.contains("\"export_class\":\"public_observer_projection_v1\""));
    assert!(exported.contains("\"viewer\":\"observer\""));
    assert!(!exported.contains("\"seed\""));
    assert!(!exported.contains("hcd:r"));

    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"public_export\":true"));
    assert!(imported.contains("\"game_id\":\"high_card_duel\""));
    let reset = replay_reset(&replay_id).expect("public replay reset returned");
    assert!(reset.contains("\"public_export\":true"));
    assert!(reset.contains("\"view\":null"));

    let pretty_exported = pretty_json_layout(&exported);
    assert!(pretty_exported.contains("\"export_class\": \"public_observer_projection_v1\""));
    assert!(pretty_exported.contains("\"game_id\": \"high_card_duel\""));
    let pretty_imported = import_replay(&pretty_exported).expect("pretty public replay imported");
    let pretty_replay_id = extract_replay_id(&pretty_imported);
    assert!(pretty_imported.contains("\"public_export\":true"));
    assert!(pretty_imported.contains("\"game_id\":\"high_card_duel\""));
    assert!(!pretty_imported.contains("hcd:r"));
    let pretty_reset = replay_reset(&pretty_replay_id).expect("pretty public replay reset");
    assert!(pretty_reset.contains("\"public_export\":true"));
    assert!(!pretty_reset.contains("hcd:r"));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"type\":\"cards_revealed\""));
}

#[test]
fn pairwise_no_leak_harness_covers_high_card_and_synthetic_n_seats() {
    let high_card = high_card_pairwise_no_leak_case();
    assert_pairwise_no_leak(&high_card);

    let synthetic = synthetic_n_seat_no_leak_case(4);
    assert_pairwise_no_leak(&synthetic);
    assert_eq!(
        synthetic.deterministic_summary(),
        synthetic_n_seat_no_leak_case(4).deterministic_summary()
    );
}

#[test]
fn hidden_info_bridge_games_invoke_pairwise_no_leak_harness() {
    for case in [
        high_card_pairwise_no_leak_case(),
        poker_lite_pairwise_no_leak_case(),
        plain_tricks_pairwise_no_leak_case(),
        masked_claims_pairwise_no_leak_case(),
        river_ledger_pairwise_no_leak_case(),
        blackglass_pact_pairwise_no_leak_case(),
    ] {
        assert_pairwise_no_leak(&case);
    }
}

#[test]
fn pairwise_no_leak_harness_negative_fixture_fails() {
    let mut synthetic = synthetic_n_seat_no_leak_case(4);
    let leaked = synthetic
        .private_terms_by_seat
        .get("seat_0")
        .and_then(|terms| terms.first())
        .expect("seat_0 private term")
        .clone();
    synthetic.surfaces.push(NoLeakSurface {
        viewer: Some("seat_2".to_owned()),
        name: "negative_induced_leak",
        payload: format!("viewer=seat_2;leaked={leaked}"),
    });

    let message = pairwise_no_leak_result(&synthetic).expect_err("induced leak is caught");
    assert!(message.contains("seat_0"));
    assert!(message.contains("seat_2"));
    assert!(message.contains("negative_induced_leak"));
}

#[test]
fn token_bazaar_surface_drives_public_accounting_group() {
    let created = new_match("token_bazaar", 81).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"token_bazaar_standard\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
    assert_eq!(observer, seat_0);
    assert!(observer.contains("\"game_id\":\"token_bazaar\""));
    assert!(observer.contains("\"variant_id\":\"token_bazaar_standard\""));
    assert!(observer.contains("\"supply\":{\"amber\":14,\"jade\":14,\"iron\":14}"));
    assert!(observer.contains("\"hidden_fields\":[]"));
    assert!(!observer.contains("candidate"));
    assert!(!observer.contains("debug"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"collect/amber\""));
    assert!(tree.contains("\"segment\":\"fulfill/slot_0\""));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied = apply_action(&match_id, "seat_0", "collect/amber", 0).expect("collect applies");
    assert!(applied.contains("\"type\":\"resource_collected\""));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"active_seat\":\"seat_0\""));

    let effects = get_effects(&match_id, 0, None).expect("effects returned");
    assert!(effects.contains("\"type\":\"resource_collected\""));
    assert!(effects.contains("\"visibility\":\"public\""));
    assert!(!effects.contains("candidate"));
    assert!(!effects.contains("debug"));

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"game_id\":\"token_bazaar\""));
    assert!(exported.contains("\"rules_version\":\"token-bazaar-rules-v1\""));
    assert!(exported.contains("\"expected_public_export_hashes\""));
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    assert!(!exported.contains("\"state\":"));
    assert!(!exported.contains("candidate"));
    assert!(!exported.contains("debug"));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"token_bazaar\""));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported =
        import_replay(&legacy_exported).expect("legacy hyphen token bazaar export imports");
    assert!(legacy_imported.contains("\"game_id\":\"token_bazaar\""));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"game_id\":\"token_bazaar\""));

    let step = replay_step(&replay_id, 1).expect("replay stepped");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"type\":\"resource_collected\""));
}

#[test]
fn secret_draft_surface_filters_hidden_commitments() {
    let created = new_match("secret_draft", 91).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"secret_draft_standard\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
    assert!(observer.contains("\"game_id\":\"secret_draft\""));
    assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
    assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
    assert!(!seat_0.contains("own_commitment"));

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized tree returned");
    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
        .expect("unauthorized tree returned");
    let observer_tree =
        get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
    assert!(authorized.contains("\"segment\":\"commit/"));
    assert!(unauthorized.contains("\"choices\":[]"));
    assert!(observer_tree.contains("\"choices\":[]"));

    let action_segment = authorized
        .split("\"segment\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("commit segment present")
        .to_owned();
    let applied = apply_action(&match_id, "seat_0", &action_segment, 0).expect("commit applies");
    assert!(applied.contains("\"type\":\"own_commit_accepted\""));
    assert!(applied.contains("\"own_committed\":true"));
    assert!(!applied.contains("\"item_id\":\"commit/"));

    let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
    assert!(observer_effects.contains("\"type\":\"commitment_placed\""));
    assert!(observer_effects.contains("\"type\":\"own_commit_accepted\""));
    assert!(!observer_effects.contains(&action_segment));

    let seat_0_effects = get_effects(&match_id, 0, Some("seat_0")).expect("seat effects");
    assert!(seat_0_effects.contains("\"type\":\"own_commit_accepted\""));
    assert!(!seat_0_effects.contains(&action_segment));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"type\":\"choices_revealed\""));
    assert!(!bot.contains("candidate"));
    assert!(!bot.contains("debug"));

    let exported = export_replay(&match_id).expect("public replay exported");
    assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
    assert!(exported.contains("\"viewer\":\"observer\""));
    assert!(!exported.contains("\"commands\""));
    assert!(!exported.contains("\"path\""));
    assert!(!exported.contains("\"seed_evidence\""));

    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"public_export\":true"));
    assert!(imported.contains("\"game_id\":\"secret_draft\""));

    let reset = replay_reset(&replay_id).expect("public replay reset returned");
    assert!(reset.contains("\"public_export\":true"));
    assert!(reset.contains("\"view\":null"));
}

#[test]
fn poker_lite_surface_filters_hidden_cards() {
    let created = new_match("poker_lite", 101).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"poker_lite_standard\""));

    let internal = poker_setup_match(Seed(101), &seats(), &poker_lite::SetupOptions::default())
        .expect("setup succeeds");
    let seat_0_card = internal.private_card_for_internal(PokerLiteSeat::Seat0);
    let seat_1_card = internal.private_card_for_internal(PokerLiteSeat::Seat1);
    let center_card = internal.center_card_internal();

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
    let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat view returned");
    assert!(observer.contains("\"game_id\":\"poker_lite\""));
    assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
    assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
    assert!(seat_0.contains(seat_0_card.as_str()));
    assert!(seat_1.contains(seat_1_card.as_str()));
    assert!(!observer.contains("\"rank\":"));
    assert_no_poker_cards(&observer, &[seat_0_card, seat_1_card, center_card]);
    assert!(!seat_1.contains(seat_0_card.as_str()));
    assert!(!seat_0.contains(seat_1_card.as_str()));
    assert!(!seat_0.contains(center_card.as_str()));
    assert!(!seat_1.contains(center_card.as_str()));
    if seat_0_card.rank() != seat_1_card.rank() {
        assert!(!seat_1.contains(&format!("\"rank\":\"{}\"", seat_0_card.rank().as_str())));
        assert!(!seat_0.contains(&format!("\"rank\":\"{}\"", seat_1_card.rank().as_str())));
    }
    if center_card.rank() != seat_0_card.rank() {
        assert!(!seat_0.contains(&format!("\"rank\":\"{}\"", center_card.rank().as_str())));
    }
    if center_card.rank() != seat_1_card.rank() {
        assert!(!seat_1.contains(&format!("\"rank\":\"{}\"", center_card.rank().as_str())));
    }

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized tree returned");
    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
        .expect("unauthorized tree returned");
    let observer_tree =
        get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
    assert!(authorized.contains("\"segment\":\"hold\""));
    assert!(authorized.contains("\"segment\":\"press\""));
    assert!(unauthorized.contains("\"choices\":[]"));
    assert!(observer_tree.contains("\"choices\":[]"));

    let applied = apply_action(&match_id, "seat_0", "press", 0).expect("press applies");
    assert!(applied.contains("\"type\":\"pledge_pressed\""));
    assert!(!applied.contains(seat_1_card.as_str()));
    assert!(!applied.contains(center_card.as_str()));

    let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
    assert!(observer_effects.contains("\"type\":\"pledge_pressed\""));
    assert_no_poker_cards(&observer_effects, &[seat_0_card, seat_1_card, center_card]);

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    assert!(bot.contains("\"ok\":true"));
    assert!(bot.contains("\"policy_id\":\"poker-lite-crest-ledger-level2-v1\""));
    assert!(!bot.contains(seat_0_card.as_str()));

    let seat_1_effects = get_effects(&match_id, 0, Some("seat_1")).expect("seat effects");
    assert!(seat_1_effects.contains("\"type\":\"bot_chose_action_private\""));
    assert!(seat_1_effects.contains("\"strength_bucket\""));
    assert!(!seat_1_effects.contains(seat_0_card.as_str()));

    let exported = export_replay(&match_id).expect("public replay exported");
    assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
    assert!(exported.contains("\"viewer\":\"observer\""));
    assert!(!exported.contains("\"commands\""));
    assert!(!exported.contains("\"path\""));
    assert!(!exported.contains("\"seed_evidence\""));
    assert_no_poker_cards(&exported, &[seat_0_card, seat_1_card]);

    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"public_export\":true"));
    assert!(imported.contains("\"game_id\":\"poker_lite\""));

    let reset = replay_reset(&replay_id).expect("public replay reset returned");
    assert!(reset.contains("\"public_export\":true"));
    assert!(reset.contains("\"view\":null"));
}

#[test]
fn plain_tricks_surface_filters_hidden_cards_and_authorizes_actor() {
    let seed = 101;
    let created = new_match("plain_tricks", seed).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"plain_tricks_standard\""));

    let internal = plain_setup_match(
        Seed(seed),
        &plain_seats(),
        &plain_tricks::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat_0_view = plain_project_view(
        &internal,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );
    let seat_1_view = plain_project_view(
        &internal,
        &Viewer {
            seat_id: Some(SeatId("seat_1".to_owned())),
        },
    );
    let seat_0_cards = plain_private_cards(&seat_0_view);
    let seat_1_cards = plain_private_cards(&seat_1_view);
    let hidden_cards = plain_hidden_cards_except(&[]);
    let seat_0_private = plain_cards_except(&seat_0_cards, &[]);
    let seat_1_private = plain_cards_except(&seat_1_cards, &[]);

    let observer = get_view(&match_id, None).expect("observer view returned");
    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat_0 view returned");
    let seat_1 = get_view(&match_id, Some("seat_1")).expect("seat_1 view returned");
    assert!(observer.contains("\"game_id\":\"plain_tricks\""));
    assert!(observer.contains("\"private_view\":{\"status\":\"observer\""));
    assert!(seat_0.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_0\""));
    assert!(seat_1.contains("\"private_view\":{\"status\":\"seat\",\"seat\":\"seat_1\""));
    assert_no_plain_cards(&observer, &hidden_cards);
    for card in &seat_0_cards {
        assert!(seat_0.contains(card.as_str()));
    }
    for card in &seat_1_cards {
        assert!(seat_1.contains(card.as_str()));
    }
    assert_no_plain_cards(&seat_0, &seat_1_private);
    assert_no_plain_cards(&seat_1, &seat_0_private);

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized tree returned");
    let unauthorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_1"))
        .expect("unauthorized tree returned");
    let observer_tree =
        get_action_tree_for_viewer(&match_id, "seat_0", None).expect("observer tree returned");
    let first_card = seat_0_cards[0];
    assert!(authorized.contains("\"segment\":\"play\""));
    assert!(authorized.contains(first_card.as_str()));
    assert!(unauthorized.contains("\"choices\":[]"));
    assert!(observer_tree.contains("\"choices\":[]"));

    let applied = apply_action(
        &match_id,
        "seat_0",
        &format!("play>{}", first_card.as_str()),
        0,
    )
    .expect("plain trick card applies");
    assert!(applied.contains("\"type\":\"card_played\""));
    assert!(applied.contains(first_card.as_str()));
    assert_no_plain_cards(&applied, &seat_1_private);

    let observer_effects = get_effects(&match_id, 0, None).expect("observer effects");
    assert!(observer_effects.contains("\"type\":\"card_played\""));
    assert_no_plain_cards(&observer_effects, &plain_hidden_cards_except(&[first_card]));
}

#[test]
fn masked_claims_bridge_filters_unrevealed_masks_and_exports_redacted_claims() {
    let created = new_match("masked_claims", 41).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"masked_claims_standard\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    assert!(observer.contains("\"game_id\":\"masked_claims\""));
    assert!(observer.contains("\"status\":\"observer\""));
    assert!(
        !observer.contains("mask_g"),
        "observer view leaked hidden mask id: {observer}"
    );

    let seat_0 = get_view(&match_id, Some("seat_0")).expect("seat view returned");
    let first_mask = first_mask_segment(&seat_0);
    assert!(seat_0.contains(&first_mask));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"claim\""));
    assert!(tree.contains(&first_mask));
    let applied = apply_action(&match_id, "seat_0", &format!("claim>{first_mask}>5"), 0)
        .expect("claim applies");
    assert!(applied.contains("reaction"));
    assert!(
        !applied.contains(&format!("\"tile_id\":\"{first_mask}\"")),
        "pending claim leaked pedestal tile id: {applied}"
    );

    let exported = export_replay(&match_id).expect("export succeeds");
    assert!(exported.contains("\"game_id\":\"masked_claims\""));
    assert!(exported.contains("claim/grade-5"));
    assert!(
        !exported.contains("claim/mask_g"),
        "export leaked raw claim path: {exported}"
    );
    let imported = import_replay(&exported).expect("public export imports");
    assert!(imported.contains("\"game_id\":\"masked_claims\""));
    assert!(imported.contains("\"public_export\":true"));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot response applies");
    assert!(bot.contains("\"policy_id\":\"masked-claims-level1-v1\""));
    assert!(!bot.contains("reserve"));
}

#[test]
fn flood_watch_bridge_projects_public_view_effects_bot_and_export_without_deck_order() {
    let created = new_match("flood_watch", 41).expect("match created");
    let match_id = extract_match_id(&created);
    assert!(created.contains("\"variant_id\":\"flood_watch_standard\""));

    let observer = get_view(&match_id, None).expect("observer view returned");
    assert!(observer.contains("\"game_id\":\"flood_watch\""));
    assert!(observer.contains("\"variant_id\":\"flood_watch_standard\""));
    assert!(observer.contains("\"undrawn_count\":"));
    assert!(!observer.contains("full_deck_order"));
    assert!(!observer.contains("\"event_deck\":"));

    let tree = get_action_tree(&match_id, "seat_0").expect("action tree returned");
    assert!(tree.contains("\"segment\":\"end_turn\""));
    assert!(tree.contains("\"freshness_token\":0"));

    let applied =
        apply_action(&match_id, "seat_0", "end_turn", 0).expect("turn-ending action applies");
    assert!(applied.contains("\"type\":\"environment_phase_began\""));
    assert!(applied.contains("\"type\":\"event_drawn\""));
    assert!(applied.contains("\"active_seat\":\"seat_1\""));
    assert!(!applied.contains("full_deck_order"));
    assert!(!applied.contains("\"event_deck\":"));

    let bot = run_bot_turn(&match_id, "seat_1", 99).expect("bot action applies");
    assert!(bot.contains("\"policy_id\":\"flood_watch_level1_public_priority_v1\""));
    assert!(!bot.contains("full_deck_order"));
    assert!(!bot.contains("\"event_deck\":"));

    let exported = export_replay(&match_id).expect("public replay exported");
    assert!(exported.contains("\"game_id\":\"flood_watch\""));
    assert!(exported.contains("\"rules_version_label\":\"flood-watch-rules-v1\""));
    assert!(exported.contains("\"viewer\":\"observer\""));
    assert!(exported.contains("\"redacted_command_summary\""));
    assert!(!exported.contains("\"commands\""));
    assert!(!exported.contains("\"full_deck_order\""));
    assert!(!exported.contains("\"event_deck\""));

    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"game_id\":\"flood_watch\""));
    assert!(imported.contains("\"public_export\":true"));

    let reset = replay_reset(&replay_id).expect("public replay reset returned");
    assert!(reset.contains("\"public_export\":true"));
    assert!(reset.contains("\"view\":null"));
}

#[test]
fn plain_tricks_public_export_omits_seed_tail_and_unplayed_cards() {
    let seed = 0;
    let created = new_match("plain_tricks", seed).expect("match created");
    let match_id = extract_match_id(&created);

    let internal = plain_setup_match(
        Seed(seed),
        &plain_seats(),
        &plain_tricks::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat_0_view = plain_project_view(
        &internal,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );
    let seat_0_cards = plain_private_cards(&seat_0_view);
    let played_card = seat_0_cards[0];
    apply_action(
        &match_id,
        "seat_0",
        &format!("play>{}", played_card.as_str()),
        0,
    )
    .expect("card applies");

    let exported = export_replay(&match_id).expect("public replay exported");
    assert!(exported.contains("\"game_id\":\"plain_tricks\""));
    assert!(exported.contains("\"export_class\":\"viewer_scoped_observation_v1\""));
    assert!(exported.contains("\"viewer\":\"observer\""));
    assert!(!exported.contains("\"commands\""));
    assert!(!exported.contains("\"seed\""));
    assert!(!exported.contains("\"seed_evidence\""));
    assert_no_plain_cards(&exported, &plain_hidden_cards_except(&[played_card]));

    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    assert!(imported.contains("\"public_export\":true"));
    assert!(imported.contains("\"game_id\":\"plain_tricks\""));

    let reset = replay_reset(&replay_id).expect("public replay reset returned");
    assert!(reset.contains("\"public_export\":true"));
    assert!(reset.contains("\"view\":null"));
}

#[test]
fn poker_lite_view_projects_terminal_rationale_template_keys() {
    let non_terminal = get_terminal_poker_view(0, &[]);
    assert!(non_terminal.contains("\"terminal_rationale\":null"));

    let private_rank_showdown = get_terminal_poker_view(2, &["hold", "hold", "hold", "hold"]);
    assert!(private_rank_showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"higher_private_rank\",\"template_key\":\"poker_lite.private_rank_tiebreak\""
        ));
    assert!(private_rank_showdown
        .contains("\"decisive_rule_ids\":[\"CL-REVEAL-002\",\"CL-SCORE-004\",\"CL-END-002\"]"));
    assert!(private_rank_showdown.contains("\"label\":\"Private rank\""));
    assert!(private_rank_showdown.contains("\"label\":\"Pair\""));

    let pair_showdown = get_terminal_poker_view(0, &["hold", "hold", "hold", "hold"]);
    assert!(pair_showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"pair_beats_high_card\",\"template_key\":\"poker_lite.pair_beats_high_card\""
        ));

    let split = get_terminal_poker_view(1, &["hold", "hold", "hold", "hold"]);
    assert!(split.contains(
            "\"terminal_rationale\":{\"result_kind\":\"split\",\"decisive_cause\":\"equal_strength_split\",\"template_key\":\"poker_lite.equal_strength_split\""
        ));

    let yield_win = get_terminal_poker_view(11, &["press", "yield"]);
    assert!(yield_win.contains(
            "\"terminal_rationale\":{\"result_kind\":\"yield_win\",\"decisive_cause\":\"opponent_yielded\",\"template_key\":\"poker_lite.yield_win_no_reveal\""
        ));
}

#[test]
fn river_ledger_view_projects_terminal_rationale_template_keys() {
    let non_terminal = get_terminal_river_view(21, 4, &[]);
    assert!(non_terminal.contains("\"terminal_rationale\":null"));
    assert!(non_terminal.contains(
        "\"hand_rankings\":[{\"category\":\"straight_flush\",\"label\":\"Straight flush\""
    ));
    assert!(non_terminal.contains("\"category\":\"high_card\",\"label\":\"High card\""));
    assert!(non_terminal.contains(
            "\"seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\""
        ));
    assert!(non_terminal.contains("{\"seat\":\"seat_5\",\"label\":\"Seat 6\"}"));
    assert!(non_terminal.contains(
            "\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"}]"
        ));
    assert!(!non_terminal.contains("\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"},{\"seat\":\"seat_3\",\"label\":\"Seat 4\"},{\"seat\":\"seat_4\""));

    let foldout = get_terminal_river_view(21, 3, &[("seat_0", "fold"), ("seat_1", "fold")]);
    assert!(foldout.contains(
            "\"active_seat_labels\":[{\"seat\":\"seat_0\",\"label\":\"Seat 1\"},{\"seat\":\"seat_1\",\"label\":\"Seat 2\"},{\"seat\":\"seat_2\",\"label\":\"Seat 3\"}]"
        ));
    assert!(foldout.contains(
            "\"terminal_rationale\":{\"result_kind\":\"last_live_hand\",\"decisive_cause\":\"last_live_after_folds\",\"template_key\":\"river_ledger.last_live_fold_win\""
        ));
    assert!(foldout.contains("\"decisive_rule_ids\":[\"RL-END-LAST-LIVE\",\"RL-SCORE-POT-AWARD\"]"));
    assert!(!foldout.contains("\"label\":\"Category\""));
    assert!(!foldout.contains("\"label\":\"Tie break\""));

    let internal = river_setup_match(
        Seed(21),
        &river_seats_for_count(3),
        &river_ledger::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let hidden_cards = (0..3)
        .flat_map(|seat_index| {
            internal
                .private_hand_for_internal(
                    RiverLedgerSeat::from_index(seat_index).expect("valid seat"),
                )
                .expect("private hand")
        })
        .collect::<Vec<_>>();
    assert_no_river_cards(&foldout, &hidden_cards);

    let showdown = get_terminal_river_view(
        0,
        4,
        &[
            ("seat_3", "call"),
            ("seat_0", "call"),
            ("seat_1", "call"),
            ("seat_2", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
            ("seat_1", "check"),
            ("seat_2", "check"),
            ("seat_3", "check"),
            ("seat_0", "check"),
        ],
    );
    assert!(showdown.contains(
            "\"terminal_rationale\":{\"result_kind\":\"showdown_win\",\"decisive_cause\":\"best_showdown_hand\",\"template_key\":\"river_ledger.showdown_best_hand_win\""
        ));
    assert!(showdown.contains("\"decisive_rule_ids\":[\"RL-SCORE-SHOWDOWN\",\"RL-END-SHOWDOWN\"]"));
    assert!(showdown.contains("\"label\":\"Category\""));
    assert!(showdown.contains("\"label\":\"Best five\""));
    assert!(showdown.contains("\"headline\":\""));
    assert!(showdown.contains("\"decisive_comparison\":\""));
    assert!(showdown.contains("\"comparison_basis\":\""));
    assert!(showdown.contains("\"strength\":{\"category\":\""));
    assert!(showdown.contains("\"result_label\":\""));
    assert!(showdown.contains("\"hand_name\":\""));
    assert!(showdown.contains("\"rank_explanation\":\""));
    assert!(showdown.contains("\"comparison_note\":\""));
    assert!(showdown.contains("\"category_ladder_position\":{\"position\":"));
    assert!(showdown.contains("\"description\":\""));
    assert!(showdown.contains("\"best_five_accessibility_label\":\""));
    assert!(showdown.contains("\"presentation_v2\":{\"result_banner\":{\"headline\":\""));
    assert!(showdown.contains("\"decisive_reason\":{\"short_text\":\""));
    assert!(showdown.contains("\"standings\":[{\"seat\":\""));
    assert!(showdown.contains("\"hole_cards\":[{\"card\":"));
    assert!(showdown.contains("\"used_in_best_five\":"));
    assert!(showdown.contains("\"folded_rows\":["));
}

#[test]
fn river_ledger_bridge_redacts_folded_showdown_explanation_fields() {
    let seat = |index| RiverLedgerSeat::from_index(index).expect("valid seat");
    let board = [
        river_ledger::Card::new(river_ledger::Rank::Ten, river_ledger::Suit::Hearts),
        river_ledger::Card::new(river_ledger::Rank::Jack, river_ledger::Suit::Hearts),
        river_ledger::Card::new(river_ledger::Rank::Queen, river_ledger::Suit::Hearts),
        river_ledger::Card::new(river_ledger::Rank::King, river_ledger::Suit::Hearts),
        river_ledger::Card::new(river_ledger::Rank::Ace, river_ledger::Suit::Hearts),
    ];
    let mut state = river_ledger::RiverLedgerState::new_after_setup(
        river_ledger::Variant::river_ledger_standard(),
        river_seats_for_count(3),
        river_ledger::state::SeatRoles {
            button: seat(0),
            small_blind: seat(1),
            big_blind: seat(2),
            active_seat: seat(0),
        },
        vec![river_ledger::STANDARD_STARTING_STACK; 3],
        vec![
            [
                river_ledger::Card::new(river_ledger::Rank::Two, river_ledger::Suit::Diamonds),
                river_ledger::Card::new(river_ledger::Rank::Three, river_ledger::Suit::Diamonds),
            ],
            [
                river_ledger::Card::new(river_ledger::Rank::Two, river_ledger::Suit::Clubs),
                river_ledger::Card::new(river_ledger::Rank::Three, river_ledger::Suit::Clubs),
            ],
            [
                river_ledger::Card::new(river_ledger::Rank::Four, river_ledger::Suit::Clubs),
                river_ledger::Card::new(river_ledger::Rank::Five, river_ledger::Suit::Clubs),
            ],
        ],
        board,
        Vec::new(),
    );
    state.board = board.to_vec();
    state.ledger.seats = (0..3)
        .map(|index| river_ledger::SeatLedger {
            seat: seat(index),
            status: if index == 0 {
                river_ledger::SeatStatus::Folded
            } else {
                river_ledger::SeatStatus::ShowdownEligible
            },
            starting_stack: river_ledger::STANDARD_STARTING_STACK,
            remaining_stack: river_ledger::STANDARD_STARTING_STACK - 3,
            street_contribution: 0,
            total_contribution: 3,
        })
        .collect();
    state.ledger.pot_total = 9;
    state.terminal_outcome = Some(river_ledger::resolve_showdown(&state));

    let json = river_view_json(&river_project_view(&state, &Viewer { seat_id: None }));
    assert!(json.contains("\"headline\":\""));
    assert!(json.contains("\"id\":\"seat_0\",\"label\":\"seat_0\",\"result\":\"folded\",\"emphasized\":false,\"strength\":null"));
    assert!(json.contains("\"id\":\"seat_1\""));
    assert!(json.contains("\"strength\":{\"category\":\"straight_flush\""));
    assert!(!json.contains("two_diamonds"));
    assert!(!json.contains("three_diamonds"));
}

#[test]
fn plain_tricks_view_projects_terminal_rationale_template_keys() {
    let non_terminal = get_terminal_plain_view(0, &[]);
    assert!(non_terminal.contains("\"terminal_rationale\":null"));
    assert!(non_terminal
        .contains("\"terminal\":{\"kind\":\"non_terminal\",\"winner\":null,\"draw\":false}"));

    let trick_win = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS);
    assert!(trick_win.contains("\"terminal\":{\"kind\":\"trick_win\""));
    assert!(trick_win.contains("\"terminal_rationale\":{"));
    assert!(!trick_win.contains("\"rationale\":{"));
    assert!(trick_win.contains("\"result_kind\":\"trick_win\""));
    assert!(trick_win.contains("\"template_key\":\"plain_tricks.trick_win\""));
    assert!(trick_win
        .contains("\"decisive_rule_ids\":[\"PT-SCORE-002\",\"PT-END-001\",\"PT-END-002\"]"));
    assert!(trick_win.contains("\"total_tricks\":"));

    let split = get_terminal_plain_view(5, &PLAIN_TRICKS_SPLIT_ACTIONS);
    assert!(split.contains("\"terminal\":{\"kind\":\"split\""));
    assert!(split.contains("\"terminal_rationale\":{"));
    assert!(!split.contains("\"rationale\":{"));
    assert!(split.contains("\"result_kind\":\"split\""));
    assert!(split.contains("\"decisive_cause\":\"split:6-6\""));
    assert!(split.contains("\"template_key\":\"plain_tricks.split\""));
    assert!(
        split.contains("\"decisive_rule_ids\":[\"PT-SCORE-002\",\"PT-END-001\",\"PT-END-002\"]")
    );
}

#[test]
fn plain_tricks_terminal_rationale_does_not_reveal_unplayed_cards() {
    let view = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS[..2]);
    assert!(view.contains("\"terminal_rationale\":null"));

    let terminal = get_terminal_plain_view(0, &PLAIN_TRICKS_WIN_ACTIONS);
    assert!(terminal.contains("\"terminal_rationale\":{"));
    assert!(terminal.contains("\"template_key\":\"plain_tricks.trick_win\""));
    assert_no_plain_cards(
        &terminal,
        &plain_hidden_cards_except(&PLAIN_TRICKS_WIN_PLAYED_CARDS),
    );
}

#[test]
fn poker_lite_yield_terminal_rationale_does_not_reveal_private_strength() {
    let seed = 11;
    let view = get_terminal_poker_view(seed, &["press", "yield"]);
    let internal = poker_setup_match(Seed(seed), &seats(), &poker_lite::SetupOptions::default())
        .expect("setup succeeds");

    assert!(view.contains("\"terminal_rationale\":{"));
    assert!(view.contains("\"template_key\":\"poker_lite.yield_win_no_reveal\""));
    assert!(!view.contains("\"label\":\"Pair\""));
    assert!(!view.contains("\"label\":\"Private rank\""));
    assert!(!view.contains("\"rank\":"));
    assert_no_poker_cards(
        &view,
        &[
            internal.private_card_for_internal(PokerLiteSeat::Seat0),
            internal.private_card_for_internal(PokerLiteSeat::Seat1),
            internal.center_card_internal(),
        ],
    );
}

#[test]
fn high_card_public_import_replays_ordered_public_effects() {
    let source_export =
        high_card_export_public_observer_replay(&high_card_duel::generate_internal_full_trace(55));
    let exported = pretty_json_layout(&source_export.to_json());
    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);

    for source_step in &source_export.steps {
        let step = replay_step(&replay_id, source_step.step_index).expect("step returned");
        assert!(step.contains(&format!("\"cursor\":{}", source_step.step_index)));
        assert!(step.contains(&format!(
            "\"public_effects\":{}",
            json_string_array(&source_step.public_effects)
        )));
        assert!(step.contains(&format!(
            "\"redacted_command_summary\":\"{}\"",
            escape_json(&source_step.redacted_command_summary)
        )));
    }

    let initial = replay_step(&replay_id, 0).expect("initial step returned");
    assert!(initial.contains("\"public_effects\":[]"));

    let reveal = replay_step(&replay_id, 2).expect("reveal step returned");
    assert!(reveal.contains("hcd_cards_revealed:round=1;"));
    assert!(reveal.contains("hcd_round_scored:round=1;"));
    assert_ordered(
        &reveal,
        "hcd_cards_revealed:round=1;",
        "hcd_round_scored:round=1;",
    );

    let terminal_index = source_export.steps.len() - 1;
    let terminal = replay_step(&replay_id, terminal_index).expect("terminal step returned");
    assert!(terminal.contains("hcd_terminal:winner="));
    assert!(terminal.contains(&format!("\"cursor\":{terminal_index}")));

    let clamped = replay_step(&replay_id, terminal_index + 99).expect("clamped step returned");
    assert!(clamped.contains(&format!("\"cursor\":{terminal_index}")));
    assert!(clamped.contains("hcd_terminal:winner="));
}

#[test]
fn high_card_public_import_step_json_adds_no_hidden_facts() {
    let source_export =
        high_card_export_public_observer_replay(&high_card_duel::generate_internal_full_trace(55));
    let exported = source_export.to_json();
    let imported = import_replay(&exported).expect("public replay imported");
    let replay_id = extract_replay_id(&imported);
    let public_card_ids = card_ids_in(&exported);

    for cursor in 0..source_export.steps.len() {
        let step = replay_step(&replay_id, cursor).expect("step returned");
        assert!(
            !step.contains("\"seed\""),
            "seed leaked at cursor {cursor}: {step}"
        );
        assert!(
            !step.contains("commit/hcd:r"),
            "private command path leaked at cursor {cursor}: {step}"
        );
        for card_id in card_ids_in(&step) {
            assert!(
                public_card_ids.contains(&card_id),
                "step introduced card id {card_id} absent from source public export"
            );
        }
    }
}

#[test]
fn stale_action_returns_diagnostic_without_mutation() {
    let created = new_match("race_to_n", 12).expect("match created");
    let match_id = extract_match_id(&created);

    apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
    let stale = apply_action(&match_id, "seat_1", "add-1", 0).expect_err("stale token rejected");
    assert!(stale.contains("\"code\":\"stale_action\""));

    let view = get_view(&match_id, None).expect("view returned");
    assert!(view.contains("\"counter\":1"));
    assert!(view.contains("\"freshness_token\":1"));
}

#[test]
fn replay_round_trip_reproduces_hashes() {
    let created = new_match("race_to_n", 21).expect("match created");
    let match_id = extract_match_id(&created);
    apply_action(&match_id, "seat_0", "add-1", 0).expect("first action applies");
    apply_action(&match_id, "seat_1", "add-2", 1).expect("second action applies");

    let exported = export_replay(&match_id).expect("replay exported");
    assert!(exported.contains("\"seat_id\":\"seat_0\""));
    assert!(exported.contains("\"actor_seat\":\"seat_0\""));
    assert!(!exported.contains("seat-0"));
    let expected = race_replay_commands(21, &["add-1".to_owned(), "add-2".to_owned()]);
    assert!(exported.contains(&format!(
        "\"expected_state_hashes\":{{\"final\":{}}}",
        expected.state_hash.0
    )));
    assert!(exported.contains(&format!(
        "\"expected_effect_hashes\":{{\"final\":{}}}",
        expected.effect_hash.0
    )));

    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);
    let stepped = replay_step(&replay_id, 2).expect("replay stepped");
    assert!(stepped.contains("\"cursor\":2"));
    assert!(stepped.contains("\"counter\":3"));
    assert!(stepped.contains("\"done\":true"));

    let legacy_exported = exported
        .replace("seat_0", "seat-0")
        .replace("seat_1", "seat-1");
    let legacy_imported = import_replay(&legacy_exported).expect("legacy hyphen export imports");
    assert!(legacy_imported.contains("\"game_id\":\"race_to_n\""));
}

#[test]
fn import_rejects_wrong_game_version_malformed_and_oversized() {
    let created = new_match("race_to_n", 22).expect("match created");
    let match_id = extract_match_id(&created);
    apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
    let exported = export_replay(&match_id).expect("replay exported");
    let matches_before = match_count();
    let replays_before = replay_count();

    let wrong_game = exported.replace("\"game_id\":\"race_to_n\"", "\"game_id\":\"wrong\"");
    assert!(import_replay(&wrong_game)
        .expect_err("wrong game rejected")
        .contains("\"code\":\"unsupported_replay_game\""));

    let wrong_rules = exported.replace(
        "\"rules_version\":\"race_to_n-rules-v1\"",
        "\"rules_version\":\"race_to_n-rules-v99\"",
    );
    assert!(import_replay(&wrong_rules)
        .expect_err("wrong rules rejected")
        .contains("\"code\":\"unsupported_replay_rules\""));

    assert!(import_replay("{ nope")
        .expect_err("malformed replay rejected")
        .contains("\"code\":\"invalid_replay\""));

    let oversized = "x".repeat(MAX_REPLAY_IMPORT_BYTES + 1);
    assert!(import_replay(&oversized)
        .expect_err("oversized replay rejected")
        .contains("\"code\":\"replay_too_large\""));

    let unexpected_export_class = exported.replacen(
        '{',
        "{\"export_class\":\"public_observer_projection_v1\",",
        1,
    );
    assert!(import_replay(&unexpected_export_class)
        .expect_err("unknown export_class rejected on generic path")
        .contains("unknown field `export_class`"));

    assert_eq!(match_count(), matches_before);
    assert_eq!(replay_count(), replays_before);
}

#[test]
fn replay_step_and_reset_match_rust_replay() {
    let created = new_match("race_to_n", 23).expect("match created");
    let match_id = extract_match_id(&created);
    apply_action(&match_id, "seat_0", "add-3", 0).expect("action applies");
    let live_view = get_view(&match_id, None).expect("live view returned");
    let exported = export_replay(&match_id).expect("replay exported");
    let imported = import_replay(&exported).expect("replay imported");
    let replay_id = extract_replay_id(&imported);

    let step = replay_step(&replay_id, 1).expect("replay step returned");
    assert!(step.contains("\"cursor\":1"));
    assert!(step.contains("\"counter\":3"));
    assert!(step.contains("\"type\":\"counter_advanced\""));
    assert!(step.contains(&format!("\"view\":{live_view}")));

    let reset = replay_reset(&replay_id).expect("replay reset returned");
    assert!(reset.contains("\"cursor\":0"));
    assert!(reset.contains("\"counter\":0"));
    assert!(reset.contains("\"effects\":[]"));
}

#[test]
fn replay_export_omits_internal_state_surfaces() {
    let created = new_match("race_to_n", 24).expect("match created");
    let match_id = extract_match_id(&created);
    apply_action(&match_id, "seat_0", "add-1", 0).expect("action applies");
    let exported = export_replay(&match_id).expect("replay exported");

    assert!(exported.contains("\"commands\":["));
    assert!(!exported.contains("initial_snapshot"));
    assert!(!exported.contains("legal_additions"));
    assert!(!exported.contains("\"state\":"));
    assert!(!exported.contains("\"effects\":["));
}

#[test]
fn three_marks_wasm_export_matches_golden_fixture() {
    let commands = vec![AppliedCommand {
        actor_seat: "seat-0".to_owned(),
        action_path: vec!["place/r1c1".to_owned()],
        freshness_token: 0,
    }];
    let exported =
        three_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
    let fixture =
        include_str!("../../../games/three_marks/tests/golden_traces/wasm-exported.trace.json");

    assert_eq!(compact_json_layout(fixture), exported);
}

#[test]
fn draughts_lite_wasm_export_matches_golden_fixture() {
    let commands = vec![AppliedCommand {
        actor_seat: "seat-0".to_owned(),
        action_path: vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
        freshness_token: 0,
    }];
    let exported =
        draughts_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
    let fixture =
        include_str!("../../../games/draughts_lite/tests/golden_traces/wasm-exported.trace.json");

    assert_eq!(compact_json_layout(fixture), exported);
}

#[test]
fn token_bazaar_wasm_export_matches_golden_fixture() {
    let commands = vec![
        AppliedCommand {
            actor_seat: "seat_0".to_owned(),
            action_path: vec!["collect/amber".to_owned()],
            freshness_token: 0,
        },
        AppliedCommand {
            actor_seat: "seat_1".to_owned(),
            action_path: vec!["collect/jade".to_owned()],
            freshness_token: 1,
        },
        AppliedCommand {
            actor_seat: "seat_0".to_owned(),
            action_path: vec!["fulfill/slot_0".to_owned()],
            freshness_token: 2,
        },
    ];
    let exported =
        token_replay_document_json("wasm-exported", 1, &commands).expect("fixture exported");
    let fixture =
        include_str!("../../../games/token_bazaar/tests/golden_traces/wasm-exported.trace.json");

    assert_eq!(compact_json_layout(fixture), exported);
}

#[test]
fn poker_lite_wasm_public_export_matches_golden_fixture() {
    let created = new_match("poker_lite", 101).expect("match created");
    let match_id = extract_match_id(&created);
    apply_action(&match_id, "seat_0", "press", 0).expect("press applies");
    run_bot_turn(&match_id, "seat_1", 99).expect("bot turn applies");
    let exported = export_replay(&match_id).expect("public replay exported");
    let fixture =
        include_str!("../../../games/poker_lite/tests/golden_traces/wasm-exported.trace.json");

    assert_eq!(compact_json_layout(fixture), exported);
}

#[test]
fn plain_tricks_wasm_export_matches_golden_fixture() {
    let commands = [
        AppliedCommand {
            actor_seat: "seat_0".to_owned(),
            action_path: vec!["play".to_owned(), "gale_1".to_owned()],
            freshness_token: 0,
        },
        AppliedCommand {
            actor_seat: "seat_1".to_owned(),
            action_path: vec!["play".to_owned(), "gale_2".to_owned()],
            freshness_token: 1,
        },
    ];
    let exported =
        plain_replay_document_json("wasm-exported", 0, &commands).expect("fixture exported");
    let fixture =
        include_str!("../../../games/plain_tricks/tests/golden_traces/wasm-exported.trace.json");

    assert_eq!(compact_json_layout(fixture), exported);
}

fn extract_match_id(created: &str) -> String {
    created
        .split("\"match_id\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("match id is present")
        .to_owned()
}

fn high_card_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let seats = vec!["seat_0".to_owned(), "seat_1".to_owned()];
    let created = new_match("high_card_duel", 707).expect("match created");
    let match_id = extract_match_id(&created);
    let mut private_terms_by_seat = BTreeMap::new();
    let mut surfaces = Vec::new();

    for viewer in [None, Some("seat_0"), Some("seat_1")] {
        surfaces.push(NoLeakSurface {
            viewer: viewer.map(ToOwned::to_owned),
            name: "payload",
            payload: get_view(&match_id, viewer).expect("viewer payload returned"),
        });
    }

    for seat in &seats {
        let view = get_view(&match_id, Some(seat)).expect("seat payload returned");
        let terms = collect_prefixed_terms(&view, "hcd:r");
        assert!(
            !terms.is_empty(),
            "expected private high-card token for {seat}"
        );
        private_terms_by_seat.insert(seat.clone(), terms);
    }

    for actor in &seats {
        for viewer in [None, Some("seat_0"), Some("seat_1")] {
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "action_tree",
                payload: get_action_tree_for_viewer(&match_id, actor, viewer)
                    .expect("viewer action tree returned"),
            });
        }
    }

    let authorized = get_action_tree_for_viewer(&match_id, "seat_0", Some("seat_0"))
        .expect("authorized action tree");
    let action_segment = first_segment(&authorized);
    let applied = apply_action(&match_id, "seat_0", &action_segment, 0).expect("commit applies");
    surfaces.push(NoLeakSurface {
        viewer: Some("seat_0".to_owned()),
        name: "payload",
        payload: applied,
    });

    for viewer in [None, Some("seat_0"), Some("seat_1")] {
        surfaces.push(NoLeakSurface {
            viewer: viewer.map(ToOwned::to_owned),
            name: "effect_log",
            payload: get_effects(&match_id, 0, viewer).expect("viewer effects returned"),
        });
    }

    surfaces.push(NoLeakSurface {
        viewer: None,
        name: "replay_export",
        payload: export_replay(&match_id).expect("public replay exported"),
    });
    for name in [
        "preview",
        "bot_explanation",
        "candidate_ranking",
        "dom_test_id",
        "storage",
        "log",
    ] {
        surfaces.push(NoLeakSurface {
            viewer: None,
            name,
            payload: format!("high_card_duel {name} not_applicable_or_redacted"),
        });
    }

    PairwiseNoLeakCase {
        seats,
        private_terms_by_seat,
        surfaces,
    }
}

fn poker_lite_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let seed = 727;
    let created = new_match("poker_lite", seed).expect("match created");
    let match_id = extract_match_id(&created);
    let internal = poker_setup_match(Seed(seed), &seats(), &poker_lite::SetupOptions::default())
        .expect("setup succeeds");
    let mut private_terms_by_seat = BTreeMap::new();
    private_terms_by_seat.insert(
        "seat_0".to_owned(),
        vec![internal
            .private_card_for_internal(PokerLiteSeat::Seat0)
            .as_str()
            .to_owned()],
    );
    private_terms_by_seat.insert(
        "seat_1".to_owned(),
        vec![internal
            .private_card_for_internal(PokerLiteSeat::Seat1)
            .as_str()
            .to_owned()],
    );
    bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
}

fn plain_tricks_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let seed = 737;
    let created = new_match("plain_tricks", seed).expect("match created");
    let match_id = extract_match_id(&created);
    let internal = plain_setup_match(
        Seed(seed),
        &plain_seats(),
        &plain_tricks::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let seat_0_view = plain_project_view(
        &internal,
        &Viewer {
            seat_id: Some(SeatId("seat_0".to_owned())),
        },
    );
    let seat_1_view = plain_project_view(
        &internal,
        &Viewer {
            seat_id: Some(SeatId("seat_1".to_owned())),
        },
    );
    let mut private_terms_by_seat = BTreeMap::new();
    private_terms_by_seat.insert(
        "seat_0".to_owned(),
        plain_private_cards(&seat_0_view)
            .iter()
            .map(|card| card.as_str().to_owned())
            .collect(),
    );
    private_terms_by_seat.insert(
        "seat_1".to_owned(),
        plain_private_cards(&seat_1_view)
            .iter()
            .map(|card| card.as_str().to_owned())
            .collect(),
    );
    bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
}

fn masked_claims_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let created = new_match("masked_claims", 747).expect("match created");
    let match_id = extract_match_id(&created);
    let mut private_terms_by_seat = BTreeMap::new();
    for seat in ["seat_0", "seat_1"] {
        let view = get_view(&match_id, Some(seat)).expect("seat view returned");
        let terms = collect_prefixed_terms(&view, "mask_g");
        assert!(!terms.is_empty(), "expected private mask ids for {seat}");
        private_terms_by_seat.insert(seat.to_owned(), terms);
    }
    bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
}

fn river_ledger_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let seed = 757;
    let created = new_match_with_seat_count("river_ledger", seed, 6).expect("match created");
    let match_id = extract_match_id(&created);
    let internal = river_setup_match(
        Seed(seed),
        &river_seats_for_count(6),
        &river_ledger::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut private_terms_by_seat = BTreeMap::new();
    for seat_index in 0..6 {
        let seat = RiverLedgerSeat::from_index(seat_index).expect("valid seat");
        let cards = internal
            .private_hand_for_internal(seat)
            .expect("seat has private hand")
            .iter()
            .map(|card| card.id().to_owned())
            .collect::<Vec<_>>();
        private_terms_by_seat.insert(seat.as_str(), cards);
    }
    bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
}

fn blackglass_pact_pairwise_no_leak_case() -> PairwiseNoLeakCase {
    let seed = 767;
    let created = new_match("blackglass_pact", seed).expect("match created");
    let match_id = extract_match_id(&created);
    let internal = blackglass_pact::setup_match(
        Seed(seed),
        &blackglass_pact::canonical_seat_ids(),
        &blackglass_pact::SetupOptions::default(),
    )
    .expect("setup succeeds");
    let mut private_terms_by_seat = BTreeMap::new();
    for seat in blackglass_pact::BlackglassSeat::ALL {
        let cards = internal
            .hand_for_internal(seat)
            .iter()
            .map(|card| card.as_str())
            .collect::<Vec<_>>();
        assert!(
            !cards.is_empty(),
            "expected private hand for {}",
            seat.as_str()
        );
        private_terms_by_seat.insert(seat.as_str().to_owned(), cards);
    }
    bridge_pairwise_no_leak_case(&match_id, private_terms_by_seat)
}

fn bridge_pairwise_no_leak_case(
    match_id: &str,
    private_terms_by_seat: BTreeMap<String, Vec<String>>,
) -> PairwiseNoLeakCase {
    let seats = private_terms_by_seat.keys().cloned().collect::<Vec<_>>();
    let viewer_options = std::iter::once(None)
        .chain(seats.iter().map(|seat| Some(seat.as_str())))
        .collect::<Vec<_>>();
    let mut surfaces = Vec::new();
    for viewer in &viewer_options {
        surfaces.push(NoLeakSurface {
            viewer: viewer.map(ToOwned::to_owned),
            name: "payload",
            payload: get_view(match_id, *viewer).expect("viewer payload returned"),
        });
        surfaces.push(NoLeakSurface {
            viewer: viewer.map(ToOwned::to_owned),
            name: "effect_log",
            payload: get_effects(match_id, 0, *viewer).expect("viewer effects returned"),
        });
    }
    for actor in &seats {
        for viewer in &viewer_options {
            surfaces.push(NoLeakSurface {
                viewer: viewer.map(ToOwned::to_owned),
                name: "action_tree",
                payload: get_action_tree_for_viewer(match_id, actor, *viewer)
                    .expect("viewer action tree returned"),
            });
        }
    }
    surfaces.push(NoLeakSurface {
        viewer: None,
        name: "replay_export",
        payload: export_replay(match_id).expect("public replay exported"),
    });
    for name in [
        "preview",
        "bot_explanation",
        "candidate_ranking",
        "dom_test_id",
        "storage",
        "log",
    ] {
        surfaces.push(NoLeakSurface {
            viewer: None,
            name,
            payload: format!("bridge {name} not_applicable_or_redacted"),
        });
    }

    PairwiseNoLeakCase {
        seats,
        private_terms_by_seat,
        surfaces,
    }
}

fn collect_prefixed_terms(input: &str, prefix: &str) -> Vec<String> {
    let mut terms = Vec::new();
    for (index, _) in input.match_indices(prefix) {
        let token = input[index..]
            .chars()
            .take_while(|character| {
                character.is_ascii_alphanumeric() || matches!(character, ':' | '_' | '-' | '/')
            })
            .collect::<String>();
        if !token.is_empty() {
            terms.push(token);
        }
    }
    terms.sort();
    terms.dedup();
    terms
}

fn first_segment(tree: &str) -> String {
    tree.split("\"segment\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("segment present")
        .to_owned()
}

fn extract_replay_id(created: &str) -> String {
    created
        .split("\"replay_id\":\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("replay id is present")
        .to_owned()
}

fn match_count() -> usize {
    MATCHES.with(|matches| matches.borrow().len())
}

fn replay_count() -> usize {
    REPLAYS.with(|replays| replays.borrow().len())
}

fn last_output_string() -> String {
    LAST_OUTPUT.with(|last| last.borrow().clone())
}

fn json_string_array(values: &[String]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn get_terminal_poker_view(seed: u64, action_paths: &[&str]) -> String {
    let created = new_match("poker_lite", seed).expect("match created");
    let match_id = extract_match_id(&created);

    for (freshness_token, action_path) in action_paths.iter().enumerate() {
        let tree = get_action_tree(&match_id, "seat_0").expect("seat_0 tree returned");
        let actor = if tree.contains("\"choices\":[]") {
            "seat_1"
        } else {
            "seat_0"
        };
        apply_action(&match_id, actor, action_path, freshness_token as u64)
            .expect("poker action applies");
    }

    get_view(&match_id, None).expect("observer view returned")
}

fn get_terminal_river_view(seed: u64, seat_count: u8, actions: &[(&str, &str)]) -> String {
    let created = new_match_with_seat_count("river_ledger", seed, usize::from(seat_count))
        .expect("match created");
    let match_id = extract_match_id(&created);

    for (freshness_token, (actor, action_path)) in actions.iter().enumerate() {
        apply_action(&match_id, actor, action_path, freshness_token as u64)
            .expect("river action applies");
    }

    get_view(&match_id, None).expect("observer view returned")
}

const PLAIN_TRICKS_WIN_ACTIONS: [(&str, &str); 24] = [
    ("seat_0", "play>gale_1"),
    ("seat_1", "play>gale_2"),
    ("seat_1", "play>ember_3"),
    ("seat_0", "play>ember_6"),
    ("seat_0", "play>river_3"),
    ("seat_1", "play>river_6"),
    ("seat_1", "play>gale_3"),
    ("seat_0", "play>river_5"),
    ("seat_1", "play>ember_2"),
    ("seat_0", "play>ember_5"),
    ("seat_0", "play>river_1"),
    ("seat_1", "play>gale_6"),
    ("seat_1", "play>ember_4"),
    ("seat_0", "play>ember_2"),
    ("seat_1", "play>gale_1"),
    ("seat_0", "play>river_5"),
    ("seat_1", "play>gale_6"),
    ("seat_0", "play>river_2"),
    ("seat_1", "play>ember_6"),
    ("seat_0", "play>river_3"),
    ("seat_1", "play>gale_3"),
    ("seat_0", "play>river_1"),
    ("seat_1", "play>gale_5"),
    ("seat_0", "play>river_6"),
];

const PLAIN_TRICKS_SPLIT_ACTIONS: [(&str, &str); 24] = [
    ("seat_0", "play>river_6"),
    ("seat_1", "play>river_5"),
    ("seat_0", "play>river_1"),
    ("seat_1", "play>river_4"),
    ("seat_1", "play>gale_5"),
    ("seat_0", "play>gale_2"),
    ("seat_1", "play>ember_6"),
    ("seat_0", "play>ember_1"),
    ("seat_1", "play>gale_1"),
    ("seat_0", "play>gale_6"),
    ("seat_0", "play>gale_3"),
    ("seat_1", "play>ember_5"),
    ("seat_1", "play>ember_5"),
    ("seat_0", "play>ember_2"),
    ("seat_1", "play>gale_2"),
    ("seat_0", "play>gale_5"),
    ("seat_0", "play>river_1"),
    ("seat_1", "play>river_2"),
    ("seat_1", "play>river_4"),
    ("seat_0", "play>river_6"),
    ("seat_0", "play>ember_6"),
    ("seat_1", "play>gale_1"),
    ("seat_0", "play>gale_3"),
    ("seat_1", "play>gale_4"),
];

const PLAIN_TRICKS_WIN_PLAYED_CARDS: [plain_tricks::TrickCardId; 24] = [
    plain_tricks::TrickCardId::Gale1,
    plain_tricks::TrickCardId::Gale2,
    plain_tricks::TrickCardId::Ember3,
    plain_tricks::TrickCardId::Ember6,
    plain_tricks::TrickCardId::River3,
    plain_tricks::TrickCardId::River6,
    plain_tricks::TrickCardId::Gale3,
    plain_tricks::TrickCardId::River5,
    plain_tricks::TrickCardId::Ember2,
    plain_tricks::TrickCardId::Ember5,
    plain_tricks::TrickCardId::River1,
    plain_tricks::TrickCardId::Gale6,
    plain_tricks::TrickCardId::Ember4,
    plain_tricks::TrickCardId::Ember2,
    plain_tricks::TrickCardId::Gale1,
    plain_tricks::TrickCardId::River5,
    plain_tricks::TrickCardId::Gale6,
    plain_tricks::TrickCardId::River2,
    plain_tricks::TrickCardId::Ember6,
    plain_tricks::TrickCardId::River3,
    plain_tricks::TrickCardId::Gale3,
    plain_tricks::TrickCardId::River1,
    plain_tricks::TrickCardId::Gale5,
    plain_tricks::TrickCardId::River6,
];

fn get_terminal_plain_view(seed: u64, actions: &[(&str, &str)]) -> String {
    let created = new_match("plain_tricks", seed).expect("match created");
    let match_id = extract_match_id(&created);

    for (freshness_token, (actor, action_path)) in actions.iter().enumerate() {
        apply_action(&match_id, actor, action_path, freshness_token as u64)
            .expect("plain_tricks action applies");
    }

    get_view(&match_id, None).expect("observer view returned")
}

fn assert_ordered(input: &str, first: &str, second: &str) {
    let first_index = input.find(first).expect("first value present");
    let second_index = input.find(second).expect("second value present");
    assert!(
        first_index < second_index,
        "`{first}` must appear before `{second}` in {input}"
    );
}

fn card_ids_in(input: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for part in input.split("hcd:r").skip(1) {
        let suffix = part
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == ':')
            .collect::<String>();
        ids.push(format!("hcd:r{suffix}"));
    }
    ids.sort();
    ids.dedup();
    ids
}

fn assert_no_poker_cards(input: &str, cards: &[poker_lite::CrestCardId]) {
    for card in cards {
        assert!(
            !input.contains(card.as_str()),
            "hidden poker_lite card {} leaked in {input}",
            card.as_str()
        );
    }
}

fn assert_no_river_cards(input: &str, cards: &[river_ledger::Card]) {
    for card in cards {
        assert!(
            !input.contains(&card.id()),
            "hidden river_ledger card {} leaked in {input}",
            card.id()
        );
    }
}

fn first_mask_segment(input: &str) -> String {
    let start = input.find("mask_g").expect("mask id is present");
    let rest = &input[start..];
    let end = rest.find('"').expect("mask id terminates");
    rest[..end].to_owned()
}

fn plain_private_cards(view: &plain_tricks::PublicView) -> Vec<plain_tricks::TrickCardId> {
    match &view.private_view {
        plain_tricks::PrivateView::Seat(private) => private
            .own_hand
            .iter()
            .map(|card| plain_tricks::TrickCardId::parse(&card.card_id).expect("known card"))
            .collect(),
        plain_tricks::PrivateView::Observer => panic!("expected private seat view"),
    }
}

fn plain_cards_except(
    cards: &[plain_tricks::TrickCardId],
    exceptions: &[plain_tricks::TrickCardId],
) -> Vec<plain_tricks::TrickCardId> {
    cards
        .iter()
        .copied()
        .filter(|card| !exceptions.contains(card))
        .collect()
}

fn plain_hidden_cards_except(
    exceptions: &[plain_tricks::TrickCardId],
) -> Vec<plain_tricks::TrickCardId> {
    plain_tricks::TrickCardId::ALL
        .iter()
        .copied()
        .filter(|card| !exceptions.contains(card))
        .collect()
}

fn assert_no_plain_cards(input: &str, cards: &[plain_tricks::TrickCardId]) {
    for card in cards {
        assert!(
            !input.contains(card.as_str()),
            "hidden plain_tricks card {} leaked in {input}",
            card.as_str()
        );
    }
}

fn compact_json_layout(input: &str) -> String {
    let mut output = String::new();
    let mut in_string = false;
    let mut escaped = false;
    for ch in input.chars() {
        if in_string {
            output.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
        } else if ch == '"' {
            in_string = true;
            output.push(ch);
        } else if !ch.is_whitespace() {
            output.push(ch);
        }
    }
    output
}

fn pretty_json_layout(input: &str) -> String {
    let mut output = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0_usize;
    for ch in input.chars() {
        if in_string {
            output.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => {
                in_string = true;
                output.push(ch);
            }
            '{' | '[' => {
                output.push(ch);
                depth += 1;
                output.push('\n');
                output.push_str(&"  ".repeat(depth));
            }
            '}' | ']' => {
                depth = depth.saturating_sub(1);
                output.push('\n');
                output.push_str(&"  ".repeat(depth));
                output.push(ch);
            }
            ':' => output.push_str(": "),
            ',' => {
                output.push(ch);
                output.push('\n');
                output.push_str(&"  ".repeat(depth));
            }
            _ => output.push(ch),
        }
    }
    output
}

fn plain_replay_document_json(
    trace_id: &str,
    seed: u64,
    commands: &[AppliedCommand],
) -> Result<String, String> {
    let mut state = plain_setup_match(
        Seed(seed),
        &plain_seats(),
        &plain_tricks::SetupOptions::default(),
    )
    .map_err(diagnostic_json)?;
    let mut effects = plain_tricks::setup_effects(&state);
    let mut replay_commands = Vec::new();

    for command in commands {
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
        effects.extend(plain_apply_action(&mut state, action).map_err(diagnostic_json)?);
        replay_commands.push(PlainReplayCommand {
            actor: trace_plain_seat(seat).to_owned(),
            path: command.action_path.clone(),
        });
    }

    let trace = PlainTricksInternalTrace {
        schema_version: SCHEMA_VERSION,
        game_id: plain_tricks::GAME_ID.to_owned(),
        rules_version: plain_tricks::RULES_VERSION_LABEL.to_owned(),
        variant: plain_tricks::VARIANT_ID.to_owned(),
        seed_evidence: seed,
        commands: replay_commands,
    };
    let public_export = plain_export_public_replay(&trace, &Viewer { seat_id: None });
    let commands_json = commands
        .iter()
        .enumerate()
        .map(|(index, command)| command_record_json(index, command))
        .collect::<Vec<_>>()
        .join(",");
    let checkpoints = if commands.is_empty() {
        "[{\"id\":\"final\",\"after_command_index\":0}]".to_owned()
    } else {
        format!(
            "[{{\"id\":\"final\",\"after_command_index\":{}}}]",
            commands.len().saturating_sub(1)
        )
    };
    let (terminal, winner, draw) = match state.terminal_outcome {
        Some(plain_tricks::TerminalOutcome::TrickWin { winner, .. }) => {
            (true, format!("\"{}\"", winner.as_str()), false)
        }
        Some(plain_tricks::TerminalOutcome::Split { .. }) => (true, "null".to_owned(), true),
        None => (false, "null".to_owned(), false),
    };

    Ok(format!(
        "{{\"schema_version\":{},\"trace_id\":\"plain-tricks-{}\",\"fixture_kind\":\"commands\",\"purpose\":\"wasm_exported\",\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"seed\":{},\"variant\":\"{}\",\"commands\":[{}],\"expected_state_hashes\":{{\"final\":{}}},\"expected_effect_hashes\":{{\"final\":{}}},\"expected_action_tree_hashes\":{{\"final\":{}}},\"expected_public_view_hashes\":{{\"observer\":{},\"seat_0\":{},\"seat_1\":{}}},\"expected_private_view_hashes\":{{\"seat_0\":{},\"seat_1\":{}}},\"expected_replay_hashes\":{{\"final\":{}}},\"expected_diagnostic_hashes\":null,\"expected_public_export_hashes\":{{\"final\":{}}},\"expected_diagnostics\":null,\"expected_terminal_state\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"note\":\"Plain Tricks wasm-exported fixture generated by the Rulepath WASM API.\",\"migration_update_note\":\"Refreshed with real WASM export evidence by GAT101PLATRI-016.\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat_0\",\"player_id\":\"player_0\"}},{{\"seat_id\":\"seat_1\",\"player_id\":\"player_1\"}}],\"checkpoints\":{},\"expected_outcome\":{{\"terminal\":{},\"winner\":{},\"draw\":{}}},\"not_applicable\":{{\"hidden_information\":\"Plain Tricks has hidden private hands and an internal tail; viewer-scoped traces and no-leak tests verify redaction.\",\"stochastic_game_events\":\"Setup uses deterministic seeded shuffle; no later stochastic rule events occur.\",\"preview_hashes\":\"Plain Tricks uses action metadata rather than a separate preview hash.\"}}}}",
        SCHEMA_VERSION,
        escape_json(trace_id),
        escape_json(GAME_PLAIN_TRICKS),
        escape_json(PLAIN_TRICKS_TRACE_RULES_VERSION),
        escape_json(ENGINE_VERSION),
        escape_json(DATA_VERSION),
        seed,
        escape_json(VARIANT_PLAIN_TRICKS_STANDARD),
        commands_json,
        plain_tricks::replay_support::state_hash(&state).0,
        plain_tricks::replay_support::effect_hash(&effects).0,
        plain_action_tree_hash(&state).0,
        plain_tricks::replay_support::view_hash(&state, &Viewer { seat_id: None }).0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_0".to_owned()))
            }
        )
        .0,
        plain_tricks::replay_support::view_hash(
            &state,
            &Viewer {
                seat_id: Some(SeatId("seat_1".to_owned()))
            }
        )
        .0,
        trace.stable_hash().0,
        public_export.stable_hash().0,
        terminal,
        winner,
        draw,
        checkpoints,
        terminal,
        winner,
        draw
    ))
}

fn plain_action_tree_hash(state: &PlainTricksState) -> HashValue {
    let parts = PlainTricksSeat::ALL
        .iter()
        .map(|seat| {
            let actor = Actor {
                seat_id: state.seats[seat.index()].clone(),
            };
            plain_tricks::replay_support::action_tree_hash(&plain_legal_action_tree(state, &actor))
                .0
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("|");
    HashValue::from_stable_bytes(parts.as_bytes())
}
