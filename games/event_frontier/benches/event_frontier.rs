use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionChoice, ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed,
    StableSerialize, Viewer,
};
use event_frontier::{
    apply_command, command_for_decision, generate_internal_full_trace, legal_action_tree,
    project_view, resolve_reckoning, setup_match, CardId, CardPhase, EventCharterLevel1Bot,
    EventFreeholdersLevel1Bot, EventFrontierState, FactionId, SetupOptions, ACTION_EVENT,
    ACTION_OPERATION, GAME_ID, RULES_VERSION_LABEL,
};

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const REPORT_SCHEMA_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

const OPERATIONS: &[(&str, &str)] = &[
    ("setup_standard", "setups_per_second"),
    ("shuffle_and_deal_epochs", "traces_per_second"),
    ("legal_tree_first_choice", "trees_per_second"),
    ("legal_tree_peak_op_branching", "trees_per_second"),
    ("apply_event", "actions_per_second"),
    ("apply_op_multi_site", "actions_per_second"),
    ("edict_modifier_projection", "views_per_second"),
    ("reckoning_pipeline", "reckonings_per_second"),
    ("serialize_view", "serializations_per_second"),
    ("bot_l1_choice_charter", "decisions_per_second"),
    ("bot_l1_choice_freeholders", "decisions_per_second"),
    ("full_random_playout", "turns_per_second"),
];

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());

    println!("event_frontier native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass");
    for result in &results {
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},1.00,{}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            current,
            current >= 1.0
        );
    }
    println!("BEGIN_EVENT_FRONTIER_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&results));
    println!("END_EVENT_FRONTIER_BENCHMARK_JSON");
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && !arg.chars().all(|ch| ch.is_ascii_digit())
            && OPERATIONS
                .iter()
                .any(|(operation, _)| operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    let benches: Vec<BenchSpec> = vec![
        ("setup_standard", "setups", 20_000, bench_setup_standard),
        (
            "shuffle_and_deal_epochs",
            "traces",
            20_000,
            bench_shuffle_and_deal_epochs,
        ),
        (
            "legal_tree_first_choice",
            "trees",
            20_000,
            bench_legal_tree_first_choice,
        ),
        (
            "legal_tree_peak_op_branching",
            "trees",
            10_000,
            bench_legal_tree_peak_op_branching,
        ),
        ("apply_event", "actions", 20_000, bench_apply_event),
        (
            "apply_op_multi_site",
            "actions",
            10_000,
            bench_apply_op_multi_site,
        ),
        (
            "edict_modifier_projection",
            "views",
            20_000,
            bench_edict_modifier_projection,
        ),
        (
            "reckoning_pipeline",
            "reckonings",
            20_000,
            bench_reckoning_pipeline,
        ),
        (
            "serialize_view",
            "serializations",
            20_000,
            bench_serialize_view,
        ),
        (
            "bot_l1_choice_charter",
            "decisions",
            20_000,
            bench_bot_l1_choice_charter,
        ),
        (
            "bot_l1_choice_freeholders",
            "decisions",
            20_000,
            bench_bot_l1_choice_freeholders,
        ),
        (
            "full_random_playout",
            "turns",
            100,
            bench_full_random_playout,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn measure(
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    benchmark: fn(u64),
) -> BenchResult {
    let started = Instant::now();
    benchmark(iterations);
    BenchResult {
        name,
        unit,
        iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_setup_standard(iterations: u64) {
    for seed in 0..iterations {
        black_box(setup_match(
            black_box(Seed(seed)),
            black_box(&seats()),
            black_box(&SetupOptions::standard()),
        ))
        .expect("setup");
    }
}

fn bench_shuffle_and_deal_epochs(iterations: u64) {
    for seed in 0..iterations {
        let state = setup_match(Seed(seed), &seats(), &SetupOptions::standard()).expect("setup");
        black_box(generate_internal_full_trace(seed, black_box(&state)));
    }
}

fn bench_legal_tree_first_choice(iterations: u64) {
    let state = setup();
    let actor = actor("seat_0");
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_tree_peak_op_branching(iterations: u64) {
    let state = peak_op_state();
    let actor = actor("seat_0");
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_apply_event(iterations: u64) {
    let base = charter_event_state();
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(&state, "seat_0", ACTION_EVENT);
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_apply_op_multi_site(iterations: u64) {
    let base = peak_op_state();
    let op = first_operation_leaf(&base, "seat_0", true);
    for _ in 0..iterations {
        let mut state = base.clone();
        let command = command(&state, "seat_0", &op);
        black_box(apply_command(black_box(&mut state), black_box(&command)).unwrap());
    }
}

fn bench_edict_modifier_projection(iterations: u64) {
    let state = edict_state();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_reckoning_pipeline(iterations: u64) {
    let base = reckoning_state();
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(resolve_reckoning(black_box(&mut state)).unwrap());
    }
}

fn bench_serialize_view(iterations: u64) {
    let state = edict_state();
    let view = project_view(&state, &Viewer { seat_id: None });
    for _ in 0..iterations {
        black_box(HashValue::from_stable_bytes(black_box(
            &view.stable_bytes(),
        )));
    }
}

fn bench_bot_l1_choice_charter(iterations: u64) {
    let state = peak_op_state();
    let bot = EventCharterLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(&state.seats[0]))
                .unwrap(),
        );
    }
}

fn bench_bot_l1_choice_freeholders(iterations: u64) {
    let state = freeholder_choice_state();
    let bot = EventFreeholdersLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(&state.seats[1]))
                .unwrap(),
        );
    }
}

fn bench_full_random_playout(iterations: u64) {
    let mut total_turns = 0u64;
    for seed in 0..iterations {
        let mut state =
            setup_match(Seed(seed), &seats(), &SetupOptions::standard()).expect("setup");
        let mut turns = 0u64;
        while state.terminal_outcome.is_none() && turns < 100 {
            if state.card_phase == CardPhase::Reckoning {
                black_box(resolve_reckoning(&mut state).expect("reckoning"));
                turns += 1;
                continue;
            }
            let Some((faction, seat)) = active_faction_and_seat(&state) else {
                break;
            };
            let decision = match faction {
                FactionId::Charter => EventCharterLevel1Bot::new(Seed(seed + turns))
                    .select_decision(&state, &seat)
                    .expect("charter decision"),
                FactionId::Freeholders => EventFreeholdersLevel1Bot::new(Seed(seed + turns))
                    .select_decision(&state, &seat)
                    .expect("freeholder decision"),
            };
            let command = command_for_decision(&state, &seat, &decision);
            black_box(apply_command(&mut state, &command).expect("bot action"));
            turns += 1;
        }
        total_turns = total_turns.saturating_add(turns);
        black_box(state.terminal_outcome.expect("playout reaches terminal"));
    }
    black_box(total_turns);
}

fn setup() -> EventFrontierState {
    setup_match(Seed(1), &seats(), &SetupOptions::standard()).expect("setup")
}

fn charter_event_state() -> EventFrontierState {
    let mut state = setup();
    state.deck.current = Some(CardId::BorderSurvey);
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    state
}

fn peak_op_state() -> EventFrontierState {
    let mut state = setup();
    state.deck.current = Some(CardId::LastLight);
    state.resources.funds = 3;
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Charter,
    };
    state
}

fn freeholder_choice_state() -> EventFrontierState {
    let mut state = setup();
    state.deck.current = Some(CardId::LastLight);
    state.resources.provisions = 3;
    state.card_phase = CardPhase::AwaitingFirstChoice {
        faction: FactionId::Freeholders,
    };
    state
}

fn edict_state() -> EventFrontierState {
    let mut state = charter_event_state();
    state.deck.current = Some(CardId::TollRoads);
    let command = command(&state, "seat_0", ACTION_EVENT);
    apply_command(&mut state, &command).expect("edict event");
    state
}

fn reckoning_state() -> EventFrontierState {
    let mut state = setup();
    state.deck.current = Some(CardId::ReckoningOne);
    state.card_phase = CardPhase::Reckoning;
    state
}

fn first_operation_leaf(
    state: &EventFrontierState,
    seat: &str,
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
            leaf.starts_with(ACTION_OPERATION)
                && (!require_multi_site || leaf.split('/').nth(2).is_some_and(|p| p.contains(',')))
        })
        .expect("operation leaf")
}

fn collect_leaves(choice: &ActionChoice, out: &mut Vec<String>) {
    if let Some(next) = &choice.next {
        for child in &next.choices {
            collect_leaves(child, out);
        }
    } else if choice.segment.starts_with(ACTION_OPERATION) {
        out.push(choice.segment.clone());
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

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn command(state: &EventFrontierState, seat: &str, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn benchmark_report_json(results: &[BenchResult]) -> String {
    format!(
        "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"data_version\":\"{}\",\"engine_version\":\"{}\",\"build_profile\":\"bench\",\"command\":\"cargo bench -p event_frontier\",\"os\":\"unknown\",\"rust_version\":\"unknown\",\"hardware_environment_notes\":\"uncontrolled smoke runner\",\"operations\":[{}]}}",
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        results
            .iter()
            .map(bench_result_json)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn bench_result_json(result: &BenchResult) -> String {
    format!(
        "{{\"operation_name\":\"{}\",\"unit\":\"{}_per_second\",\"iterations\":{},\"elapsed_ms\":{:.3},\"current_value\":{:.6}}}",
        result.name,
        result.unit,
        result.iterations,
        result.elapsed.as_secs_f64() * 1_000.0,
        result.current_value()
    )
}
