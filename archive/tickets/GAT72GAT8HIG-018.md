# GAT72GAT8HIG-018: Web UI HighCardDuelBoard + shell integration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/HighCardDuelBoard.tsx` (new), `apps/web/src/components/AppShell.tsx`, `apps/web/src/components/GamePicker.tsx`
**Deps**: GAT72GAT8HIG-017

## Problem

Gate 8 needs a polished, accessible, responsive High Card Duel UI integrated
into the app shell: a viewer selector (Seat 0 / Seat 1 / Observer), safe hotseat
switching, own-hand-only visibility, face-down opponent commitments until reveal,
reveal animation driven by Rust effects, reduced-motion support, and a dev panel
that exposes no hidden state — with no hidden ids leaking through DOM/labels/
paths/test-ids/CSS/console.

## Assumption Reassessment (2026-06-07)

1. Verified the renderer convention: sibling boards
   `apps/web/src/components/DraughtsLiteBoard.tsx` (and `ColumnFourBoard`,
   `DirectionalFlipBoard`, etc.); the shell wires boards by `game_id` via the
   catalog in `AppShell.tsx`/`GamePicker.tsx`. The viewer-mode plumbing landed in
   GAT72GAT8HIG-017 (`client.ts`/`shellReducer.ts`).
2. Verified against the spec: §4.2.6 fixes the required UX (viewer selector,
   hotseat, observer-as-watcher, own-hand-only, opponent backs/count, face-down
   until reveal, effect-driven reveal animation, reduced-motion, keyboard/focus,
   responsive) and the dev-panel/no-leak constraints; visual design must be
   original/neutral (no casino/poker/blackjack motifs, §10).
3. Cross-artifact boundary under audit: the renderer↔shell wiring and the
   semantic-effect→animation contract (§7). Animation is driven by Rust-emitted
   `hcd_*` effects, not guessed state diffs.
4. FOUNDATIONS principle under audit (§7 public UI play-first + §11 no-leak):
   legal actions come only from the Rust action tree; animation is effect-driven;
   no hidden id reaches DOM/labels/paths/test-ids/CSS/console; dev panel shows
   only public-safe state.
5. Enforcement surface named: the §11 no-leak firewall on browser-visible
   surfaces. Confirm pending action controls, accessible labels, and the dev
   panel carry no hidden ids; the reveal animation is triggered by
   `hcd_cards_revealed`, not by a renderer diff. Proven by the e2e no-leak smoke
   (019).

## Architecture Check

1. A dedicated `HighCardDuelBoard` consuming viewer-safe payloads + Rust effects
   keeps presentation declarative and the firewall upstream in Rust — cleaner
   than a board that derives reveal state locally.
2. No backwards-compatibility shims — new component + additive shell wiring.
3. No engine/`game-stdlib` change; original neutral visuals only (CSS/geometric
   marks/local SVG), no proprietary trade dress (§10).

## Verification Layers

1. Effect-driven reveal -> manual review + e2e (019): reveal animation triggers on `hcd_cards_revealed`, settling to the latest viewer-safe view.
2. Own-hand-only -> no-leak visibility test (e2e 019): Seat 0 sees only Seat 0's hand; observer sees counts/backs.
3. DOM/label/test-id no-leak -> no-leak visibility test (e2e 019): no `hcd:r..`/hidden token in DOM/attributes/CSS/console.
4. Accessibility + reduced-motion -> manual review + a11y scan (019): keyboard/focus path works; reduced-motion simplifies animation; labels leak no hidden facts.

## What to Change

### 1. `HighCardDuelBoard.tsx`

Render viewer-safe view + effects: own hand (current seat only), opponent
count/backs, face-down commitments until reveal, effect-driven reveal animation
with reduced-motion path, score/round/phase/roles/terminal, viewer selector +
hotseat, keyboard operation, responsive layout. Legal actions only from the Rust
action tree; controls carry no hidden ids.

### 2. Shell integration

Wire the board by `game_id` in `AppShell.tsx`; add `High Card Duel` to
`GamePicker.tsx` via the catalog. Dev panel (if shown) exposes only viewer/phase/
public score/effect cursor/public-safe availability.

## Files to Touch

- `apps/web/src/components/HighCardDuelBoard.tsx` (new)
- `apps/web/src/components/AppShell.tsx` (modify)
- `apps/web/src/components/GamePicker.tsx` (modify)

## Out of Scope

- e2e/a11y/no-leak smoke + CI (GAT72GAT8HIG-019).
- Any Rust/WASM behavior (legality/views/effects all consumed from 016).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` — type-checks and builds with the new board.
2. `npm --prefix apps/web run smoke:ui` — the shell renders with the game selectable.

### Invariants

1. Legal actions come only from the Rust action tree; animation is effect-driven (§7).
2. No browser-visible surface (DOM/attr/CSS/test-id/console/dev-panel) carries hidden ids (§11) — asserted in 019.

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/HighCardDuelBoard.tsx` — exercised by `smoke:ui` and the e2e smoke (019).

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. Build + UI smoke are the correct boundary here; the dedicated no-leak/a11y assertions live in the e2e ticket (019).
