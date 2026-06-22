# PREGAT18REUDOC-023: Capstone — index reconcile, spec Done-flip, exit-criteria verification

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs/status-only (`specs/README.md`, `specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md`)
**Deps**: 003, 013, 014, 015, 017, 018, 019, 020, 021, 022

## Problem

The realignment pass needs a closeout: confirm/reconcile the already-present `specs/README.md` Part C (`8C`) seed and Gate 18 interlock, flip the `8M` realignment row and the spec's own Status to `Done`, and verify the spec's exit criteria (doc gates green, no code/fixture drift). This capstone exercises every prior ticket end-to-end and introduces no new doctrine.

## Assumption Reassessment (2026-06-22)

1. Verified `specs/README.md` already carries the `8M` realignment row as `Planned` (line ≈97), the `8C` "Mechanical-scaffolding code extraction (Part C)" seed as `Not started`/Blocked (line ≈98), and the Gate 18 interlock (line ≈99) — confirmed via `/reassess-spec` finding I2 this session. So this capstone **flips `8M` → `Done` and confirms/reconciles** the `8C` seed + Gate 18 interlock (it does **not** duplicate or recreate them).
2. Verified against spec D13 / WB11 (reframed per reassess I2): the only `specs/README.md` mutation on exit is the `8M` → `Done` flip; the ROADMAP debt interlocks are owned by ticket 022. The spec's own Status field also flips to `Done`.
3. Cross-artifact boundary under audit: this capstone exercises all prior tickets; its own status-reconciliation surfaces are `specs/README.md` (existing) and the spec file's Status. It does not modify upstream tickets' doc surfaces.
4. FOUNDATIONS §12 stop-condition final sweep: confirm no stop condition was crossed — `engine-core` gained no mechanic noun (`boundary-check.sh`), no hidden-info leak, no `*.trace.json`/hash/RNG/serialization byte changed, the behavioral third-use gate stayed effective.
5. Acceptance precondition: ADRs 0008/0009 (tickets 004/005, reached transitively via 022/021) are `Accepted`, and this capstone is gated on the exit evidence (the doc gates passing), not merely on file existence.

## Architecture Check

1. A single capstone carrying the status reconciliation + exit verification is the §Ticket-shapes `Done`-flip default, cleaner than scattering the flip across implementation tickets.
2. No backwards-compatibility shims; the capstone adds no production logic.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs/status-only.

## Verification Layers

1. `specs/README.md` `8M` row reads `Done` -> codebase grep-proof.
2. The `8C` Part C seed + Gate 18 interlock are present exactly once (not duplicated) -> grep count.
3. The spec's own Status reads `Done` -> grep on `specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md`.
4. Spec exit criteria green -> `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh`.
5. No code/fixture/bench drift -> `git status --porcelain -- crates/ games/ tools/ apps/ scripts/ '**/*.trace.json'` is empty (FOUNDATIONS §12 final sweep).

## What to Change

### 1. Reconcile the index

In `specs/README.md`, confirm the `8C` Part C seed and Gate 18 interlock are correct (reconcile wording only if drifted; do **not** duplicate the rows), and flip the `8M` realignment row to `Done`.

### 2. Flip the spec Status

Set the spec's Status field to `Done`.

### 3. Run the exit verification

Run the doc gates and the no-drift check; record results as the acceptance evidence.

## Files to Touch

- `specs/README.md` (modify — `8M` row → `Done`, reconcile `8C`/Gate 18 rows)
- `specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md` (modify — Status → `Done`)

## Out of Scope

- Modifying any upstream ticket's doc surfaces (this capstone exercises them, it does not change them).
- Seeding new Part C / Gate 18 rows (they already exist; only reconciliation + the `8M` flip happen here).
- The ROADMAP debt interlocks (ticket 022).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE "pre-gate-18.*Done|8M.*Done" specs/README.md` shows the realignment row `Done`; the `8C` seed and Gate 18 interlock each appear once.
2. `grep -nE "^\\| Status \\| Done|Status.*Done" specs/pre-gate-18-reuse-doctrine-and-evidence-realignment.md` shows the spec Status `Done`.
3. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh` all pass, and `git status --porcelain -- crates/ games/ tools/ apps/ scripts/ '**/*.trace.json'` is empty.

### Invariants

1. The `8C` Part C seed and Gate 18 interlock are reconciled, not duplicated.
2. No §12 stop condition was crossed across the whole pass (docs/law/template diff only).

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named index/status surfaces and exercises the prior tickets' acceptance evidence (the doc gates + no-drift check), adding no test file.`

### Commands

1. `grep -nE "8M|8C|Part C|Gate 18" specs/README.md`
2. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh && git status --porcelain -- crates/ games/ tools/ apps/ scripts/ '**/*.trace.json'`
3. The index/status greps + the full doc-gate run + no-drift `git status` are the correct boundary: this capstone verifies the whole pass without touching code.
