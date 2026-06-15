# RIVLEDSHO-006: Neutral River Ledger card component (glyph + suit word + accessible label)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerCard.tsx` (new), `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: RIVLEDSHO-004

## Problem

Board, hole cards, and best-five render as plain text blocks ("CLUBS / 4C / four") with no suit glyphs, no red/black, no card-like visuals, and no group accessibility label. This ticket builds a neutral, original River Ledger card component (rank large + suit glyph + suit word, colorblind-safe, accessible labels) and adopts it for the board, hole cards, and best-five (spec WB6 / D4).

## Assumption Reassessment (2026-06-15)

1. Verified against current code: `RiverLedgerBoard.tsx` renders card markup inline (no separate card component); the component consumes the Rust `CardView` (`games/river_ledger/src/visibility.rs:45`, mirrored in `apps/web/src/wasm/client.ts`). No existing `River*Card` component — the new filename is collision-free.
2. Verified against specs/docs: spec §6 D4 + §8 WB6; `docs/IP-POLICY.md` + `RULES.md` `RL-UI-NOCASINO-001` (no casino trade dress); `games/river_ledger/docs/UI.md` §Accessibility And Motion.
3. Cross-artifact boundary under audit: the component consumes the Rust `CardView` and is adopted by the board, hole-card, and best-five render sites; it derives no rank/suit text from raw IDs.
4. FOUNDATIONS §7 (cozy, not casino) + §10 (IP conservatism) motivate this ticket: original neutral card styling, suit glyph + suit word, red/black as a secondary aid only — color is never the sole information channel (WCAG 1.4.1).

## Architecture Check

1. A single local River-Ledger card component (rather than per-site inline markup) gives the board, hole cards, and best-five one consistent, accessible, IP-clean rendering; it stays River-Ledger-local — no shared-surface/`game-stdlib` promotion (that would route through `docs/MECHANIC-ATLAS.md` and a separate spec).
2. No backwards-compatibility aliasing/shims; inline card markup is replaced by the component.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — `apps/web` presentation only (a web component may know mechanic nouns; the kernel stays noun-free).

## Verification Layers

1. Cards render rank + suit glyph + suit word with a group-level accessible label per board/best-five group -> `npm --prefix apps/web run smoke:ui`.
2. No casino trade dress (no felt, chips, currency, copied card art) -> manual IP-conservatism review (§10).
3. Suit/winner conveyed by text + glyph, not color alone; contrast meets WCAG AA -> manual accessibility review (§7).

## What to Change

### 1. `apps/web/src/components/RiverLedgerCard.tsx` (new)

A neutral card component consuming `CardView`: rank large, suit glyph + suit word, neutral high-contrast surface, red/black secondary only, and a group-level accessible-label helper for best-five/board groups.

### 2. `apps/web/src/components/RiverLedgerBoard.tsx`

Adopt the component for the public board, the viewer's hole cards, and (via the panel) the best-five render sites; remove the inline text-block card markup.

## Files to Touch

- `apps/web/src/components/RiverLedgerCard.tsx` (new)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- The showdown panel layout (RIVLEDSHO-004).
- The hand-ranking reference (RIVLEDSHO-007).
- Action-panel / seat / turn-flow affordances (RIVLEDSHO-008/009).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` — board/hole/best-five render via the component with accessible group labels.
2. `npm --prefix apps/web run smoke:e2e` — `river-ledger.smoke.mjs` still green with the new card markup.
3. `npm --prefix apps/web run build` — component type-checks against `CardView`.

### Invariants

1. The component derives all rank/suit text from the Rust `CardView`, never from raw IDs (§2).
2. No casino trade dress; color is not the sole information channel (§10, WCAG 1.4.1).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — assert card glyph + suit word + accessible label.
2. `apps/web/e2e/river-ledger.smoke.mjs` (modify, as surfaced) — board/best-five render unchanged-or-better with no new leak.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`
3. `smoke:ui` plus manual IP/accessibility review is the correct boundary; behavioral correctness is unaffected (presentation-only).
