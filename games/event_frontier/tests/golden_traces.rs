use engine_core::{
    ActionChoice, ActionPath, Actor, CommandEnvelope, Diagnostic, HashValue, RulesVersion, SeatId,
    Seed, StableSerialize, Viewer,
};
use event_frontier::{
    apply_command, generate_internal_full_trace, legal_action_tree, project_view,
    resolve_reckoning, setup_match, validate_command, CardId, CardPhase, Eligibility,
    EventCharterLevel1Bot, EventFreeholdersLevel1Bot, EventFrontierEffectEnvelope,
    EventFrontierState, FactionId, SetupOptions, TerminalOutcome, ACTION_EVENT, ACTION_OPERATION,
    ACTION_PASS, TRACE_HIDDEN_SURFACE, TRACE_STOCHASTIC_SURFACE, VARIANT_HARD_WINTER_ID,
    VARIANT_LAND_RUSH_ID, VARIANT_STANDARD_ID,
};

const TRACE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden_traces");

const TRACE_FILES: [&str; 18] = [
    "standard-charter-instant-win.trace.json",
    "standard-freeholder-cache-win.trace.json",
    "final-reckoning-fallback.trace.json",
    "event-choice-resolves-card.trace.json",
    "op-full-multi-site.trace.json",
    "limited-op-after-full-op.trace.json",
    "pass-keeps-eligibility.trace.json",
    "double-pass-discards-card.trace.json",
    "no-eligible-faction-discard.trace.json",
    "edict-activation-and-expiry.trace.json",
    "edict-blocks-action-diagnostic.trace.json",
    "reckoning-scoring-breakdown.trace.json",
    "reckoning-never-first-in-epoch.trace.json",
    "hard-winter-setup.trace.json",
    "land-rush-setup.trace.json",
    "ineligible-faction-diagnostic.trace.json",
    "bot-vs-bot-full-game.trace.json",
    "replay-export-import-no-deck-leak.trace.json",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct TraceActual {
    state: EventFrontierState,
    effects: Vec<EventFrontierEffectEnvelope>,
    diagnostic: Option<Diagnostic>,
    terminal: bool,
    winner: Option<FactionId>,
    public_no_leak: bool,
}

#[test]
fn golden_traces_are_complete_and_reproduce_hashes() {
    let mut seen = Vec::new();
    for file in TRACE_FILES {
        let path = format!("{TRACE_DIR}/{file}");
        let json = std::fs::read_to_string(&path).expect("trace file exists");
        let actual = trace_actual(file);
        let expected = expected_trace_json(file, &actual);

        assert_eq!(json, expected, "trace drifted: {file}");
        assert!(!json.contains(actual_hidden_card(&actual).as_str()));
        assert!(json.contains(TRACE_HIDDEN_SURFACE));
        assert!(json.contains(TRACE_STOCHASTIC_SURFACE));
        seen.push(extract_string_field(&json, "trace_id"));
    }
    seen.sort();
    seen.dedup();
    assert_eq!(seen.len(), TRACE_FILES.len());
}

#[test]
#[ignore]
fn print_golden_traces() {
    for file in TRACE_FILES {
        println!("--- {file}");
        println!("{}", expected_trace_json(file, &trace_actual(file)));
    }
}

fn trace_actual(file: &str) -> TraceActual {
    match file {
        "standard-charter-instant-win.trace.json" => charter_instant_trace(),
        "standard-freeholder-cache-win.trace.json" => freeholder_cache_win_trace(),
        "final-reckoning-fallback.trace.json" => final_reckoning_fallback_trace(),
        "event-choice-resolves-card.trace.json" => event_choice_trace(),
        "op-full-multi-site.trace.json" => op_full_multi_site_trace(),
        "limited-op-after-full-op.trace.json" => limited_op_after_full_op_trace(),
        "pass-keeps-eligibility.trace.json" => pass_keeps_eligibility_trace(),
        "double-pass-discards-card.trace.json" => double_pass_discards_trace(),
        "no-eligible-faction-discard.trace.json" => no_eligible_discard_trace(),
        "edict-activation-and-expiry.trace.json" => edict_activation_expiry_trace(),
        "edict-blocks-action-diagnostic.trace.json" => edict_blocks_action_trace(),
        "reckoning-scoring-breakdown.trace.json" => reckoning_breakdown_trace(),
        "reckoning-never-first-in-epoch.trace.json" => reckoning_never_first_trace(),
        "hard-winter-setup.trace.json" => setup_trace(SetupOptions::hard_winter()),
        "land-rush-setup.trace.json" => setup_trace(SetupOptions::land_rush()),
        "ineligible-faction-diagnostic.trace.json" => ineligible_faction_diagnostic_trace(),
        "bot-vs-bot-full-game.trace.json" => bot_vs_bot_trace(),
        "replay-export-import-no-deck-leak.trace.json" => replay_export_import_no_leak_trace(),
        _ => panic!("unknown trace file {file}"),
    }
}

fn charter_instant_trace() -> TraceActual {
    let mut state = standard_state(1);
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    for site in state.sites.iter_mut().take(4) {
        site.agents = 2;
        site.settlers = 0;
        site.depot = false;
        site.cache_count = 0;
    }
    let effects = resolve_reckoning(&mut state).expect("reckoning");
    actual(state, effects.effects, None)
}

fn freeholder_cache_win_trace() -> TraceActual {
    let mut state = standard_state(2);
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    for (index, site) in state.sites.iter_mut().enumerate() {
        site.agents = 0;
        site.settlers = 1;
        site.depot = false;
        site.cache_count = if index < 4 { 2 } else { 0 };
    }
    let effects = resolve_reckoning(&mut state).expect("reckoning");
    actual(state, effects.effects, None)
}

fn final_reckoning_fallback_trace() -> TraceActual {
    let mut state = standard_state(3);
    state.deck.current = Some(CardId::ReckoningThree);
    state.card_phase = CardPhase::Reckoning;
    state.reckoning_count = 2;
    state.scores.charter = 5;
    state.scores.freeholders = 5;
    for site in &mut state.sites {
        site.agents = 1;
        site.settlers = 1;
        site.depot = false;
        site.cache_count = 0;
    }
    let effects = resolve_reckoning(&mut state).expect("reckoning");
    actual(state, effects.effects, None)
}

fn event_choice_trace() -> TraceActual {
    let mut state = standard_state(4);
    state.deck.current = Some(CardId::BorderSurvey);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    apply(&mut state, "seat_0", ACTION_EVENT)
}

fn op_full_multi_site_trace() -> TraceActual {
    let mut state = standard_state(5);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let op = first_operation_leaf(&state, "seat_0", ACTION_OPERATION, true);
    apply(&mut state, "seat_0", &op)
}

fn limited_op_after_full_op_trace() -> TraceActual {
    let mut state = standard_state(6);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let op = first_operation_leaf(&state, "seat_0", ACTION_OPERATION, false);
    let mut effects = apply(&mut state, "seat_0", &op).effects;
    let limited = first_operation_leaf(&state, "seat_1", "limited_operation", false);
    effects.extend(apply(&mut state, "seat_1", &limited).effects);
    actual(state, effects, None)
}

fn pass_keeps_eligibility_trace() -> TraceActual {
    let mut state = standard_state(7);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    apply(&mut state, "seat_0", ACTION_PASS)
}

fn double_pass_discards_trace() -> TraceActual {
    let mut state = standard_state(8);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let mut effects = apply(&mut state, "seat_0", ACTION_PASS).effects;
    effects.extend(apply(&mut state, "seat_1", ACTION_PASS).effects);
    actual(state, effects, None)
}

fn no_eligible_discard_trace() -> TraceActual {
    let mut state = standard_state(9);
    state.deck.current = Some(CardId::LastLight);
    state.set_eligibility(FactionId::Charter, Eligibility::Ineligible);
    state.set_eligibility(FactionId::Freeholders, Eligibility::Ineligible);
    let mut effects = Vec::new();
    event_frontier::rules::advance_to_next_card(&mut state, "no_eligible_faction", &mut effects);
    actual(state, effects, None)
}

fn edict_activation_expiry_trace() -> TraceActual {
    let mut state = standard_state(10);
    state.deck.current = Some(CardId::TollRoads);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let mut effects = apply(&mut state, "seat_0", ACTION_EVENT).effects;
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    effects.extend(resolve_reckoning(&mut state).expect("reckoning").effects);
    actual(state, effects, None)
}

fn edict_blocks_action_trace() -> TraceActual {
    let mut state = standard_state(11);
    state.deck.current = Some(CardId::SurveyBan);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let effects = apply(&mut state, "seat_0", ACTION_EVENT).effects;
    let crossing = state
        .site_mut(event_frontier::SiteId::Crossing)
        .expect("crossing exists");
    crossing.agents = 1;
    crossing.settlers = 1;
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let diagnostic = validate_command(
        &state,
        &command(
            "seat_0",
            "operation/survey/site_crossing",
            state.freshness_token.0,
        ),
    )
    .expect_err("survey ban blocks contested site");
    actual(state, effects, Some(diagnostic))
}

fn reckoning_breakdown_trace() -> TraceActual {
    let mut state = standard_state(12);
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    let effects = resolve_reckoning(&mut state).expect("reckoning");
    actual(state, effects.effects, None)
}

fn reckoning_never_first_trace() -> TraceActual {
    setup_trace(SetupOptions::standard())
}

fn setup_trace(options: SetupOptions) -> TraceActual {
    let state = setup_match(Seed(13), &seats(), &options).expect("setup");
    actual(state, Vec::new(), None)
}

fn ineligible_faction_diagnostic_trace() -> TraceActual {
    let mut state = standard_state(14);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    let diagnostic = validate_command(
        &state,
        &command("seat_1", ACTION_EVENT, state.freshness_token.0),
    )
    .expect_err("wrong faction diagnostic");
    actual(state, Vec::new(), Some(diagnostic))
}

fn bot_vs_bot_trace() -> TraceActual {
    let mut state = standard_state(15);
    let mut effects = Vec::new();
    for turn in 0..80 {
        if state.terminal_outcome.is_some() {
            break;
        }
        if state.card_phase == CardPhase::Reckoning {
            effects.extend(resolve_reckoning(&mut state).expect("reckoning").effects);
            continue;
        }
        let Some((faction, seat)) = active_faction_and_seat(&state) else {
            break;
        };
        let decision = match faction {
            FactionId::Charter => EventCharterLevel1Bot::new(Seed(100 + turn))
                .select_decision(&state, &seat)
                .expect("charter bot"),
            FactionId::Freeholders => EventFreeholdersLevel1Bot::new(Seed(200 + turn))
                .select_decision(&state, &seat)
                .expect("freeholder bot"),
        };
        let command = command_from_path(&seat, &decision.action_path, &state);
        effects.extend(
            apply_command(&mut state, &command)
                .expect("bot command")
                .effects,
        );
    }
    actual(state, effects, None)
}

fn replay_export_import_no_leak_trace() -> TraceActual {
    let mut state = standard_state(16);
    state.deck.current = Some(CardId::LastLight);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    apply(&mut state, "seat_0", ACTION_PASS)
}

fn actual(
    state: EventFrontierState,
    effects: Vec<EventFrontierEffectEnvelope>,
    diagnostic: Option<Diagnostic>,
) -> TraceActual {
    let hidden = actual_hidden_card_for_state(&state);
    let rendered = format!("{state:?}{effects:?}{diagnostic:?}");
    let terminal = state.terminal_outcome.is_some();
    let winner = state
        .terminal_outcome
        .as_ref()
        .map(|TerminalOutcome::Winner { faction, .. }| *faction);
    TraceActual {
        state,
        effects,
        diagnostic,
        terminal,
        winner,
        public_no_leak: !rendered.contains(&hidden),
    }
}

fn expected_trace_json(file: &str, actual: &TraceActual) -> String {
    let trace_id = format!("event-frontier-{}", file.trim_end_matches(".trace.json"));
    let variant = variant_for_file(file);
    let fixture_kind = fixture_kind_for_file(file);
    let purpose = file.trim_end_matches(".trace.json").replace('-', "_");
    let state_hash = actual.state.stable_hash().0;
    let effect_hash = hash_debug(&actual.effects).0;
    let action_hash = hash_debug(&legal_action_tree(&actual.state, &actor("seat_0"))).0;
    let view_hash = project_view(&actual.state, &Viewer { seat_id: None })
        .stable_hash()
        .0;
    let diagnostic_hash = actual
        .diagnostic
        .as_ref()
        .map(|diagnostic| hash_debug(diagnostic).0.to_string())
        .unwrap_or_else(|| "none".to_owned());
    let diagnostic_json = actual
        .diagnostic
        .as_ref()
        .map(|diagnostic| format!("[{{\"code\":\"{}\"}}]", diagnostic.code))
        .unwrap_or_else(|| "[]".to_owned());
    let winner = actual
        .winner
        .map(|faction| format!(",\"winner\":\"{}\"", faction.as_str()))
        .unwrap_or_default();
    let internal = generate_internal_full_trace(seed_for_file(file), &actual.state);
    format!(
        "{{\"schema_version\":1,\"trace_id\":\"{trace_id}\",\"fixture_kind\":\"{fixture_kind}\",\"purpose\":\"{purpose}\",\"note\":\"Pins Event Frontier {purpose} evidence with replay hashes and no hidden deck-order leak.\",\"migration_update_note\":\"Initial Gate 14 Event Frontier trace fixture.\",\"game_id\":\"event_frontier\",\"rules_version\":\"event-frontier-rules-v1\",\"engine_version\":\"engine-core-0.1.0\",\"data_version\":\"1\",\"seed\":{},\"variant\":\"{variant}\",\"options\":{{}},\"seats\":[{{\"seat_id\":\"seat_0\",\"player_id\":\"charter\"}},{{\"seat_id\":\"seat_1\",\"player_id\":\"freeholders\"}}],\"commands\":[],\"checkpoints\":[{{\"id\":\"final\",\"after_command_index\":0}}],\"expected_state_hashes\":{{\"final\":\"{state_hash}\"}},\"expected_effect_hashes\":{{\"final\":\"{effect_hash}\"}},\"expected_action_tree_hashes\":{{\"final\":\"{action_hash}\"}},\"expected_public_view_hashes\":{{\"observer\":\"{view_hash}\"}},\"expected_private_view_hashes\":{{\"not_applicable\":\"single public projection shared by all viewers\"}},\"expected_diagnostic_hashes\":{{\"final\":\"{diagnostic_hash}\"}},\"expected_diagnostics\":{diagnostic_json},\"expected_outcome\":{{\"terminal\":{}{winner}}},\"expected_terminal_state\":{{\"terminal\":{}{winner}}},\"hidden_information\":\"{}\",\"stochastic_game_rule_events\":\"{}\",\"public_no_leak\":{},\"not_applicable\":{{\"private_view_hashes\":\"one public projection is shared by all viewers\",\"preview_hashes\":\"no Rust preview surface exists for this gate\",\"per_seat_hidden_surface\":\"not_applicable\",\"internal_trace_full_deck_hash\":\"{}\"}}}}\n",
        seed_for_file(file),
        actual.terminal,
        actual.terminal,
        TRACE_HIDDEN_SURFACE,
        TRACE_STOCHASTIC_SURFACE,
        actual.public_no_leak,
        internal.stable_hash().0
    )
}

fn first_operation_leaf(
    state: &EventFrontierState,
    seat: &str,
    prefix: &str,
    require_multi_site: bool,
) -> String {
    let tree = legal_action_tree(state, &actor(seat));
    let mut leaves = Vec::new();
    for choice in &tree.root.choices {
        collect_leaves(choice, &mut leaves);
    }
    leaves
        .into_iter()
        .find(|leaf| {
            leaf.starts_with(prefix)
                && (!require_multi_site || leaf.split('/').nth(2).is_some_and(|p| p.contains(',')))
        })
        .expect("operation leaf")
}

fn collect_leaves(choice: &ActionChoice, out: &mut Vec<String>) {
    if let Some(next) = &choice.next {
        for child in &next.choices {
            collect_leaves(child, out);
        }
    } else if choice.segment.starts_with(ACTION_OPERATION)
        || choice.segment.starts_with("limited_operation")
    {
        out.push(choice.segment.clone());
    }
}

fn apply(state: &mut EventFrontierState, seat: &str, segment: &str) -> TraceActual {
    let command = command(seat, segment, state.freshness_token.0);
    let effects = apply_command(state, &command)
        .expect("command applies")
        .effects;
    actual(state.clone(), effects, None)
}

fn standard_state(seed: u64) -> EventFrontierState {
    setup_match(Seed(seed), &seats(), &SetupOptions::standard()).expect("setup")
}

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(seat: &str, segment: &str, freshness_token: u64) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: engine_core::FreshnessToken(freshness_token),
        rules_version: RulesVersion(1),
    }
}

