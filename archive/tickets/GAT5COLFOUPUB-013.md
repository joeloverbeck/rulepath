# GAT5COLFOUPUB-013: Column Four tool registration & RULE-COVERAGE

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modify `tools/simulate/src/main.rs`, `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs`; new `games/column_four/docs/RULE-COVERAGE.md`
**Deps**: 010, 011

## Problem

`column_four` must participate in the per-game validation tools (simulation, replay drift, fixture validation, rule coverage) the CI gate runs (spec §18 tool coverage). `tools/rule-coverage` validates `RULE-COVERAGE.md`, so the coverage matrix is authored here alongside the tool registration that checks it.

## Assumption Reassessment (2026-06-06)

1. Each tool registers games via an explicit per-game match: `tools/simulate/src/main.rs` checks `config.game` against `GAME_ID`/`GAME_THREE_MARKS` and dispatches; `tools/replay-check`, `tools/fixture-check`, and `tools/rule-coverage` each have a `RegisteredGame` match arm keyed by game id with paths (trace_dir, fixture_dir, manifest/variants paths, rules/coverage/benchmarks paths) — verified. Adding `column_four` means a new arm in each. `seed-reducer`/`trace-viewer` are `race_to_n`-only and are out of scope (A-4).
2. Spec §18 (tool coverage) requires `column_four` in simulate/replay-check/fixture-check/rule-coverage. The trace_dir is `games/column_four/tests/golden_traces` and fixture/manifest/variants come from GAT5COLFOUPUB-002/010; `RULES.md` (001), `BENCHMARKS.md` (011) feed `rule-coverage`'s `rules_path`/`benchmarks_path`.
3. Cross-artifact boundary under audit: the per-tool `RegisteredGame` registries (consumers of game ids/paths) and the rule-coverage doc contract (`tools/rule-coverage` ↔ `RULE-COVERAGE.md`). Adding arms is additive; no existing game's arm changes.
4. FOUNDATIONS §11 (deterministic replay/traces; fail-closed validation) and §6 (evidence coverage) motivate this ticket: `replay-check` and `fixture-check` are fail-closed validators that must pass deterministically for `column_four`.
5. Deterministic replay/hash and fail-closed validation are the enforcement surfaces under audit: `replay-check --all` must pass for the GAT5COLFOUPUB-010 traces, `fixture-check` must accept the fixtures (and reject unknown/behavior-looking keys), and `rule-coverage` must confirm every rule ID maps — this ticket turns the 006/010 substrate into enforced CI checks.

## Architecture Check

1. Adding one match arm per tool (rather than a shared dynamic registry) follows the established per-tool convention — cleaner and lower-risk than refactoring all tools to a registry this gate. Authoring `RULE-COVERAGE.md` with its validator keeps the doc and its checker consistent.
2. No backwards-compatibility aliasing/shims — additive arms.
3. No engine-core/game-stdlib change; tools consume game ids/paths only.

## Verification Layers

1. Registration invariant -> codebase grep-proof: `column_four` arm present in all four tools' game match.
2. Replay-drift invariant -> deterministic replay-hash check: `replay-check --game column_four --all` passes for the GAT5COLFOUPUB-010 traces.
3. Fixture-validity invariant -> schema/serialization validation: `fixture-check --game column_four` passes (fail-closed; rejects unknown/behavior-looking keys).
4. Rule-coverage invariant -> codebase grep-proof + tool run: `rule-coverage --game column_four` confirms every `RULES.md` rule ID maps to impl/tests/traces/etc.
5. Simulation invariant -> simulation/CLI run: `simulate --game column_four --games 1000` runs to completion with no illegal-state failure.

## What to Change

### 1. Tool registration

Add a `column_four` arm to `tools/simulate/src/main.rs` (dispatch + a `run_column_four_simulation`), and `RegisteredGame` arms to `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, and `tools/rule-coverage/src/main.rs` with the correct `games/column_four/...` paths; update each tool's available-games help text.

### 2. `games/column_four/docs/RULE-COVERAGE.md`

From `templates/GAME-RULE-COVERAGE.md`: one row per `RULES.md` rule ID mapping to Rust impl, unit/rule tests, golden traces, replay checks, simulation, serialization, visibility, bot, UI smoke, and benchmark relevance; include the golden-trace catalog and explicit coverage callouts for vertical/horizontal/diagonal/draw/diagnostics/bot/terminal-replay/wasm-exported.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/column_four/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- `seed-reducer`/`trace-viewer` registration (race_to_n-only precedent, A-4).
- CI workflow wiring that runs these tools (GAT5COLFOUPUB-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game column_four --games 1000` — simulation completes.
2. `cargo run -p replay-check -- --game column_four --all && cargo run -p fixture-check -- --game column_four && cargo run -p rule-coverage -- --game column_four` — all three pass.
3. `cargo build --workspace` — tools compile.

### Invariants

1. Each tool gains an additive `column_four` arm; no existing game's arm changes.
2. `RULE-COVERAGE.md` maps every `RULES.md` rule ID; `rule-coverage` confirms it.

## Test Plan

### New/Modified Tests

1. `games/column_four/docs/RULE-COVERAGE.md` — coverage matrix validated by `tools/rule-coverage`.
2. Tool arms exercised by the commands below (no new unit test framework; the tools are the validators).

### Commands

1. `cargo run -p replay-check -- --game column_four --all`
2. `cargo run -p fixture-check -- --game column_four && cargo run -p rule-coverage -- --game column_four`
3. `cargo run -p simulate -- --game column_four --games 1000`
