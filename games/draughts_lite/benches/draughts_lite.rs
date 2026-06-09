use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use draughts_lite::{
    apply_action, command_for_state, legal_action_tree, project_view, replay_commands, setup_match,
    validate_command, CellOccupancy, DraughtsLiteLevel1Bot, DraughtsLiteRandomBot,
    DraughtsLiteSeat, DraughtsLiteState, Piece, PieceId, PieceKind, SetupOptions, Variant, GAME_ID,
    RULES_VERSION_LABEL,
};
use engine_core::{Actor, CommandEnvelope, FreshnessToken, SeatId, Seed, Viewer};
use game_stdlib::board_space::Coord;

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const BUILD_PROFILE: &str = "bench";
const REPORT_SCHEMA_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

#[derive(Clone, Copy)]
struct Threshold {
    operation: &'static str,
    unit: &'static str,
    threshold: f64,
    rationale_class: &'static str,
    caveat: &'static str,
}

const THRESHOLDS: &[Threshold] = &[
    threshold("standard_setup", "setups_per_second"),
    threshold("legal_actions_initial", "trees_per_second"),
    threshold("legal_actions_midgame_no_capture", "trees_per_second"),
    threshold("legal_actions_mandatory_capture", "trees_per_second"),
    threshold("legal_actions_multi_jump", "trees_per_second"),
    threshold("validate_apply_quiet", "actions_per_second"),
    threshold("validate_apply_single_capture", "actions_per_second"),
    threshold("validate_apply_multi_jump", "actions_per_second"),
    threshold("public_view_generation", "views_per_second"),
    threshold("replay_throughput", "replays_per_second"),
    threshold("level0_bot_decision", "decisions_per_second"),
    threshold("level1_bot_decision", "decisions_per_second"),
];

const fn threshold(operation: &'static str, unit: &'static str) -> Threshold {
    Threshold {
        operation,
        unit,
        threshold: 1.0,
        rationale_class: "baseline_pending_non_blocking",
        caveat: "Smoke floor only until draughts_lite has stable CI measurements.",
    }
}

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());
    let metadata = ReportMetadata::new();

    print_human_summary(&results);
    println!("BEGIN_DRAUGHTS_LITE_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_DRAUGHTS_LITE_BENCHMARK_JSON");
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && THRESHOLDS
                .iter()
                .any(|threshold| threshold.operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    let benches: Vec<BenchSpec> = vec![
        ("standard_setup", "setups", 20_000, bench_standard_setup),
        (
            "legal_actions_initial",
            "trees",
            50_000,
            bench_legal_actions_initial,
        ),
        (
            "legal_actions_midgame_no_capture",
            "trees",
            50_000,
            bench_legal_actions_midgame_no_capture,
        ),
        (
            "legal_actions_mandatory_capture",
            "trees",
            50_000,
            bench_legal_actions_mandatory_capture,
        ),
        (
            "legal_actions_multi_jump",
            "trees",
            50_000,
            bench_legal_actions_multi_jump,
        ),
        (
            "validate_apply_quiet",
            "actions",
            50_000,
            bench_validate_apply_quiet,
        ),
        (
            "validate_apply_single_capture",
            "actions",
            50_000,
            bench_validate_apply_single_capture,
        ),
        (
            "validate_apply_multi_jump",
            "actions",
            25_000,
            bench_validate_apply_multi_jump,
        ),
        ("public_view_generation", "views", 25_000, bench_public_view),
        ("replay_throughput", "replays", 5_000, bench_replay),
        (
            "level0_bot_decision",
            "decisions",
            25_000,
            bench_level0_bot_decision,
        ),
        (
            "level1_bot_decision",
            "decisions",
            10_000,
            bench_level1_bot_decision,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|filter| name.contains(filter)))
        .map(|(name, unit, iterations, benchmark)| measure(name, unit, iterations, benchmark))
        .collect()
}

fn print_human_summary(results: &[BenchResult]) {
    println!("draughts_lite native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second,threshold,pass");
    for result in results {
        let threshold = threshold_for(result.name);
        let current = result.current_value();
        println!(
            "{},{},{},{:.3},{:.2},{:.2},{}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            current,
            threshold.threshold,
            current >= threshold.threshold
        );
    }
}

