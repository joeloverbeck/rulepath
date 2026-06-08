# BENCICAL-004: Reference variance-aware calibration doctrine in TESTING §15/§17

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — foundation doc only (`docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17). No code, data, schema, or workflow change.
**Deps**: ADR 0005 (`docs/adr/0005-variance-aware-ci-benchmark-floors.md`) accepted

## Problem

`docs/TESTING-REPLAY-BENCHMARKING.md` §15 and §17 currently describe the Gate 2
benchmark floor as "the enforced floor for the CI runner" calibrated per ADR 0003
("a conservative floor below the stable `ubuntu-latest` measurement"). ADR 0005
supersedes that single-sample calibration with variance-aware floors (≥15% below
the minimum across at least three representative CI runs). The testing law is the
canonical statement of the gating doctrine; it must point at ADR 0005 so the
calibration rule readers follow matches the accepted decision, rather than the
superseded single-sample story.

## Assumption Reassessment (2026-06-08)

1. The exact text under audit was confirmed by reading
   `docs/TESTING-REPLAY-BENCHMARKING.md` lines 341–357 (§15) and 406–415 (§17):
   §15 contains the ADR 0003 paragraph "the committed `thresholds.json` value is
   the enforced floor for the CI runner …"; §17 contains "The enforced thresholds
   are CI-runner floors per [ADR 0003] …". Both reference ADR 0002 (lane split)
   and ADR 0003 (CI calibration) and must additionally reference ADR 0005.
2. ADR 0005 (`docs/adr/0005-variance-aware-ci-benchmark-floors.md`) Migration
   notes name "§15 and §17 — reference this ADR for the variance-aware
   calibration doctrine (extends the existing ADR 0003 reference)." This ticket
   executes that migration note. No FOUNDATIONS principle's *meaning* changes —
   the hard-fail-on-`main`/scheduled and no-hidden-performance rules are
   preserved verbatim; only the calibration-method reference is extended.
3. This is a documentation-only foundation-doc edit; the shared contract under
   audit is the doc cross-reference graph, validated by the doc link checker.
4. FOUNDATIONS §13 restated: benchmark-gating doctrine is fixed by accepted ADRs
   and supersedes only by accepted ADR. The doctrine change itself lives in ADR
   0005 (the dependency); this ticket only updates the law doc to cite it, so it
   does not itself weaken doctrine.
5. Mismatch + correction: §15/§17 must not delete the ADR 0003 reference (it
   remains the lane-and-CI-calibration origin) but layer ADR 0005 on top as the
   current calibration rule — phrase it as "calibrated per ADR 0003 and
   variance-aware per ADR 0005" so the chain stays auditable.

## Architecture Check

1. Extending the existing ADR reference (rather than rewriting §15/§17) keeps the
   doctrine's audit chain intact: a reader can trace lane split (0002) →
   CI calibration (0003) → variance-aware floors (0005).
2. No backwards-compatibility shim — prose edit only.
3. No code, `engine-core`, `game-stdlib`, or data surface touched; TypeScript and
   Rust behavior unaffected.

## Verification Layers

1. Doctrine reference is correct and complete -> §15 and §17 both cite ADR 0005
   alongside ADR 0003 (manual review + grep for `0005` in the doc).
2. Doc links resolve -> `node scripts/check-doc-links.mjs` passes with the new
   ADR 0005 link.
3. No meaning drift -> the "MUST hard-fail the scheduled / manual / `main`-push
   lane" and "do not hide unknown performance" sentences are unchanged
   (grep-proof / diff review).

## What to Change

### 1. §15 Provisional performance budgets

Extend the ADR 0003 paragraph (lines ~352–357) so the committed `thresholds.json`
value is described as the variance-aware CI floor per ADR 0005 — ≥15% below the
minimum across at least three representative CI runs, not a single sample — while
keeping the native-baseline-stays-visible sentence intact and the ADR 0003
reference as the calibration origin.

### 2. §17 CI expectations

Extend the sentence "The enforced thresholds are CI-runner floors per [ADR 0003]"
(lines ~409–411) to add "and variance-aware per
[ADR 0005](adr/0005-variance-aware-ci-benchmark-floors.md)", preserving the
hard-fail and ADR 0001 native-target clauses verbatim.

## Files to Touch

- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify — §15 and §17 only)

## Out of Scope

- Any threshold value or per-game BENCHMARKS.md change — handled by BENCICAL-003.
- Re-litigating the lane split (ADR 0002) or the hard-fail requirement.
- Editing other §sections of the testing doc.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the new ADR 0005 reference resolving.
2. `grep -n "0005" docs/TESTING-REPLAY-BENCHMARKING.md` shows the reference in both §15 and §17.

### Invariants

1. The "MUST hard-fail the scheduled / manual / `main`-push benchmark lane" and "do not hide unknown performance" statements remain unchanged in meaning.
2. The ADR 0002 (lane split) and ADR 0003 (CI calibration origin) references are retained, with ADR 0005 layered as the current calibration rule.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (doc link check + grep) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n "0003\|0005" docs/TESTING-REPLAY-BENCHMARKING.md`
3. A narrower command is correct here because the change is foundation-doc prose with no executable surface; link integrity plus reference-presence grep fully cover it.

## Outcome

Completed: 2026-06-08

What changed:

- Updated `docs/TESTING-REPLAY-BENCHMARKING.md` §15 to cite ADR 0005 alongside
  ADR 0003 and state the variance-aware floor rule: at least 15% below the
  minimum observed across representative CI runs, not a single-sample floor.
- Updated §17 to preserve the ADR 0002 lane split and ADR 0003 CI-runner floor
  reference while layering ADR 0005 as the current variance-aware calibration
  rule.

Deviations from original plan:

- None.

Verification results:

- `node scripts/check-doc-links.mjs` passed.
- `grep -n "0003\\|0005" docs/TESTING-REPLAY-BENCHMARKING.md` showed ADR 0005
  references in both §15 and §17 while retaining ADR 0003.
- Diff review confirmed the scheduled / manual / `main`-push hard-fail language
  and native-target preservation language remain intact.
