use std::{
    env,
    hint::black_box,
    process::Command,
    time::{Duration, Instant},
};

use engine_core::{ActionPath, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, Viewer};
use masked_claims::{
    actor_for_seat, apply_action, legal_action_tree, project_view, setup_match, validate_command,
    MaskedClaimsLevel1Bot, MaskedClaimsSeat, MaskedClaimsState, PublicReplayExport,
    PublicReplayStep, SetupOptions, ACTION_CLAIM, ACTION_RESPOND_ACCEPT, ACTION_RESPOND_CHALLENGE,
    GAME_ID, RULES_VERSION_LABEL,
};

const DATA_VERSION: &str = "1";
const ENGINE_VERSION: &str = "engine-core-0.1.0";
const REPORT_SCHEMA_VERSION: u32 = 1;
const BUILD_PROFILE: &str = "bench";

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

const OPERATIONS: &[(&str, &str)] = &[
    ("legal_actions_claim_phase", "trees_per_second"),
    ("legal_actions_reaction_window", "trees_per_second"),
    ("validate_claim", "validations_per_second"),
    ("apply_claim_open_window", "actions_per_second"),
    ("apply_accept_resolution", "actions_per_second"),
    ("apply_challenge_resolve_reveal", "actions_per_second"),
    ("project_public_view_pending_reaction", "views_per_second"),
    ("project_public_view_after_reveal", "views_per_second"),
    ("state_hash_terminal", "hashes_per_second"),
    ("public_export_timeline", "exports_per_second"),
    ("level1_bot_claim_decision", "decisions_per_second"),
    ("level1_bot_response_decision", "decisions_per_second"),
];

