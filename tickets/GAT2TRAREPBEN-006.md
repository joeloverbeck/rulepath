# GAT2TRAREPBEN-006: Structured benchmark JSON output + thresholds file

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends the existing `games/race_to_n` custom benchmark harness to emit a structured JSON report plus human summary, and adds a no-YAML thresholds file.
**Deps**: GAT2TRAREPBEN-001

## Problem

The `race_to_n` benchmark harness prints a human-readable table to stdout with no
machine-readable report and no threshold file, so nothing downstream can gate on it.
Spec §D5 keeps the existing custom harness but requires structured JSON output with
required metadata and an explicit, rationale-bearing thresholds file — the inputs the
`bench-report` gate (GAT2TRAREPBEN-007) consumes.

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/benches/race_to_n.rs` is a custom harness (`harness = false`,
   `fn main()`, a `Measurement { name, unit, iterations, elapsed }` struct and a
   `measure(...)` table) covering `legal_actions`, `apply_action`,
   `public_view_generation`, `effect_filtering`, `serialization_roundtrip`, replay,
   `random_playout`, and bot latency. It prints a table; no JSON is emitted. No
   `games/race_to_n/benches/thresholds.json` exists (verified absent).
2. Spec §D5 lists the required JSON metadata + operation rows and the threshold-policy
   rules (every threshold needs a rationale: foundation target / measured baseline /
   conservative CI floor / accepted ADR). Benchmark hard-fail doctrine is recorded in
   `docs/TESTING-REPLAY-BENCHMARKING.md §14–§16` and GAT2TRAREPBEN-001.
   `BENCHMARKS.md` already records `random_playout` ≈ 134,277 games/sec vs the
   500,000 Stage-1 budget — the **value** of that threshold's resolution is owned by
   GAT2TRAREPBEN-008; this ticket establishes the file and current measurements.
3. Cross-artifact boundary under audit: the JSON report schema + `thresholds.json`
   are the contract `bench-report` (007) parses. The operation names must be stable
   and match between producer and consumer.
4. FOUNDATIONS §11 (benchmarks are deterministic, evidence-backed): restate that the
   report records hardware/OS/Rust/profile/version metadata and known caveats so a
   noisy WSL2/CI run is interpretable, not a silent waiver.

## Architecture Check

1. Extending the existing custom harness (rather than migrating to Criterion /
   Iai-Callgrind) is exactly spec §D5's stance — the harness already measures the
   right operations; only structured output + a threshold file are missing.
2. No backwards-compatibility shims; the human table remains as the summary view
   alongside the new JSON, not as a second source of truth.
3. `engine-core` untouched; the benchmark + threshold file live under
   `games/race_to_n`.

## Verification Layers

1. JSON report validity → schema/serialization validation: emitted JSON parses and
   carries every required metadata field + operation row.
2. Threshold rationale present → manual review: each row in `thresholds.json` names
   its rationale class (foundation target / measured baseline / CI floor / ADR).
3. Stable operation names → codebase grep-proof: the JSON operation names match the
   set `bench-report` (007) expects.

## What to Change

### 1. `games/race_to_n/benches/race_to_n.rs`

Emit a structured JSON report (machine report + retained human summary) with the
required metadata (hardware/env notes, OS, Rust version, command, build profile,
game ID, rules version, data version, engine version) and per-operation rows
(operation name, iterations, unit, current value, threshold, pass/fail, caveats).

### 2. `games/race_to_n/benches/thresholds.json`

Add a no-YAML threshold file: one entry per required operation with its threshold
value and rationale class. The `random_playout` row records the current measured
value and is marked as the Stage-1 budget under resolution (finalized by
GAT2TRAREPBEN-008).

## Files to Touch

- `games/race_to_n/benches/race_to_n.rs` (modify) — emit JSON + human summary
- `games/race_to_n/benches/thresholds.json` (new) — per-threshold rationale file

## Out of Scope

- The `bench-report` parser/gate (GAT2TRAREPBEN-007).
- The Stage-1 `random_playout` triage + final accepted threshold (GAT2TRAREPBEN-008).
- `BENCHMARKS.md` narrative finalization (GAT2TRAREPBEN-008 / -014).
- Criterion / Iai-Callgrind migration (spec §D5 out-of-scope by default).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p race_to_n` — emits valid JSON with all required metadata + operation rows, plus the human summary.
2. The emitted JSON parses and every required operation name is present.
3. `cat games/race_to_n/benches/thresholds.json` — every threshold entry carries a rationale class; no YAML is introduced.

### Invariants

1. Benchmark output is structured, version-stamped, and caveat-bearing (§11; TESTING §16).
2. Operation names are stable and shared with `bench-report` (§D5).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/benches/race_to_n.rs` — JSON serialization of the report (an in-harness or unit assertion that the report round-trips and lists required operations).
2. `games/race_to_n/benches/thresholds.json` — the threshold fixture itself.

### Commands

1. `cargo bench -p race_to_n`
2. `cargo bench -p race_to_n -- legal_actions` — single-case smoke still emits the structured report.
3. A narrower per-operation invocation is the correct boundary for a fast CI smoke; the full set runs in the benchmark workflow (GAT2TRAREPBEN-013).
