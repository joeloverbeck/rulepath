# GAT4THRMARBOA-014: Native CLI tools multi-game support for Three Marks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` (`src/main.rs` + `Cargo.toml` each)
**Deps**: GAT4THRMARBOA-006, GAT4THRMARBOA-007, GAT4THRMARBOA-015

## Problem

The four CI-gated native CLI tools are hardcoded to `race_to_n` and reject any other `--game`. `docs/OFFICIAL-GAME-CONTRACT.md` §1 and `docs/ROADMAP.md` §2 require CLI simulation / replay / fixture / rule-coverage evidence for every official game, so each tool must gain a game-resolution layer accepting `--game three_marks` against the Three Marks crate, traces, fixtures, and rule-coverage matrix — without weakening the `race_to_n` path.

## Assumption Reassessment (2026-06-06)

1. All four tools are hardcoded: `tools/simulate/src/main.rs` (`const GAME_ID = "race_to_n"`, rejection at line 131, bot policy `race_to_n-random-legal-v1`); `tools/replay-check/src/main.rs` (game gate line 22, trace `game_id`/`rules_version` gates lines 269-274, `DEFAULT_TRACE_DIR` line 11, imports `race_to_n::replay_support::{replay_bot_action, replay_commands, replay_invalid, ReplayHashes}` line 9); `tools/fixture-check/src/main.rs` (gate line 95, `TRACE_DIR`/`FIXTURE_DIR` lines 8-9, `race_to_n::load_manifest`/`load_variants` lines 190-194); `tools/rule-coverage/src/main.rs` (gate line 16, `RULES_PATH`/`COVERAGE_PATH`/`BENCHMARKS_PATH` lines 3-5). Each `tools/*/Cargo.toml` depends on `race_to_n` only. Verified all sites.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §5.2 (native per-game CLI tools deliverable), §6 (CI evidence), §21 (CLI simulation / replay-fixture-coverage acceptance rows). The Three Marks crate (replay support 007, bots 006, manifest/variants 002) and `games/three_marks/docs/RULE-COVERAGE.md` (GAT4THRMARBOA-015, consumed by `rule-coverage`) are the inputs.
3. Cross-crate boundary under audit: each tool gains a dependency on the `three_marks` crate (mirroring its `race_to_n` dep) and a `--game` resolution layer; the generic per-game tool surface (`docs/ARCHITECTURE.md` tool conventions) is preserved.
4. FOUNDATIONS §6 (official games need CLI simulation + benchmarks evidence) and §2 (determinism — replay reproduces hashes) motivate this ticket: `--game three_marks` exercises the same Rust behaviour authority as `race_to_n` with no TS/tool-side rule logic.
5. Fail-closed validation + deterministic replay enforcement surfaces (§11): `fixture-check`'s manifest/variants validation and `replay-check`'s hash verification are the gates — name them. They must stay deterministic and blocking for `three_marks` (reject unknown fields, fail loudly on hash drift); the unsupported-game error path remains for any unregistered `--game`.
6. Generalizes each tool's hardcoded `race_to_n` constants into a game-resolution dispatch. Consumer: CI `gate-1-game-smoke.yml` (wired in GAT4THRMARBOA-016). The change is additive — `race_to_n` invocations and outputs (including failure-report `replay_command` strings) are preserved/regression-tested; the per-game paths/policy ids/dirs become game-parameterized.

## Architecture Check

1. A `--game` resolution layer (closed dispatch to `race_to_n` | `three_marks`) mirrors the WASM registry approach (GAT4THRMARBOA-009) and keeps the tools generic over a small registered set — cleaner than copying each tool. Alternative (separate per-game tool binaries) duplicates harness code and is rejected.
2. No backwards-compatibility aliasing/shims — constants are generalized in place; the `race_to_n` behaviour is preserved, not aliased.
3. `engine-core` untouched; no `game-stdlib` extraction; tools carry no rule logic — they drive the game crates' Rust APIs.

## Verification Layers

