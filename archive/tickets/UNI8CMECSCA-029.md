# UNI8CMECSCA-029: Finalize MSC-8C-010 rejected/local-only and review exclusions (C-10)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance doc (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`)
**Deps**: UNI8CMECSCA-028

## Problem

C-10 affirms the existing Non-Promotion List through a register decision rather than rewriting the shipped doctrine. This ticket finalizes `MSC-8C-010` as `rejected / local-only` and reviews every accepted entry against the Non-Promotion List, recording explicit exclusions for deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome. The register↔atlas seam stays intact and no §10A behavioral promotion debt is created.

## Assumption Reassessment (2026-06-22)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` has entries `MSC-8C-001`…`MSC-8C-010` (UNI8CMECSCA-002), with 001–009 promoted to `accepted` by their helper tickets and validated callerless-free by UNI8CMECSCA-028; `MSC-8C-010` started `rejected / local-only`. `docs/MECHANIC-ATLAS.md` §10A reads `Current debt: None`.
2. Spec §5 8C-029 + §10.A fix the work: finalize `MSC-8C-010` rejected/local-only; record explicit exclusions for deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome; register and atlas seam intact; no §10A behavioral promotion debt; no rewrite of already-shipped doctrine.
3. Cross-artifact boundary under audit: the register (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) and the behavioral mechanic atlas (`docs/MECHANIC-ATLAS.md` §10A). C-10 is satisfied by a register decision affirming the list 8M shipped, not by duplicating or moving it.
4. FOUNDATIONS §4/§12 + ADR 0008: behavior-free scaffolding stays distinct from behavioral mechanics; recording the rejection and exclusions affirms the seam rather than promoting any policy.
5. No-leak/determinism (§11): a governance-doc-only change; it touches no code, byte, or fixture, so no determinism/leak surface is affected. It confirms no accepted helper smuggled a behavioral exclusion.

## Architecture Check

1. Affirming the Non-Promotion List by reference (vs. rewriting it) keeps the doctrine 8M shipped authoritative and records 8C's compliance plus a future review trigger.
2. No backwards-compatibility shim — a register decision; nothing aliased; the atlas list is not duplicated.
3. `engine-core`/`game-stdlib` untouched; the register↔atlas seam is preserved.

## Verification Layers

1. `MSC-8C-010` reads `rejected / local-only` with a next-mechanic-gate review trigger → grep-proof.
2. Explicit exclusions recorded for deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome → grep-proof.
3. `docs/MECHANIC-ATLAS.md` §10A still `Current debt: None` → grep-proof.
4. Doc links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

Finalize `MSC-8C-010` as `rejected / local-only` with the review trigger; review each accepted entry and record explicit exclusions for the behavioral-policy bundle (deal/reveal/projection/betting/pot/trick/team/graph/accounting/reaction/scoring/outcome). Do not rewrite the atlas Non-Promotion List.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)

## Out of Scope

- Rewriting or relocating the behavioral Non-Promotion List / `docs/MECHANIC-ATLAS.md`.
- Flipping any accepted entry's decision (the helper tickets did that).
- The spec `Done`-flip or C-11 seeds (UNI8CMECSCA-030/031).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n 'MSC-8C-010' docs/MECHANICAL-SCAFFOLDING-REGISTER.md` shows `rejected` / `local-only` with a review trigger.
2. The register records explicit exclusions for the named behavioral-policy bundle.
3. `node scripts/check-doc-links.mjs` passes and `docs/MECHANIC-ATLAS.md` §10A still reads `Current debt: None`.

### Invariants

1. The register↔atlas seam is intact; the behavioral Non-Promotion List is affirmed by reference, not rewritten.
2. No §10A behavioral promotion debt is created.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n 'MSC-8C-010' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
2. `node scripts/check-doc-links.mjs`
3. The register is a governance doc; grep + doc-link integrity is the correct verification boundary.

## Outcome

Completed: 2026-06-22

Finalized `MSC-8C-010` as `rejected / local-only` with explicit behavior
exclusions for deal, reveal, projection, betting, pot, trick, team, graph,
accounting, reaction, scoring, and outcome policy. The register now records the
completed C-10 review, keeps the accepted helper entries' behavior exclusions as
the evidence boundary, and moves the next review trigger to a future
mechanic-ladder gate only.

`docs/MECHANIC-ATLAS.md` was intentionally unchanged. Section 10A still reads
`Current debt: _None_`, so no behavioral promotion debt was created. No code,
fixtures, hashes, WASM surface, or runtime behavior changed.

Verification:

1. `grep -n 'MSC-8C-010' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
2. `grep -n 'deal, reveal, projection, betting' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
3. `grep -n 'pot, trick, team, graph' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
4. `grep -n 'accounting, reaction, scoring, and outcome' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
5. `rg -n "Current debt: _None_|Current debt: None" docs/MECHANIC-ATLAS.md`
6. `node scripts/check-doc-links.mjs`
7. `git diff --check`
