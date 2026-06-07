# GAT6DIRFLI-016: Tool registration & RULE-COVERAGE.md

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report,seed-reducer,trace-viewer}/src/main.rs` (accept `--game directional_flip`), and `games/directional_flip/docs/RULE-COVERAGE.md` (tool-validated doc).
**Deps**: 013, 014

## Problem

The seven Rulepath tools each accept a known set of game ids and must be extended deliberately (spec §13.1). This ticket registers `directional_flip` across all of them — simulate, replay-check, fixture-check, rule-coverage, bench-report, seed-reducer, trace-viewer — and authors `RULE-COVERAGE.md`, the matrix `tools/rule-coverage` validates (it must co-land with the rule-coverage registration so the tool has something valid to check). Realizes spec §8.9/§13.1 and rule id `DF-PRIM-001` coverage mapping.

## Assumption Reassessment (2026-06-07)

1. All seven tools exist with `src/main.rs` (confirmed: `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, `tools/bench-report`, `tools/seed-reducer`, `tools/trace-viewer`). They accept a `--game` value with a per-game match/registry; the `column_four` registration (GAT5COLFOUPUB-013 precedent) shows the pattern. `games/column_four/docs/RULE-COVERAGE.md` is the doc precedent.
2. Spec §13.1 (per-tool required updates) and §8.5/§8.6 (the traces/tests the coverage matrix maps to) are authoritative. `tools/rule-coverage` fails on gaps; the matrix must map every `DF-*` id to unit/property/golden/replay/simulation/UI evidence.
3. Cross-artifact boundary under audit: each tool's accepted-game-id list/dispatch ↔ `games/directional_flip` surfaces (replay traces from 013, benches from 014, fixture from 004, tests from 012). This is a multi-tool **dispatch blast-radius** change; enumerate each tool's game-id registration site before editing.
4. FOUNDATIONS §11 fail-closed validation motivates the rule-coverage piece: restate before authoring — `RULE-COVERAGE.md` validation is blocking and fails on silent gaps; the matrix must have no unmapped rule id. `fixture-check` likewise rejects unknown/behavior-looking static-data fields (§5/§11).

## Architecture Check

1. Registering all tools in one ticket (each a small match arm) keeps the tool surface consistent and lets a single review confirm the game is wired across the whole verification pipeline; the `RULE-COVERAGE.md` co-lands with its validator so registration has a valid target.
2. No backwards-compatibility shims; additive game-id registration.
3. `engine-core` untouched; tools consume game-local surfaces (§3).

## Verification Layers

1. Tool acceptance -> simulation/CLI run: each tool runs with `--game directional_flip` and produces expected output (simulate report, replay-check pass, fixture-check pass, rule-coverage pass, bench-report inclusion, seed-reducer normalize, trace-viewer render).
2. Rule-coverage fail-closed -> schema/serialization validation (`DF-PRIM-001` + all `DF-*`): `tools/rule-coverage --game directional_flip` fails on any unmapped id; the committed matrix passes.
3. Replay corpus gate -> deterministic replay-hash check: `tools/replay-check --game directional_flip --all` verifies the GAT6DIRFLI-013 corpus.
4. Trace-viewer grouped-flip display -> manual review: grouped flip children and pass actions render (spec §13.1).

## What to Change

### 1. Per-tool registration

Add `directional_flip` to the accepted `--game` values, help text, and dispatch in all seven tools; add per-tool smoke/tests for the new game path; ensure machine-readable failure output where relevant; ensure seed-reducer can normalize directional-flip simulation reports and trace-viewer can display grouped flip effects + pass actions (spec §13.1).

### 2. `RULE-COVERAGE.md`

Author `games/directional_flip/docs/RULE-COVERAGE.md` mapping every `DF-*` rule id to its unit/property/golden/replay/simulation/UI evidence, with no silent gaps; co-lands with the rule-coverage registration.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `tools/seed-reducer/src/main.rs` (modify)
- `tools/trace-viewer/src/main.rs` (modify)
- `games/directional_flip/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- CI workflow wiring (GAT6DIRFLI-019) — this ticket makes the tools accept the game; CI invokes them.
- The traces/benches/tests themselves (GAT6DIRFLI-012/013/014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game directional_flip --games 1000` — many random legal games with terminal/throughput report.
2. `cargo run -p replay-check -- --game directional_flip --all` and `cargo run -p fixture-check -- --game directional_flip` and `cargo run -p rule-coverage -- --game directional_flip` — all pass.
3. `cargo test --workspace` — tool smoke/tests pass.

### Invariants

1. `rule-coverage` is fail-closed on unmapped `DF-*` ids; `fixture-check` rejects unknown/behavior-looking fields (FOUNDATIONS §5/§11).
2. Every tool deliberately registers the new game id (no accidental wildcard acceptance) (spec §13.1).

## Test Plan

### New/Modified Tests

1. Per-tool smoke additions for the `directional_flip` path (mirroring the column_four tool tests).
2. `games/directional_flip/docs/RULE-COVERAGE.md` — the matrix `rule-coverage` validates.

### Commands

1. `cargo run -p rule-coverage -- --game directional_flip && cargo run -p replay-check -- --game directional_flip --all && cargo run -p fixture-check -- --game directional_flip`
2. `cargo run -p simulate -- --game directional_flip --games 1000 && cargo test --workspace`
3. Running the tool suite against the new game is the correct boundary; CI lane invocation is GAT6DIRFLI-019.

## Outcome

Completed: 2026-06-07

What changed:
- Registered `directional_flip` in `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, `bench-report`, `seed-reducer`, and `trace-viewer`.
- Added `games/directional_flip/docs/RULE-COVERAGE.md` with one row for every `DF-*` rule id in `RULES.md`.
- Extended replay-check and trace-viewer with Directional Flip custom fixture-state support for the golden traces that intentionally start outside the default opening.
- Extended seed-reducer to normalize Directional Flip command streams using Directional Flip replay hashes.
- Added bench-report `--game directional_flip` threshold-path registration.
- Raised the effective Directional Flip simulation cap to 128 actions so default `--games 1000` runs can reach terminal outcomes.

Deviations from original plan:
- Bench-report remained primarily file-based, but now accepts `--game directional_flip` to select and validate the registered threshold path.
- Trace-viewer renders the generic trace sections plus actual replay hash annotations; it does not add a bespoke visual grouped-flip renderer in this ticket.

Verification results:
- `cargo run -p rule-coverage -- --game directional_flip` passed.
- `cargo run -p replay-check -- --game directional_flip --all` passed all 14 Directional Flip traces.
- `cargo run -p fixture-check -- --game directional_flip` passed.
- `cargo run -p simulate -- --game directional_flip --games 1000` passed with 1000 terminal games.
- `cargo run -p seed-reducer -- --game directional_flip --seed 14 --commands seat_0:place/r3c4,seat_1:place/r3c5` passed.
- `cargo run -p trace-viewer -- --game directional_flip --trace games/directional_flip/tests/golden_traces/forced-pass.trace.json` passed.
- `cargo bench -p directional_flip` plus `cargo run -p bench-report -- --game directional_flip --input /tmp/directional_flip_full_bench.json` passed all 10 operations.
- `cargo fmt --all --check` passed.
- `cargo test --workspace` passed.
