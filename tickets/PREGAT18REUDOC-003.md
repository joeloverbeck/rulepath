# PREGAT18REUDOC-003: Dispose ADR 0005 (accept / reject / supersede)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/adr/0005-variance-aware-ci-benchmark-floors.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/README.md`)
**Deps**: 001

## Problem

ADR 0005 (variance-aware CI benchmark floors) is `Proposed` yet still carries binding-sounding MUST / MUST-NOT prose; Phase 0 only annotated it non-binding. The pre-Gate-18 pass must make the actual disposition — accept (with wording fixes), reject, or supersede/withdraw — using shipped benchmark data, then reconcile every reference so no non-accepted ADR reads as binding law.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/adr/0005-variance-aware-ci-benchmark-floors.md` is `Status: Proposed` (L3), carries a Phase-0 non-binding note (L5–7), and still contains binding MUST / MUST-NOT prose in its Decision section (≈L86–98) (confirmed via `/reassess-spec` this session; spec §Assumptions A9).
2. Verified `docs/TESTING-REPLAY-BENCHMARKING.md` references ADR 0005 as non-binding/informative (≈L395, L451); these references must state the chosen outcome.
3. Cross-artifact boundary under audit: the ADR status index in `docs/README.md` (created by ticket 001) lists 0005's status; this ticket updates that row to the disposed status. Hence `Deps: 001`.
4. FOUNDATIONS L3 / §13: a `Proposed` ADR carries no binding authority — restating the principle that if 0005 stays non-accepted, its binding-sounding prose must be removed, not merely annotated.
5. This is a **decision ticket**: the implementer reviews shipped benchmark data (`games/*/benches/*/thresholds.json`, recorded bench runs) and selects accept-with-fixes / reject / supersede; the selected branch drives the prose edits. The disposition must not weaken any deterministic-benchmark acceptance invariant (§11) — a rejection removes binding prose but leaves the existing non-binding calibration guidance intact.

## Architecture Check

1. Making the explicit disposition (vs leaving 0005 indefinitely `Proposed` with binding prose) removes the standing ambiguity that lets a non-accepted ADR be cited as law — the cleanest robust end state.
2. No backwards-compatibility shims; no benchmark threshold values are rewritten (decision + prose reconciliation only).
3. Docs-only: `engine-core` (§3) / `game-stdlib` (§4) untouched.

## Verification Layers

1. ADR 0005 `Status` reflects the disposition + binding prose removed if non-accepted -> codebase grep-proof.
2. `TESTING-REPLAY-BENCHMARKING.md` references state the outcome -> grep + manual review.
3. `docs/README.md` ADR status-index row for 0005 matches the disposition -> grep.
4. All links resolve -> `node scripts/check-doc-links.mjs`.
5. No benchmark threshold byte changed -> `git diff --stat -- games/*/benches` is empty (FOUNDATIONS §11 determinism: bench outputs unchanged).

## What to Change

### 1. Decision

Review shipped benchmark data and decide accept-with-wording-fixes / reject / supersede-withdraw; record the rationale in the ADR.

### 2. ADR 0005 prose

Set `Status` to the chosen value; if non-accepted, remove the binding MUST / MUST-NOT prose, retaining only non-binding calibration guidance.

### 3. Reconcile references

Update `TESTING-REPLAY-BENCHMARKING.md` references and the `docs/README.md` ADR status-index row to state the outcome; remove any binding-sounding citation if 0005 stays non-accepted.

## Files to Touch

- `docs/adr/0005-variance-aware-ci-benchmark-floors.md` (modify)
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)
- `docs/README.md` (modify; ADR status-index row created by ticket 001)

## Out of Scope

- Rewriting any benchmark threshold value or `thresholds.json` (decision + prose only).
- The new TESTING test-support law / fixture profiles / hash-migration protocol (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE "^Status:" docs/adr/0005-variance-aware-ci-benchmark-floors.md` shows the disposed status (not bare `Proposed` with binding prose).
2. `grep -niE "ADR 0005" docs/TESTING-REPLAY-BENCHMARKING.md docs/README.md` shows references consistent with the outcome.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No non-accepted ADR is cited as binding law anywhere in the doc set.
2. No deterministic benchmark threshold byte is changed by this ticket.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline coverage (`check-doc-links.mjs`, `git diff` on benches) is named in Assumption Reassessment.`

### Commands

1. `grep -nE "^Status:|MUST" docs/adr/0005-variance-aware-ci-benchmark-floors.md`
2. `node scripts/check-doc-links.mjs && git diff --stat -- games/*/benches`
3. The grep + `git diff` pair is the correct boundary: it proves the disposition landed and that no benchmark byte drifted.
