# GAT2TRAREPBEN-007: Implement `tools/bench-report` threshold gate

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — replaces the `tools/bench-report` no-op placeholder with a real fail-closed benchmark-report validator that hard-fails threshold misses.
**Deps**: GAT2TRAREPBEN-006

## Problem

`tools/bench-report` is a 16-line no-op, so benchmark output cannot gate CI. Gate 2
requires a real tool that parses the structured benchmark JSON, rejects malformed or
incomplete reports, compares every required operation to explicit thresholds, and
hard-fails accepted floors and unapproved regressions (spec §D5; user decision that
CI benchmark gates hard-fail).

## Assumption Reassessment (2026-06-05)

1. `tools/bench-report/src/main.rs` is a 16-line no-op; the crate is in the workspace
   `Cargo.toml` `members` list. The structured benchmark JSON + `thresholds.json`
   it consumes are produced by GAT2TRAREPBEN-006.
2. Spec §D5 fixes the required `bench-report` behavior (parse JSON; reject malformed;
   reject missing required metadata; reject missing required operations; compare every
   required operation to explicit thresholds; hard-fail floors + unapproved
   regressions; print a concise failure report). The accepted Stage-1 `random_playout`
   threshold value is finalized by GAT2TRAREPBEN-008; this tool enforces whatever the
   thresholds file declares.
3. Cross-crate boundary under audit: `bench-report` consumes the JSON report schema +
   `thresholds.json` contract from GAT2TRAREPBEN-006; the operation-name set must
   match exactly between producer and consumer.
4. FOUNDATIONS §11 (fail-closed, deterministic validation): restate that the gate must
   be deterministic and blocking — a missing/malformed/regressed report exits non-zero,
   never a warning-only pass. This is the §D5 "no report-only mode for required gates"
   rule.

## Architecture Check

1. A standalone parser/gate (rather than baking thresholds into the bench harness)
   keeps measurement (006) and policy enforcement (007) separable — thresholds can be
   reviewed and recalibrated without touching measurement code.
2. No backwards-compatibility shims; there is no report-only fallback for required
   thresholds.
3. `engine-core` untouched; `bench-report` reasons over JSON, adding no kernel noun.

## Verification Layers

1. Threshold hard-fail → benchmark check: a report with a regressed/below-floor
   operation exits non-zero with operation, current value, threshold, rationale,
   and environment caveat.
2. Malformed/missing-metadata/missing-operation rejection → schema/serialization
   validation: each negative input exits non-zero.
3. Valid-report pass → CLI run: a conformant report passes.

## What to Change

### 1. CLI parsing (`tools/bench-report/src/main.rs`)

Implement `--input <report>` and `--thresholds <thresholds>`.

### 2. Validation + comparison

Parse the benchmark JSON; reject malformed output, missing required metadata, and
missing required operations; compare every required operation to its threshold; hard
-fail accepted floors and unapproved regressions; print a concise failure report
(operation, current value, threshold, rationale, environment caveat). Exit non-zero
on any failure.

### 3. `tools/bench-report/Cargo.toml`

Add the JSON-parse dependency.

## Files to Touch

- `tools/bench-report/src/main.rs` (modify) — replace no-op with the real gate
- `tools/bench-report/Cargo.toml` (modify) — add JSON-parse dep

## Out of Scope

- Emitting the benchmark JSON / `thresholds.json` (GAT2TRAREPBEN-006).
- Deciding the Stage-1 `random_playout` threshold value (GAT2TRAREPBEN-008).
- CI wiring (GAT2TRAREPBEN-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p bench-report -- --input <valid report> --thresholds games/race_to_n/benches/thresholds.json` — passes a conformant report.
2. A report with a regressed/below-floor operation exits non-zero with the required failure fields.
3. A report missing required metadata, and one missing a required operation, each exit non-zero.

### Invariants

1. The gate is deterministic and blocking; there is no report-only mode for required thresholds (§11; §D5).
2. Required-operation coverage is enforced — a dropped operation is a failure, not a silent pass.

## Test Plan

### New/Modified Tests

1. `tools/bench-report/src/main.rs` (or `tools/bench-report/tests/`) — pass, regressed-fail, malformed-fail, missing-metadata-fail, missing-operation-fail.
2. Small JSON report fixtures (valid + each negative case).

### Commands

1. `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json`
2. `cargo test -p bench-report`
3. A targeted negative-fixture run is the correct boundary; full-pipeline benchmark gating is exercised in CI (GAT2TRAREPBEN-013).