struct ReportMetadata {
    command: String,
    os: String,
    rust_version: String,
    hardware_environment_notes: String,
}

impl ReportMetadata {
    fn new() -> Self {
        Self {
            command: env::args().collect::<Vec<_>>().join(" "),
            os: format!("{} {}", env::consts::OS, env::consts::ARCH),
            rust_version: rust_version(),
            hardware_environment_notes:
                "Local native benchmark run; no CPU pinning, thermal isolation, or hardware probe."
                    .to_owned(),
        }
    }
}

fn rust_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn benchmark_report_json(metadata: &ReportMetadata, results: &[BenchResult]) -> String {
    let operations = results
        .iter()
        .map(operation_json)
        .collect::<Vec<_>>()
        .join(",\n    ");
    format!(
        concat!(
            "{{\n",
            "  \"schema_version\": {},\n",
            "  \"game_id\": \"{}\",\n",
            "  \"rules_version\": \"{}\",\n",
            "  \"data_version\": \"{}\",\n",
            "  \"engine_version\": \"{}\",\n",
            "  \"build_profile\": \"{}\",\n",
            "  \"command\": \"{}\",\n",
            "  \"os\": \"{}\",\n",
            "  \"rust_version\": \"{}\",\n",
            "  \"hardware_environment_notes\": \"{}\",\n",
            "  \"known_caveats\": [\"Local workstation runs can be noisy; thresholds are smoke floors until stable CI baselines exist.\"],\n",
            "  \"operations\": [\n",
            "    {}\n",
            "  ]\n",
            "}}"
        ),
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        DATA_VERSION,
        ENGINE_VERSION,
        BUILD_PROFILE,
        escape_json(&metadata.command),
        escape_json(&metadata.os),
        escape_json(&metadata.rust_version),
        escape_json(&metadata.hardware_environment_notes),
        operations
    )
}

fn operation_json(result: &BenchResult) -> String {
    let threshold = threshold_for(result.name);
    let current = result.current_value();
    format!(
        concat!(
            "{{\"operation_name\":\"{}\",",
            "\"iterations\":{},",
            "\"unit\":\"{}\",",
            "\"current_value\":{:.2},",
            "\"threshold\":{:.2},",
            "\"pass\":{},",
            "\"rationale_class\":\"{}\",",
            "\"known_caveats\":\"{}\"}}"
        ),
        result.name,
        result.iterations,
        threshold.unit,
        current,
        threshold.threshold,
        current >= threshold.threshold,
        threshold.rationale_class,
        escape_json(threshold.caveat)
    )
}

