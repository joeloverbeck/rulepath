# RIVLEDSHO-012: Closeout — RULE-COVERAGE / UI.md reconciliation + index flip

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None
**Deps**: RIVLEDSHO-005, RIVLEDSHO-006, RIVLEDSHO-007, RIVLEDSHO-008, RIVLEDSHO-009, RIVLEDSHO-010, RIVLEDSHO-011

## Problem

Once the showdown-legibility and table-presentation surfaces have shipped, the River Ledger docs and the spec index must be reconciled: `RULE-COVERAGE.md`'s UI rows still read `intentionally-deferred` though the outcome UI is now proven, `UI.md` must reflect the new Rust-authored explanation fields, and the spec's `specs/README.md` row flips to `Done` with evidence. This verification-only closeout capstone owns that reconciliation (spec WB12 / R8 + §9 exit criteria).

## Assumption Reassessment (2026-06-15)

1. Verified against current code/docs: `games/river_ledger/docs/RULE-COVERAGE.md:92-99` marks `RL-UI-PRESENT/SEATS/ACTIONS/PREVIEW/LEDGER/SHOWDOWN/NOCASINO/NOLEAK-001` (and `RL-OOS-BROWSER-001:114`) `intentionally-deferred`; `games/river_ledger/docs/UI.md` §Outcome/§Per-Seat-Final-Breakdown describe the rationale fields; `specs/README.md` carries this spec's row as `Planned`.
2. Verified against specs/docs: spec §9 exit criteria + §12 documentation-updates + §8 WB12; the doc-link/catalog checkers (`scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs`) are the closeout gates; `tools/rule-coverage` validates `RULE-COVERAGE.md`.
3. Cross-artifact boundary under audit: this capstone edits docs/status surfaces only after every implementation ticket (005–011) lands; it introduces no production logic and exercises the surfaces those tickets composed. The `apps/web/README.md` catalog edit is conditional — only if a Shell-Surface/catalog name changed (`check-catalog-docs.mjs` must stay green).

## Architecture Check

1. A single trailing closeout capstone (docs reconciliation + index flip) keeps the status-reconciliation surfaces atomic and gated on all upstream surfaces existing coherently — no mid-gate doc drift.
2. No backwards-compatibility aliasing/shims; docs/status edits only.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); no production logic — verification-only capstone.

## Verification Layers

1. `RULE-COVERAGE.md` UI rows reflect the shipped surface (no longer `intentionally-deferred` where the redesign proves them) -> grep `RULE-COVERAGE.md` for the updated status + `cargo run -p rule-coverage -- --game river_ledger`.
2. `UI.md` reconciled with the new Rust-authored explanation fields -> manual review + `node scripts/check-doc-links.mjs`.
3. Catalog docs stay consistent and the spec index flips to `Done` -> `node scripts/check-catalog-docs.mjs` + grep `specs/README.md` for the `Done` row.

## What to Change

### 1. `games/river_ledger/docs/RULE-COVERAGE.md`

Refresh the `RL-UI-*` rows from `intentionally-deferred` to the proven status with the new evidence (the smoke/e2e surfaces from 004–011).

### 2. `games/river_ledger/docs/UI.md`

Reconcile the outcome/contract rows with the redesigned panel and the new Rust-authored explanation fields.

### 3. `specs/README.md` (+ conditional `apps/web/README.md`)

Flip this spec's index row to `Done` with evidence; touch `apps/web/README.md` only if a catalog/Shell-Surface name changed.

## Files to Touch

- `games/river_ledger/docs/RULE-COVERAGE.md` (modify)
- `games/river_ledger/docs/UI.md` (modify)
- `specs/README.md` (modify)
- `apps/web/README.md` (modify; only if a catalog/Shell-Surface name changed)

## Out of Scope

- Any implementation surface (005–011 own those; this capstone exercises, does not modify them).
- The `RULE-COVERAGE.md` teaching-aid row if RIVLEDSHO-010 was dropped per Assumption A5.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game river_ledger` — coverage matrix consistent with refreshed UI rows.
2. `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` — both green.
3. `npm --prefix apps/web run smoke:e2e` — the composed acceptance surface (panel, cards, ladder, action/seat/turn, copy) is green end-to-end.

### Invariants

1. No `RL-UI-*` row claims coverage a shipped ticket does not prove (doc fidelity).
2. The spec index row reads `Done` only after the §9 exit criteria pass with evidence (`specs/README.md` status discipline).

## Test Plan

### New/Modified Tests

1. None — verification-only capstone; it exercises the acceptance suite composed by RIVLEDSHO-001…011 and adds no tests.

### Commands

1. `cargo run -p rule-coverage -- --game river_ledger`
2. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run smoke:e2e` — full lane is the correct closeout boundary; the gate is exercised by running existing scripts plus the doc checkers.

## Outcome

Completed 2026-06-15. Reconciled River Ledger's docs/status surfaces after the
RIVLEDSHO implementation tickets:

- Refreshed `games/river_ledger/docs/RULE-COVERAGE.md` UI rows for the shipped
  presentation, action metadata, ledger copy, showdown explanation, no-casino,
  and browser no-leak surfaces. `RL-UI-PREVIEW-001` remains
  `intentionally-deferred` because this series did not ship a separate River
  Ledger preview surface.
- Updated `games/river_ledger/docs/UI.md` for Rust-authored showdown fields,
  best-five labels, terminal-only category-ladder teaching aid, hand-ranking
  reference, action metadata copy, seat/street affordances, and ledger wording.
- Flipped the active spec index row in `specs/README.md` to `Done` with
  closeout evidence. No `apps/web/README.md` edit was needed because no catalog
  or shell surface name changed.

Verification:

- `cargo run -p rule-coverage -- --game river_ledger`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `npm --prefix apps/web run smoke:e2e`
- `git diff --check`
