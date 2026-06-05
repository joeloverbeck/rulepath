use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use engine_core::{
    ActionPath, Actor, CommandEnvelope, EffectCursor, EffectLog, HashValue, RulesVersion, SeatId,
    Seed, StableSerialize, Viewer,
};
use race_to_n::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, RaceEffect,
    RaceRandomBot, RaceSnapshot, RaceState, SetupOptions,
};

const RULES_VERSION: u32 = 1;

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

impl BenchResult {
    fn per_second(&self) -> f64 {
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }
}

fn main() {
    let results = vec![
        measure("legal_actions", "trees", 1_000_000, bench_legal_actions),
        measure("apply_action", "actions", 1_000_000, bench_apply_action),
        measure(
            "public_view_generation",
            "views",
            1_000_000,
            bench_public_view,
        ),
        measure(
            "effect_filtering",
            "filters",
            1_000_000,
            bench_effect_filtering,
        ),
        measure(
            "serialization_roundtrip",
            "roundtrips",
            500_000,
            bench_serialization_roundtrip,
        ),
        measure("replay_throughput", "replays", 100_000, bench_replay),
        measure("random_playout", "games", 100_000, bench_random_playout),
        measure("bot_decision", "decisions", 1_000_000, bench_bot_decision),
    ];

    println!("race_to_n native benchmarks");
    println!("operation,iterations,unit,elapsed_ms,per_second");
    for result in results {
        println!(
            "{},{},{},{:.3},{:.2}",
            result.name,
            result.iterations,
            result.unit,
            result.elapsed.as_secs_f64() * 1_000.0,
            result.per_second()
        );
    }
}

fn measure(
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    mut benchmark: impl FnMut(u64),
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

fn bench_legal_actions(iterations: u64) {
    let state = initial_state(1);
    let actor = actor_for_state(&state);
    for _ in 0..iterations {
        black_box(legal_action_tree(black_box(&state), black_box(&actor)));
    }
}

fn bench_apply_action(iterations: u64) {
    let mut state = initial_state(2);
    for _ in 0..iterations {
        if state.winner.is_some() {
            state = initial_state(2);
        }
        let command = command_for_state(&state, "add-1");
        let action = validate_command(&state, &command).expect("benchmark command validates");
        black_box(apply_action(black_box(&mut state), black_box(action)));
    }
}

fn bench_public_view(iterations: u64) {
    let state = initial_state(3);
    for _ in 0..iterations {
        black_box(project_view(black_box(&state)));
    }
}

fn bench_effect_filtering(iterations: u64) {
    let mut log = EffectLog::new();
    let mut state = initial_state(4);
    let action = validate_command(&state, &command_for_state(&state, "add-1"))
        .expect("benchmark command validates");
    for effect in apply_action(&mut state, action) {
        log.push(effect);
    }
    let viewer = Viewer { seat_id: None };

    for _ in 0..iterations {
        black_box(log.since(black_box(EffectCursor(0)), black_box(&viewer)));
    }
}

fn bench_serialization_roundtrip(iterations: u64) {
    let snapshot = RaceSnapshot::from_state(&initial_state(5));
    let json = snapshot.to_json();
    for _ in 0..iterations {
        let parsed = RaceSnapshot::from_json(black_box(&json)).expect("snapshot parses");
        black_box(parsed.stable_hash());
    }
}

fn bench_replay(iterations: u64) {
    let commands = [
        "add-3", "add-3", "add-3", "add-3", "add-3", "add-3", "add-3",
    ];
    for index in 0..iterations {
        black_box(replay_commands(black_box(index), black_box(&commands)));
    }
}

fn bench_random_playout(iterations: u64) {
    for seed in 0..iterations {
        black_box(run_random_playout(black_box(seed)));
    }
}

fn bench_bot_decision(iterations: u64) {
    let state = initial_state(6);
    for seed in 0..iterations {
        let bot = RaceRandomBot::new(Seed(seed));
        black_box(
            bot.select_action(black_box(&state), black_box(state.active_seat))
                .expect("bot selects a legal action"),
        );
    }
}

fn run_random_playout(seed: u64) -> u64 {
    let mut state = initial_state(seed);
    let mut actions = 0;

    while state.winner.is_none() {
        let bot = RaceRandomBot::new(Seed(seed.wrapping_add(actions)));
        let action_path = bot
            .select_action(&state, state.active_seat)
            .expect("bot selects a legal action");
        let command = CommandEnvelope {
            actor: actor_for_state(&state),
            action_path,
            freshness_token: state.freshness_token,
            rules_version: RulesVersion(RULES_VERSION),
        };
        let action = validate_command(&state, &command).expect("bot action validates");
        apply_action(&mut state, action);
        actions += 1;
    }

    actions
}

fn replay_commands(seed: u64, commands: &[&str]) -> HashValue {
    let mut state = initial_state(seed);
    let mut effects = Vec::new();

    for segment in commands {
        let command = command_for_state(&state, segment);
        let action = validate_command(&state, &command).expect("replay command validates");
        effects.extend(apply_action(&mut state, action));
    }

    let bytes = effects
        .iter()
        .map(effect_json)
        .collect::<Vec<_>>()
        .join("\n");
    HashValue::from_stable_bytes(bytes.as_bytes())
}

fn initial_state(seed: u64) -> RaceState {
    setup_match(Seed(seed), &seats(), &SetupOptions::default()).expect("setup succeeds")
}

fn seats() -> Vec<SeatId> {
    vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())]
}

fn actor_for_state(state: &RaceState) -> Actor {
    Actor {
        seat_id: state.seats[state.active_seat.index()].clone(),
    }
}

fn command_for_state(state: &RaceState, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor_for_state(state),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(RULES_VERSION),
    }
}

fn effect_json(effect: &engine_core::EffectEnvelope<RaceEffect>) -> String {
    match &effect.payload {
        RaceEffect::ActionStarted { actor, amount } => {
            format!("ActionStarted:{}:{amount}", actor.as_str())
        }
        RaceEffect::CounterAdvanced {
            actor,
            from,
            to,
            amount,
        } => format!(
            "CounterAdvanced:{}:{}:{}:{amount}",
            actor.as_str(),
            from.0,
            to.0
        ),
        RaceEffect::TurnChanged { next_actor } => {
            format!("TurnChanged:{}", next_actor.as_str())
        }
        RaceEffect::GameEnded { winner } => format!("GameEnded:{}", winner.as_str()),
        RaceEffect::ActionCompleted { actor } => format!("ActionCompleted:{}", actor.as_str()),
    }
}
