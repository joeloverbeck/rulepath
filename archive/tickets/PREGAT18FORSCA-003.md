# PREGAT18FORSCA-003: ARCHITECTURE forward-conformance section + truthfulness fix

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/ARCHITECTURE.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

`ARCHITECTURE.md` describes the reuse-ownership matrix but (a) still labels the `game-test-support` crate "future" though it has shipped, and (b) has no forward mechanical-scaffolding conformance section. This ticket fixes the stale label and adds the forward-conformance doctrine so the architecture doc matches the standing obligation in `FOUNDATIONS.md` §11/§12.

## Assumption Reassessment (2026-06-25)

1. `docs/ARCHITECTURE.md` §3A "Reuse Ownership Matrix" (L71) row L81 reads `| Dev-only evidence/test scaffolding | future `game-test-support` crate |`; §3.1 "Promoted-helper conformance" is at L90; §14 "Architecture acceptance checks" is at L360. Verified this session.
2. The `game-test-support` crate exists and is a workspace member (`crates/game-test-support/Cargo.toml`, `name = "game-test-support"`); the "future" label is therefore stale — the fix is a truthfulness correction, not a behavioral change.
3. The spec (D3, plan §5.3) requires: §3A truthfulness fix ("future" → present-tense), a new §3B "Forward mechanical-scaffolding conformance" after §3A and before §3.1, and a §14 acceptance-check bullet.
4. FOUNDATIONS §3 (`engine-core` contract kernel) and §4 (`game-stdlib` earned) under audit: the new §3B describes the forward conformance lane parallel to the behavioral promotion path; it adds no mechanic noun to `engine-core` and reaffirms `game-stdlib` discipline.

## Architecture Check

1. Correcting the "future" label and adding §3B keeps `ARCHITECTURE.md` an accurate ownership map; placing §3B between §3A and §3.1 keeps the scaffolding lane adjacent to the reuse-ownership matrix it extends.
2. No backwards-compatibility shim — the label edit is a present-tense correction; §3B is additive.
3. `engine-core` stays noun-free; `game-test-support` remains dev-only; the §14 bullet states the conformance acceptance check without moving behavior into TypeScript.

## Verification Layers

1. Truthfulness fix → grep-proof `future `game-test-support`` no longer appears in §3A and the row reads present-tense.
2. Dev-only crate boundary → `cargo tree --workspace -e normal,build --invert game-test-support` shows no non-dev consumer (boundary unchanged).
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. §3A truthfulness fix

Change the `game-test-support` ownership row from "future" to present-tense (the crate exists).

### 2. New §3B — Forward mechanical-scaffolding conformance

Add the §3B section (plan §5.3 draft) after §3A, before §3.1: the forward reuse-first-audit / register-new / queue-or-dispose conformance expectations for any new game, parallel to §3.1's promoted-helper conformance.

### 3. §14 acceptance bullet

Add the forward mechanical-scaffolding conformance acceptance-check bullet to §14.

## Files to Touch

- `docs/ARCHITECTURE.md` (modify)

## Out of Scope

- Any change to §3.1 promoted-helper conformance text (parallel, not replaced).
- Any code change to `crates/game-test-support` (label correction is doc-only).
- The boundary-check script or `engine-core` kernel.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "future .game-test-support" docs/ARCHITECTURE.md` returns nothing; the row reads present-tense.
2. `grep -n "3B" docs/ARCHITECTURE.md` confirms the new section between §3A and §3.1.
3. `node scripts/check-doc-links.mjs` and `bash scripts/boundary-check.sh` pass.

### Invariants

1. `game-test-support` is described as a present, dev-only crate.
2. §3B is additive and parallel to §3.1; no behavior moves to TypeScript.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. `cargo tree --workspace -e normal,build --invert game-test-support` (confirms dev-only boundary the label now states)

## Outcome

Completed: 2026-06-25

Changed `docs/ARCHITECTURE.md` to align the architecture ownership map with the
Unit 8F forward scaffolding governance obligation:

- changed the §3A dev-only evidence/test scaffolding owner from future
  `game-test-support` to present-tense `game-test-support` crate;
- added §3B `Forward mechanical-scaffolding conformance` between §3A and §3.1;
- added the §14 acceptance-check bullet requiring new-game audit receipt,
  current register disposition, and prior-game retrofit/no-refactor disposition.

Deviation: none. §3.1 promoted-helper conformance remains unchanged and parallel
to the new §3B scaffolding conformance section.

Verification:

- `grep -n "future .game-test-support" docs/ARCHITECTURE.md` returned no matches.
- `rg -n "3B\\. Forward mechanical-scaffolding conformance|3\\.1 Promoted-helper conformance|game-test-support|mechanical-scaffolding audit receipt" docs/ARCHITECTURE.md` confirmed the present-tense row, §3B placement before §3.1, and the §14 bullet.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `bash scripts/boundary-check.sh` passed, including `game-test-support dev-only boundary check passed`.
- `cargo tree --workspace -e normal,build --invert game-test-support` showed only `game-test-support v0.1.0`.
- `git diff --check` passed.