impl BenchResult {
    fn current_value(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let filter = operation_filter();
    let metadata = ReportMetadata::new();
    let results = run_benchmarks(filter.as_deref());

    println!("masked_claims native benchmarks");
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
    println!("BEGIN_MASKED_CLAIMS_BENCHMARK_JSON");
    println!("{}", benchmark_report_json(&metadata, &results));
    println!("END_MASKED_CLAIMS_BENCHMARK_JSON");
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
        (
            "legal_actions_claim_phase",
            "trees",
            50_000,
            bench_legal_actions_claim_phase,
        ),
        (
            "legal_actions_reaction_window",
            "trees",
            50_000,
            bench_legal_actions_reaction_window,
        ),
        (
            "validate_claim",
            "validations",
            50_000,
            bench_validate_claim,
        ),
        (
            "apply_claim_open_window",
            "actions",
            50_000,
            bench_apply_claim_open_window,
        ),
        (
            "apply_accept_resolution",
            "actions",
            50_000,
            bench_apply_accept_resolution,
        ),
        (
            "apply_challenge_resolve_reveal",
            "actions",
            50_000,
            bench_apply_challenge_resolve_reveal,
        ),
        (
            "project_public_view_pending_reaction",
            "views",
            50_000,
            bench_project_public_view_pending_reaction,
        ),
        (
            "project_public_view_after_reveal",
            "views",
            50_000,
            bench_project_public_view_after_reveal,
        ),
        (
            "state_hash_terminal",
            "hashes",
            50_000,
            bench_state_hash_terminal,
        ),
        (
            "public_export_timeline",
            "exports",
            10_000,
            bench_public_export_timeline,
        ),
        (
            "level1_bot_claim_decision",
            "decisions",
            50_000,
            bench_level1_bot_claim_decision,
        ),
        (
            "level1_bot_response_decision",
            "decisions",
            50_000,
            bench_level1_bot_response_decision,
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

fn bench_legal_actions_claim_phase(iterations: u64) {
    let state = setup();
    let actor = actor_for_seat(&state, MaskedClaimsSeat::Seat0);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_legal_actions_reaction_window(iterations: u64) {
    let state = state_after_claim();
    let actor = actor_for_seat(&state, MaskedClaimsSeat::Seat1);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_validate_claim(iterations: u64) {
    let state = setup();
    let command = claim_command(&state, "5");
    for _ in 0..iterations {
        black_box(validate_command(black_box(&state), black_box(&command)).unwrap());
    }
}

fn bench_apply_claim_open_window(iterations: u64) {
    let base = setup();
    let action = validate_command(&base, &claim_command(&base, "5")).expect("claim validates");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
        black_box(state);
    }
}

fn bench_apply_accept_resolution(iterations: u64) {
    let base = state_after_claim();
    let action = validate_command(&base, &response_command(&base, ACTION_RESPOND_ACCEPT))
        .expect("accept validates");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
        black_box(state);
    }
}

fn bench_apply_challenge_resolve_reveal(iterations: u64) {
    let base = state_after_claim();
    let action = validate_command(&base, &response_command(&base, ACTION_RESPOND_CHALLENGE))
        .expect("challenge validates");
    for _ in 0..iterations {
        let mut state = base.clone();
        black_box(apply_action(black_box(&mut state), black_box(action)).unwrap());
        black_box(state);
    }
}

fn bench_project_public_view_pending_reaction(iterations: u64) {
    let state = state_after_claim();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_project_public_view_after_reveal(iterations: u64) {
    let state = state_after_challenge();
    let viewer = Viewer { seat_id: None };
    for _ in 0..iterations {
        black_box(project_view(black_box(&state), black_box(&viewer)));
    }
}

fn bench_state_hash_terminal(iterations: u64) {
    let state = terminal_state();
    for _ in 0..iterations {
        black_box(HashValue::from_stable_bytes(
            black_box(&state.stable_internal_summary()).as_bytes(),
        ));
    }
}

fn bench_public_export_timeline(iterations: u64) {
    let state = state_after_challenge();
    let view = project_view(&state, &Viewer { seat_id: None });
    for _ in 0..iterations {
        let export = PublicReplayExport::new(
            "observer",
            vec![PublicReplayStep::from_view(
                0,
                &view,
                Vec::new(),
                "respond/challenge",
                false,
            )],
        );
        let json = export.to_json();
        black_box(PublicReplayExport::from_json(black_box(&json)).unwrap());
    }
}

fn bench_level1_bot_claim_decision(iterations: u64) {
    let state = setup();
    let bot = MaskedClaimsLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(MaskedClaimsSeat::Seat0))
                .unwrap(),
        );
    }
}

fn bench_level1_bot_response_decision(iterations: u64) {
    let state = state_after_claim();
    let bot = MaskedClaimsLevel1Bot::new(Seed(7));
    for _ in 0..iterations {
        black_box(
            bot.select_decision(black_box(&state), black_box(MaskedClaimsSeat::Seat1))
                .unwrap(),
        );
    }
}

fn setup() -> MaskedClaimsState {
    setup_match(Seed(12), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn state_after_claim() -> MaskedClaimsState {
    let mut state = setup();
    let action = validate_command(&state, &claim_command(&state, "5")).expect("claim validates");
    apply_action(&mut state, action).expect("claim applies");
    state
}

fn state_after_challenge() -> MaskedClaimsState {
    let mut state = state_after_claim();
    let action = validate_command(&state, &response_command(&state, ACTION_RESPOND_CHALLENGE))
        .expect("challenge validates");
    apply_action(&mut state, action).expect("challenge applies");
    state
}

fn terminal_state() -> MaskedClaimsState {
    let mut state = setup();
    while !matches!(state.phase, masked_claims::Phase::Terminal) {
        let seat = match state.phase {
            masked_claims::Phase::Claim { .. } => state.active_seat.expect("active claimant"),
            masked_claims::Phase::Reaction { responder, .. } => responder,
            masked_claims::Phase::Terminal => unreachable!(),
        };
        let decision = MaskedClaimsLevel1Bot::new(Seed(9))
            .select_decision(&state, seat)
            .expect("bot decision");
        let command = CommandEnvelope {
            actor: actor_for_seat(&state, seat),
            action_path: decision.action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(1),
        };
        let action = validate_command(&state, &command).expect("decision validates");
        apply_action(&mut state, action).expect("decision applies");
    }
    state
}

fn claim_command(state: &MaskedClaimsState, declared: &str) -> CommandEnvelope {
    let tree = legal_action_tree(state, &actor_for_seat(state, MaskedClaimsSeat::Seat0));
    let claim = tree
        .root
        .choices
        .iter()
        .find(|choice| choice.segment == ACTION_CLAIM)
        .expect("claim family exists");
    let tile = &claim.next.as_ref().expect("tile choices").choices[0];
    CommandEnvelope {
        actor: actor_for_seat(state, MaskedClaimsSeat::Seat0),
        action_path: ActionPath {
            segments: vec![
                ACTION_CLAIM.to_owned(),
                tile.segment.clone(),
                declared.to_owned(),
            ],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn response_command(state: &MaskedClaimsState, response: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_seat(state, MaskedClaimsSeat::Seat1),
        action_path: ActionPath {
            segments: vec![response.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
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
    let result_json = results
        .iter()
        .map(|result| {
            format!(
                "{{\"operation_name\":\"{}\",\"unit\":\"{}\",\"iterations\":{},\"elapsed_ms\":{:.6},\"current_value\":{:.6},\"threshold\":1.0,\"pass\":{}}}",
                result.name,
                unit_for(result.name),
                result.iterations,
                result.elapsed.as_secs_f64() * 1_000.0,
                result.current_value(),
                result.current_value() >= 1.0
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":{},\"game_id\":\"{}\",\"rules_version\":\"{}\",\"engine_version\":\"{}\",\"data_version\":\"{}\",\"build_profile\":\"{}\",\"command\":\"{}\",\"os\":\"{}\",\"rust_version\":\"{}\",\"hardware_environment_notes\":\"{}\",\"operations\":[{}]}}",
        REPORT_SCHEMA_VERSION,
        GAME_ID,
        RULES_VERSION_LABEL,
        ENGINE_VERSION,
        DATA_VERSION,
        BUILD_PROFILE,
        escape_json(&metadata.command),
        escape_json(&metadata.os),
        escape_json(&metadata.rust_version),
        escape_json(&metadata.hardware_environment_notes),
        result_json
    )
}

fn unit_for(operation: &str) -> &'static str {
    OPERATIONS
        .iter()
        .find(|(candidate, _)| *candidate == operation)
        .map(|(_, unit)| *unit)
        .expect("bench operation has threshold metadata")
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
