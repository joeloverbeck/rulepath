# RIVLEDSHOSEA-009: Contain and center River Ledger card suit text

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/styles.css`; `apps/web/src/components/RiverLedgerCard.tsx` (only if a minimal wrapper is required)
**Deps**: None

## Problem

`RiverLedgerCard` renders the correct neutral content (rank text + suit glyph + full suit word), but its compact box does not form a strict containment contract: rank and suit are centered independently rather than as a deliberate two-row composition, and the suit text has no bounded inline size, so the longest word (`diamonds`) can extend past the private card's right edge and the group does not read as optically centered. This is a surgical CSS/layout containment task that must not change card identity, public labels, data shape, or visual language (spec §10 / D7).

## Assumption Reassessment (2026-06-18)

1. `apps/web/src/components/RiverLedgerCard.tsx` renders `<strong>{card.label}</strong>`, a `.river-ledger-card-suit` span containing the glyph (`<b aria-hidden>`) and the full suit word (`<small>{card.suit}</small>`), and `.river-ledger-card-rank`. Confirmed.
2. `apps/web/src/styles.css`: `.river-ledger-card` is `display: grid; align-content: center; gap; padding: 12px` with a fixed `min-height`; `.river-ledger-card-suit` is `display: inline-flex; align-items/justify-content: center`. `box-sizing: border-box` is already applied globally (`* { box-sizing: border-box }`), so the card inherits it — the spec's "set box-sizing" step is mostly satisfied; the missing piece is bounded inline sizing and deliberate two-row centering. Confirmed.
3. Shared boundary under audit: the `.river-ledger-card` containment contract across its `board`, `private`, and `showdown` tones. End state: rank/glyph/suit-word stay within the card border box for all four suits at supported sizes; containment is applied to all tones so one context does not regress another.
4. FOUNDATIONS §7 (polished, accessible public UI) + §10 IP-conservatism: the full neutral suit word stays in visible text and the accessibility name (color-independent redundancy); no abbreviation, truncation, ellipsis, glyph-only fallback, new art, or casino treatment. Restated before trusting the spec — clipping the suit word is not an acceptable fix.

## Architecture Check

1. A two-row grid/contained-flex column with `place-items: center` plus `min-width: 0`/`max-inline-size: 100%` on children is the minimal primitive set that contains the longest word without changing the component's visual identity — preferable to abbreviating or restructuring markup.
2. No shim: containment is applied to the real card classes/tones; overflow containment is only a safety net, not the normal-render mechanism.
3. Presentation-only CSS; no `engine-core`/Rust/data change. A CSS-only repair must change no golden trace.

## Verification Layers

1. No overflow for any suit/tone -> `river-ledger.smoke.mjs` bounding-box check: no child extends beyond the card's inline edge in private, board, and showdown tones for `clubs`/`diamonds`/`hearts`/`spades`.
2. Reflow without loss -> e2e at 200% text zoom and 320 CSS-px width: every suit word stays visible (no clip/ellipsis/disappearance).
3. No data churn -> grep/trace check: no golden trace changes; card IDs, `card.suit`, accessibility labels, and tones are unchanged.

## What to Change

### 1. `styles.css` — containment + centering

Apply a two-row grid (or contained flex column) with `place-items`/`align-items: center`; give rank and suit children `min-width: 0` and `max-inline-size: 100%`; make the suit group occupy the available inline width and center its glyph/word; add a bounded responsive suit-word font token + line-height that keeps all four full English suit words inside the card at supported sizes; use overflow containment only as a safety net. Apply to `board`, `private`, and `showdown` tones.

### 2. `RiverLedgerCard.tsx` — minimal wrapper only if required

Touch the component only if a minimal wrapper element is needed for alignment; otherwise leave markup and public vocabulary unchanged.

## Files to Touch

- `apps/web/src/styles.css` (modify)
- `apps/web/src/components/RiverLedgerCard.tsx` (modify; only if a minimal alignment wrapper is required)

## Out of Scope

- Any card redesign, abbreviated rank system, new art/suit icons, card-ratio change, or casino styling.
- Changing card identity, public labels, `card.suit`/data shape, or accessibility text.
- Any Rust, WASM, or golden-trace change (a trace change here signals accidental churn and must be investigated).
- River Ledger showdown/seat work (RIVLEDSHOSEA-001..008).

## Acceptance Criteria

### Tests That Must Pass

1. `clubs`, `diamonds`, `hearts`, `spades` remain fully visible and centered in every River Ledger card tone; no child extends beyond the card border box at normal settings, 200% text resize, supported narrow viewport, and reduced motion.
2. Full suit word remains in DOM + accessibility name (no glyph-only/color-only dependency); appearance, tones, card IDs, and data payloads unchanged.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e` green; `a11y-noleak.smoke.mjs` confirms the layout change exposes no hidden cards in observer mode.

### Invariants

1. Rank, glyph, and suit word stay within the card's inline border box for all four suits across supported sizes.
2. No golden trace or card data payload changes for this CSS-only repair.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` — card + child bounding-rectangle checks in private, board, and showdown tones, plus a four-suit fixture at 200% zoom / 320px width.

### Commands

1. `npm --prefix apps/web run smoke:e2e` (River Ledger card bounding-box + resize assertions).
2. `npm --prefix apps/web ci && npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
3. `cargo run -p replay-check -- --game river_ledger --all` as a negative check — it must stay green/unchanged, proving the CSS repair caused no data churn.
