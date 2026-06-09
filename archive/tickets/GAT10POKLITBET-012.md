# GAT10POKLITBET-012: Native tool registration + RULE-COVERAGE.md + gate-1 native CI steps

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` (+ each tool's `Cargo.toml` dep), `.github/workflows/gate-1-game-smoke.yml`, and new `games/poker_lite/docs/RULE-COVERAGE.md`. No kernel change.
**Deps**: GAT10POKLITBET-010

## Problem

`poker_lite` must be registered in the four native tools that enumerate every game — `simulate`, `replay-check`, `fixture-check`, `rule-coverage` — and wired into the gate-1 CI native steps, so simulation, replay scanning, fixture checking, and rule-coverage run for the game. `RULE-COVERAGE.md` co-lands here because `tools/rule-coverage` consumes it (a tool-validated doc must exist for the tool registration to check).

## Assumption Reassessment (2026-06-08)

1. Each of the four tools registers all 8 current games via an explicit match arm / id list, verified this session: `tools/simulate/src/main.rs` (game-id match ~L144), `tools/replay-check/src/main.rs` (~L70), `tools/fixture-check/src/main.rs` (~L147), `tools/rule-coverage/src/main.rs` (~L33). Each needs a `poker_lite` arm plus a `poker_lite` path dep in its `Cargo.toml`.
2. The spec's resolved tool-scope set (`specs/gate-10-poker-lite-betting-showdown.md` §4, confirmed in §12 Assumptions) names exactly these four as requiring an arm; `bench-report` is optional (GAT10POKLITBET-013) and `seed-reducer`/`trace-viewer` are **not** expected (they register only `race_to_n` + `directional_flip`). This ticket therefore touches only the four all-games tools — not seed-reducer/trace-viewer.
3. Cross-artifact boundary under audit: the per-tool game-id dispatch surface and the `tools/rule-coverage` → `RULES.md`/`RULE-COVERAGE.md` validator contract (the tool reads the rule-coverage doc). `RULE-COVERAGE.md` maps the rule IDs authored in GAT10POKLITBET-001 to the tests/traces from 005/007/009/010 — so it co-lands with the rule-coverage arm, not in a trailing docs ticket. `simulate` exercises the bots from GAT10POKLITBET-010; `replay-check` scans the golden traces from 009.
4. FOUNDATIONS §6 (official games are evidence-heavy — CLI simulation, rule coverage, deterministic replay support are mandatory) motivates this ticket. Restated: registration is what makes the simulation/replay/coverage evidence runnable for `poker_lite`.

## Architecture Check

1. Adding a match arm per tool (rather than a generic registry abstraction) matches the established per-tool dispatch and keeps each tool's diff small and reviewable. Co-landing `RULE-COVERAGE.md` with its validator prevents a registration that has nothing valid to check.
2. No backwards-compatibility aliasing/shims — additive match arms.
3. `engine-core` untouched (§3); no `game-stdlib` promotion (§4); tools depend on the `games/poker_lite` crate, not the reverse.

## Verification Layers

1. Tool registration (each tool resolves `--game poker_lite`) -> run each tool's CLI for `poker_lite`.
2. Simulation legality/termination -> `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16`.
3. Replay scan + fixture + coverage -> `cargo run -p replay-check -- --game poker_lite`; `cargo run -p fixture-check -- --game poker_lite`; `cargo run -p rule-coverage -- --game poker_lite`.
4. CI wiring (gate-1 native steps present for poker_lite) -> codebase grep-proof in `gate-1-game-smoke.yml`.

## What to Change

### 1. `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` (+ `Cargo.toml` deps)

Add a `poker_lite` match arm to each tool's game-id dispatch, mirroring the `secret_draft` arm; add the `poker_lite` path dependency to each tool's `Cargo.toml`.

### 2. `games/poker_lite/docs/RULE-COVERAGE.md` (new)

Instantiate from `templates/GAME-RULE-COVERAGE.md`. Map each Crest Ledger rule ID (GAT10POKLITBET-001) to its tests/traces/docs/UI, in the shape `tools/rule-coverage` validates.

### 3. `.github/workflows/gate-1-game-smoke.yml` (modify)

Add the `poker_lite` native steps (simulate / replay-check / fixture-check / rule-coverage) mirroring the existing per-game step blocks. (The gate-1 e2e step is added in GAT10POKLITBET-016.)

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/simulate/Cargo.toml` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/replay-check/Cargo.toml` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/fixture-check/Cargo.toml` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/rule-coverage/Cargo.toml` (modify)
- `games/poker_lite/docs/RULE-COVERAGE.md` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify — native steps only)

## Out of Scope

- `bench-report` registration and benchmarks (GAT10POKLITBET-013).
- `seed-reducer` / `trace-viewer` registration (not expected; resolved tool-scope set).
- WASM registration (GAT10POKLITBET-014) and the gate-1 e2e step + catalog README reconciliation (GAT10POKLITBET-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16` runs to terminal cleanly.
2. `cargo run -p replay-check -- --game poker_lite`, `cargo run -p fixture-check -- --game poker_lite`, `cargo run -p rule-coverage -- --game poker_lite` all pass.
3. `gate-1-game-smoke.yml` contains poker_lite native (simulate/replay-check/fixture-check/rule-coverage) steps.

### Invariants

1. `poker_lite` resolves in exactly the four all-games tools; seed-reducer/trace-viewer remain unchanged (resolved tool-scope set).
2. `RULE-COVERAGE.md` is present and valid before/with the rule-coverage arm (tool-validated doc co-lands).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/docs/RULE-COVERAGE.md` — rule-ID → test/trace mapping consumed by `rule-coverage`.
2. No new Rust unit tests — the tool arms are exercised by the CLI runs above.

### Commands

1. `cargo run -p rule-coverage -- --game poker_lite`
2. `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16 && cargo run -p replay-check -- --game poker_lite && cargo run -p fixture-check -- --game poker_lite`
3. `cargo build --workspace` — confirms the four tool `Cargo.toml` deps resolve.

## Outcome

Completed on 2026-06-09.

Changed:

- Registered `poker_lite` in `simulate`, `replay-check`, `fixture-check`, and `rule-coverage`.
- Added the validated Crest Ledger `RULE-COVERAGE.md`.
- Added poker_lite native simulation, replay, fixture, and rule-coverage steps to `gate-1-game-smoke.yml`.
- Normalized poker_lite golden traces to the strict fixture/replay schema now enforced by the native tools.

Deviations:

- `rule-coverage` points the poker_lite benchmark reference at `RULE-COVERAGE.md` until GAT10POKLITBET-013 adds the dedicated benchmark doc and thresholds.

Verification:

- `cargo fmt --all --check`
- `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16`
- `cargo run -p replay-check -- --game poker_lite`
- `cargo run -p fixture-check -- --game poker_lite`
- `cargo run -p rule-coverage -- --game poker_lite`
- `cargo test -p poker_lite --test replay`
- `cargo test -p poker_lite --test bots`
- `cargo test -p poker_lite`
- `node scripts/check-doc-links.mjs`
- `cargo build --workspace`
