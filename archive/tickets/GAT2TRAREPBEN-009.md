# GAT2TRAREPBEN-009: `simulate` machine-readable failure report

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — extends `tools/simulate` to emit a machine-readable failure report defining the `seed-reducer` input contract.
**Deps**: None

## Problem

`tools/simulate` already computes rich failure context but prints it only as human
text (`print!` / `eprint!`). The `seed-reducer` tool (GAT2TRAREPBEN-010) takes a
`--failure-report <path>` input, so `simulate` must first emit a parseable failure
report. This was surfaced as reassessment finding M3 and added to spec §D7 / §WB8 as
net-new work, not a parse-only task.

## Assumption Reassessment (2026-06-05)

1. `tools/simulate/src/main.rs` (≈524 lines, a real tool) computes failure fields
   (`seed`, `turn_index`, `actor`, `command_stream`, `state_hash`, `effect_hash`,
   `view_hash`, `failure_reason`) and supports `--inject-failure-seed` for injection,
   but emits them via `print!` / `eprint!` as human text — no machine-readable report.
2. Spec §D7 / §WB8 (as corrected by reassessment M3) require `simulate` to emit a
   machine-readable failure report whose schema is the `seed-reducer --failure-report`
   input contract. `docs/TESTING-REPLAY-BENCHMARKING.md §7` lists the canonical
   failure-output field set this report should carry.
3. Cross-crate boundary under audit: the failure-report schema is the contract
   consumed by `seed-reducer` (GAT2TRAREPBEN-010). It must carry enough context
   (seed/options/variant/command-stream/hashes/versions/failure-reason) to normalize
   into a replay command or Trace Schema v1 reproducer.
4. FOUNDATIONS §11 (determinism + no-leak): restate that the report is deterministic
   for a fixed seed and viewer-safe — `race_to_n` is perfect-information, so the
   report leaks no hidden information; the fields it carries are already public.
5. Schema/contract extension: this adds a new structured output to `simulate`. It is
   additive — existing human output and the `--games`/`--start-seed`/`--action-cap`/
   `--inject-failure-seed` flags are unchanged; a new flag (e.g. `--failure-report-out
   <path>` or a structured stdout mode) gates the report. No existing behavior is
   retconned; the rationale is reassessment M3 + spec §D7.

## Architecture Check

1. Reusing `simulate`'s already-computed failure struct (rather than re-deriving
   failure context in `seed-reducer`) keeps one source of failure truth and makes the
   reproducer faithful to what actually failed.
2. No backwards-compatibility shims; the report is a new additive output, not a
   rewrite of the human path.
3. `engine-core` untouched; the change is `tools/simulate`-local.

## Verification Layers

1. Report is machine-parseable → schema/serialization validation: the emitted report
   parses and carries every contract field.
2. Determinism → deterministic replay-hash check: a fixed injected-failure seed emits
   an identical report across runs.
3. No-leak → no-leak visibility test: the report contains only public fields
   (perfect-information game; nothing hidden to leak).

## What to Change

### 1. `tools/simulate/src/main.rs`

Add a machine-readable failure-report emission (a new `--failure-report-out <path>`
flag or an explicit structured-output mode) serializing the failure fields plus
game/rules/data/engine versions, options, and variant. Keep the existing human output
and flags unchanged.

## Files to Touch

- `tools/simulate/src/main.rs` (modify) — add machine-readable failure-report output

## Out of Scope

- `seed-reducer` itself (GAT2TRAREPBEN-010) — this ticket only emits the report it consumes.
- Trace Schema v1 reproducer generation (GAT2TRAREPBEN-010).
- Changing simulation rules, caps, or the existing human summary.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game race_to_n --inject-failure-seed <n> --failure-report-out <path>` — writes a parseable failure report carrying the contract fields.
2. The report parses and contains seed, options/variant, command stream, state/effect/view hashes, failure reason, and game/rules/data/engine versions.
3. `cargo run -p simulate -- --game race_to_n --games 1000` — the existing human-output smoke path is unchanged.

### Invariants

1. The failure report is deterministic for a fixed seed and contains only public fields (§11).
2. The report schema is the documented `seed-reducer --failure-report` input contract (spec §D7; reassess M3).

## Test Plan

### New/Modified Tests

1. `tools/simulate/src/main.rs` (or `tools/simulate/tests/`) — assert the emitted report parses and carries the required fields for an injected failure.

### Commands

1. `cargo run -p simulate -- --game race_to_n --inject-failure-seed <n> --failure-report-out /tmp/fail.json`
2. `cargo test -p simulate`
3. `cargo run -p simulate -- --game race_to_n --games 1000` — confirms the existing simulation smoke is unaffected.
