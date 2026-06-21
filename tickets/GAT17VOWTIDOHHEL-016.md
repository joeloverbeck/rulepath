# GAT17VOWTIDOHHEL-016: replay-check + rule-coverage registration and RULE-COVERAGE.md

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — modifies `tools/replay-check/src/main.rs`, `tools/rule-coverage/src/main.rs`; new `games/vow_tide/docs/RULE-COVERAGE.md`
**Deps**: 011, 015

## Problem

Register `vow_tide` in the hard-coded `replay-check` and `rule-coverage` registries and author `RULE-COVERAGE.md`, mapping every `VT-*` rule to its unit/rule/property/trace/simulation/replay/serialization/visibility/UI evidence. `rule-coverage` must fail on undocumented or unproved rules; `replay-check` must validate the golden-trace pack deterministically.

## Assumption Reassessment (2026-06-21)

1. `tools/replay-check/src/main.rs` (`resolve_game()` at `:70`) and `tools/rule-coverage/src/main.rs` (`resolve_game()` at `:33`) are both hard-coded registries (the reassessment-resolved finding); the `briar_circuit` arms (replay-check `:147` → `game_id`/`rules_version`/`trace_dir`; rule-coverage `:140` → `rules_path`/`coverage_path`/`benchmarks_path`/`benchmarks_required`) are the templates.
2. The 011 golden-trace pack lives at `games/vow_tide/tests/golden_traces/`; `RULES.md` (001) supplies the `VT-*` IDs; this ticket authors `RULE-COVERAGE.md` mapping them.
3. Cross-artifact boundary under audit: `rule-coverage` reads `RULES.md` + `RULE-COVERAGE.md` + `BENCHMARKS.md`, but `BENCHMARKS.md` lands with the benches ticket (020) — so fully-green `rule-coverage --game vow_tide` depends on 020; flag the partial-green window.
4. FOUNDATIONS §6/§11 under audit: official games need rule coverage + deterministic replay; the reassessment-resolved hard-coded-registry finding is the change rationale.

## Architecture Check

1. Pairing the two trace/coverage tool arms with `RULE-COVERAGE.md` keeps each validator's target and registration in one diff, the official-game-pattern tool-validated-docs placement.
2. No shims; additive registry arms.
3. `engine-core`/`game-stdlib` untouched.

## Verification Layers

1. Golden-trace pack replays deterministically → `cargo run -p replay-check -- --game vow_tide --all`.
2. Every `VT-*` rule mapped to proof; undocumented/unproved rules fail → `cargo run -p rule-coverage -- --game vow_tide` (fully green after 020 lands `BENCHMARKS.md`).
3. Coverage matrix one row per rule, no broad-match substitution → manual review of `RULE-COVERAGE.md`.

## What to Change

### 1. replay-check arm

Add the `vow_tide` `resolve_game()` arm (`game_id`, `rules_version="vow-tide-rules-v1"`, `trace_dir="games/vow_tide/tests/golden_traces"`) + any bot-match trace handling if vow_tide traces use the per-seed format.

### 2. rule-coverage arm + RULE-COVERAGE.md

Add the `vow_tide` arm (`rules_path`, `coverage_path`, `benchmarks_path`, `benchmarks_required`). Author `RULE-COVERAGE.md` mapping each Appendix A `VT-*` rule to its direct evidence.

## Files to Touch

- `tools/replay-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/vow_tide/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- `BENCHMARKS.md` (020) — `rule-coverage` is partial-green until it lands.
- `simulate`/`fixture-check` arms (013/015); WASM/web (017/018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game vow_tide --all` — all golden traces deterministic.
2. `cargo run -p rule-coverage -- --game vow_tide` — every `VT-*` rule mapped (fully green once 020 lands `BENCHMARKS.md`).
3. `cargo build -p replay-check -p rule-coverage`.

### Invariants

1. No `VT-*` rule is undocumented or proved only by a broad full-match trace.
2. Replay determinism holds across the whole pack.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/docs/RULE-COVERAGE.md` — the per-rule evidence matrix (validated by the tool).

### Commands

1. `cargo run -p replay-check -- --game vow_tide --all`
2. `cargo run -p rule-coverage -- --game vow_tide`
3. Narrower command rationale: the two tools are the trace/coverage boundary; the partial-green window on `BENCHMARKS.md` closes at 020.
