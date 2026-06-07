# GAT7DRALITCOM-017: Tool registration & RULE-COVERAGE.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate/src/main.rs`, `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs`, `tools/bench-report/src/main.rs` (register `draughts_lite`), `games/draughts_lite/docs/RULE-COVERAGE.md` (new).
**Deps**: 014, 015

## Problem

The per-game tools must enumerate `draughts_lite` so the standard verification commands run. This ticket adds the game's match arms to `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, and `bench-report`, and authors `RULE-COVERAGE.md` (co-located with the rule-coverage tool that consumes it). Because Gate 7 omits draw adjudication, `simulate` must distinguish terminal games, bounded nonterminal games that hit the action cap (a valid smoke result), and true failures — without inventing a fake draw rule.

## Assumption Reassessment (2026-06-07)

1. `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}/src/main.rs` all carry per-game dispatch (verified: `simulate`/`replay-check`/`fixture-check`/`rule-coverage`/`bench-report` reference `directional_flip`/`column_four`). `tools/rule-coverage` reads the game's `RULES.md`/`RULE-COVERAGE.md`/`BENCHMARKS.md`, so `RULE-COVERAGE.md` must co-land here. `tools/seed-reducer` and `tools/trace-viewer` are game-opt-in (only `race_to_n` + `directional_flip`) and are NOT required for this gate (spec §R23 / Assumption A-4).
2. The tool contract is fixed by spec §R23 "Crate, workspace, and registration" (simulate/replay-check/fixture-check/rule-coverage/bench-report enumerate the game) and §R18 "Simulation tool" (distinguish terminal / bounded-nonterminal-at-cap / true-failure; no fake draw rule) and "Rule coverage tool" (recognize Draughts Lite's major clauses). `RULE-COVERAGE.md` maps clauses → tests/traces (GAT7DRALITCOM-013/014).
3. Cross-artifact boundary under audit: the tools consume the golden traces + fixture (014), the benches (015), and the rules docs (001); `RULE-COVERAGE.md` is the tool-validated doc that `rule-coverage` checks. The CI workflows (GAT7DRALITCOM-020) invoke these tools per game.
4. FOUNDATIONS §6/§11 motivate this ticket: restate before coding — official games carry CLI simulation, replay, fixture, and rule-coverage evidence; the simulation runner must not fabricate a draw rule to force termination (spec §R18) — a bounded nonterminal smoke is a valid, non-failing result.
5. Determinism enforcement surface (§11): `replay-check --all` re-validates every golden trace's deterministic hashes; `simulate` runs deterministic seeded games. Confirm the simulate failure taxonomy treats only invalid bot action / replay drift / panic / invariant violation as failures (not action-cap nonterminal).

## Architecture Check

1. Adding per-game match arms (the established registration pattern) is the minimal, convention-consistent wiring; a draw-adjudication-free simulate taxonomy correctly models the ruleset without polluting rules with a fake draw.
2. No backwards-compatibility shims; new match arms + doc.
3. `engine-core` stays noun-free (§3); tools dispatch on the generic game id and call game-local surfaces.

## Verification Layers

1. simulate -> `cargo run -p simulate -- --game draughts_lite --games 1000`: runs, reporting terminal vs bounded-nonterminal-at-cap vs failure correctly (no false failure on legal cycles).
2. replay-check -> `cargo run -p replay-check -- --game draughts_lite --all`: every golden trace (incl. WASM-exported) validates.
3. fixture-check -> `cargo run -p fixture-check -- --game draughts_lite`: the standard fixture validates.
4. rule-coverage -> `cargo run -p rule-coverage -- --game draughts_lite`: recognizes the major rule clauses; `RULE-COVERAGE.md` maps clauses → coverage.
5. bench-report -> recognizes `draughts_lite` and reads `thresholds.json`.

## What to Change

### 1. Tool match arms

Register `draughts_lite` in `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, and `bench-report` following the `directional_flip` arms. Implement the simulate failure taxonomy (terminal / bounded-nonterminal-at-cap / true-failure).

### 2. RULE-COVERAGE.md

Author `docs/RULE-COVERAGE.md` from `templates/GAME-RULE-COVERAGE.md`: map each major rule clause to its test(s)/trace(s)/tool coverage.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `games/draughts_lite/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- CI workflow steps invoking these tools (GAT7DRALITCOM-020).
- `seed-reducer` / `trace-viewer` support (optional, not required this gate; spec A-4).
- Authoring traces/fixtures/benches (GAT7DRALITCOM-014/015; consumed here).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game draughts_lite --games 1000` — runs without false failure on nonterminal action caps.
2. `cargo run -p replay-check -- --game draughts_lite --all` && `cargo run -p fixture-check -- --game draughts_lite` && `cargo run -p rule-coverage -- --game draughts_lite` — all pass.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. The simulation runner distinguishes bounded-nonterminal from failure and invents no draw rule (FOUNDATIONS §6; spec §R18).
2. Rule coverage recognizes the major clauses and maps them to evidence (FOUNDATIONS §6; spec §R18).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/docs/RULE-COVERAGE.md` — clause→coverage map (validated by `rule-coverage`).
2. `None additional — verification is the per-tool CLI runs below over the artifacts from GAT7DRALITCOM-013/014/015.`

### Commands

1. `cargo run -p simulate -- --game draughts_lite --games 1000`
2. `cargo run -p replay-check -- --game draughts_lite --all && cargo run -p fixture-check -- --game draughts_lite && cargo run -p rule-coverage -- --game draughts_lite`
3. The per-game CLI runs are the correct boundary; CI wires them into the gate workflows in GAT7DRALITCOM-020.
