# GAT192MELLEDSET-003: Web settlement panel + retire effects-buffer capture

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/MeldfallLedgerBoard.tsx`, `apps/web/src/styles.css`, `apps/web/e2e/meldfall-ledger.smoke.mjs`, `apps/web/e2e/a11y-noleak.smoke.mjs`
**Deps**: GAT192MELLEDSET-002

## Problem

The "Last round settled" panel currently reconstructs settlement data by parsing
the transient `round_score` effect out of a 12-entry effects buffer
(`parseRoundSettlement`, `MeldfallLedgerBoard.tsx:53-109`), which only carries the
net delta and scrolls out before a human can read it under bot autoplay. With
`view.last_settlement` now projected (GAT192MELLEDSET-001/002), the renderer can
read the persistent, complete breakdown directly. This ticket rewrites the panel
to render from `view.last_settlement` — the round-end reason and a per-seat
tabled-positive / in-hand-penalty composition of the delta — and retires the
effects-buffer capture heuristic (spec §3.1.4, §4, §8).

## Assumption Reassessment (2026-06-27)

1. `MeldfallLedgerBoard.tsx` holds the effects-buffer capture to retire: the local
   `RoundSettlement` TS type (line 38), `parseRoundSettlement` (line 53), the
   `useState`/`useEffect` capture wiring (lines 95-109), and `SettlementSummary`
   (lines 355-384) which currently reads `settlement.deltas` / `settlement.cumulative`
   only. `roundEndLabel(view.round_end)` (line 188) already renders the reason from
   the live view; the persistent panel must instead read it from
   `view.last_settlement` so it survives the auto-deal.
2. `view.last_settlement` (`MeldfallLedgerSettlementView | null`) and its fields are
   delivered by GAT192MELLEDSET-002 (`apps/web/src/wasm/client.ts`); the CSS classes
   (`meldfall-settlement*`) live in `apps/web/src/styles.css` (not a component-local
   stylesheet). The web smokes `apps/web/e2e/meldfall-ledger.smoke.mjs` and
   `apps/web/e2e/a11y-noleak.smoke.mjs` already exist and exercise the board.
3. Cross-artifact boundary under audit: the renderer consumes the
   `MeldfallLedgerPublicView.last_settlement` contract (producer:
   GAT192MELLEDSET-002). This ticket is the canonical end-state read path; the
   effects-buffer transport for settlement is removed entirely, so there is no
   longer a second lawful path for the same fact (`tickets/README.md`
   pre-implementation check 8).
4. FOUNDATIONS §2 / `ML-UI-001` restated: TypeScript renders Rust-authored values
   only — no settlement math (tabled/penalty/delta composition is already computed
   in Rust). The renderer formats and lays out; it computes no score.
5. §11 no-leak firewall on the browser side (`ML-UI-003`): DOM text, a11y names,
   `data-testid`, storage, and console logs must not contain any unauthorized
   hidden card identity or stock order as a result of the new panel content. The
   `a11y-noleak` smoke must continue to pass with the breakdown rendered; the new
   panel shows only `ML-VIS-006` totals/counts.

## Architecture Check

1. Reading the persistent Rust-owned `view.last_settlement` is cleaner and more
   robust than parsing a bounded effects buffer: it removes a presentation-side
   reconstruction heuristic, eliminates the scroll-out race, and surfaces the full
   breakdown the data already contains — with zero settlement logic in TS.
2. No backwards-compatibility shim: the effects-buffer `RoundSettlement` type and
   `parseRoundSettlement` are deleted outright, not aliased; the panel reads one
   canonical source.
3. `engine-core` untouched; no `game-stdlib` change; the renderer stays
   presentation-only and decides no legality.

## Verification Layers

1. Render fidelity: the panel shows the round-end reason and each seat's
   tabled-positive / in-hand-penalty / delta / cumulative / rank from
   `view.last_settlement` -> `meldfall-ledger.smoke.mjs` assertion on the rendered
   breakdown.
2. Persistence: the panel still shows the prior settlement after the next round is
   auto-dealt (the scroll-out race is gone) -> e2e smoke captures the panel across a
   deal.
3. No-leak: no forbidden term (hidden card identity / stock order) appears in DOM
   text, a11y names, `data-testid`, storage, or console -> `a11y-noleak.smoke.mjs`.
4. Heuristic retired: `parseRoundSettlement` and the effects-buffer `RoundSettlement`
   capture no longer exist -> grep-proof in `MeldfallLedgerBoard.tsx`.

## What to Change

### 1. Render from `view.last_settlement` (`MeldfallLedgerBoard.tsx`)

Rewrite `SettlementSummary` to read `view.last_settlement`: render the round-end
reason and, per seat, a row showing `tabled_positive` and `in_hand_penalty`
composing `delta`, alongside the existing cumulative / leader presentation. Drive
the persistent panel's visibility and round number from `view.last_settlement`
(null before any settlement).

### 2. Retire the effects-buffer capture (`MeldfallLedgerBoard.tsx`)

Delete the local `RoundSettlement` type, `parseRoundSettlement`, and the
`useState`/`useEffect` settlement-capture wiring. Remove the now-unused `round_score`
parsing path (keep any unrelated effects handling, e.g. table-change animation
cues, intact).

### 3. Panel styles (`styles.css`)

Extend the `meldfall-settlement*` classes as needed for the per-seat
tabled/penalty/delta row layout; no new design system, restrained per §7 cozy-table
guidance.

### 4. Smoke assertions (`meldfall-ledger.smoke.mjs`, `a11y-noleak.smoke.mjs`)

Assert the breakdown renders (round-end reason + per-seat tabled/penalty/delta) and
that no forbidden term leaks into DOM/a11y/testid/storage/console.

## Files to Touch

- `apps/web/src/components/MeldfallLedgerBoard.tsx` (modify) — render from `view.last_settlement`; delete effects-buffer capture
- `apps/web/src/styles.css` (modify) — per-seat breakdown row layout
- `apps/web/e2e/meldfall-ledger.smoke.mjs` (modify) — breakdown-render + persistence assertions
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify) — no-leak assertion over the new panel content

## Out of Scope

- The Rust projection / WASM bridge / TS type — GAT192MELLEDSET-001/002.
- Docs reconciliation + `specs/README.md` `Done`-flip — GAT192MELLEDSET-004.
- Any settlement computation in TypeScript (`ML-UI-001`, §2).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — UI smoke green with the panel sourced from `view.last_settlement`.
2. `node apps/web/e2e/meldfall-ledger.smoke.mjs` — breakdown + persistence assertions pass.
3. `node apps/web/e2e/a11y-noleak.smoke.mjs` — no forbidden term leaks.
4. `npm --prefix apps/web run build` — TypeScript build clean.

### Invariants

1. The panel renders only Rust-authored `view.last_settlement` values; no
   settlement math runs in TypeScript.
2. `parseRoundSettlement` and the effects-buffer `RoundSettlement` capture are
   removed; the panel reads one canonical source.
3. No hidden card identity or stock order reaches DOM/a11y/testid/storage/console.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/meldfall-ledger.smoke.mjs` — assert the rendered round-end reason
   and per-seat tabled/penalty/delta breakdown, and panel persistence across an
   auto-dealt round.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — extend the no-leak negative assertions to
   cover the new panel content.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `node apps/web/e2e/meldfall-ledger.smoke.mjs && node apps/web/e2e/a11y-noleak.smoke.mjs`
3. `npm --prefix apps/web run build`
4. The targeted e2e smokes are the correct boundary for render + no-leak; the build
   guards the TS contract.
