# GAT72GAT8HIG-013: Native tool registration + RULE-COVERAGE.md

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate/src/main.rs`, `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs`; `games/high_card_duel/docs/RULE-COVERAGE.md`
**Deps**: GAT72GAT8HIG-002, GAT72GAT8HIG-010, GAT72GAT8HIG-012

## Problem

The four native verification tools resolve games through hardcoded registries,
not by path. `high_card_duel` must be registered in each so the gate's
acceptance commands (`simulate --game high_card_duel …`, `replay-check --game
high_card_duel`, `fixture-check --game high_card_duel`, `rule-coverage --game
high_card_duel`) run, and `rule-coverage` needs a `RULE-COVERAGE.md` to validate.

## Assumption Reassessment (2026-06-07)

1. Verified the registries are closed and game-keyed:
   - `tools/simulate/src/main.rs:122,179` — `--game` dispatch + usage string list
     `race_to_n|three_marks|column_four|directional_flip|draughts_lite`.
   - `tools/replay-check/src/main.rs:60-91` — `RegisteredGame` structs with a
     `trace_dir` per game; `--game`/`--all`/`--trace` (lines 135-150).
   - `tools/fixture-check/src/main.rs:130,218,240` — `RegisteredGame` + `--game`.
   - `tools/rule-coverage/src/main.rs:26,87` — `RegisteredGame` + `--game`.
2. Verified against the spec: §4.2.13 lists exactly these four tool registries
   as a concrete deliverable, and §4.2.12 routes `RULE-COVERAGE.md` to co-land
   with the rule-coverage arm (tool-validated doc). `seed-reducer`/`trace-viewer`
   are selectively wired and out of scope (spec §4.2.13 note).
3. Cross-artifact boundary under audit: the per-tool game-registry contract and
   the rule-coverage doc contract (`tools/rule-coverage` reads `RULES.md` +
   `RULE-COVERAGE.md`). The `trace_dir` for `replay-check` is
   `games/high_card_duel/tests/golden_traces` (created in 012).
4. Schema/contract extension classification: each registry gains one additive
   game arm; consumers are the tool CLIs themselves. Additive-only — no existing
   game arm changes. `RULE-COVERAGE.md` is the rule-to-test matrix consumed by
   `rule-coverage` and must cite the exact `HCD-*` IDs from RULES.md (002) and
   the tests from 004–012.

## Architecture Check

1. Registering in each tool's existing `RegisteredGame` pattern (one arm each)
   matches convention and keeps the tools' closed-registry design intact —
   cleaner than a path-based special case for one game.
2. No backwards-compatibility shims — additive registry arms.
3. `engine-core`/`game-stdlib` untouched; tools depend on the game crate.

## Verification Layers

1. Tool resolution -> simulation/CLI run: each tool runs with `--game high_card_duel` and exits success.
2. Rule coverage -> simulation/CLI run: `rule-coverage --game high_card_duel` validates the matrix against `RULES.md`.
3. Trace/fixture validation -> deterministic replay-hash + schema validation: `replay-check --game high_card_duel` and `fixture-check --game high_card_duel` pass over the 012 artifacts.
4. Registry additivity -> codebase grep-proof: a new arm per tool; no existing game arm altered.

## What to Change

### 1. Tool registries

Add a `high_card_duel` arm to each of the four tools' registries (and the
`simulate` usage string), with `replay-check`'s `trace_dir` =
`games/high_card_duel/tests/golden_traces`.

### 2. `RULE-COVERAGE.md`

Author the rule-to-test matrix from `templates/GAME-RULE-COVERAGE.md`, mapping
every `HCD-*` rule ID to its covering test(s).

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/high_card_duel/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- CI workflow steps (gate-1 e2e → 019; gate-2 bench → 014).
- WASM catalog registration (016).
- `seed-reducer`/`trace-viewer` (selectively wired; not required this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game high_card_duel` — passes.
2. `cargo run -p replay-check -- --game high_card_duel` and `cargo run -p fixture-check -- --game high_card_duel` — pass.
3. `cargo run -p simulate -- --game high_card_duel --games 100 --start-seed 1` — completes with all-legal play.

### Invariants

1. Each registry gains exactly one additive arm; no existing game's behavior changes.
2. `RULE-COVERAGE.md` maps every `HCD-*` ID to a test (no uncovered rule).

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/docs/RULE-COVERAGE.md` — rule-to-test matrix (validated by the tool).

### Commands

1. `cargo run -p rule-coverage -- --game high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel && cargo run -p fixture-check -- --game high_card_duel`
3. The four CLI runs are the correct boundary — they exercise the registries end-to-end against the 012 artifacts.

## Outcome (2026-06-07)

Registered `high_card_duel` in the native verification tools and added the rule-coverage evidence:

1. Added additive `high_card_duel` registry arms for `simulate`, `replay-check`, `fixture-check`, and `rule-coverage`.
2. Added the tool dependencies needed by `simulate`, `replay-check`, and `fixture-check`.
3. Added High Card Duel replay-check hash/diagnostic handling for the ten golden traces from GAT72GAT8HIG-012.
4. Added `games/high_card_duel/docs/RULE-COVERAGE.md` with one row per `HCD-*` rule ID.
5. Added a minimal `games/high_card_duel/docs/BENCHMARKS.md` placeholder so `rule-coverage` can validate its required benchmark-doc input; measured benchmarks remain GAT72GAT8HIG-014.

Deviations: `replay-check --game <game>` now defaults to the registered trace directory when no explicit `--trace`, `--directory`, or `--all` mode is supplied, matching the Gate 8 acceptance command while preserving explicit modes.

Verification:

1. `cargo run -p rule-coverage -- --game high_card_duel` — passed.
2. `cargo run -p replay-check -- --game high_card_duel` — passed.
3. `cargo run -p fixture-check -- --game high_card_duel` — passed.
4. `cargo run -p simulate -- --game high_card_duel --games 100 --start-seed 1` — passed; 100 games, average length 12.00.
5. `cargo fmt --all --check` — passed.
6. `cargo test -p replay-check -p fixture-check -p simulate -p rule-coverage` — passed.
