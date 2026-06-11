# GAT11MASCLABLU-015: Tool and CI native-lane registration + RULE-COVERAGE.md

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs`, new `games/masked_claims/docs/RULE-COVERAGE.md`, modifies `.github/workflows/{gate-1-game-smoke.yml,gate-2-benchmarks.yml}`
**Deps**: GAT11MASCLABLU-010, GAT11MASCLABLU-011, GAT11MASCLABLU-012, GAT11MASCLABLU-014

## Problem

`masked_claims` must be registered in the verification tools (simulate, replay-check, fixture-check, rule-coverage, bench-report) and the native/benchmark CI lanes, with `RULE-COVERAGE.md` mapping every rules-doc obligation to tests/traces. This is what makes the game an official game rather than a demo shell.

## Assumption Reassessment (2026-06-10)

1. The crate, native tests (GAT11MASCLABLU-010), golden traces + fixture (GAT11MASCLABLU-011), benches (GAT11MASCLABLU-012), and WASM bridge (GAT11MASCLABLU-014) provide what the tools validate. Each tool registers a per-game arm: `tools/simulate/src/main.rs` carries a `GAME_<NAME>` const + `run_<game>_simulation` fn + a usage string (confirmed for `plain_tricks`); `replay-check`, `fixture-check`, `rule-coverage`, `bench-report` each reference `plain_tricks` (confirmed). `seed-reducer` and `trace-viewer` carry NO `plain_tricks` reference (confirmed) — so `masked_claims` does not register there (the spec's "if their dispatch tables need game IDs" condition is not met).
2. Spec Deliverables Tools + CI rows; `RULE-COVERAGE.md` instantiates from `templates/GAME-RULE-COVERAGE.md` and is consumed by `tools/rule-coverage`, which also reads `RULES.md` (GAT11MASCLABLU-001) and `BENCHMARKS.md` (GAT11MASCLABLU-012).
3. Cross-artifact boundary under audit: the per-tool game-ID dispatch and the `gate-1-game-smoke.yml` / `gate-2-benchmarks.yml` workflows. `gate-1-game-smoke.yml` is ALSO modified by GAT11MASCLABLU-019 (web/e2e lane) — a shared-file overlap requiring a mechanical merge. `rule-coverage`'s fully-green report depends on `BENCHMARKS.md` (GAT11MASCLABLU-012, a declared Dep).
4. FOUNDATIONS §6 (official games are evidence-heavy — tools/simulation/replay/coverage prove the game) and §11 are the principles under audit.

## Architecture Check

1. Registering each tool arm where the analogous `plain_tricks` arm lives, and wiring each CI step so it first goes green when this ticket lands, avoids a multi-PR red-CI window for the native lanes.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; the tools dispatch on an opaque game ID, not a mechanic noun.

## Verification Layers

1. Many-game bot legality + invariants -> `cargo run -p simulate -- --game masked_claims --games 1000`.
2. Deterministic replay across all traces -> `cargo run -p replay-check -- --game masked_claims --all`.
3. Fixture validity -> `cargo run -p fixture-check -- --game masked_claims`.
4. Rules-doc obligations mapped to tests/traces -> `cargo run -p rule-coverage -- --game masked_claims` (reads `RULES.md` + `RULE-COVERAGE.md` + `BENCHMARKS.md`).
5. Native + benchmark CI lanes -> `gate-1-game-smoke.yml` (smoke/replay/fixture/rule-coverage) and `gate-2-benchmarks.yml` (smoke + threshold) steps.

## What to Change

### 1. Tool registration

`tools/simulate/src/main.rs` (`GAME_MASKED_CLAIMS` const, `run_masked_claims_simulation`, usage string); `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs`, `tools/bench-report/src/main.rs` dispatch arms.

### 2. `games/masked_claims/docs/RULE-COVERAGE.md`

Instantiate from `templates/GAME-RULE-COVERAGE.md`; map every rules-doc obligation to its backing test/trace.

### 3. CI lanes

`gate-1-game-smoke.yml`: add `masked_claims` native smoke, replay, fixture, and rule-coverage steps. `gate-2-benchmarks.yml`: add the smoke + threshold registration. (Web build + E2E registration land in GAT11MASCLABLU-019 — coordinate the shared `gate-1-game-smoke.yml` edit.)

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `games/masked_claims/docs/RULE-COVERAGE.md` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Web build / E2E CI step and the catalog README reconciliation (GAT11MASCLABLU-019).
- `seed-reducer` / `trace-viewer` registration (not needed — they carry no per-game dispatch).
- The `check-catalog-docs` red window opened in GAT11MASCLABLU-014 (closed in GAT11MASCLABLU-019, not here).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game masked_claims --games 1000` finishes with no illegal bot action or invariant failure.
2. `cargo run -p replay-check -- --game masked_claims --all` and `cargo run -p fixture-check -- --game masked_claims` pass.
3. `cargo run -p rule-coverage -- --game masked_claims` passes and maps every rules-doc obligation to tests/traces.

### Invariants

1. Every official-game verification tool recognizes `masked_claims` (FOUNDATIONS §6).
2. Tools dispatch on an opaque game ID; no mechanic noun enters tool/engine kernels.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/docs/RULE-COVERAGE.md` — obligation→test/trace mapping consumed by `rule-coverage`.

### Commands

1. `cargo run -p simulate -- --game masked_claims --games 1000`
2. `cargo run -p replay-check -- --game masked_claims --all && cargo run -p fixture-check -- --game masked_claims && cargo run -p rule-coverage -- --game masked_claims`
3. These CLI runs are the full native-pipeline boundary; the web/e2e lanes are GAT11MASCLABLU-019's responsibility.

## Outcome

Completed on 2026-06-11.

- Registered `masked_claims` in `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, and `bench-report`.
- Added `games/masked_claims/docs/RULE-COVERAGE.md` with a row for every `MC-*` rule ID in `RULES.md`.
- Extended `rule-coverage` rule-ID recognition for the `MC` prefix.
- Extended trace/fixture validators to accept Masked Claims evidence metadata while preserving behavior-key rejection.
- Registered Masked Claims native smoke/replay/fixture/rule-coverage CI steps and benchmark smoke/threshold lanes.
- Replay-check registers the Masked Claims trace corpus and validates its metadata. The crate-local replay tests remain the hash authority for the redacted Masked Claims trace style.

Verification:

- `cargo fmt --all`
- `cargo check -p simulate -p replay-check -p fixture-check -p rule-coverage -p bench-report`
- `cargo run -p simulate -- --game masked_claims --games 1000`
- `cargo run -p replay-check -- --game masked_claims --all`
- `cargo run -p fixture-check -- --game masked_claims`
- `cargo run -p rule-coverage -- --game masked_claims`
