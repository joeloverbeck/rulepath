use std::{
    env,
    hint::black_box,
    time::{Duration, Instant},
};

use game_stdlib::trick_taking::{follow_suit_indices, winning_play_index};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Suit {
    Reed,
    Shell,
    Star,
    Moon,
}

#[derive(Clone, Copy)]
struct Play {
    suit: Suit,
    rank: u8,
}

struct BenchResult {
    name: &'static str,
    unit: &'static str,
    iterations: u64,
    elapsed: Duration,
}

type BenchSpec = (&'static str, &'static str, u64, fn(u64));

fn main() {
    let filter = operation_filter();
    let results = run_benchmarks(filter.as_deref());

    println!("game-stdlib trick_taking native benchmarks");
    for result in results {
        let value = result.iterations as f64 / result.elapsed.as_secs_f64();
        println!(
            "{}: {:.2} {} ({:?}, {} iterations)",
            result.name, value, result.unit, result.elapsed, result.iterations
        );
    }
}

fn operation_filter() -> Option<String> {
    env::args().skip(1).find(|arg| {
        !arg.starts_with('-')
            && [
                "follow_suit_present",
                "follow_suit_void",
                "winning_play_trump",
                "winning_play_led",
            ]
            .iter()
            .any(|operation| operation.contains(arg))
    })
}

fn run_benchmarks(filter: Option<&str>) -> Vec<BenchResult> {
    let benches: Vec<BenchSpec> = vec![
        (
            "follow_suit_present",
            "selections_per_second",
            250_000,
            bench_follow_suit_present,
        ),
        (
            "follow_suit_void",
            "selections_per_second",
            250_000,
            bench_follow_suit_void,
        ),
        (
            "winning_play_trump",
            "comparisons_per_second",
            500_000,
            bench_winning_play_trump,
        ),
        (
            "winning_play_led",
            "comparisons_per_second",
            500_000,
            bench_winning_play_led,
        ),
    ];

    benches
        .into_iter()
        .filter(|(name, _, _, _)| filter.is_none_or(|needle| name.contains(needle)))
        .map(|(name, unit, iterations, run)| {
            let started = Instant::now();
            run(iterations);
            BenchResult {
                name,
                unit,
                iterations,
                elapsed: started.elapsed(),
            }
        })
        .collect()
}

fn bench_follow_suit_present(iterations: u64) {
    let held = sample_held();
    for _ in 0..iterations {
        black_box(follow_suit_indices(
            black_box(&held),
            black_box(Suit::Shell),
            |play| play.suit,
        ));
    }
}

fn bench_follow_suit_void(iterations: u64) {
    let held = sample_held();
    for _ in 0..iterations {
        black_box(follow_suit_indices(
            black_box(&held),
            black_box(Suit::Moon),
            |play| play.suit,
        ));
    }
}

fn bench_winning_play_trump(iterations: u64) {
    let plays = sample_trick();
    for _ in 0..iterations {
        black_box(winning_play_index(
            black_box(&plays),
            black_box(Suit::Reed),
            black_box(Some(Suit::Shell)),
            |play| play.suit,
            |play| play.rank,
        ));
    }
}

fn bench_winning_play_led(iterations: u64) {
    let plays = sample_trick();
    for _ in 0..iterations {
        black_box(winning_play_index(
            black_box(&plays),
            black_box(Suit::Reed),
            black_box(None),
            |play| play.suit,
            |play| play.rank,
        ));
    }
}

fn sample_held() -> [Play; 10] {
    [
        Play {
            suit: Suit::Reed,
            rank: 14,
        },
        Play {
            suit: Suit::Shell,
            rank: 3,
        },
        Play {
            suit: Suit::Star,
            rank: 11,
        },
        Play {
            suit: Suit::Shell,
            rank: 9,
        },
        Play {
            suit: Suit::Reed,
            rank: 7,
        },
        Play {
            suit: Suit::Star,
            rank: 2,
        },
        Play {
            suit: Suit::Shell,
            rank: 13,
        },
        Play {
            suit: Suit::Reed,
            rank: 5,
        },
        Play {
            suit: Suit::Star,
            rank: 10,
        },
        Play {
            suit: Suit::Shell,
            rank: 6,
        },
    ]
}

fn sample_trick() -> [Play; 7] {
    [
        Play {
            suit: Suit::Reed,
            rank: 12,
        },
        Play {
            suit: Suit::Shell,
            rank: 4,
        },
        Play {
            suit: Suit::Reed,
            rank: 14,
        },
        Play {
            suit: Suit::Star,
            rank: 13,
        },
        Play {
            suit: Suit::Shell,
            rank: 9,
        },
        Play {
            suit: Suit::Reed,
            rank: 8,
        },
        Play {
            suit: Suit::Moon,
            rank: 2,
        },
    ]
}