1. Simulation invariant -> simulation/CLI run (`simulate --game three_marks --games 1000` runs without crash; emits a seed/command failure report on failure).
2. Replay-hash invariant -> deterministic replay-hash check (`replay-check --game three_marks --all` reproduces hashes across the Three Marks golden traces).
3. Fixture/manifest invariant -> fail-closed validation (`fixture-check --game three_marks` validates manifest/variants/fixtures; rejects unknown fields).
4. Rule-coverage invariant -> schema/coverage validation (`rule-coverage --game three_marks` passes the Three Marks coverage matrix).
5. Race-to-N non-regression invariant -> simulation/CLI run (each tool's `--game race_to_n` path unchanged).

## What to Change

### 1. Game-resolution layer in each tool

Replace the hardcoded `GAME_ID`/`game != "race_to_n"` gate with a resolver dispatching to `race_to_n` | `three_marks`; parameterize per-game constants (trace dir, fixture dir, doc paths, rules version, bot policy id, replay-support entry points). Add `three_marks = { path = "../../games/three_marks" }` to each `tools/*/Cargo.toml`.

### 2. Per-tool wiring

- `simulate`: drive Three Marks Level 0 random playout + failure report.
- `replay-check`: accept `three_marks` traces/`rules_version`; use `three_marks::replay_support` hashes.
- `fixture-check`: validate `three_marks` manifest/variants/fixtures.
- `rule-coverage`: validate `games/three_marks/docs/RULE-COVERAGE.md` (from GAT4THRMARBOA-015).

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/simulate/Cargo.toml` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/replay-check/Cargo.toml` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/fixture-check/Cargo.toml` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/rule-coverage/Cargo.toml` (modify)

## Out of Scope

- `tools/trace-viewer` and `tools/seed-reducer` (deferred, spec §25 — not Gate 4 evidence).
- CI workflow step additions (GAT4THRMARBOA-016).
- Benchmarks / `bench-report` (GAT4THRMARBOA-008).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game three_marks --games 1000` and `cargo run -p replay-check -- --game three_marks --all` succeed.
2. `cargo run -p fixture-check -- --game three_marks` and `cargo run -p rule-coverage -- --game three_marks` pass.
3. The same four commands with `--game race_to_n` still pass (non-regression); `cargo test --workspace` green.

### Invariants

1. Each tool resolves `--game three_marks` against the Three Marks crate and rejects unregistered games; validation stays deterministic and blocking.
2. `race_to_n` tool behaviour and output strings are byte-preserved.

## Test Plan

### New/Modified Tests

1. `tools/*/src/main.rs` `#[cfg(test)]` — extend existing per-tool tests to cover `three_marks` resolution and preserve `race_to_n` assertions.

### Commands

1. `cargo run -p simulate -- --game three_marks --games 1000 && cargo run -p replay-check -- --game three_marks --all`
2. `cargo run -p fixture-check -- --game three_marks && cargo run -p rule-coverage -- --game three_marks`
3. `cargo test --workspace` is the full-pipeline boundary (tool unit tests + game crates); the per-tool CLI invocations are the targeted evidence commands.

## Outcome

Status: Done
Date: 2026-06-06

Changes:
- Added closed `race_to_n` / `three_marks` game resolution to `simulate`, `replay-check`, `fixture-check`, and `rule-coverage`.
- Wired the Three Marks crate into the simulation, replay, and fixture tools while preserving the race_to_n paths.
- Parameterized trace, fixture, manifest, variant, and docs paths per registered game.
- Routed Three Marks replay diagnostics through the correct replay helper for stale vs occupied-cell diagnostics and allowed documented trace metadata in fixture validation.

Verification:
- `cargo run -p simulate -- --game three_marks --games 1000`
- `cargo run -p replay-check -- --game three_marks --all`
- `cargo run -p fixture-check -- --game three_marks`
- `cargo run -p rule-coverage -- --game three_marks`
- `cargo run -p simulate -- --game race_to_n --games 1000`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p fixture-check -- --game race_to_n`
- `cargo run -p rule-coverage -- --game race_to_n`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
