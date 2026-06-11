# GAT12FLOWATCOO-008: Public projection, visibility, and no-leak helpers

**Status**: ACCEPTED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/flood_watch/src/visibility.rs` (single public projection, effect filtering, no-leak helpers), `src/effects.rs` (viewer-scoped payloads), action/effect/view hashes
**Deps**: GAT12FLOWATCOO-007

## Problem

`flood_watch` has a single public projection — both seats and observers receive the same view — and exactly one piece of hidden information: the undrawn event-deck order, hidden from everyone. This ticket builds the viewer-safe projection (districts, levees, budget, roles, drawn cards, forecast-if-revealed, remaining-composition counts, deck size, turn/phase, terminal outcome), the per-viewer effect filtering, the no-leak helpers, and the stable action/effect/view hashes. A card's identity first appears only in its `ForecastRevealed` or `EventDrawn` effect.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-004 keeps the undrawn `event_deck` order internal-only and exposes `remaining_composition` as derived; GAT12FLOWATCOO-006/007 emit the effect set including `Terminal`. `games/masked_claims/src/visibility.rs` is the verified exemplar for viewer-scoped projection + effect filtering + no-leak helpers; `flood_watch` is simpler (no per-seat private view — Assumption A7).
2. The spec (§Implementation reference "Visibility and no-leak model", Work-breakdown item 7, Assumption A7) fixes: one public projection identical for both seats and observers; hidden-from-everyone undrawn-deck order; `remaining_composition` (scenario counts − drawn) is public derived data and not a leak; a card's identity first appears in its `ForecastRevealed`/`EventDrawn` effect. Action/effect/view hashes must be stable and deterministic.
3. Cross-artifact boundary under audit: the public/private view contract (`engine-core` `VisibilityScope`, public/private view) is consumed by the WASM `get_view` (GAT12FLOWATCOO-014), the bots' allowed view (GAT12FLOWATCOO-010), and the no-leak tests + golden traces (GAT12FLOWATCOO-011). The projection shape is a serialization contract — its field order and hashing must be deterministic so view hashes are replay-stable.
4. FOUNDATIONS §11 (public/private views are viewer-safe; hidden information does not leak through payloads, DOM, logs, previews, diagnostics, effect logs, bot explanations, candidate rankings, UI test IDs, or replay exports) and the §12 stop condition "hidden information reaches browser payloads…" motivate this ticket: the projection is the no-leak firewall, and it must be impossible to derive the undrawn order from any public field.
5. Enforcement surface: this is the §11 no-leak firewall itself. The projection is built from the public state only; the internal deck order is never read into a view or effect payload. Effect filtering ensures `EventDrawn`/`ForecastRevealed` are the sole reveal points. The deterministic-hash surface (§11/§2) requires stable serialization order for action/effect/view hashes.

## Architecture Check

1. A single public projection (no per-seat private view) is the correct model for a fully-public cooperative game — it removes the per-seat filtering branch entirely, so the only no-leak concern is the undrawn-deck order, which is structurally absent from the projection input.
2. No backwards-compatibility aliasing/shims; built on GAT12FLOWATCOO-004/007 state and effects.
3. `engine-core` stays noun-free — projection uses the generic `VisibilityScope`/public-view contract; all district/deck/forecast fields are game-local.

## Verification Layers

1. Single viewer-safe projection -> no-leak visibility test: both seats and observers receive identical views; no undrawn-deck order or identity in views, action trees, previews, diagnostics, effect payloads, public effect text, or command summaries.
2. Reveal-only-on-draw/forecast -> no-leak test: a card identity appears first in its `EventDrawn`/`ForecastRevealed` effect and nowhere earlier.
3. Public derived composition is not a leak -> rule test: `remaining_composition` equals scenario counts minus drawn and exposes no order.
4. Deterministic hashes -> deterministic replay-hash check: action-tree, effect, and view hashes reproduce across runs with stable serialization order.

## What to Change

### 1. `games/flood_watch/src/visibility.rs`

Build the single public projection (districts/levees/budget/roles/drawn/forecast-if-revealed/remaining-composition/deck-size/turn/phase/terminal). Implement per-viewer effect filtering (all viewers get the same public effects), the no-leak helpers, and the stable action/effect/view hashing with deterministic serialization order. Add safe waiting metadata pass-through for the teammate (from GAT12FLOWATCOO-005's tree).

### 2. `games/flood_watch/src/effects.rs`

Finalize viewer-scoped effect payloads: public fields only, drawn-card identity in `EventDrawn`/`ForecastRevealed`, no undrawn-deck data in any payload or public effect text.

## Files to Touch

- `games/flood_watch/src/visibility.rs` (modify — fill the stub)
- `games/flood_watch/src/effects.rs` (modify — viewer-scoped payloads / public effect text)

## Out of Scope

- Replay support, internal full trace with deck order, and viewer-scoped export/import (GAT12FLOWATCOO-009) — this ticket builds the live projection; export is there.
- WASM `get_view` wiring (GAT12FLOWATCOO-014).
- Bot view consumption (GAT12FLOWATCOO-010).

## Acceptance Criteria

### Tests That Must Pass

1. Visibility/no-leak tests search public views, action trees, previews, diagnostics, effect payloads, public effect text, and command summaries for undrawn-deck order/identities and find none.
2. A test asserts both seats and observers receive an identical projection (no per-seat private content).
3. A deterministic-hash test asserts action/effect/view hashes reproduce under identical inputs.

### Invariants

1. The undrawn event-deck order never appears in any view, tree, preview, diagnostic, or effect payload; reveals happen only via `EventDrawn`/`ForecastRevealed`.
2. View/effect/action hashes are deterministic with stable serialization order.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/visibility.rs` — single-projection equality + no-leak negative searches (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/replay.rs` — action/effect/view hash determinism (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test visibility`
2. `cargo test -p flood_watch`
3. The browser DOM/storage no-leak assertions run at GAT12FLOWATCOO-018; the Rust no-leak visibility tests are the correct boundary for the projection diff.

## Outcome

Accepted on 2026-06-11. Implemented Flood Watch's single public projection,
public effect filtering, public effect text, no-leak helper, and deterministic
action/effect/view hash helpers. The projection exposes public district state,
roles, budget/phase, drawn event kinds, forecasted event kind when revealed,
remaining composition counts, undrawn count, turn/freshness, and shared
terminal outcome while omitting the undrawn deck order and card copy IDs. Tests
cover identical observer/seat views, no hidden event identities in views,
action trees, diagnostics, or unrevealed effect text, and deterministic hashes.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p flood_watch --test visibility`
3. `cargo clippy -p flood_watch --all-targets -- -D warnings`
4. `cargo test -p flood_watch`
