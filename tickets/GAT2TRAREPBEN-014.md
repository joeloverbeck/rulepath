# GAT2TRAREPBEN-014: Cross-cutting docs finalization

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (docs) — finalizes `RULE-COVERAGE.md`, `BENCHMARKS.md`, and `TESTING-REPLAY-BENCHMARKING.md` once all Gate 2 tool surfaces exist. No code.
**Deps**: GAT2TRAREPBEN-003, GAT2TRAREPBEN-004, GAT2TRAREPBEN-005, GAT2TRAREPBEN-007, GAT2TRAREPBEN-008, GAT2TRAREPBEN-010, GAT2TRAREPBEN-011, GAT2TRAREPBEN-012

## Problem

The game and doctrine docs must be updated atomically once the Gate 2 tools exist:
`RULE-COVERAGE.md` needs evidence rows for the new tools, `BENCHMARKS.md` needs the
structured-report format + Stage-1 decision, and `TESTING-REPLAY-BENCHMARKING.md`
needs its Trace Schema v1 + benchmark hard-fail doctrine finalized. Doing this in one
docs ticket avoids a staleness window where docs reference surfaces that do not yet
exist (spec §10; §WB12).

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/docs/RULE-COVERAGE.md` carries the evidence-files table + rule
   matrix; `games/race_to_n/docs/BENCHMARKS.md` records the current benchmark report
   and the unresolved Stage-1 miss; `docs/TESTING-REPLAY-BENCHMARKING.md` carries the
   trace/benchmark doctrine that GAT2TRAREPBEN-001 began updating. The tool surfaces
   these docs cite are produced by tickets 003/004/005/007/008/010/011/012.
2. Spec §10 (Documentation updates required) and §WB12 name exactly these doc surfaces;
   the `/reassess-spec` session already pinned the `rules_version` canonical form (M1),
   the orphan-trace disposition (M2), the seed-reducer/simulate contract (M3), the
   TESTING §3 field-name reconciliation (M4 — begun in 001), and the SHA verification
   (M5).
3. Cross-artifact boundary under audit: each docs surface independently references a
   tool/decision, so it can go stale independently — hence the per-surface `Deps`
   enumeration rather than a single transitive head. This ticket does NOT flip the
   `specs/README.md` index or the spec Status (GAT2TRAREPBEN-015 owns the index/Done
   flip after exit evidence passes).

## Architecture Check

1. A single trailing docs ticket (rather than co-locating each doc edit with its tool
   ticket) is the right shape here because `RULE-COVERAGE.md`/`BENCHMARKS.md` must
   reference the full coherent tool set at once; per-ticket edits would each ship a
   partially-true doc.
2. No backwards-compatibility shims; docs describe the landed end state.
3. `engine-core` untouched; Markdown only.

## Verification Layers

1. Tool-evidence rows present → codebase grep-proof: `RULE-COVERAGE.md` cites
   replay-check, fixture-check, rule-coverage, trace-viewer, bench-report, seed-reducer,
   and the Stage-1 resolution.
2. Benchmark doctrine current → manual review: `BENCHMARKS.md` records the structured
   report format, thresholds, CI caveats, current values, and the Stage-1 decision
   (consistent with GAT2TRAREPBEN-008).
3. Doc-link integrity → docs link check: `node scripts/check-doc-links.mjs` passes;
   TESTING points at the canonical Trace Schema v1 doc.

## What to Change

### 1. `games/race_to_n/docs/RULE-COVERAGE.md`

Add/refresh evidence rows for replay-check, fixture-check, rule-coverage, trace-viewer,
bench-report, seed-reducer, and the Stage-1 budget resolution.

### 2. `games/race_to_n/docs/BENCHMARKS.md`

Record the structured benchmark report format, the thresholds and their rationale, CI
caveats, the current values, the Stage-1 triage decision, and the accepted benchmark
command (consistent with GAT2TRAREPBEN-006/-008).

### 3. `docs/TESTING-REPLAY-BENCHMARKING.md`

Finalize the Trace Schema v1 + benchmark hard-fail doctrine (the canonical-doc link and
§3 reconciliation began in GAT2TRAREPBEN-001; this confirms the end state is coherent
with the shipped tools).

## Files to Touch

- `games/race_to_n/docs/RULE-COVERAGE.md` (modify) — tool + Stage-1 evidence rows
- `games/race_to_n/docs/BENCHMARKS.md` (modify) — structured report format + Stage-1 decision
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify) — finalize trace/benchmark doctrine

## Out of Scope

- Any tool/code implementation (owned by the dependency tickets).
- `specs/README.md` index flip and the spec Status → Done flip (GAT2TRAREPBEN-015).
- `games/race_to_n/docs/UI.md` — not applicable; Gate 2 changes no web smoke/replay surface.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes after the doc edits.
2. `cargo run -p rule-coverage -- --game race_to_n` — passes against the updated `RULE-COVERAGE.md` (the new evidence rows are consistent, not drift).
3. `grep -l "bench-report\|replay-check\|seed-reducer" games/race_to_n/docs/RULE-COVERAGE.md` — the new tool-evidence rows are present.

### Invariants

1. Docs describe the landed Gate 2 end state; no doc references a surface that does not exist (§6 evidence completeness).
2. The Stage-1 decision recorded in `BENCHMARKS.md` matches the threshold enforced by `bench-report` (no contradiction with GAT2TRAREPBEN-008).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links.mjs`, `rule-coverage`) and the existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo run -p rule-coverage -- --game race_to_n`
3. A docs-consistency command set is the correct boundary; this ticket ships no code.
