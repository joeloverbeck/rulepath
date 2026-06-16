# RIVLEDSHO-011: Public-copy casino-vocabulary audit

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`, `apps/web/src/components/outcomeExplanationTemplates.ts`
**Deps**: RIVLEDSHO-004, RIVLEDSHO-008, RIVLEDSHO-009

## Problem

Once the showdown panel, action-panel copy, and seat/turn-flow affordances have landed, the River Ledger normal-mode public copy must be swept for casino vocabulary and real-money framing and replaced with the neutral River Ledger identity. "Pot" may remain only in debug/rule detail; public copy leans on abstract contribution units (spec WB11 / D7).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: public copy lives in `RiverLedgerBoard.tsx` and the `river_ledger` entries of `outcomeExplanationTemplates.ts`; this ticket audits the copy after RIVLEDSHO-004/008/009 have added their surfaces, so the audit covers the final copy set.
2. Verified against specs/docs: spec §6 D7 + §8 WB11; `RULES.md` `RL-UI-NOCASINO-001`; `docs/IP-POLICY.md` (no casino trade dress / real-money framing).
3. Cross-artifact boundary under audit: the public-copy strings across the board, the action panel, and the outcome templates — a copy-only pass, no behavior or layout change.
4. FOUNDATIONS §10 (IP conservatism) + §7 (cozy, not casino) motivate this ticket: replace casino/real-money vocabulary with the neutral River Ledger "ledger/abstract-unit" identity in normal-mode surfaces.

## Architecture Check

1. A single trailing copy audit (after the surfaces exist) catches the final copy set in one reviewable diff rather than re-auditing per feature ticket.
2. No backwards-compatibility aliasing/shims; string replacements only.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — `apps/web` copy only; no Rust behavior.

## Verification Layers

1. No normal-mode River Ledger surface carries casino vocabulary or real-money framing -> grep `RiverLedgerBoard.tsx` + `outcomeExplanationTemplates.ts` for the banned vocabulary set + manual review (§10).
2. "Pot" appears only in debug/rule-detail surfaces, not normal-mode public copy -> targeted grep + manual review.
3. The audit does not regress the rendered surfaces -> `npm --prefix apps/web run smoke:ui` + `node apps/web/e2e/river-ledger.smoke.mjs`.

## What to Change

### 1. `apps/web/src/components/RiverLedgerBoard.tsx`

Replace casino/real-money vocabulary in normal-mode copy with neutral River Ledger / abstract-unit terms.

### 2. `apps/web/src/components/outcomeExplanationTemplates.ts`

Audit the `river_ledger` template copy for residual casino framing; keep "pot" only where it is debug/rule detail.

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)

## Out of Scope

- Layout/visual changes (RIVLEDSHO-004/006/009).
- Rust-authored copy (RIVLEDSHO-001 — already neutral by construction).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — surfaces render with neutral copy.
2. `node apps/web/e2e/river-ledger.smoke.mjs` — no regression from the copy pass.
3. Grep proof: the banned casino-vocabulary set returns no normal-mode hit in `RiverLedgerBoard.tsx` / `outcomeExplanationTemplates.ts`.

### Invariants

1. Normal-mode public copy uses the neutral River Ledger identity; abstract units stay abstract (§10, `RL-UI-NOCASINO-001`).
2. No behavior or layout change — copy only (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — optional assertion that a representative neutral term renders.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `node apps/web/e2e/river-ledger.smoke.mjs`
3. The UI/e2e smokes plus a banned-vocabulary grep are the correct boundary; this is a copy-only pass with no Rust surface.

## Outcome

Completed: 2026-06-15

Changes:
- Replaced normal-mode River Ledger public `Pot` copy with neutral `Ledger` / `Ledger total` wording in metrics, screen-reader status text, and outcome breakdown rows.
- Confirmed River Ledger template copy remains ledger/revealed-hand focused after prior tickets.

Verification:
- `rg -n "\\b(Pot|pot|casino|chip|chips|cash|money|wager|rake|pool)\\b" apps/web/src/components/RiverLedgerBoard.tsx apps/web/src/components/outcomeExplanationTemplates.ts`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
- `git diff --check`

Notes:
- The banned-vocabulary grep returned only Poker Lite `pool` template copy in `outcomeExplanationTemplates.ts`; it returned no River Ledger normal-mode public-copy hits.
- `apps/web/scripts/smoke-ui.mjs` remained unchanged because this was a copy-only browser surface audit. The rendered River Ledger board stayed covered by `river-ledger.smoke.mjs`.