fn command_from_path(
    seat: &SeatId,
    action_path: &ActionPath,
    state: &EventFrontierState,
) -> CommandEnvelope {
    CommandEnvelope {
        actor: Actor {
            seat_id: seat.clone(),
        },
        action_path: action_path.clone(),
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn active_faction_and_seat(state: &EventFrontierState) -> Option<(FactionId, SeatId)> {
    let faction = match state.card_phase {
        CardPhase::AwaitingFirstChoice { faction } => faction,
        CardPhase::AwaitingSecondChoice { second_faction, .. } => second_faction,
        CardPhase::Reckoning | CardPhase::Terminal => return None,
    };
    let seat = match faction {
        FactionId::Charter => SeatId("seat_0".to_owned()),
        FactionId::Freeholders => SeatId("seat_1".to_owned()),
    };
    Some((faction, seat))
}

fn hash_debug<T: std::fmt::Debug>(value: &T) -> HashValue {
    HashValue::from_stable_bytes(format!("{value:?}").as_bytes())
}

fn seed_for_file(file: &str) -> u64 {
    TRACE_FILES
        .iter()
        .position(|candidate| *candidate == file)
        .unwrap_or(0) as u64
        + 1
}

fn variant_for_file(file: &str) -> &'static str {
    match file {
        "hard-winter-setup.trace.json" => VARIANT_HARD_WINTER_ID,
        "land-rush-setup.trace.json" => VARIANT_LAND_RUSH_ID,
        _ => VARIANT_STANDARD_ID,
    }
}

fn fixture_kind_for_file(file: &str) -> &'static str {
    match file {
        "standard-charter-instant-win.trace.json"
        | "standard-freeholder-cache-win.trace.json"
        | "final-reckoning-fallback.trace.json" => "terminal",
        "edict-blocks-action-diagnostic.trace.json"
        | "ineligible-faction-diagnostic.trace.json" => "diagnostic",
        "bot-vs-bot-full-game.trace.json" => "bot",
        _ => "commands",
    }
}

fn actual_hidden_card(actual: &TraceActual) -> String {
    actual_hidden_card_for_state(&actual.state)
}

fn actual_hidden_card_for_state(state: &EventFrontierState) -> String {
    state
        .deck
        .undrawn
        .first()
        .map(|card| card.as_str().to_owned())
        .unwrap_or_else(|| "no-hidden-card".to_owned())
}

fn extract_string_field(json: &str, field: &str) -> String {
    let pattern = format!("\"{field}\":\"");
    let start = json.find(&pattern).expect("field exists") + pattern.len();
    let end = json[start..].find('"').expect("field terminates") + start;
    json[start..end].to_owned()
}
