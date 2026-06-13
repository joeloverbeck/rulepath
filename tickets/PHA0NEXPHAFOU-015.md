# PHA0NEXPHAFOU-015: Capstone — reconcile specs/README.md index and run the realignment gates

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — `specs/README.md` status reconciliation only; verification-only closeout.
**Deps**: PHA0NEXPHAFOU-003, PHA0NEXPHAFOU-004, PHA0NEXPHAFOU-008, PHA0NEXPHAFOU-009, PHA0NEXPHAFOU-010, PHA0NEXPHAFOU-011, PHA0NEXPHAFOU-012, PHA0NEXPHAFOU-013, PHA0NEXPHAFOU-014

## Problem

Once the realignment lands (ADR 0007 accepted, the contract doc and clarifications written, templates updated, ROADMAP edited), the Phase 0 spec index row must flip to `Done` and the Gate 15+ rows' interlock notes must be refreshed to reflect that ADR 0007 is accepted and ROADMAP now records the public scaling phase. The full realignment must also pass the repo's documentation/boundary gates. This is the verification + status-reconciliation closeout; it exercises the prior tickets and introduces no production logic.

## Assumption Reassessment (2026-06-13)

1. No code change. `specs/README.md` exists (authored this session) with the Phase 0 row at `Planned` and the Gate 15+ rows marked *pending ADR 0007*; `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs`, and `scripts/boundary-check.sh` exist (verified).
2. Docs: `specs/README.md` (the active-epoch tracker); the realignment outputs of PHA0NEXPHAFOU-001…014 are the surfaces this capstone verifies and reconciles against.
3. Cross-artifact boundary under audit: the whole realignment; shared surface = the `specs/README.md` index plus the three CI gate scripts.
4. FOUNDATIONS principle restate: §11 (tests/docs/checks cover the change). This capstone is the verification gate for the Phase 0 spec's exit criteria.
5. The change is a status-line reconciliation on a doc-governed contract (the spec index): flipping the Phase 0 row to `Done` and refreshing the Gate 15+ interlock notes. Blast radius is `specs/README.md` only; it exercises — but does not modify — the upstream tickets' files.

## Architecture Check

1. A single capstone that runs the gate set and reconciles the index is cleaner than scattering status flips across the content tickets; it introduces no production logic.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` is untouched and stays noun-free (`boundary-check.sh` green is part of the closeout).

## Verification Layers

1. Phase 0 row flipped to `Done` and Gate 15+ interlock notes refreshed → codebase grep-proof on `specs/README.md`.
2. Doc links across `docs/` + `specs/` resolve → `node scripts/check-doc-links.mjs`.
3. Web-shell catalog docs intact → `node scripts/check-catalog-docs.mjs`.
4. `engine-core` stays noun-free → `bash scripts/boundary-check.sh`.

## What to Change

### 1. `specs/README.md`

Flip the Phase 0 row `Status` to `Done` (only after the exit criteria pass with evidence). Refresh the Gate 15+ rows' interlock notes to reflect that ADR 0007 is accepted and ROADMAP now records the public scaling phase (the rows stay `Not started`, but the blocking note changes from *pending ADR 0007* to naming ROADMAP/admission as the live gate). Record the gate-run results as the spec's acceptance evidence.

### 2. Run the realignment gate set

Run `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, and `bash scripts/boundary-check.sh`; all must pass before the Phase 0 row is flipped to `Done`.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Modifying any upstream ticket's files (this capstone exercises them; it does not change them).
- Flipping the Phase 0 row to `Done` before the exit criteria pass.
- Writing any Gate 15+ or Infra spec.

## Acceptance Criteria

### Tests That Must Pass

1. `specs/README.md`'s Phase 0 row reads `Done`, and the Gate 15+ interlock notes reflect ADR 0007 accepted + ROADMAP recording the phase.
2. `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` pass.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The Phase 0 index status is flipped to `Done` only after the exit criteria pass with evidence.
2. `engine-core` remains free of mechanic nouns (boundary-check green); no production code was changed by the realignment.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh`
3. `grep -nE "Phase 0 .*Done|Done" specs/README.md` (confirm the Phase 0 row flip)
