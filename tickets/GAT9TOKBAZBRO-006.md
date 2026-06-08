# GAT9TOKBAZBRO-006: Public-view projection (visibility.rs) + UI metadata (ui.rs)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/src/visibility.rs` (new), `src/ui.rs` (new), `src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-005

## Problem

The browser renders only Rust-projected, viewer-safe views. This ticket builds the
public-view projection (inventories, public supply, market slots with contract
label/cost/points/empty-state, scores, turn count, active seat, recent effects)
and the viewer-facing UI metadata (`ui.rs`: labels, accessibility labels, preview
copy). Token Bazaar is fully public, so observer and seat views are identical —
but the projection must still flow through the normal viewer/effect boundary so
the no-leak harness and future hidden-state work are not regressed, and it must
assert no debug-only/candidate field is exposed.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/state.rs` + `effects.rs` (GAT9TOKBAZBRO-003/005)
   provide the state and effects this projection reads. The sibling
   `games/high_card_duel/src/visibility.rs` + `src/ui.rs` establish the house
   pattern (verified present) — note `high_card_duel` redacts private cards, which
   Token Bazaar does not need, but the projection seam is reused. `src/lib.rs`
   modified to add `mod visibility; mod ui;`.
2. The view contents are fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Players and visibility" (all state public; observer and seat viewers see the
   same state) and the browser "Main board shows…" list, plus "Action-tree previews
   and metadata" (labels/accessibility labels owned by the game projection).
3. Cross-artifact boundary under audit: the public/private view contract from
   `docs/ARCHITECTURE.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md`. The view shape is
   consumed by replay public-view hashing (-007), visibility tests (-009), WASM
   `get_view` (-013), and the React board (-015). It must conform to the engine
   view contract.
4. FOUNDATIONS §11 (viewer-safe views; no-leak firewall) is the motivating
   invariant even though the game is public: the projection must carry only
   presented public state, and `ui.rs` metadata must not embed bot candidate
   tables, debug scores, or internal valuation. The no-leak test (-009) asserts
   this across payload/DOM/effect-log/bot-rationale surfaces.
5. No-leak firewall surface: this ticket is where view content is decided, so it
   is the enforcement point. Confirm the projection has no path to an internal-only
   field; since all state is public there is nothing to redact, but a debug/
   candidate field must not be introduced into the view or `ui.rs` metadata.
6. Public/private view schema: the projection emits a public view object; consumers
   listed above. The view is additive (new game), and its field set is enumerated
   here so the visibility test, WASM, and board cover it.

## Architecture Check

1. Co-locating `ui.rs` (viewer-facing Rust metadata: labels/a11y/preview copy)
   with the view projection keeps all viewer-facing output in Rust and the React
   layer presentation-only; this is the `OFFICIAL-GAME-CONTRACT` pipeline shape.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` untouched; the view/metadata types are game-local. No
   `game-stdlib` helper added.

## Verification Layers

1. Public view contains the spec's required fields -> `cargo test -p token_bazaar`
   (visibility unit test asserting inventories/supply/slots/scores/turn/effects).
2. Observer view == seat view (all public) -> visibility unit test comparing both.
3. No-leak: no debug/candidate/internal field in view or `ui.rs` metadata ->
   no-leak assertion (full suite in -009).
4. View conforms to the engine view contract + stable serialization -> schema/
   serialization validation in tests.

## What to Change

### 1. `games/token_bazaar/src/visibility.rs`

`view_for(state, viewer)` projecting the public view: inventories per seat,
public supply, the three slots (contract label + cost chips + points + empty
state), scores, per-seat turn counts, active seat, and a recent-effects view.
Observer and seat viewers yield identical content; route through the normal
viewer/effect boundary.

### 2. `games/token_bazaar/src/ui.rs`

Viewer-facing UI metadata: stable display labels, accessibility labels, and
preview copy keyed to actions/effects/contracts. No debug or valuation data.

### 3. `games/token_bazaar/src/lib.rs` (modify)

Add `mod visibility; mod ui;`; re-export the view + metadata surface.

## Files to Touch

- `games/token_bazaar/src/visibility.rs` (new)
- `games/token_bazaar/src/ui.rs` (new)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- Replay export/import (GAT9TOKBAZBRO-007).
- The React renderer / effect-log component (GAT9TOKBAZBRO-015) — this ticket
  ships only Rust-side viewer metadata.
- The full no-leak/visibility test suite (GAT9TOKBAZBRO-009) — targeted assertions only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — public view exposes every required board field.
2. `cargo test -p token_bazaar` — observer view equals seat view (fully public).
3. `cargo test -p token_bazaar` — no view/metadata field carries debug/candidate data.

### Invariants

1. All viewer-facing content is Rust-projected; TypeScript presents it without
   recomputing state (§2, enforced downstream).
2. No internal/debug/candidate field reaches the view or `ui.rs` metadata (§11
   no-leak firewall), even though the game is public.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/visibility.rs` (unit) — view field coverage + observer==seat.
2. `games/token_bazaar/src/ui.rs` (unit) — label/a11y metadata presence, no debug field.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo build -p token_bazaar && bash scripts/boundary-check.sh`
3. Browser-side no-leak is additionally proved by the e2e smoke (GAT9TOKBAZBRO-016);
   the Rust projection boundary is correctly verified per-crate here.
