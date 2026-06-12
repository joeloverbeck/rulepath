# ACTCONMAT-007: Deep-detail tier + status-copy clarity

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier` + `games/flood_watch` (`cards_presentation.toml` `details` parallel array + loader, `CardFaceView` projection, status/threshold copy); `apps/web/src/wasm/client.ts` (`CardFaceView.details`), `apps/web/src/components/DeckFlowPanel.tsx` (render the disclosure).
**Deps**: ACTCONMAT-001

## Problem

Card "Details" disclosures repeat the one-line summary verbatim — the detail tier is a no-op. Edicts never state their precise scope ("Survey Ban" says "limits survey operations" but not which operations at which sites), so the rule is unlearnable in-app. Status copy also assumes rulebook knowledge ("3 sites needed", "Reckoning 1 resolved: none." — none of *what*?). Cards and edicts must explain themselves fully on demand, and status copy must frame its thresholds.

## Assumption Reassessment (2026-06-12)

1. `CardFaceView` (`apps/web/src/wasm/client.ts:958-964`) has `id, label, summary, family, accessibility_label` — no `details`. `cards_presentation.toml` is parallel string arrays (`card_ids, labels, summaries, families, accessibility_labels`), loaded in `games/event_frontier/src/ui.rs`/`cards.rs`. `DeckFlowPanel.tsx` renders the card face via `CardFaceView`. EF has 21 cards (confirmed `card_ids`).
2. Spec D7 / §4.2: add an optional `details` prose field (a new parallel array) to the presentation schema (`cards_presentation.toml`, `CardFaceView`); the Details disclosure renders it when present and is omitted when absent. EF authors details for all 21 cards (edicts state precise scope); `flood_watch` reaches parity. Status-copy clarity pass (O8): victory-threshold framing + reckoning-resolution copy.
3. Cross-artifact boundary under audit: the card-presentation static-data manifest and `CardFaceView` projection (Rust → wasm → TS), plus the status/threshold copy in `visibility.rs`/board panels. Shared file `apps/web/src/wasm/client.ts` is also touched by ACTCONMAT-002 (metadata types) and ACTCONMAT-009 (variants) — coordinate the mechanical merge.
4. FOUNDATIONS §5 (static data is content): `details` prose is inert typed content keyed to Rust card IDs — the sanctioned category; the loader rejects unknown fields and refuses behavior-looking fields.
5. Schema extension: `cards_presentation.toml` gains a `details` parallel array and `CardFaceView` gains an optional `details` field. Consumers: `DeckFlowPanel.tsx` (renders the disclosure). Additive — the disclosure is omitted when `details` is absent, so no consumer breaks.

## Architecture Check

1. A parallel `details` array reuses the existing `cards_presentation.toml` shape (no new file, no row restructuring) and an optional `CardFaceView` field renders through the existing Details disclosure — minimal surface for real deep content. Omitting the disclosure when absent removes the no-op rather than aliasing it.
2. No shim: the no-op disclosure is replaced by present-or-absent rendering; no compatibility flag.
3. `engine-core` untouched — card/detail data is `games/*` content (§3/§5). No `game-stdlib` promotion.

## Verification Layers

1. `details` renders when present, disclosure omitted when absent -> UI smoke (`apps/web/e2e/event-frontier.smoke.mjs`).
2. Loader rejects unknown/behavior-looking fields -> schema validation (fail-closed) unit test.
3. Every EF edict states precise scope; `flood_watch` parity -> codebase grep-proof of the `details` array coverage (21 EF cards).
4. Status/threshold copy framed -> UI smoke assertion on victory-threshold and reckoning-resolution strings.

## What to Change

### 1. details schema field + loader

Add a `details` parallel array to `games/event_frontier/data/cards_presentation.toml` and `games/flood_watch/data/cards_presentation.toml`; extend the typed loader (`ui.rs`/`cards.rs`) and the `CardFaceView` projection.

### 2. CardFaceView.details + disclosure

Add optional `details` to `CardFaceView` (`client.ts`); render it in the `DeckFlowPanel.tsx` Details disclosure, omitting the disclosure when absent.

### 3. Author EF + flood_watch details

Author detail prose for all 21 EF cards (edicts state precise scope, e.g. "Blocks Survey and Rally at contested sites — sites holding both agents and settlers — until the next Reckoning."); `flood_watch` reaches parity.

### 4. Status-copy clarity (O8)

Frame victory-threshold copy and reckoning-resolution copy in `visibility.rs`/board panels ("3 sites needed" → framed; "Reckoning 1 resolved: none." → clarified).

## Files to Touch

- `games/event_frontier/data/cards_presentation.toml` (modify; `details` array)
- `games/flood_watch/data/cards_presentation.toml` (modify; parity)
- `games/event_frontier/src/ui.rs` (modify; loader) and/or `games/event_frontier/src/cards.rs` (modify)
- `games/event_frontier/src/visibility.rs` (modify; `CardFaceView` details + status/threshold copy)
- `games/flood_watch/src/` (modify; details loader + CardFaceView parity)
- `apps/web/src/wasm/client.ts` (modify; `CardFaceView.details`)
- `apps/web/src/components/DeckFlowPanel.tsx` (modify; render the disclosure)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; details + status-copy assertions)

## Out of Scope

- Reserved action-metadata keys (ACTCONMAT-002) — this is card/edict detail prose.
- Rules-page (HOW-TO-PLAY) content (ACTCONMAT-008).
- Any behavior-looking field in the detail data (§5) — prose only.

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` asserts card/edict Details disclosures show authored deep detail or are absent, and every EF edict states its precise scope.
2. Loader rejects an unknown field in the `details` data (fail-closed) — unit test.
3. `cargo test -p event_frontier && cargo test -p flood_watch` and `npm --prefix apps/web run smoke:e2e` green.

### Invariants

1. `details` is inert typed content keyed to Rust card IDs (§5); no selectors/branches/triggers.
2. The disclosure renders authored detail or is omitted — never the summary verbatim.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/` + `games/flood_watch/tests/` — `details` projection + unknown-field rejection.
2. `apps/web/e2e/event-frontier.smoke.mjs` — disclosure render + status-copy framing.

### Commands

1. `cargo test -p event_frontier && cargo test -p flood_watch`
2. `cargo run -p fixture-check -- --game event_frontier && cargo run -p fixture-check -- --game flood_watch`
3. `npm --prefix apps/web run smoke:e2e`
