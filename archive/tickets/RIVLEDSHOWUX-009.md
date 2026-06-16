# RIVLEDSHOWUX-009: V2 showdown renderer + card-usage visualization

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/OutcomeExplanationPanel.tsx`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: RIVLEDSHOWUX-008

## Problem

The V2 payload (RIVLEDSHOWUX-008) needs a renderer: a River-Ledger V2 branch in the shared `OutcomeExplanationPanel` that leads with the banner + decisive contrast-seat, shows the board once with hole/board card-usage marks, and renders compact ranked standings (winner + closest challenger visible, the rest collapsed). Used cards are marked by text/shape, not colour alone; the other 14 games' outcome rendering is unchanged.

## Assumption Reassessment (2026-06-16)

1. Verified: `OutcomeExplanationPanel.tsx` is a shared catalog surface with an existing River Ledger branch (16 `river_ledger` references); `RiverLedgerCard.tsx` is the local card renderer it imports (a recorded River-specific exception per spec D7).
2. Verified against spec §6 D7 + §8 WB9 (#7, #8); the V2 TS fields land via RIVLEDSHOWUX-008 (hence `Deps`); `RULES.md` `RL-UI-SHOWDOWN-001`.
3. Shared boundary under audit: the shared `OutcomeExplanationPanel` — the V2 layout is a River-Ledger rendering path **keyed to the V2 payload's presence**, so the other 14 games' rendering is unchanged; a cross-game non-regression check guards it.
4. FOUNDATIONS §2 (TS renders Rust-authored fields and sorts only by Rust-provided order — it computes no hand name, usage, order, or "why") + §7 (progressive disclosure, not debug-dominated) + WCAG 1.4.1 (card-usage marked by shape/text, not colour alone) motivate this ticket.

## Architecture Check

1. A V2-payload-presence-keyed River branch (vs restructuring the shared panel for all games) is minimum-blast-radius — the 14 other boards keep their rendering and the V2 layout activates only when the fields are present (mirrors the shipped V1 branch and RIVLEDSHO-004's design).
2. No shims; the V1 River branch is superseded by the V2 branch, not aliased.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — TS presentation only; `RiverLedgerCard` import stays the recorded exception (no new shared coupling).

## Verification Layers

1. V2 panel renders banner → decisive contrast → board-once + usage marks → compact standings (winner+challenger visible, rest collapsed) -> `npm --prefix apps/web run smoke:ui`.
2. Card usage marked by shape/text, not colour alone -> `node apps/web/e2e/outcome-explanation.smoke.mjs` (usage-mark assertion) + manual WCAG review.
3. No regression to the other 14 games' outcome rendering -> `node apps/web/e2e/outcome-explanation.smoke.mjs` + `npm --prefix apps/web run smoke:e2e`.
4. No TS-computed hand name / usage / order / "why" -> grep `OutcomeExplanationPanel.tsx` for evaluator/sort logic (none; sorts only by Rust order fields).

## What to Change

### 1. `apps/web/src/components/OutcomeExplanationPanel.tsx`

Add a `RiverLedgerShowdownV2` subrenderer behind the discriminated V2 payload: banner → decisive reason + named closest challenger → board once with `used_by_selected` marks → winner's five → compact ranked `standings` (winner + challenger expanded, remaining seats collapsed) → `folded_rows` redaction → detail expanders. Supersede the V1 River branch.

### 2. `apps/web/src/components/RiverLedgerBoard.tsx` (card-usage marks)

Render hole/board card-usage marks via `RiverLedgerCard` using shape/ring/text indicators (not colour alone), driven by the Rust `used_in_best_five` / `used_by_selected` flags.

## Files to Touch

- `apps/web/src/components/OutcomeExplanationPanel.tsx` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- The terminal result live-region treatment (RIVLEDSHOWUX-010).
- Table recomposition (RIVLEDSHOWUX-011); any Rust V2 field change (RIVLEDSHOWUX-007/008).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — V2 panel renders banner/standings/usage-marks; type-checks against the RIVLEDSHOWUX-008 types.
2. `node apps/web/e2e/outcome-explanation.smoke.mjs` + `npm --prefix apps/web run smoke:e2e` — the other 14 games' outcome rendering is unchanged.
3. Manual WCAG review — used cards conveyed by shape/text, not colour alone.

### Invariants

1. TypeScript renders only Rust-authored fields and sorts only by Rust order; it computes no hand name, usage, order, or comparison (§2).
2. The V2 layout activates only for River Ledger (V2-payload keyed); no other game regresses (§7).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/outcome-explanation.smoke.mjs` (modify) — V2 banner/standings/usage-mark assertions + cross-game non-regression.
2. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — River Ledger V2 panel render assertion.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `node apps/web/e2e/outcome-explanation.smoke.mjs`
3. `npm --prefix apps/web run smoke:e2e`

## Outcome

Completed on 2026-06-16.

- Added a River Ledger V2 showdown branch in `OutcomeExplanationPanel` keyed to `riverLedgerShowdownV2` payload presence.
- Rendered the Rust-authored banner, decisive reason, closest challenger label, board-once usage section, ranked standings, folded redaction rows, detail rows, and rule references.
- Passed the V2 payload from `RiverLedgerBoard` into `outcomeSurfaceData`.
- Added text/shape card-usage marks (`Used in best five`, `Not used`, outlined used cards) so usage is not color-only.
- Updated outcome and River Ledger smokes for V2 board usage, card counts, folded redaction, and cross-game outcome regression.
- Verified with `cargo fmt --all --check`, `cargo test -p river_ledger`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:ui`, `node apps/web/e2e/outcome-explanation.smoke.mjs`, `node apps/web/e2e/river-ledger.smoke.mjs`, and `npm --prefix apps/web run smoke:e2e`.
