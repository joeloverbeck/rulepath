# GAT13FROCONASY-013: Native tools, RULE-COVERAGE.md, boundary-check, and gate-1 CI

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs` (modify), `scripts/boundary-check.sh` (modify), `games/frontier_control/docs/RULE-COVERAGE.md` (new), `.github/workflows/gate-1-game-smoke.yml` (modify)
**Deps**: GAT13FROCONASY-009, GAT13FROCONASY-012

## Problem

Frontier Control must be registered across the native tool suite (`simulate`, `replay-check`, `fixture-check`, `rule-coverage`, `bench-report`), carry a `RULE-COVERAGE.md` mapping every rules-doc obligation to tests/traces, extend the mechanical boundary check to cover the two FOUNDATIONS §3 forbidden kernel nouns this gate is built around (`faction`, `territory`), and add the gate-1 native CI lanes. `simulate` output must report per-faction win counts and average scores (the ROADMAP "simulations produce useful metrics" line).

## Assumption Reassessment (2026-06-11)

1. Each tool registers `flood_watch` via a string-dispatch match arm / `RegisteredGame` struct (verified: `simulate` main.rs L10/29/224/479, `replay-check` L107-110, `fixture-check` L261-266, `rule-coverage` L84-88, `bench-report` L122-124). `tools/seed-reducer` and `tools/trace-viewer` carry **no** per-game dispatch (verified absent), so no registration is expected there. Frontier mirrors the five-tool registration.
2. `scripts/boundary-check.sh` line 4 `mechanic_pattern` ends at `…|role|scenario` (verified verbatim); `engine-core` is clean of `faction`/`territory` today (verified 0 matches), so appending the two nouns must stay green on the existing tree. Spec work-item 11 directs evaluating `adjacency`/`movement` for word-boundary false positives before including them and excluding `graph` (too generic).
3. Cross-artifact boundary under audit: `replay-check` consumes the seventeen golden traces and `fixture-check` the two fixtures (GAT13FROCONASY-009/003); `rule-coverage` reads `RULES.md` (GAT13FROCONASY-001) + `RULE-COVERAGE.md` (here) + `BENCHMARKS.md` (GAT13FROCONASY-010) — so a fully-green `rule-coverage --game frontier_control` depends on GAT13FROCONASY-010 having landed `BENCHMARKS.md` (expected partial-green window if run before 010; see Step 6). `simulate` needs the bots (GAT13FROCONASY-008).
4. FOUNDATIONS §3 (`engine-core` is a contract kernel) is the principle under audit: extending `mechanic_pattern` with `faction`/`territory` mechanically enforces the kernel-boundary stop condition; the extension must not regress on the current clean tree, and the new game's faction/territory nouns stay in `games/frontier_control`.

## Architecture Check

1. Per-tool string-dispatch arms mirror every prior game — uniform and reviewable; extending the existing `mechanic_pattern` (vs a new bespoke check) keeps one boundary guard authoritative.
2. No backwards-compatibility aliasing/shims.
3. Confirms `engine-core` stays noun-free (the boundary-check extension is the proof) and `game-stdlib` gains nothing.

## Verification Layers

1. Tool registration -> simulation/CLI run + `replay-check`/`fixture-check`/`rule-coverage` runs (each passes for `frontier_control`; `simulate` reports per-faction metrics).
2. Kernel boundary (§3) -> `bash scripts/boundary-check.sh` (passes with `faction|territory` appended; stays green on the existing tree).
3. Rule coverage -> `cargo run -p rule-coverage -- --game frontier_control` (every rules-doc obligation maps to tests/traces).
4. gate-1 native CI -> workflow parse (native simulate/replay/fixture/rule-coverage steps present).

## What to Change

### 1. Tool registration

Add `frontier_control` arms to `simulate` (per-faction win counts + average scores + average rounds + tiebreak frequency), `replay-check`, `fixture-check` (both variants), `rule-coverage`, and `bench-report`.

### 2. boundary-check extension

Append `faction` and `territory` to `mechanic_pattern`; evaluate `adjacency`/`movement` for false positives before including; exclude `graph`.

### 3. RULE-COVERAGE.md

Instantiate from `templates/GAME-RULE-COVERAGE.md`; map every rules-doc obligation to its test/trace.

### 4. gate-1 CI native lanes

Add the `frontier_control` simulate/replay/fixture/rule-coverage steps to `.github/workflows/gate-1-game-smoke.yml` (the E2E/web-build lanes co-land in GAT13FROCONASY-016).

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `scripts/boundary-check.sh` (modify)
- `games/frontier_control/docs/RULE-COVERAGE.md` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- WASM registration (GAT13FROCONASY-012) and web smoke / catalog reconciliation / gate-1 E2E lane (GAT13FROCONASY-016).
- `seed-reducer`/`trace-viewer` (no per-game dispatch; confirm still true at implementation).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game frontier_control --games 1000` finishes with no illegal action / invariant failure and reports per-faction metrics.
2. `cargo run -p replay-check -- --game frontier_control --all`, `cargo run -p fixture-check -- --game frontier_control`, and `cargo run -p rule-coverage -- --game frontier_control` pass.
3. `bash scripts/boundary-check.sh` passes with `faction|territory` appended.

### Invariants

1. `engine-core` gains no mechanic noun; the boundary check mechanically enforces `faction`/`territory` (§3).
2. `simulate` reports per-faction win counts and average scores.

## Test Plan

### New/Modified Tests

1. `games/frontier_control/docs/RULE-COVERAGE.md` — rules-doc-to-test/trace map (validated by `rule-coverage`).

### Commands

1. `cargo run -p rule-coverage -- --game frontier_control && bash scripts/boundary-check.sh`
2. `cargo run -p simulate -- --game frontier_control --games 1000 && cargo run -p replay-check -- --game frontier_control --all && cargo run -p fixture-check -- --game frontier_control`
3. The native tool CLIs are the correct boundary; the browser E2E + catalog reconciliation are GAT13FROCONASY-016.
