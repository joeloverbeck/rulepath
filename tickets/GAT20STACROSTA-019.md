# GAT20STACROSTA-019: forward-v1 governance closeout (scaffolding receipt + register reconciliation)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (CI evidence receipt) — `ci/scaffolding-audits.json`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/MECHANIC-ATLAS.md`
**Deps**: GAT20STACROSTA-002, GAT20STACROSTA-003, GAT20STACROSTA-013, GAT20STACROSTA-016, GAT20STACROSTA-017

## Problem

The forward-v1 governance lane is closed only after the build: the machine `ci/scaffolding-audits.json` `forward-v1` receipt must be added and the register/atlas reconciled, validated by `scripts/check-scaffolding-governance.mjs`. This ticket pairs with the pre-code audit (GAT20STACROSTA-003) and certifies that implementation introduced no behavior-bearing scaffolding.

## Assumption Reassessment (2026-06-27)

1. `ci/scaffolding-audits.json` carries `coverage: "forward-v1"` entries for `blackglass_pact` and `meldfall_ledger` (confirmed); a `starbridge_crossing` entry is appended (gate 20, evidence path, register disposition path, no migration debt).
2. `scripts/check-scaffolding-governance.mjs` validates the receipt against the register; this ticket makes that checker green (red until now).
3. Cross-artifact boundary: this reconciles `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (forward-v1 summary + any local-only/candidate first-use plumbing dispositions) and `docs/MECHANIC-ATLAS.md` (the 002 defer/reject decision is already recorded; this confirms no §10A debt) — `docs/MECHANIC-ATLAS.md` is also edited by 002, so this ticket `Deps: 002` to serialize that file.
4. §11 (scaffolding receipt invariant) motivates this ticket: every new game closes with its forward-v1 receipt present, and any newly introduced behavior-free scaffolding is registered with behavior exclusions + next-review trigger; graph/topology/path legality is NOT registered as scaffolding.
5. Scaffolding-governance enforcement surface (§11/ADR 0008): `ci/scaffolding-audits.json` + `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` checked by `check-scaffolding-governance.mjs`. Confirm the receipt records reuse dispositions (no new behavior scaffolding expected) and that no register entry encodes legality/visibility/turn policy.

## Architecture Check

1. Landing the machine receipt + register reconciliation after the build certifies actual (not planned) scaffolding reuse, pairing cleanly with the pre-code audit (003).
2. No backwards-compatibility shims; governance artifacts only.
3. `engine-core` untouched; no `game-stdlib` helper added (002 defer/reject holds); the receipt asserts this.

## Verification Layers

1. Receipt validity -> `node scripts/check-scaffolding-governance.mjs` (green).
2. Register reconciliation (§11) -> manual review: `MECHANICAL-SCAFFOLDING-REGISTER.md` carries the Gate 20 forward-v1 summary + dispositions.
3. No promotion debt (§10A) -> grep-proof: `docs/MECHANIC-ATLAS.md` §10A still `None`.
4. No behavior-as-scaffolding -> FOUNDATIONS alignment check: no graph/topology/path entry in the register.

## What to Change

### 1. Add the `ci/scaffolding-audits.json` receipt

`starbridge_crossing` with `coverage: "forward-v1"`, gate 20, evidence path, register disposition path, no migration debt.

### 2. Reconcile `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

Gate 20 forward-v1 summary; C-01…C-10 dispositions; any local-only/candidate first-use plumbing with behavior exclusions + Gate 21 review trigger.

### 3. Confirm `docs/MECHANIC-ATLAS.md`

§10A remains `None`; the 002 defer/reject decision is consistent.

## Files to Touch

- `ci/scaffolding-audits.json` (modify)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify; also edited by 002 — Deps serializes)

## Out of Scope

- The `specs/README.md` Done-flip + exit-criteria verification — GAT20STACROSTA-020.
- Any code change (governance artifacts only).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-scaffolding-governance.mjs`
2. `node scripts/check-doc-links.mjs`
3. `bash scripts/boundary-check.sh`

### Invariants

1. The forward-v1 receipt is present and validates; no behavior scaffolding is registered (§11).
2. No §10A promotion debt results from Gate 20.

## Test Plan

### New/Modified Tests

1. `None — CI evidence receipt + governance docs; verification is the checker run above.`

### Commands

1. `node scripts/check-scaffolding-governance.mjs`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. The governance checker is the correct boundary; it validates the receipt against the register at run time.
