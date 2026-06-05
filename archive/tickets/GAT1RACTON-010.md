# GAT1RACTON-010: Native benchmarks (OGC §1 / TESTING §14 set) + BENCHMARKS.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — a native benchmark target for `race_to_n` (workspace-wired) and `games/race_to_n/docs/BENCHMARKS.md`.
**Deps**: GAT1RACTON-006, GAT1RACTON-007

## Problem

Official games need native benchmarks (FOUNDATIONS §6; OFFICIAL-GAME-CONTRACT §1;
TESTING §14). The spec (corrected by `/reassess-spec` this session) requires the
TESTING §14 "measure at least" floor: legal-action generation, apply, view/effect
filtering, serialization + replay throughput, random-playout throughput, and
random-bot decision latency — against the Stage-1 budget (TESTING §15:
500,000+ games/sec). This ticket also pins where the benchmark lives (the spec's
M3 finding) and fills `BENCHMARKS.md` (TESTING §16).

## Assumption Reassessment (2026-06-05)

1. `benches/` currently contains only `README.md` ("Gate 0 placeholder") and is
   **not** a workspace member (verified `grep -n benches Cargo.toml` → 0 hits).
   The spec §3 tree shows top-level `benches/`; ARCHITECTURE §1 also shows
   top-level `benches/`. The game (006) and bot (007) provide every benchmarked
   surface.
2. The spec's WB6 (reassess-spec M2+M3) requires the broadened benchmark set and
   asks this ticket to **pin the benchmark home** — either a top-level `benches/`
   workspace crate (ARCHITECTURE §1) or an in-crate
   `games/race_to_n/benches/` criterion target — and wire it. Decision (Architecture
   Check below): in-crate criterion target, with `benches/README.md` updated to
   point at it.
3. Cross-crate boundary under audit: the benchmark depends on `games/race_to_n`
   (+ `engine-core`/`ai-core`) and is a dev-dependency/bench target; it adds no
   production code. Benchmark fixtures reuse `games/race_to_n/data/fixtures/`.
4. FOUNDATIONS §11 (benchmarks cover the change; no performance claim without
   evidence) and §6 (evidence-heavy) motivate this ticket. Native-first
   benchmarking is TESTING §14 doctrine (browser/WASM perf is later smoke).
5. Determinism note: benchmarks run over deterministic seeds (no wall-clock in
   the measured behavior, only in timing). This is not a replay/hash semantics
   change — no §13 trigger. No leakage (perfect-info; benchmark data is public).
6. Schema/contract: `BENCHMARKS.md` follows `templates/GAME-BENCHMARKS.md` +
   TESTING §16 contents (hardware, OS, Rust version, build profile, versions,
   command, baseline numbers, regression threshold, bottlenecks). Additive doc;
   no schema changed.

## Architecture Check

1. An in-crate `games/race_to_n/benches/` criterion target keeps the benchmark
   co-located with the code it measures and avoids a near-empty top-level bench
   crate; `benches/README.md` is updated to index it. This is cleaner than a
   top-level workspace bench crate that would re-declare game deps. (If the
   reviewer prefers the top-level crate per ARCHITECTURE §1, the spec permits
   either; this ticket records the chosen home.)
2. No backwards-compatibility shims — `benches/README.md` placeholder is updated,
   not aliased.
3. `engine-core` untouched; benchmark code lives with the game. `game-stdlib`
   untouched.

## Verification Layers

1. Benchmark coverage -> benchmark check (the target measures all TESTING §14
   floor items: legal-action gen, apply, view/effect filtering, serialization +
   replay throughput, random-playout throughput, bot decision latency).
2. Stage-1 budget -> benchmark check (random-playout throughput compared to the
   500,000+ games/sec Stage-1 target; document if unmet per TESTING §15).
3. Doc completeness -> manual review (`BENCHMARKS.md` carries every TESTING §16
   field; no silent gaps).
4. Workspace wiring -> codebase grep-proof (`cargo bench -p race_to_n` resolves
   the target).

## What to Change

### 1. Benchmark target (`games/race_to_n/benches/race_to_n.rs` + `Cargo.toml`)

Add a criterion (or equivalent) bench target with cases for: legal-action
generation, apply, view/effect filtering, serialization + replay throughput,
random-playout throughput, and random-bot decision latency. Declare `[[bench]]`
(harness=false) + dev-deps in `games/race_to_n/Cargo.toml`.

### 2. benches/ index

Update `benches/README.md` to point at the in-crate target (remove the "arrive in
later gates" placeholder wording for `race_to_n`).

### 3. BENCHMARKS.md

Author `games/race_to_n/docs/BENCHMARKS.md` from `templates/GAME-BENCHMARKS.md` +
TESTING §16: baseline numbers, the `cargo bench` command, regression threshold,
known bottlenecks, comparison to the Stage-1 budget.

## Files to Touch

- `games/race_to_n/benches/race_to_n.rs` (new)
- `games/race_to_n/Cargo.toml` (modify) — `[[bench]]` + dev-deps
- `benches/README.md` (modify)
- `games/race_to_n/docs/BENCHMARKS.md` (new)

## Out of Scope

- The simulation runner (GAT1RACTON-009).
- `bench-report` generalization + general benchmark harness (Gate 2; spec §2).
- WASM/browser performance smoke (later; native-first per TESTING §14).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p race_to_n` resolves and runs the benchmark target without error.
2. `BENCHMARKS.md` contains every TESTING §16 field and records the Stage-1 budget comparison.
3. `cargo build --workspace` — clean (bench target compiles).

### Invariants

1. The benchmark set covers the TESTING §14 floor (OGC §1) — no measured area silently omitted.
2. No performance claim appears without a corresponding benchmark number (FOUNDATIONS §11; TESTING §14).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/benches/race_to_n.rs` — criterion cases for the TESTING §14 floor.
2. `None additional — benchmarks are the test surface; correctness tests live in 005/006/008.`

### Commands

1. `cargo bench -p race_to_n`
2. `cargo build --workspace`
3. A narrower `cargo bench -p race_to_n -- legal_actions` confirms a single case; the full `cargo bench` is the evidence command for `BENCHMARKS.md`.