fn threshold_for(operation: &str) -> Threshold {
    THRESHOLDS
        .iter()
        .copied()
        .find(|threshold| threshold.operation == operation)
        .unwrap_or_else(|| panic!("missing threshold for operation `{operation}`"))
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

fn measure(
    name: &'static str,
    unit: &'static str,
    mut iterations: u64,
    mut benchmark: impl FnMut(u64),
) -> BenchResult {
    if cfg!(debug_assertions) {
        iterations = iterations.min(10);
    }
    let started = Instant::now();
    benchmark(iterations);
    BenchResult {
        name,
        unit,
        iterations,
        elapsed: started.elapsed(),
    }
}

fn bench_standard_setup(iterations: u64) {
    for seed in 0..iterations {
        let state = setup_match(
            black_box(Seed(seed)),
            black_box(&seats()),
            black_box(&SetupOptions::default()),
        )
        .expect("setup succeeds");
        black_box(state);
    }
}

fn bench_legal_actions_initial(iterations: u64) {
    let state = initial_state(1);
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_midgame_no_capture(iterations: u64) {
    let state = midgame_no_capture_state();
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_mandatory_capture(iterations: u64) {
    let state = single_capture_state();
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_multi_jump(iterations: u64) {
    let state = multi_jump_state();
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_apply_quiet(iterations: u64) {
    bench_validate_apply(
        iterations,
        midgame_no_capture_state,
        &["from/r3c2", "to/r4c1"],
    );
}

fn bench_validate_apply_single_capture(iterations: u64) {
    bench_validate_apply(
        iterations,
        single_capture_state,
        &["from/r3c2", "jump/r5c4"],
    );
}

fn bench_validate_apply_multi_jump(iterations: u64) {
    bench_validate_apply(
        iterations,
        multi_jump_state,
        &["from/r3c2", "jump/r5c4", "jump/r7c6"],
    );
}

fn bench_validate_apply(
    iterations: u64,
    mut state_factory: impl FnMut() -> DraughtsLiteState,
    action_path: &[&str],
) {
    for _ in 0..iterations {
        let mut state = state_factory();
        let command = command_for_segments(&state, action_path);
        let action = validate_command(&state, &command).expect("benchmark command validates");
        black_box(apply_action(black_box(&mut state), black_box(action)));
    }
}

fn bench_public_view(iterations: u64) {
    let state = multi_jump_state();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_replay(iterations: u64) {
    let commands = [
        vec!["from/r3c2".to_owned(), "to/r4c1".to_owned()],
        vec!["from/r6c7".to_owned(), "to/r5c8".to_owned()],
        vec!["from/r2c1".to_owned(), "to/r3c2".to_owned()],
    ];
    for seed in 0..iterations {
        black_box(replay_commands(black_box(seed), black_box(&commands)));
    }
}

fn bench_level0_bot_decision(iterations: u64) {
    let state = initial_state(2);
    for seed in 0..iterations {
        let bot = DraughtsLiteRandomBot::new(Seed(seed));
        black_box(
            bot.select_decision(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn bench_level1_bot_decision(iterations: u64) {
    let state = multi_jump_state();
    let bot = DraughtsLiteLevel1Bot::new(Seed(3));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn initial_state(seed: u64) -> DraughtsLiteState {
    setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn midgame_no_capture_state() -> DraughtsLiteState {
    empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat0, 2, 3, 6),
            man(DraughtsLiteSeat::Seat1, 1, 7, 2),
            man(DraughtsLiteSeat::Seat1, 2, 7, 6),
        ],
    )
}

fn single_capture_state() -> DraughtsLiteState {
    empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            man(DraughtsLiteSeat::Seat1, 2, 8, 7),
        ],
    )
}

fn multi_jump_state() -> DraughtsLiteState {
    empty_state(
        DraughtsLiteSeat::Seat0,
        vec![
            man(DraughtsLiteSeat::Seat0, 1, 3, 2),
            man(DraughtsLiteSeat::Seat1, 1, 4, 3),
            man(DraughtsLiteSeat::Seat1, 2, 6, 5),
            man(DraughtsLiteSeat::Seat1, 3, 8, 7),
        ],
    )
}

fn command_for_segments(state: &DraughtsLiteState, segments: &[&str]) -> CommandEnvelope {
    command_for_state(
        state,
        segments
            .iter()
            .map(|segment| (*segment).to_owned())
            .collect(),
    )
}

fn actor_for_state(state: &DraughtsLiteState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn coord(row: u8, col: u8) -> Coord {
    Coord::checked(row, col).unwrap()
}

fn piece_id(owner: DraughtsLiteSeat, ordinal: u8) -> PieceId {
    PieceId::new(owner, ordinal).unwrap()
}

fn man(owner: DraughtsLiteSeat, ordinal: u8, row: u8, col: u8) -> Piece {
    Piece {
        id: piece_id(owner, ordinal),
        owner,
        kind: PieceKind::Man,
        cell: coord(row, col),
    }
}

fn empty_state(active_seat: DraughtsLiteSeat, mut pieces: Vec<Piece>) -> DraughtsLiteState {
    let board = draughts_lite::ids::board_dimensions();
    pieces.sort_by_key(|piece| piece.id);
    let mut cells = DraughtsLiteState::empty_cells();
    for piece in &pieces {
        cells[piece.cell.row_col_index(board).unwrap()] = CellOccupancy::Occupied(piece.id);
    }

    DraughtsLiteState {
        variant: Variant::draughts_lite_standard(),
        board,
        cells,
        pieces,
        active_seat,
        seats: [seats()[0].clone(), seats()[1].clone()],
        ply_count: 0,
        command_count: 0,
        terminal_outcome: None,
        terminal_reason: None,
        freshness_token: FreshnessToken(0),
    }
}
