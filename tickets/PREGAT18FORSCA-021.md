# PREGAT18FORSCA-021: Unit 8F closeout capstone — reconcile, verify, flip 8F→Done

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs/status-only (closeout reconciliation + `specs/README.md` `Done`-flip + verification)
**Deps**: PREGAT18FORSCA-003, PREGAT18FORSCA-004, PREGAT18FORSCA-005, PREGAT18FORSCA-006, PREGAT18FORSCA-008, PREGAT18FORSCA-009, PREGAT18FORSCA-010, PREGAT18FORSCA-011, PREGAT18FORSCA-012, PREGAT18FORSCA-013, PREGAT18FORSCA-014, PREGAT18FORSCA-015, PREGAT18FORSCA-016, PREGAT18FORSCA-020

## Problem

Unit 8F is done only when every law/template/check is consistent, the cross-cutting preservation invariants hold, the `PRIMITIVE-PRESSURE-LEDGER.md` no-change proof is recorded, and the tracker reflects completion without admitting Gate 18 early. This capstone runs the full acceptance command set, records the closeout evidence + the three-gap → change map, flips the `8F` row to `Done`, and confirms Gate 18 stays `Not started`.

## Assumption Reassessment (2026-06-25)

1. The prior tickets land: the ADR extension + FOUNDATIONS invariants (001/002), the area/contract/atlas/register docs (003–007), agent/testing/roadmap/tracker law (008–011), templates (012–016), and the receipt/checker/tests/CI (017–020). `specs/README.md` carries the `8F` row (`Planned`, L103) and the Gate 18 block (`Blocked until 8F is Done`, L104). Verified this session.
2. `templates/PRIMITIVE-PRESSURE-LEDGER.md` already redirects behavior-free plumbing to the register (Behavioral-scope-only note) and keeps the behavioral third-game gate authoritative — so D17's "no change" is correct; this capstone records the explicit no-change proof (adding forward-scaffolding fields there would create two competing owners).
3. Shared boundary under audit: the cross-cutting preservation set — `MECHANIC-ATLAS.md` §4/§5A byte-exact, ADR 0008 lane/homes/Non-Promotion List unchanged, no game/helper/trace/fixture/hash/RNG/serialization/benchmark/byte change, `engine-core` noun-free, no YAML/DSL/runtime registry.
4. FOUNDATIONS §11 / §12 under audit (preservation invariants): the capstone proves the forward additions strengthened §11/§12 without crossing any stop condition; it is the exit gate for the whole unit.
5. Enforcement/preservation surface: the capstone runs the spec's minimum acceptance transcript (`check-scaffolding-governance` + its `node --test` suite, `check-ci-games`, `check-doc-links`, `check-catalog-docs`, `boundary-check.sh`, `cargo tree --invert game-test-support`, `git diff --check`) plus the §4 byte-exact source comparison and the changed-file audit (`git status --porcelain -- crates/ games/ tools/ apps/` returns nothing beyond `scripts/`/`ci/` governance files); it introduces no leak/nondeterminism (verification-only).

## Architecture Check

1. A single trailing capstone that only reconciles status and runs the acceptance set keeps completion gated on exit evidence, not on any one implementation ticket — and it owns the `Done`-flip per the §Ticket-shapes default.
2. No backwards-compatibility shim — verification-only; no production logic added.
3. `engine-core` untouched; the `Done`-flip and no-change proof are docs/status edits; `boundary-check.sh` is in the acceptance set.

## Verification Layers

1. Acceptance-set green → run the full minimum transcript; all checks pass.
2. §4/§5A + ADR preservation → source comparison: the behavioral third-use hard-gate sentence and ADR 0008 lane/homes/Non-Promotion List are byte-for-byte unchanged.
3. No unauthorized diff → `git status --porcelain -- crates/ games/ tools/ apps/ '**/*.trace.json'` returns nothing beyond the new `scripts/`/`ci/` governance files.
4. Tracker correctness → grep-proof the `8F` row reads `Done` and the Gate 18 row still reads `Not started` / `Blocked`.
5. No-change proof → the `PRIMITIVE-PRESSURE-LEDGER.md` no-change rationale is recorded in the closeout evidence.

## What to Change

### 1. Closeout evidence + three-gap map

Record the closeout matrix mapping every changed law/template/check to one of the three forward gaps, plus the `PRIMITIVE-PRESSURE-LEDGER.md` explicit no-change proof, in the spec's Outcome section.

### 2. Flip 8F → Done

In `specs/README.md`, flip the `8F` row Status to `Done`; confirm the Gate 18 row stays `Not started` / `Blocked until 8F`.

### 3. Spec Status

Set the spec's own Status to `Done`.

## Files to Touch

- `specs/README.md` (modify — `8F` row `Done`-flip; shares this file with PREGAT18FORSCA-011, which this capstone `Deps` transitively, so the edits serialize)
- `specs/pre-gate-18-forward-scaffolding-reuse-governance.md` (modify — Status `Done` + Outcome)

## Out of Scope

- Authoring or admitting Gate 18 / Spades (must stay `Not started` until `8F` is `Done`).
- Any new production logic, game, helper, trace, fixture, hash, or benchmark change.
- Re-editing the upstream tickets' surfaces (this capstone exercises them, it does not modify them).

## Acceptance Criteria

### Tests That Must Pass

1. Full minimum acceptance transcript passes: `node scripts/check-scaffolding-governance.mjs`; `node --test scripts/check-scaffolding-governance.test.mjs`; `node scripts/check-ci-games.mjs`; `node scripts/check-doc-links.mjs`; `node scripts/check-catalog-docs.mjs`; `bash scripts/boundary-check.sh`; `cargo tree --workspace -e normal,build --invert game-test-support`; `git diff --check`.
2. The behavioral third-use hard-gate sentence (`MECHANIC-ATLAS.md` §4) is byte-for-byte unchanged (source comparison).
3. `git status --porcelain -- crates/ games/ tools/ apps/ '**/*.trace.json'` returns nothing beyond the new `scripts/`/`ci/` governance files.

### Invariants

1. The `8F` row reads `Done`; Gate 18 remains `Not started`.
2. No game/helper/trace/fixture/hash/RNG/serialization/benchmark/byte changed; no YAML/DSL/runtime registry introduced.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the spec Outcome and `specs/README.md` `Done`-flip and exercises the acceptance suite composed by prior tickets, adding no test file.`

### Commands

1. `node scripts/check-scaffolding-governance.mjs && node --test scripts/check-scaffolding-governance.test.mjs`
2. `node scripts/check-ci-games.mjs && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh`
3. `git status --porcelain -- crates/ games/ tools/ apps/ '**/*.trace.json'` (must be empty) and `git diff --check`
