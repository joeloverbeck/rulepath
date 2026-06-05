# GAT2TRAREPBEN-013: CI hard-fail wiring for Gate 2 checks

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — extends `.github/workflows/ci.yml` to run the Gate 2 hardening tools as hard-failing gates while keeping the Gate 0/1 smoke checks.
**Deps**: GAT2TRAREPBEN-004, GAT2TRAREPBEN-005, GAT2TRAREPBEN-007, GAT2TRAREPBEN-008, GAT2TRAREPBEN-011

## Problem

CI currently runs the Gate 0/1 smoke set but none of the Gate 2 hardening checks.
Gate 2 requires CI to hard-fail on replay-check drift, fixture-check rejections,
rule-coverage drift, and benchmark threshold misses — so regressions cannot land
green (spec §D10; user decision that benchmark gates hard-fail).

## Assumption Reassessment (2026-06-05)

1. `.github/workflows/ci.yml` runs a single `smoke` job: `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace`,
   `cargo test --workspace`, `cargo test -p race_to_n`,
   `cargo run -p simulate -- --game race_to_n --games 1000`,
   `cargo bench -p race_to_n -- legal_actions`, `bash scripts/boundary-check.sh`,
   `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run build`,
   `npm --prefix apps/web run smoke:ui`, `node scripts/check-doc-links.mjs`. It does
   NOT run replay-check, fixture-check, rule-coverage, or a bench-report threshold gate.
2. Spec §D10 / §WB11 list the required CI additions and the hard-fail rule, and allow
   splitting benchmark smoke vs full workflows provided a hard-failing benchmark
   threshold check exists somewhere explicit and required. The real tools come from
   GAT2TRAREPBEN-004 (replay-check), -005 (fixture-check), -007 (bench-report), -011
   (rule-coverage); the accepted Stage-1 threshold comes from -008.
3. Cross-artifact boundary under audit: this ticket orchestrates the tool crates into
   CI; it adds no production logic. `seed-reducer` (-010) and `trace-viewer` (-012) are
   triage tools the spec §D10 list does not gate on — they are exercised at the
   capstone (-015), not here.
4. FOUNDATIONS §11 / §12 (fail-closed, no weakening tests): restate that every added
   check must hard-fail (non-zero), there is no report-only mode for required gates,
   and no existing Gate 1 smoke check is removed or weakened to get green CI.
5. §11 enforcement surface: the benchmark threshold gate
   (`bench-report --input … --thresholds …`) is the surface that makes a Stage-1
   regression a hard failure. Confirm it runs in a required job (main or a required
   benchmark workflow) and blocks merge.

## Architecture Check

1. Wiring the real tools (rather than re-running only in-crate tests) is exactly the
   Gate 2 contract — CI gates on the same CLIs a developer runs locally.
2. No backwards-compatibility shims; no report-only fallback for required gates.
3. `engine-core` untouched; this is workflow YAML orchestration only.

## Verification Layers

1. Drift gating → golden-trace / replay-hash check: CI fails on a corrupted trace hash
   (replay-check non-zero).
2. Fixture + coverage gating → schema validation + CLI run: CI fails on a malformed
   fixture (fixture-check) and on rule-coverage drift.
3. Benchmark gating → benchmark check: CI fails on a `random_playout` threshold miss
   (bench-report non-zero).
4. Regression safety → manual review: the existing fmt/clippy/test/sim/boundary/WASM/
   web/UI/docs steps remain present and unweakened.

## What to Change

### 1. `.github/workflows/ci.yml`

Add hard-failing steps for `cargo run -p replay-check -- --game race_to_n --all`,
`cargo run -p fixture-check -- --game race_to_n`,
`cargo run -p rule-coverage -- --game race_to_n`, and the bench-report threshold gate
(`cargo bench -p race_to_n` producing the report, then
`cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json`).
Decide whether the benchmark threshold gate runs in the main job or a required
benchmark workflow; keep all existing Gate 0/1 smoke steps.

## Files to Touch

- `.github/workflows/ci.yml` (modify) — add Gate 2 hard-fail gates; keep Gate 1 smokes

## Out of Scope

- The tool implementations themselves (GAT2TRAREPBEN-004/005/007/008/011).
- `seed-reducer` / `trace-viewer` CI gating (not in the §D10 list; exercised at the capstone).
- Hosted services or benchmark dashboards (§D10 non-goals).
- Final docs/index updates (GAT2TRAREPBEN-014 / -015).

## Acceptance Criteria

### Tests That Must Pass

1. CI passes on the valid Gate 2 evidence set (all added gates green).
2. A corrupted trace hash, a malformed fixture, a rule-coverage drift, and a benchmark threshold miss each make CI fail (verified by inducing each locally via the gating commands, then reverting).
3. The existing Gate 0/1 smoke steps still run and pass.

### Invariants

1. Every added Gate 2 check hard-fails; no report-only mode for required gates (§11; §D10).
2. No existing Gate 1 smoke check is removed or weakened (§12; §Forbidden changes).

## Test Plan

### New/Modified Tests

1. `None — workflow-wiring ticket; verification is the gating commands run in CI and locally (named in Acceptance Criteria).`

### Commands

1. `cargo run -p replay-check -- --game race_to_n --all && cargo run -p fixture-check -- --game race_to_n && cargo run -p rule-coverage -- --game race_to_n`
2. `cargo bench -p race_to_n && cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json`
3. Re-running the full `ci.yml` step set locally is the correct full-pipeline boundary; the GitHub-hosted run is the authoritative gate.
